//! Tantivy full-text search index.
//!
//! Provides high-quality full-text search with intelligent CJK tokenization.
//! Indexes plain text extracted from Markdown documents.

use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, Value, STORED, STRING,
};
use tantivy::tokenizer::{LowerCaser, TextAnalyzer};
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument};
use tantivy_jieba::JiebaTokenizer;

use crate::error::{IndexError, Result};

/// Tantivy full-text search index.
pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    writer: Option<IndexWriter>,
    schema: SearchSchema,
}

/// Schema field handles.
#[derive(Clone)]
struct SearchSchema {
    path: Field,
    title: Field,
    content: Field,
}

/// A search result hit.
#[derive(Debug, Clone)]
pub struct SearchHit {
    /// File path (relative to Location root).
    pub path: String,
    /// Document title (if extracted).
    pub title: Option<String>,
    /// Relevance score.
    pub score: f32,
}

/// Statistics from search index operations.
#[derive(Debug, Clone, Default)]
pub struct SearchStats {
    pub documents_added: usize,
    pub documents_removed: usize,
}

impl SearchIndex {
    /// Register custom tokenizers (jieba for CJK + LowerCaser for case-insensitive English).
    fn register_tokenizers(index: &Index) {
        let tokenizer = TextAnalyzer::builder(JiebaTokenizer {})
            .filter(LowerCaser)
            .build();
        index.tokenizers().register("jieba", tokenizer);
    }

    /// Open or create a search index at the given directory.
    pub fn open(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir)?;

        let schema = Self::build_schema();
        let index = if dir.join("meta.json").exists() {
            Index::open_in_dir(dir)?
        } else {
            Index::create_in_dir(dir, schema.schema.clone())?
        };

        Self::register_tokenizers(&index);

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            index,
            reader,
            writer: None,
            schema: schema.fields,
        })
    }

    /// Open an in-memory search index (for testing).
    pub fn open_in_memory() -> Result<Self> {
        let schema = Self::build_schema();
        let index = Index::create_in_ram(schema.schema.clone());

        Self::register_tokenizers(&index);

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        Ok(Self {
            index,
            reader,
            writer: None,
            schema: schema.fields,
        })
    }

    /// Build the Tantivy schema.
    fn build_schema() -> SchemaWithFields {
        let mut schema_builder = Schema::builder();

        // Path field: STRING (indexed for exact match, enables delete_term) + STORED
        let path = schema_builder.add_text_field("path", STRING | STORED);

        // Text fields with jieba tokenizer for CJK support
        let text_options = TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("jieba")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored();

        let title = schema_builder.add_text_field("title", text_options.clone());
        let content = schema_builder.add_text_field("content", text_options);

        SchemaWithFields {
            schema: schema_builder.build(),
            fields: SearchSchema {
                path,
                title,
                content,
            },
        }
    }

    /// Begin a write transaction.
    pub fn begin_write(&mut self) -> Result<()> {
        if self.writer.is_none() {
            // 50MB heap size for writer
            let writer = self.index.writer(50_000_000)?;
            self.writer = Some(writer);
        }
        Ok(())
    }

    /// Add or update a document in the index.
    ///
    /// Call `begin_write()` first, then `add_document()` for each file,
    /// then `commit()` to persist.
    pub fn add_document(&mut self, path: &str, title: Option<&str>, content: &str) -> Result<()> {
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| IndexError::InvalidState("Writer not initialized".to_string()))?;

        // Delete existing document with this path (if any)
        let path_term = tantivy::Term::from_field_text(self.schema.path, path);
        writer.delete_term(path_term);

        // Add new document
        let mut doc = TantivyDocument::new();
        doc.add_text(self.schema.path, path);
        if let Some(t) = title {
            doc.add_text(self.schema.title, t);
        }
        doc.add_text(self.schema.content, content);
        writer.add_document(doc)?;

        Ok(())
    }

    /// Remove a document from the index.
    pub fn remove_document(&mut self, path: &str) -> Result<()> {
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| IndexError::InvalidState("Writer not initialized".to_string()))?;

        let path_term = tantivy::Term::from_field_text(self.schema.path, path);
        writer.delete_term(path_term);
        Ok(())
    }

    /// Commit pending changes.
    pub fn commit(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.commit()?;
            self.reader.reload()?;
        }
        Ok(())
    }

    /// Rollback pending changes.
    pub fn rollback(&mut self) -> Result<()> {
        if let Some(ref mut writer) = self.writer {
            writer.rollback()?;
        }
        Ok(())
    }

    /// Search the index.
    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchHit>> {
        let searcher = self.reader.searcher();

        // Parse query against title and content fields
        let query_parser =
            QueryParser::for_index(&self.index, vec![self.schema.title, self.schema.content]);
        let query = query_parser.parse_query(query_str)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut hits = Vec::with_capacity(top_docs.len());
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;

            let path = doc
                .get_first(self.schema.path)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let title = doc
                .get_first(self.schema.title)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            hits.push(SearchHit { path, title, score });
        }

        Ok(hits)
    }

    /// Get the number of documents in the index.
    pub fn document_count(&self) -> Result<usize> {
        let searcher = self.reader.searcher();
        Ok(searcher.num_docs() as usize)
    }

    /// Clear all documents from the index.
    pub fn clear(&mut self) -> Result<()> {
        self.begin_write()?;
        if let Some(ref mut writer) = self.writer {
            writer.delete_all_documents()?;
            writer.commit()?;
            self.reader.reload()?;
        }
        Ok(())
    }
}

struct SchemaWithFields {
    schema: Schema,
    fields: SearchSchema,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_search() {
        let mut index = SearchIndex::open_in_memory().unwrap();

        index.begin_write().unwrap();
        index
            .add_document(
                "notes/rust.md",
                Some("Learning Rust"),
                "Rust is a systems programming language focused on safety and performance.",
            )
            .unwrap();
        index
            .add_document(
                "notes/python.md",
                Some("Python Basics"),
                "Python is a high-level programming language.",
            )
            .unwrap();
        index.commit().unwrap();

        assert_eq!(index.document_count().unwrap(), 2);

        // Search for "Rust"
        let hits = index.search("Rust", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "notes/rust.md");

        // Search for "programming" (should match both)
        let hits = index.search("programming", 10).unwrap();
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn test_chinese_search() {
        let mut index = SearchIndex::open_in_memory().unwrap();

        index.begin_write().unwrap();
        index
            .add_document(
                "notes/ml.md",
                Some("机器学习入门"),
                "机器学习是人工智能的一个分支，研究如何让计算机从数据中学习。",
            )
            .unwrap();
        index
            .add_document(
                "notes/dl.md",
                Some("深度学习"),
                "深度学习是机器学习的一个子领域，使用神经网络进行特征学习。",
            )
            .unwrap();
        index.commit().unwrap();

        // Search for "机器学习" (should find both, but ml.md should rank higher)
        let hits = index.search("机器学习", 10).unwrap();
        assert!(!hits.is_empty());
        // The exact ranking depends on BM25, but both should be found
        let paths: Vec<_> = hits.iter().map(|h| h.path.as_str()).collect();
        assert!(paths.contains(&"notes/ml.md"));
    }

    #[test]
    fn test_case_insensitive_search() {
        let mut index = SearchIndex::open_in_memory().unwrap();

        index.begin_write().unwrap();
        index
            .add_document(
                "notes/rust.md",
                Some("Learning Rust"),
                "Rust is a Systems Programming language. Section one covers ownership.",
            )
            .unwrap();
        index.commit().unwrap();

        // Lowercase query should find capitalized content
        let hits = index.search("rust", 10).unwrap();
        assert_eq!(hits.len(), 1, "lowercase 'rust' should match 'Rust'");

        let hits = index.search("systems", 10).unwrap();
        assert_eq!(hits.len(), 1, "lowercase 'systems' should match 'Systems'");

        let hits = index.search("section", 10).unwrap();
        assert_eq!(hits.len(), 1, "lowercase 'section' should match 'Section'");

        // Uppercase query should also work
        let hits = index.search("RUST", 10).unwrap();
        assert_eq!(hits.len(), 1, "uppercase 'RUST' should match 'Rust'");

        // Mixed case query
        let hits = index.search("Programming", 10).unwrap();
        assert_eq!(hits.len(), 1, "mixed case should match");
    }

    #[test]
    fn test_update_document() {
        let mut index = SearchIndex::open_in_memory().unwrap();

        index.begin_write().unwrap();
        index
            .add_document("test.md", Some("Version 1"), "First content")
            .unwrap();
        index.commit().unwrap();

        // Update the document
        index.begin_write().unwrap();
        index
            .add_document("test.md", Some("Version 2"), "Updated content")
            .unwrap();
        index.commit().unwrap();

        // Should only have one document
        assert_eq!(index.document_count().unwrap(), 1);

        // Search for updated content
        let hits = index.search("Updated", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].title, Some("Version 2".to_string()));
    }

    #[test]
    fn test_remove_document() {
        let mut index = SearchIndex::open_in_memory().unwrap();

        index.begin_write().unwrap();
        index.add_document("a.md", Some("A"), "Content A").unwrap();
        index.add_document("b.md", Some("B"), "Content B").unwrap();
        index.commit().unwrap();

        assert_eq!(index.document_count().unwrap(), 2);

        // Remove one document
        index.begin_write().unwrap();
        index.remove_document("a.md").unwrap();
        index.commit().unwrap();

        assert_eq!(index.document_count().unwrap(), 1);

        let hits = index.search("Content", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "b.md");
    }
}
