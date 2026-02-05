//! Convert comrak AST nodes to Fracta Block model.
//!
//! This module is the only place that depends on comrak's internal types.
//! Everything else in fracta-note works with the Fracta-native Block/Inline types.

use comrak::nodes::{AstNode, ListType, NodeValue, TableAlignment};

use crate::block::*;

/// Convert a comrak AST root node into a list of Fracta Blocks.
///
/// Skips the `FrontMatter` node (handled separately) and `Document` wrapper.
pub fn ast_to_blocks<'a>(root: &'a AstNode<'a>) -> Vec<Block> {
    root.children()
        .filter_map(|child| node_to_block(child))
        .collect()
}

fn node_to_block<'a>(node: &'a AstNode<'a>) -> Option<Block> {
    // Extract what we need from the node data, then drop the borrow
    // before recursing into children (which also borrow node data).
    enum BlockKind {
        Heading {
            level: u8,
        },
        Paragraph,
        CodeBlock {
            language: Option<String>,
            code: String,
        },
        BlockQuote,
        List {
            ordered: bool,
            start: Option<usize>,
        },
        Table {
            alignments: Vec<Alignment>,
        },
        ThematicBreak,
        HtmlBlock {
            html: String,
        },
        Skip,
    }

    let kind = {
        let data = node.data.borrow();
        match &data.value {
            NodeValue::Heading(h) => BlockKind::Heading { level: h.level },
            NodeValue::Paragraph => BlockKind::Paragraph,
            NodeValue::CodeBlock(cb) => {
                let language = if cb.info.is_empty() {
                    None
                } else {
                    Some(
                        cb.info
                            .split_whitespace()
                            .next()
                            .unwrap_or(&cb.info)
                            .to_string(),
                    )
                };
                BlockKind::CodeBlock {
                    language,
                    code: cb.literal.clone(),
                }
            }
            NodeValue::BlockQuote => BlockKind::BlockQuote,
            NodeValue::List(list) => {
                let ordered = matches!(list.list_type, ListType::Ordered);
                let start = if ordered { Some(list.start) } else { None };
                BlockKind::List { ordered, start }
            }
            NodeValue::Table(table) => {
                let alignments = table
                    .alignments
                    .iter()
                    .map(|a| match a {
                        TableAlignment::Left => Alignment::Left,
                        TableAlignment::Center => Alignment::Center,
                        TableAlignment::Right => Alignment::Right,
                        TableAlignment::None => Alignment::None,
                    })
                    .collect();
                BlockKind::Table { alignments }
            }
            NodeValue::ThematicBreak => BlockKind::ThematicBreak,
            NodeValue::HtmlBlock(html) => BlockKind::HtmlBlock {
                html: html.literal.clone(),
            },
            NodeValue::FrontMatter(_) | NodeValue::Document => BlockKind::Skip,
            _ => BlockKind::Skip,
        }
    }; // data borrow dropped here

    match kind {
        BlockKind::Heading { level } => Some(Block::Heading {
            level,
            content: collect_inlines(node),
        }),
        BlockKind::Paragraph => Some(Block::Paragraph {
            content: collect_inlines(node),
        }),
        BlockKind::CodeBlock { language, code } => Some(Block::CodeBlock { language, code }),
        BlockKind::BlockQuote => Some(Block::BlockQuote {
            children: ast_to_blocks(node),
        }),
        BlockKind::List { ordered, start } => {
            let items = node
                .children()
                .map(|item| list_item_from_node(item))
                .collect();
            Some(Block::List {
                ordered,
                start,
                items,
            })
        }
        BlockKind::Table { alignments } => {
            let rows = node
                .children()
                .map(|row_node| {
                    let row_data = row_node.data.borrow();
                    let header = matches!(row_data.value, NodeValue::TableRow(true));
                    drop(row_data);

                    let cells = row_node
                        .children()
                        .map(|cell_node| collect_inlines(cell_node))
                        .collect();

                    TableRow { header, cells }
                })
                .collect();
            Some(Block::Table { alignments, rows })
        }
        BlockKind::ThematicBreak => Some(Block::ThematicBreak),
        BlockKind::HtmlBlock { html } => Some(Block::HtmlBlock { html }),
        BlockKind::Skip => None,
    }
}

/// Process a list item node into a Fracta ListItem.
///
/// In comrak's AST, task list items are represented by replacing the `Item`
/// node with a `TaskItem` node in-place. So we check the node itself,
/// not its children, for task status.
fn list_item_from_node<'a>(node: &'a AstNode<'a>) -> ListItem {
    // Check if this node is a TaskItem (comrak mutates Item → TaskItem in-place)
    let checked = {
        let data = node.data.borrow();
        match &data.value {
            // Some(char) = checked (e.g., 'x'), None = unchecked
            NodeValue::TaskItem(ch) => Some(ch.is_some()),
            _ => None,
        }
    }; // data borrow dropped here

    let children = node
        .children()
        .filter_map(|child| node_to_block(child))
        .collect();

    ListItem { checked, children }
}

/// Collect inline content from a node's children.
fn collect_inlines<'a>(node: &'a AstNode<'a>) -> Vec<Inline> {
    node.children()
        .filter_map(|child| node_to_inline(child))
        .collect()
}

fn node_to_inline<'a>(node: &'a AstNode<'a>) -> Option<Inline> {
    // Same pattern: extract data, drop borrow, then recurse.
    enum InlineKind {
        Text(String),
        Code(String),
        Emph,
        Strong,
        Strikethrough,
        Link { url: String, title: Option<String> },
        Image { url: String, title: Option<String> },
        SoftBreak,
        HardBreak,
        Html(String),
        Skip,
    }

    let kind = {
        let data = node.data.borrow();
        match &data.value {
            NodeValue::Text(t) => InlineKind::Text(t.clone()),
            NodeValue::Code(c) => InlineKind::Code(c.literal.clone()),
            NodeValue::Emph => InlineKind::Emph,
            NodeValue::Strong => InlineKind::Strong,
            NodeValue::Strikethrough => InlineKind::Strikethrough,
            NodeValue::Link(link) => InlineKind::Link {
                url: link.url.clone(),
                title: if link.title.is_empty() {
                    None
                } else {
                    Some(link.title.clone())
                },
            },
            NodeValue::Image(link) => InlineKind::Image {
                url: link.url.clone(),
                title: if link.title.is_empty() {
                    None
                } else {
                    Some(link.title.clone())
                },
            },
            NodeValue::SoftBreak => InlineKind::SoftBreak,
            NodeValue::LineBreak => InlineKind::HardBreak,
            NodeValue::HtmlInline(html) => InlineKind::Html(html.clone()),
            _ => InlineKind::Skip,
        }
    }; // data borrow dropped here

    match kind {
        InlineKind::Text(value) => Some(Inline::Text { value }),
        InlineKind::Code(value) => Some(Inline::Code { value }),
        InlineKind::Emph => Some(Inline::Emphasis {
            children: collect_inlines(node),
        }),
        InlineKind::Strong => Some(Inline::Strong {
            children: collect_inlines(node),
        }),
        InlineKind::Strikethrough => Some(Inline::Strikethrough {
            children: collect_inlines(node),
        }),
        InlineKind::Link { url, title } => Some(Inline::Link {
            url,
            title,
            children: collect_inlines(node),
        }),
        InlineKind::Image { url, title } => {
            let alt = collect_plain_text(node);
            Some(Inline::Image { url, title, alt })
        }
        InlineKind::SoftBreak => Some(Inline::SoftBreak),
        InlineKind::HardBreak => Some(Inline::HardBreak),
        InlineKind::Html(value) => Some(Inline::Html { value }),
        InlineKind::Skip => None,
    }
}

/// Collect all text content from a node's descendants (for alt text, etc.).
fn collect_plain_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut buf = String::new();
    collect_plain_text_recursive(node, &mut buf);
    buf
}

fn collect_plain_text_recursive<'a>(node: &'a AstNode<'a>, buf: &mut String) {
    {
        let data = node.data.borrow();
        match &data.value {
            NodeValue::Text(t) => buf.push_str(t),
            NodeValue::Code(c) => buf.push_str(&c.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => buf.push(' '),
            _ => {}
        }
    } // data borrow dropped here
    for child in node.children() {
        collect_plain_text_recursive(child, buf);
    }
}
