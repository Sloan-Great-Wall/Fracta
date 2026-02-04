//! Plain text extraction from Block model.
//!
//! Used by fracta-index for full-text search indexing (FTS5).
//! Strips all formatting and returns pure text content.

use crate::block::*;

/// Extract all plain text from a slice of blocks.
///
/// Returns a single string with block content separated by newlines.
/// Suitable for full-text search indexing.
pub fn extract_text(blocks: &[Block]) -> String {
    let mut buf = String::new();
    for block in blocks {
        extract_block_text(block, &mut buf);
    }
    // Trim trailing whitespace
    let result = buf.trim_end().to_string();
    result
}

/// Extract plain text from inline elements.
pub fn inlines_to_text(inlines: &[Inline]) -> String {
    let mut buf = String::new();
    extract_inline_text(inlines, &mut buf);
    buf
}

fn extract_block_text(block: &Block, buf: &mut String) {
    match block {
        Block::Heading { content, .. } => {
            extract_inline_text(content, buf);
            buf.push('\n');
        }
        Block::Paragraph { content } => {
            extract_inline_text(content, buf);
            buf.push('\n');
        }
        Block::CodeBlock { code, .. } => {
            buf.push_str(code);
            if !code.ends_with('\n') {
                buf.push('\n');
            }
        }
        Block::BlockQuote { children } => {
            for child in children {
                extract_block_text(child, buf);
            }
        }
        Block::List { items, .. } => {
            for item in items {
                for child in &item.children {
                    extract_block_text(child, buf);
                }
            }
        }
        Block::Table { rows, .. } => {
            for row in rows {
                for (i, cell) in row.cells.iter().enumerate() {
                    if i > 0 {
                        buf.push(' ');
                    }
                    extract_inline_text(cell, buf);
                }
                buf.push('\n');
            }
        }
        Block::ThematicBreak => {}
        Block::HtmlBlock { .. } => {}
    }
}

fn extract_inline_text(inlines: &[Inline], buf: &mut String) {
    for inline in inlines {
        match inline {
            Inline::Text { value } => buf.push_str(value),
            Inline::Code { value } => buf.push_str(value),
            Inline::Emphasis { children }
            | Inline::Strong { children }
            | Inline::Strikethrough { children } => {
                extract_inline_text(children, buf);
            }
            Inline::Link { children, .. } => {
                extract_inline_text(children, buf);
            }
            Inline::Image { alt, .. } => buf.push_str(alt),
            Inline::SoftBreak => buf.push(' '),
            Inline::HardBreak => buf.push('\n'),
            Inline::Html { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_text() {
        let blocks = vec![
            Block::Heading {
                level: 1,
                content: vec![Inline::Text {
                    value: "Title".into(),
                }],
            },
            Block::Paragraph {
                content: vec![Inline::Text {
                    value: "Hello world".into(),
                }],
            },
        ];
        assert_eq!(extract_text(&blocks), "Title\nHello world");
    }

    #[test]
    fn test_extract_strips_formatting() {
        let blocks = vec![Block::Paragraph {
            content: vec![
                Inline::Text {
                    value: "Normal ".into(),
                },
                Inline::Strong {
                    children: vec![Inline::Text {
                        value: "bold".into(),
                    }],
                },
                Inline::Text {
                    value: " and ".into(),
                },
                Inline::Code {
                    value: "code".into(),
                },
            ],
        }];
        assert_eq!(extract_text(&blocks), "Normal bold and code");
    }

    #[test]
    fn test_extract_code_block() {
        let blocks = vec![Block::CodeBlock {
            language: Some("rust".into()),
            code: "fn main() {}\n".into(),
        }];
        assert_eq!(extract_text(&blocks), "fn main() {}");
    }

    #[test]
    fn test_inlines_to_text() {
        let inlines = vec![
            Inline::Text {
                value: "Click ".into(),
            },
            Inline::Link {
                url: "https://example.com".into(),
                title: None,
                children: vec![Inline::Text {
                    value: "here".into(),
                }],
            },
        ];
        assert_eq!(inlines_to_text(&inlines), "Click here");
    }
}
