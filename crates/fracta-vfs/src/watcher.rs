//! Filesystem watcher.
//!
//! Watches a Location for file changes and emits events that other subsystems
//! (Index, Pipelines) can react to.
//!
//! Placeholder — full implementation in Phase 1.

/// Events emitted by the filesystem watcher.
#[derive(Debug, Clone)]
pub enum FsEvent {
    /// A file or folder was created.
    Created(std::path::PathBuf),
    /// A file was modified.
    Modified(std::path::PathBuf),
    /// A file or folder was deleted.
    Deleted(std::path::PathBuf),
    /// A file or folder was renamed.
    Renamed {
        from: std::path::PathBuf,
        to: std::path::PathBuf,
    },
}
