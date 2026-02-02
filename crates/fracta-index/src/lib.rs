//! # fracta-index — SQLite Index + FTS5
//!
//! SQLite indexing, full-text search (FTS5), incremental updates.
//!
//! Maintains derived indexes over VFS-managed files. The index is a
//! rebuildable cache — the filesystem remains the source of truth.
//!
//! Status: Phase 1 active.
