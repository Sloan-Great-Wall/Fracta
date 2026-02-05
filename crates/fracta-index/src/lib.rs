//! # fracta-index — SQLite Metadata + Tantivy Full-Text Search
//!
//! Two-layer index architecture (ADR-0015):
//! - **SQLite**: file registry and metadata (path, mtime, tags, area, etc.)
//! - **Tantivy**: full-text search with intelligent CJK tokenization
//!
//! Both layers are cache — the filesystem remains the source of truth.
//! Deleting `.fracta/cache/index.sqlite` and `.fracta/cache/search/`
//! triggers a full rebuild.
//!
//! ## Usage
//!
//! ```ignore
//! let mut index = Index::open(&cache_dir)?;
//! index.build_full(&location)?;
//! let hits = index.search("机器学习", 10)?;
//! ```

pub mod error;
pub mod metadata;
pub mod search;

pub use error::{IndexError, Result};
pub use metadata::{FileEntry, FileMetadata, MetadataStore};
pub use search::{SearchHit, SearchIndex};

use std::path::{Path, PathBuf};

use fracta_note::Document;
use fracta_vfs::{Entry, EntryKind, Location, Scope, WalkOptions};

/// Unified index combining SQLite metadata and Tantivy search.
pub struct Index {
    /// SQLite metadata store.
    pub metadata: MetadataStore,
    /// Tantivy search index.
    pub search: SearchIndex,
    /// Cache directory path.
    #[allow(dead_code)]
    cache_dir: PathBuf,
}

/// Statistics from an index build operation.
#[derive(Debug, Clone, Default)]
pub struct BuildStats {
    /// Number of files scanned.
    pub files_scanned: usize,
    /// Number of Markdown files indexed.
    pub markdown_indexed: usize,
    /// Number of files added/updated in metadata.
    pub metadata_updated: usize,
    /// Number of stale files removed.
    pub stale_removed: usize,
    /// Duration of the build.
    pub duration_ms: u64,
}

impl Index {
    /// Open or create an index in the given cache directory.
    ///
    /// Creates `index.sqlite` for metadata and `search/` for Tantivy.
    pub fn open(cache_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(cache_dir)?;

        let sqlite_path = cache_dir.join("index.sqlite");
        let search_dir = cache_dir.join("search");

        let metadata = MetadataStore::open(&sqlite_path)?;
        let search = SearchIndex::open(&search_dir)?;

        Ok(Self {
            metadata,
            search,
            cache_dir: cache_dir.to_path_buf(),
        })
    }

    /// Open an in-memory index (for testing).
    pub fn open_in_memory() -> Result<Self> {
        let metadata = MetadataStore::open_in_memory()?;
        let search = SearchIndex::open_in_memory()?;

        Ok(Self {
            metadata,
            search,
            cache_dir: PathBuf::new(),
        })
    }

    /// Build a full index from scratch.
    ///
    /// Scans all managed files in the Location and indexes them.
    pub fn build_full(&mut self, location: &Location) -> Result<BuildStats> {
        let start = std::time::Instant::now();
        let mut stats = BuildStats::default();

        // Collect all managed files
        let options = WalkOptions {
            include_ignored: false,
            max_depth: None,
        };
        let entries = location.walk(&location.root, &options)?;

        let managed_files: Vec<_> = entries
            .into_iter()
            .filter(|e| e.kind == EntryKind::File && e.scope == Scope::Managed)
            .collect();

        stats.files_scanned = managed_files.len();

        // Begin write transactions
        self.search.begin_write()?;

        // Index each file
        let current_paths: Vec<String> = managed_files
            .iter()
            .filter_map(|e| self.relative_path(location, &e.path))
            .collect();

        for entry in &managed_files {
            self.index_file(location, entry, &mut stats)?;
        }

        // Remove stale files from metadata
        stats.stale_removed = self.metadata.remove_stale_files(&current_paths)?;

        // Commit search index
        self.search.commit()?;

        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(stats)
    }

    /// Incremental update: re-index only changed files.
    ///
    /// Compares mtime against the stored value and re-indexes if changed.
    pub fn update_incremental(&mut self, location: &Location) -> Result<BuildStats> {
        let start = std::time::Instant::now();
        let mut stats = BuildStats::default();

        // Collect all managed files
        let options = WalkOptions {
            include_ignored: false,
            max_depth: None,
        };
        let entries = location.walk(&location.root, &options)?;

        let managed_files: Vec<_> = entries
            .into_iter()
            .filter(|e| e.kind == EntryKind::File && e.scope == Scope::Managed)
            .collect();

        stats.files_scanned = managed_files.len();

        // Begin write transaction
        self.search.begin_write()?;

        let current_paths: Vec<String> = managed_files
            .iter()
            .filter_map(|e| self.relative_path(location, &e.path))
            .collect();

        // Check each file for changes
        for entry in &managed_files {
            let rel_path = match self.relative_path(location, &entry.path) {
                Some(p) => p,
                None => continue,
            };

            let needs_update = match (entry.modified, self.metadata.get_file(&rel_path)?) {
                (Some(entry_mtime), Some(existing)) => {
                    // Compare mtime (with 1-second tolerance for filesystem precision)
                    (entry_mtime - existing.mtime).num_seconds().abs() > 1
                }
                (None, _) => true, // Missing mtime: conservative, assume needs update
                (_, None) => true, // New file
            };

            if needs_update {
                self.index_file(location, entry, &mut stats)?;
            }
        }

        // Remove stale files
        stats.stale_removed = self.metadata.remove_stale_files(&current_paths)?;

        // Commit
        self.search.commit()?;

        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(stats)
    }

    /// Compute relative path from Location root.
    fn relative_path(&self, location: &Location, abs_path: &Path) -> Option<String> {
        abs_path
            .strip_prefix(&location.root)
            .ok()
            .map(|p| p.to_string_lossy().to_string())
    }

    /// Index a single file.
    fn index_file(
        &mut self,
        location: &Location,
        entry: &Entry,
        stats: &mut BuildStats,
    ) -> Result<()> {
        let rel_path = match self.relative_path(location, &entry.path) {
            Some(p) => p,
            None => return Ok(()),
        };

        // Create file entry for metadata
        // Use current time as fallback when mtime is unavailable (conservative: marks as "fresh")
        let file_entry = FileEntry {
            path: rel_path.clone(),
            mtime: entry.modified.unwrap_or_else(chrono::Utc::now),
            size: entry.size,
            content_hash: None, // TODO: compute blake3 hash
            indexed: false,
        };

        // Check if it's a Markdown file
        let is_markdown = rel_path.ends_with(".md") || rel_path.ends_with(".markdown");

        if is_markdown {
            // Read and parse the file
            if let Ok(content) = std::fs::read_to_string(&entry.path) {
                let doc = Document::parse(&content);

                // Extract metadata from front matter
                let mut file_meta = FileMetadata::default();
                if let Some(fm) = &doc.front_matter {
                    file_meta.title = fm.get_str("title").map(|s| s.to_string());
                    file_meta.tags = fm
                        .get_string_list("tags")
                        .unwrap_or_default()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect();
                    file_meta.date = fm.get_str("date").map(|s| s.to_string());
                    file_meta.area = fm.get_str("area").map(|s| s.to_string());
                }

                // Fall back to title from first h1 if not in front matter
                if file_meta.title.is_none() {
                    file_meta.title = doc.title();
                }

                // Extract plain text for search
                let plain_text = doc.plain_text();

                // Update metadata store
                let mut indexed_entry = file_entry.clone();
                indexed_entry.indexed = true;
                self.metadata.upsert_file(&indexed_entry)?;
                self.metadata.upsert_metadata(&rel_path, &file_meta)?;

                // Update search index
                self.search
                    .add_document(&rel_path, file_meta.title.as_deref(), &plain_text)?;

                stats.markdown_indexed += 1;
            } else {
                // File couldn't be read, store metadata only
                self.metadata.upsert_file(&file_entry)?;
            }
        } else {
            // Non-markdown file: store metadata only
            self.metadata.upsert_file(&file_entry)?;
        }

        stats.metadata_updated += 1;
        Ok(())
    }

    /// Full-text search.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>> {
        self.search.search(query, limit)
    }

    /// Search by metadata criteria.
    pub fn search_by_metadata(
        &self,
        area: Option<&str>,
        tag: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        limit: usize,
    ) -> Result<Vec<String>> {
        self.metadata
            .search_by_metadata(area, tag, date_from, date_to, limit)
    }

    /// Get metadata for a file.
    pub fn get_metadata(&self, path: &str) -> Result<Option<FileMetadata>> {
        self.metadata.get_metadata(path)
    }

    /// Get a file entry.
    pub fn get_file(&self, path: &str) -> Result<Option<FileEntry>> {
        self.metadata.get_file(path)
    }

    /// List files in a directory (from cache, no disk access).
    pub fn list_directory(&self, dir: &str) -> Result<Vec<FileEntry>> {
        self.metadata.list_directory(dir)
    }

    /// Get total file count.
    pub fn file_count(&self) -> Result<usize> {
        self.metadata.file_count()
    }

    /// Get indexed (searchable) file count.
    pub fn indexed_count(&self) -> Result<usize> {
        self.metadata.indexed_count()
    }

    /// Get search document count.
    pub fn search_document_count(&self) -> Result<usize> {
        self.search.document_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_location() -> (TempDir, Location) {
        let temp = TempDir::new().unwrap();
        let mut location = Location::new("test", temp.path());
        location.init().unwrap();
        (temp, location)
    }

    #[test]
    fn test_build_empty_location() {
        let (_temp, location) = create_test_location();
        let mut index = Index::open_in_memory().unwrap();

        let stats = index.build_full(&location).unwrap();
        assert_eq!(stats.files_scanned, 0);
        assert_eq!(stats.markdown_indexed, 0);
    }

    #[test]
    fn test_index_markdown_file() {
        let (temp, location) = create_test_location();

        // Create a Markdown file
        let content = r#"---
title: Test Note
tags: [rust, fracta]
area: library
---

# Hello World

This is a test document about Rust programming.
"#;
        std::fs::write(temp.path().join("test.md"), content).unwrap();

        let mut index = Index::open_in_memory().unwrap();
        let stats = index.build_full(&location).unwrap();

        assert_eq!(stats.files_scanned, 1);
        assert_eq!(stats.markdown_indexed, 1);
        assert_eq!(index.file_count().unwrap(), 1);
        assert_eq!(index.indexed_count().unwrap(), 1);

        // Check metadata
        let meta = index.get_metadata("test.md").unwrap().unwrap();
        assert_eq!(meta.title, Some("Test Note".to_string()));
        assert_eq!(meta.tags, vec!["rust", "fracta"]);
        assert_eq!(meta.area, Some("library".to_string()));

        // Search
        let hits = index.search("Rust", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "test.md");
    }

    #[test]
    fn test_chinese_search() {
        let (temp, location) = create_test_location();

        let content = r#"---
title: 机器学习笔记
tags: [AI, 学习]
---

# 机器学习入门

机器学习是人工智能的核心技术，通过数据训练模型。
"#;
        std::fs::write(temp.path().join("ml.md"), content).unwrap();

        let mut index = Index::open_in_memory().unwrap();
        index.build_full(&location).unwrap();

        // Search in Chinese
        let hits = index.search("机器学习", 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "ml.md");

        // Search for "人工智能"
        let hits = index.search("人工智能", 10).unwrap();
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn test_incremental_update() {
        let (temp, location) = create_test_location();

        // Initial file
        std::fs::write(temp.path().join("a.md"), "# File A\n\nContent A").unwrap();

        let mut index = Index::open_in_memory().unwrap();
        let stats = index.build_full(&location).unwrap();
        assert_eq!(stats.markdown_indexed, 1);

        // Add another file
        std::fs::write(temp.path().join("b.md"), "# File B\n\nContent B").unwrap();

        // Incremental update
        let stats = index.update_incremental(&location).unwrap();
        assert_eq!(stats.files_scanned, 2);
        // b.md should be indexed (new file)
        assert!(stats.markdown_indexed >= 1);

        assert_eq!(index.file_count().unwrap(), 2);
    }

    #[test]
    fn test_stale_file_removal() {
        let (temp, location) = create_test_location();

        // Create two files
        std::fs::write(temp.path().join("a.md"), "# A").unwrap();
        std::fs::write(temp.path().join("b.md"), "# B").unwrap();

        let mut index = Index::open_in_memory().unwrap();
        index.build_full(&location).unwrap();
        assert_eq!(index.file_count().unwrap(), 2);

        // Delete one file
        std::fs::remove_file(temp.path().join("a.md")).unwrap();

        // Rebuild
        let stats = index.build_full(&location).unwrap();
        assert_eq!(stats.stale_removed, 1);
        assert_eq!(index.file_count().unwrap(), 1);
    }

    #[test]
    fn test_search_by_metadata() {
        let (temp, location) = create_test_location();

        // Create files with different areas
        std::fs::write(
            temp.path().join("lib.md"),
            "---\narea: library\ntags: [rust]\n---\n# Lib",
        )
        .unwrap();
        std::fs::write(
            temp.path().join("now.md"),
            "---\narea: now\ntags: [project]\n---\n# Now",
        )
        .unwrap();

        let mut index = Index::open_in_memory().unwrap();
        index.build_full(&location).unwrap();

        // Search by area
        let results = index
            .search_by_metadata(Some("library"), None, None, None, 10)
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "lib.md");

        // Search by tag
        let results = index
            .search_by_metadata(None, Some("rust"), None, None, 10)
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_non_markdown_files() {
        let (temp, location) = create_test_location();

        // Create various file types
        std::fs::write(temp.path().join("doc.md"), "# Markdown").unwrap();
        std::fs::write(temp.path().join("data.json"), "{}").unwrap();
        std::fs::write(temp.path().join("image.png"), [0u8; 100]).unwrap();

        let mut index = Index::open_in_memory().unwrap();
        let stats = index.build_full(&location).unwrap();

        // All files should be in metadata
        assert_eq!(stats.files_scanned, 3);
        assert_eq!(index.file_count().unwrap(), 3);

        // Only Markdown should be indexed for search
        assert_eq!(stats.markdown_indexed, 1);
        assert_eq!(index.indexed_count().unwrap(), 1);
    }
}
