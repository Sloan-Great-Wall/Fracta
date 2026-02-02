//! File and folder entries.

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::scope::Scope;

/// Whether an entry is a file or a folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    Folder,
}

/// Metadata about a file or folder within a Location.
///
/// This is the VFS-level view of an item — it has no knowledge of Framework
/// concepts like Quest or Event. Higher layers derive semantic meaning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Absolute path on the filesystem.
    pub path: PathBuf,

    /// File or folder.
    pub kind: EntryKind,

    /// File name (last component of path).
    pub name: String,

    /// File extension (lowercase, without dot), if any.
    pub extension: Option<String>,

    /// Size in bytes. 0 for folders.
    pub size: u64,

    /// Last modification time.
    pub modified: DateTime<Utc>,

    /// Creation time (if available from the OS).
    pub created: Option<DateTime<Utc>>,

    /// Managed / Ignored / Plain scope of this entry.
    pub scope: Scope,
}
