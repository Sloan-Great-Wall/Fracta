//! VFS error types.

use std::path::PathBuf;

/// All errors that VFS operations can produce.
#[derive(Debug, thiserror::Error)]
pub enum VfsError {
    #[error("path not found: {0}")]
    NotFound(PathBuf),

    #[error("path already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("path is outside any registered Location: {0}")]
    OutsideLocation(PathBuf),

    #[error("path is in Ignored scope: {0}")]
    IgnoredScope(PathBuf),

    #[error("atomic write failed for {path}: {reason}")]
    AtomicWriteFailed { path: PathBuf, reason: String },

    #[error("watcher error: {0}")]
    WatcherError(String),

    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}

/// Convenience alias for VFS results.
pub type VfsResult<T> = Result<T, VfsError>;
