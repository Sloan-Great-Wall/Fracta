//! Location management.
//!
//! A Location is a directory tree that the user grants Fracta access to.
//! Fracta supports multiple Locations and aggregates them into a Global View.
//!
//! Each managed Location has exactly one `.fracta/` system directory at its root.
//! VFS is responsible for:
//! - Registering and tracking Locations
//! - Determining Scope (Managed/Ignored/Plain) for any path
//! - Providing CRUD operations scoped to Locations

use std::path::{Path, PathBuf};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entry::{Entry, EntryKind};
use crate::error::{VfsError, VfsResult};
use crate::scope::Scope;

/// A user-granted directory tree that Fracta manages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Unique identifier for this Location (persisted across sessions).
    pub id: Uuid,

    /// Human-readable label (e.g., "My Drive", "Documents").
    pub label: String,

    /// Absolute path to the Location root directory.
    pub root: PathBuf,

    /// Whether the compute layer is enabled for this Location.
    pub managed: bool,
}

/// The `.fracta/` directory name within a managed Location.
pub const FRACTA_DIR: &str = ".fracta";

impl Location {
    /// Create a new Location from a root path.
    pub fn new(label: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        Self {
            id: Uuid::now_v7(),
            label: label.into(),
            root: root.into(),
            managed: false,
        }
    }

    /// Path to the `.fracta/` system directory for this Location.
    pub fn fracta_dir(&self) -> PathBuf {
        self.root.join(FRACTA_DIR)
    }

    /// Whether a given path is inside this Location.
    pub fn contains(&self, path: &Path) -> bool {
        path.starts_with(&self.root)
    }

    /// Determine the scope of a path within this Location.
    ///
    /// Returns `None` if the path is not inside this Location.
    pub fn scope_of(&self, path: &Path) -> Option<Scope> {
        if !self.contains(path) {
            return None;
        }

        if !self.managed {
            return Some(Scope::Plain);
        }

        // TODO: Check ignore rules from `.fracta/config/ignore`
        // For now, everything in a managed Location is Managed
        // (except the .fracta directory itself, which is always Managed/internal).

        Some(Scope::Managed)
    }

    /// List the immediate children of a directory within this Location.
    pub fn list_directory(&self, dir: &Path) -> VfsResult<Vec<Entry>> {
        if !self.contains(dir) {
            return Err(VfsError::OutsideLocation(dir.to_path_buf()));
        }

        let mut entries = Vec::new();

        let read_dir = std::fs::read_dir(dir).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => VfsError::NotFound(dir.to_path_buf()),
            std::io::ErrorKind::PermissionDenied => {
                VfsError::PermissionDenied(dir.to_path_buf())
            }
            _ => VfsError::Io { source: e },
        })?;

        for dir_entry in read_dir {
            let dir_entry = dir_entry?;
            let metadata = dir_entry.metadata()?;
            let path = dir_entry.path();
            let name = dir_entry.file_name().to_string_lossy().into_owned();

            // Skip .fracta directory in listings (internal system dir)
            if name == FRACTA_DIR {
                continue;
            }

            let kind = if metadata.is_dir() {
                EntryKind::Folder
            } else {
                EntryKind::File
            };

            let extension = if kind == EntryKind::File {
                path.extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
            } else {
                None
            };

            let scope = self.scope_of(&path).unwrap_or(Scope::Plain);

            entries.push(Entry {
                path,
                kind,
                name,
                extension,
                size: metadata.len(),
                modified: metadata.modified().ok().map(DateTime::from).unwrap_or_default(),
                created: metadata.created().ok().map(DateTime::from),
                scope,
            });
        }

        // Sort: folders first, then alphabetical
        entries.sort_by(|a, b| {
            match (&a.kind, &b.kind) {
                (EntryKind::Folder, EntryKind::File) => std::cmp::Ordering::Less,
                (EntryKind::File, EntryKind::Folder) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok(entries)
    }

    /// Read a file's contents as bytes.
    pub fn read_file(&self, path: &Path) -> VfsResult<Vec<u8>> {
        if !self.contains(path) {
            return Err(VfsError::OutsideLocation(path.to_path_buf()));
        }
        std::fs::read(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => VfsError::NotFound(path.to_path_buf()),
            std::io::ErrorKind::PermissionDenied => {
                VfsError::PermissionDenied(path.to_path_buf())
            }
            _ => VfsError::Io { source: e },
        })
    }

    /// Read a file's contents as a UTF-8 string.
    pub fn read_file_string(&self, path: &Path) -> VfsResult<String> {
        let bytes = self.read_file(path)?;
        String::from_utf8(bytes).map_err(|_| VfsError::Io {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "file is not valid UTF-8",
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_location_contains() {
        let loc = Location::new("test", "/tmp/test-location");
        assert!(loc.contains(Path::new("/tmp/test-location/subdir/file.md")));
        assert!(!loc.contains(Path::new("/tmp/other-location/file.md")));
    }

    #[test]
    fn test_list_directory() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Create some test files and folders
        std::fs::create_dir(root.join("folder_a")).unwrap();
        std::fs::create_dir(root.join("folder_b")).unwrap();
        std::fs::write(root.join("notes.md"), "# Hello").unwrap();
        std::fs::write(root.join("data.json"), "{}").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
        };

        let entries = loc.list_directory(&root).unwrap();

        // Should have 4 entries (2 folders + 2 files)
        assert_eq!(entries.len(), 4);

        // Folders should come first
        assert_eq!(entries[0].kind, EntryKind::Folder);
        assert_eq!(entries[1].kind, EntryKind::Folder);
        assert_eq!(entries[2].kind, EntryKind::File);
        assert_eq!(entries[3].kind, EntryKind::File);
    }

    #[test]
    fn test_fracta_dir_hidden_in_listing() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Create .fracta directory (should be hidden from listings)
        std::fs::create_dir(root.join(".fracta")).unwrap();
        std::fs::write(root.join("visible.md"), "hello").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
        };

        let entries = loc.list_directory(&root).unwrap();

        // .fracta should be hidden
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "visible.md");
    }
}
