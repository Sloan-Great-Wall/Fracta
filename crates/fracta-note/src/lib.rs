//! # fracta-note — Markdown Parser + Block Model
//!
//! Parses Markdown files into a structured Document model:
//! - **YAML front matter** → typed metadata (title, date, tags, etc.)
//! - **Markdown body** → Block tree (headings, paragraphs, lists, tables, code, etc.)
//! - **Plain text extraction** → for full-text search indexing (FTS5)
//!
//! The source Markdown file is always the SOT (source of truth).
//! This crate only reads and parses — it never modifies user files.
//!
//! ## Architecture
//!
//! - `Document`: the top-level parsed result (front matter + blocks)
//! - `Block` / `Inline`: Fracta-native representation, independent of comrak
//! - `FrontMatter`: parsed YAML metadata with typed accessors
//! - `convert`: comrak AST → Block model (the only comrak-coupled code)
//! - `text`: plain text extraction from blocks

pub mod block;
pub mod convert;
pub mod front_matter;
pub mod text;

pub use block::{Alignment, Block, Inline, ListItem, TableRow};
pub use front_matter::FrontMatter;

use comrak::{Arena, Options};

/// A parsed Markdown document.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Parsed YAML front matter (if present).
    pub front_matter: Option<FrontMatter>,
    /// The block-level content of the document.
    pub blocks: Vec<Block>,
}

impl Document {
    /// Parse a Markdown string into a Document.
    ///
    /// Enables GFM extensions: tables, task lists, strikethrough,
    /// autolinks, footnotes, and YAML front matter.
    pub fn parse(markdown: &str) -> Self {
        let arena = Arena::new();
        let options = Self::comrak_options();
        let root = comrak::parse_document(&arena, markdown, &options);

        // Extract front matter from AST
        let mut front_matter = None;
        for child in root.children() {
            let data = child.data.borrow();
            if let comrak::nodes::NodeValue::FrontMatter(ref yaml) = data.value {
                front_matter = FrontMatter::parse(yaml);
                break;
            }
        }

        // Convert remaining AST nodes to Block model
        let blocks = convert::ast_to_blocks(root);

        Document {
            front_matter,
            blocks,
        }
    }

    /// Extract all plain text content (for full-text search indexing).
    pub fn plain_text(&self) -> String {
        text::extract_text(&self.blocks)
    }

    /// Get the document title from front matter or first heading.
    pub fn title(&self) -> Option<String> {
        // Try front matter first
        if let Some(fm) = &self.front_matter {
            if let Some(title) = fm.get_str("title") {
                return Some(title.to_string());
            }
        }
        // Fall back to first h1
        for block in &self.blocks {
            if let Block::Heading {
                level: 1, content, ..
            } = block
            {
                return Some(text::inlines_to_text(content));
            }
        }
        None
    }

    /// Build comrak options with all GFM extensions enabled.
    fn comrak_options() -> Options<'static> {
        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.footnotes = true;
        options.extension.front_matter_delimiter = Some("---".to_owned());
        options
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Basic parsing ──────────────────────────────────────────────────

    #[test]
    fn test_parse_simple_document() {
        let doc = Document::parse("# Hello\n\nThis is a paragraph.\n");
        assert!(doc.front_matter.is_none());
        assert_eq!(doc.blocks.len(), 2);

        // First block: heading
        match &doc.blocks[0] {
            Block::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content.len(), 1);
                match &content[0] {
                    Inline::Text { value } => assert_eq!(value, "Hello"),
                    _ => panic!("expected Text inline"),
                }
            }
            _ => panic!("expected Heading block"),
        }

        // Second block: paragraph
        match &doc.blocks[1] {
            Block::Paragraph { content } => {
                assert_eq!(content.len(), 1);
                match &content[0] {
                    Inline::Text { value } => assert_eq!(value, "This is a paragraph."),
                    _ => panic!("expected Text inline"),
                }
            }
            _ => panic!("expected Paragraph block"),
        }
    }

    // ── Front matter ───────────────────────────────────────────────────

    #[test]
    fn test_parse_with_front_matter() {
        let md = "---\ntitle: My Note\ntags: [rust, fracta]\n---\n\n# Content\n";
        let doc = Document::parse(md);

        let fm = doc.front_matter.as_ref().unwrap();
        assert_eq!(fm.get_str("title"), Some("My Note"));
        let tags = fm.get_string_list("tags").unwrap();
        assert_eq!(tags, vec!["rust", "fracta"]);

        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn test_title_from_front_matter() {
        let md = "---\ntitle: From Front Matter\n---\n\n# From Heading\n";
        let doc = Document::parse(md);
        // Front matter title takes precedence
        assert_eq!(doc.title(), Some("From Front Matter".to_string()));
    }

    #[test]
    fn test_title_from_heading() {
        let doc = Document::parse("# My Title\n\nSome content.\n");
        assert_eq!(doc.title(), Some("My Title".to_string()));
    }

    #[test]
    fn test_no_title() {
        let doc = Document::parse("Just a paragraph.\n");
        assert_eq!(doc.title(), None);
    }

    // ── GFM extensions ─────────────────────────────────────────────────

    #[test]
    fn test_task_list() {
        let md = "- [x] Done\n- [ ] Todo\n- Regular\n";
        let doc = Document::parse(md);
        assert_eq!(doc.blocks.len(), 1);

        match &doc.blocks[0] {
            Block::List { ordered, items, .. } => {
                assert!(!ordered);
                assert_eq!(items.len(), 3);
                assert_eq!(items[0].checked, Some(true));
                assert_eq!(items[1].checked, Some(false));
                assert_eq!(items[2].checked, None);
            }
            _ => panic!("expected List block"),
        }
    }

    #[test]
    fn test_table() {
        let md = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |\n";
        let doc = Document::parse(md);
        assert_eq!(doc.blocks.len(), 1);

        match &doc.blocks[0] {
            Block::Table { alignments, rows } => {
                assert_eq!(alignments.len(), 2);
                assert_eq!(rows.len(), 3); // header + 2 data rows
                assert!(rows[0].header);
                assert!(!rows[1].header);
            }
            _ => panic!("expected Table block"),
        }
    }

    #[test]
    fn test_strikethrough() {
        let doc = Document::parse("~~deleted~~\n");

        match &doc.blocks[0] {
            Block::Paragraph { content } => {
                assert_eq!(content.len(), 1);
                match &content[0] {
                    Inline::Strikethrough { children } => {
                        assert_eq!(children.len(), 1);
                        match &children[0] {
                            Inline::Text { value } => assert_eq!(value, "deleted"),
                            _ => panic!("expected Text"),
                        }
                    }
                    _ => panic!("expected Strikethrough"),
                }
            }
            _ => panic!("expected Paragraph"),
        }
    }

    // ── Code blocks ────────────────────────────────────────────────────

    #[test]
    fn test_fenced_code_block() {
        let md = "```rust\nfn main() {}\n```\n";
        let doc = Document::parse(md);

        match &doc.blocks[0] {
            Block::CodeBlock { language, code } => {
                assert_eq!(language.as_deref(), Some("rust"));
                assert_eq!(code, "fn main() {}\n");
            }
            _ => panic!("expected CodeBlock"),
        }
    }

    // ── Inline formatting ──────────────────────────────────────────────

    #[test]
    fn test_inline_formatting() {
        let doc = Document::parse("**bold** and *italic* and `code`\n");

        match &doc.blocks[0] {
            Block::Paragraph { content } => {
                // Should have: Strong, Text(" and "), Emphasis, Text(" and "), Code
                assert!(content.len() >= 5);
                assert!(matches!(&content[0], Inline::Strong { .. }));
                assert!(matches!(&content[2], Inline::Emphasis { .. }));
                assert!(matches!(&content[4], Inline::Code { .. }));
            }
            _ => panic!("expected Paragraph"),
        }
    }

    #[test]
    fn test_link() {
        let doc = Document::parse("[Fracta](https://fracta.app)\n");

        match &doc.blocks[0] {
            Block::Paragraph { content } => {
                match &content[0] {
                    Inline::Link {
                        url, children, ..
                    } => {
                        assert_eq!(url, "https://fracta.app");
                        match &children[0] {
                            Inline::Text { value } => assert_eq!(value, "Fracta"),
                            _ => panic!("expected Text in link"),
                        }
                    }
                    _ => panic!("expected Link"),
                }
            }
            _ => panic!("expected Paragraph"),
        }
    }

    // ── Plain text extraction ──────────────────────────────────────────

    #[test]
    fn test_plain_text_extraction() {
        let md = "---\ntitle: Test\n---\n\n# Hello\n\nWorld with **bold** and `code`.\n";
        let doc = Document::parse(md);
        let text = doc.plain_text();
        assert!(text.contains("Hello"));
        assert!(text.contains("World with bold and code."));
        assert!(!text.contains("---"));
        assert!(!text.contains("**"));
    }

    // ── Block quote ────────────────────────────────────────────────────

    #[test]
    fn test_block_quote() {
        let doc = Document::parse("> A wise quote\n");

        match &doc.blocks[0] {
            Block::BlockQuote { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    Block::Paragraph { content } => {
                        match &content[0] {
                            Inline::Text { value } => assert_eq!(value, "A wise quote"),
                            _ => panic!("expected Text"),
                        }
                    }
                    _ => panic!("expected Paragraph in BlockQuote"),
                }
            }
            _ => panic!("expected BlockQuote"),
        }
    }

    // ── Complex document ───────────────────────────────────────────────

    #[test]
    fn test_fracta_story_document() {
        let md = "\
---
title: 2025-01-15 — Building Fracta
date: 2025-01-15
tags: [dev, rust]
mood: 8
---

# Building Fracta

Today I worked on the **Note engine**. Key tasks:

- [x] Parse Markdown with comrak
- [x] Extract YAML front matter
- [ ] Build block model

## Code Sample

```rust
let doc = Document::parse(markdown);
```

> Ship it when it's ready.
";
        let doc = Document::parse(md);

        // Front matter
        let fm = doc.front_matter.as_ref().unwrap();
        assert_eq!(fm.get_str("title"), Some("2025-01-15 — Building Fracta"));
        assert_eq!(fm.get_i64("mood"), Some(8));

        // Title
        assert_eq!(
            doc.title(),
            Some("2025-01-15 — Building Fracta".to_string())
        );

        // Blocks: heading, paragraph, list, heading, code block, block quote
        assert!(doc.blocks.len() >= 5);

        // Plain text should contain key content
        let text = doc.plain_text();
        assert!(text.contains("Building Fracta"));
        assert!(text.contains("Note engine"));
        assert!(text.contains("Parse Markdown"));
        assert!(text.contains("Ship it"));
    }

    // ── Serialization ──────────────────────────────────────────────────

    #[test]
    fn test_blocks_serialize_to_json() {
        let doc = Document::parse("# Hello\n\nWorld\n");
        let json = serde_json::to_string(&doc.blocks).unwrap();
        assert!(json.contains("\"type\":\"heading\""));
        assert!(json.contains("\"type\":\"paragraph\""));
    }
}
