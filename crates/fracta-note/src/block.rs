//! Fracta block model.
//!
//! A Markdown document is represented as a tree of Blocks, where each Block
//! may contain inline content or nested Blocks. This model is independent
//! of the parsing library (comrak) — it's Fracta's own representation that
//! can be serialized, sent over FFI, and rendered by any UI.

use serde::{Deserialize, Serialize};

/// A block-level element in a Markdown document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    /// Heading (h1–h6).
    Heading { level: u8, content: Vec<Inline> },

    /// A paragraph of inline content.
    Paragraph { content: Vec<Inline> },

    /// Fenced or indented code block.
    CodeBlock {
        language: Option<String>,
        code: String,
    },

    /// Block quote (may contain nested blocks).
    BlockQuote { children: Vec<Block> },

    /// Ordered or unordered list.
    List {
        ordered: bool,
        start: Option<usize>,
        items: Vec<ListItem>,
    },

    /// Table (GFM extension).
    Table {
        alignments: Vec<Alignment>,
        rows: Vec<TableRow>,
    },

    /// Horizontal rule / thematic break.
    ThematicBreak,

    /// Raw HTML block (preserved as-is).
    HtmlBlock { html: String },
}

/// A list item, optionally a task list item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListItem {
    /// `None` = regular item, `Some(true)` = checked task, `Some(false)` = unchecked task.
    pub checked: Option<bool>,
    /// Block content of this list item.
    pub children: Vec<Block>,
}

/// A table row.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRow {
    /// Whether this is the header row.
    pub header: bool,
    /// Cell contents.
    pub cells: Vec<Vec<Inline>>,
}

/// Table column alignment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
    None,
}

/// An inline element within a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Inline {
    /// Plain text.
    Text { value: String },
    /// Inline code span.
    Code { value: String },
    /// Emphasis (italic).
    Emphasis { children: Vec<Inline> },
    /// Strong (bold).
    Strong { children: Vec<Inline> },
    /// Strikethrough (GFM extension).
    Strikethrough { children: Vec<Inline> },
    /// Hyperlink.
    Link {
        url: String,
        title: Option<String>,
        children: Vec<Inline>,
    },
    /// Image.
    Image {
        url: String,
        title: Option<String>,
        alt: String,
    },
    /// Soft line break (rendered as space).
    SoftBreak,
    /// Hard line break (explicit `<br>`).
    HardBreak,
    /// Raw inline HTML.
    Html { value: String },
}
