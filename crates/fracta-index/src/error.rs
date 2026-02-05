//! Index error types.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during indexing operations.
#[derive(Debug, Error)]
pub enum IndexError {
    /// SQLite database error.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Tantivy search engine error.
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),

    /// Tantivy query parse error.
    #[error("Query parse error: {0}")]
    QueryParse(#[from] tantivy::query::QueryParserError),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// VFS error.
    #[error("VFS error: {0}")]
    Vfs(#[from] fracta_vfs::VfsError),

    /// Index not found at path.
    #[error("Index not found at {0}")]
    NotFound(PathBuf),

    /// Index already exists at path.
    #[error("Index already exists at {0}")]
    AlreadyExists(PathBuf),

    /// Invalid index state.
    #[error("Invalid index state: {0}")]
    InvalidState(String),

    /// Corrupted data in index (e.g., invalid datetime format).
    #[error("Corrupted index data: {0}")]
    CorruptedData(String),
}

/// Result type for index operations.
pub type Result<T> = std::result::Result<T, IndexError>;
