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
//! - Recursive directory traversal with scope filtering

use std::path::{Path, PathBuf};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entry::{Entry, EntryKind};
use crate::error::{VfsError, VfsResult};
use crate::ignore::IgnoreRules;
use crate::init::init_fracta_dir;
use crate::scope::Scope;
use crate::settings::LocationSettings;
use crate::writer::atomic_write;

/// The `.fracta/` directory name within a managed Location.
pub const FRACTA_DIR: &str = ".fracta";

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

    /// Ignore rules loaded from `.fracta/config/ignore`.
    /// Skipped during serialization — reload after deserializing.
    #[serde(skip)]
    ignore_rules: IgnoreRules,
}

/// Options for recursive directory traversal.
#[derive(Debug, Clone, Default)]
pub struct WalkOptions {
    /// Include entries with Ignored scope in results.
    /// When false, ignored directories are not recursed into.
    pub include_ignored: bool,

    /// Maximum recursion depth (None = unlimited).
    pub max_depth: Option<usize>,
}

// ── Constructors ───────────────────────────────────────────────────────

impl Location {
    /// Create a new (unmanaged) Location. Does not touch the filesystem.
    pub fn new(label: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        Self {
            id: Uuid::now_v7(),
            label: label.into(),
            root: root.into(),
            managed: false,
            ignore_rules: IgnoreRules::empty(),
        }
    }

    /// Open an existing managed Location, loading settings and ignore rules from disk.
    ///
    /// The Location ID is loaded from `.fracta/config/settings.json` if it exists,
    /// ensuring the same ID persists across sessions. Falls back to default ignore
    /// rules if `.fracta/config/ignore` is missing.
    pub fn open(label: impl Into<String>, root: impl Into<PathBuf>) -> VfsResult<Self> {
        let root = root.into();
        if !root.is_dir() {
            return Err(VfsError::NotFound(root));
        }

        let ignore_path = root.join(FRACTA_DIR).join("config").join("ignore");
        let ignore_rules = IgnoreRules::load(&ignore_path).unwrap_or_default();

        // Load persistent ID from settings, or generate a new one
        let mut settings = LocationSettings::load(&root)?;
        let id = settings.get_or_create_id();

        // If we generated a new ID, persist it
        if settings.id.is_some() {
            settings.save(&root)?;
        }

        Ok(Self {
            id,
            label: label.into(),
            root,
            managed: true,
            ignore_rules,
        })
    }

    /// Initialize this Location: create `.fracta/` structure and mark as managed.
    ///
    /// Persists the Location ID to `.fracta/config/settings.json`.
    pub fn init(&mut self) -> VfsResult<()> {
        init_fracta_dir(&self.root)?;
        self.managed = true;
        self.reload_ignore_rules()?;

        // Persist the Location ID
        let mut settings = LocationSettings::load(&self.root)?;
        settings.id = Some(self.id);
        settings.label = Some(self.label.clone());
        settings.save(&self.root)?;

        Ok(())
    }

    /// Reload ignore rules from disk.
    pub fn reload_ignore_rules(&mut self) -> VfsResult<()> {
        let ignore_path = self.root.join(FRACTA_DIR).join("config").join("ignore");
        self.ignore_rules =
            IgnoreRules::load(&ignore_path).map_err(|e| VfsError::Io { source: e })?;
        Ok(())
    }
}

// ── Path queries ───────────────────────────────────────────────────────

impl Location {
    /// Path to the `.fracta/` system directory for this Location.
    pub fn fracta_dir(&self) -> PathBuf {
        self.root.join(FRACTA_DIR)
    }

    /// Whether a given path is inside this Location.
    ///
    /// This method resolves symlinks to prevent path traversal attacks.
    /// For non-existent paths, it resolves the existing parent and checks
    /// if the remaining components would stay within the Location.
    pub fn contains(&self, path: &Path) -> bool {
        self.resolve_and_check(path).is_some()
    }

    /// Resolve a path and verify it stays within this Location.
    ///
    /// Returns the canonicalized path if it's inside the Location, None otherwise.
    /// This prevents symlink escape attacks like `managed_dir/link_to_parent/../../../etc`.
    fn resolve_and_check(&self, path: &Path) -> Option<PathBuf> {
        // Canonicalize the Location root (resolve symlinks in root itself)
        let canonical_root = self.root.canonicalize().ok()?;

        // Try to canonicalize the full path first (works for existing paths)
        if let Ok(canonical_path) = path.canonicalize() {
            if canonical_path.starts_with(&canonical_root) {
                return Some(canonical_path);
            }
            return None;
        }

        // Path doesn't exist yet - resolve the existing parent portion
        // and verify the full constructed path stays within bounds
        let mut existing = path.to_path_buf();
        let mut pending_components = Vec::new();

        // Walk up until we find an existing ancestor
        while !existing.exists() {
            if let Some(file_name) = existing.file_name() {
                pending_components.push(file_name.to_os_string());
                existing = match existing.parent() {
                    Some(p) => p.to_path_buf(),
                    None => return None, // No existing ancestor found
                };
            } else {
                return None;
            }
        }

        // Canonicalize the existing ancestor
        let mut resolved = existing.canonicalize().ok()?;

        // Append the pending components back (in reverse order)
        for component in pending_components.into_iter().rev() {
            // Block ".." in pending components to prevent escape
            if component == ".." {
                return None;
            }
            resolved.push(component);
        }

        // Final check: is the resolved path inside our root?
        if resolved.starts_with(&canonical_root) {
            Some(resolved)
        } else {
            None
        }
    }

    /// Get the relative path from Location root.
    fn relative_path(&self, path: &Path) -> Option<PathBuf> {
        path.strip_prefix(&self.root).ok().map(PathBuf::from)
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

        let rel_path = match self.relative_path(path) {
            Some(p) if p.as_os_str().is_empty() => return Some(Scope::Managed),
            Some(p) => p,
            None => return Some(Scope::Managed),
        };

        // .fracta/ itself is always Managed (internal system directory)
        if rel_path.starts_with(FRACTA_DIR) {
            return Some(Scope::Managed);
        }

        let is_dir = path.is_dir();
        if self.ignore_rules.is_ignored(&rel_path, is_dir) {
            Some(Scope::Ignored)
        } else {
            Some(Scope::Managed)
        }
    }
}

// ── Directory listing & walking ────────────────────────────────────────

impl Location {
    /// List the immediate children of a directory within this Location.
    pub fn list_directory(&self, dir: &Path) -> VfsResult<Vec<Entry>> {
        if !self.contains(dir) {
            return Err(VfsError::OutsideLocation(dir.to_path_buf()));
        }

        let mut entries = Vec::new();

        let read_dir = std::fs::read_dir(dir).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => VfsError::NotFound(dir.to_path_buf()),
            std::io::ErrorKind::PermissionDenied => VfsError::PermissionDenied(dir.to_path_buf()),
            _ => VfsError::Io { source: e },
        })?;

        for dir_entry in read_dir {
            let dir_entry = dir_entry?;
            let name = dir_entry.file_name().to_string_lossy().into_owned();

            // Skip .fracta directory in listings (internal system dir)
            if name == FRACTA_DIR {
                continue;
            }

            entries.push(self.build_entry(&dir_entry.path(), &dir_entry.metadata()?));
        }

        // Sort: folders first, then alphabetical (case-insensitive)
        entries.sort_by(|a, b| match (&a.kind, &b.kind) {
            (EntryKind::Folder, EntryKind::File) => std::cmp::Ordering::Less,
            (EntryKind::File, EntryKind::Folder) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        Ok(entries)
    }

    /// Recursively walk the directory tree starting from `dir`.
    ///
    /// Returns a flat list of all entries. Use `WalkOptions` to control
    /// whether ignored entries are included and maximum depth.
    pub fn walk(&self, dir: &Path, options: &WalkOptions) -> VfsResult<Vec<Entry>> {
        if !self.contains(dir) {
            return Err(VfsError::OutsideLocation(dir.to_path_buf()));
        }

        let mut results = Vec::new();
        self.walk_recursive(dir, options, 0, &mut results)?;
        Ok(results)
    }

    fn walk_recursive(
        &self,
        dir: &Path,
        options: &WalkOptions,
        depth: usize,
        results: &mut Vec<Entry>,
    ) -> VfsResult<()> {
        if let Some(max) = options.max_depth {
            if depth >= max {
                return Ok(());
            }
        }

        // Gracefully handle permission denied - skip inaccessible directories
        let read_dir = match std::fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    // Skip directories we can't access (e.g., ~/Library/Application Support/MobileSync)
                    return Ok(());
                }
                std::io::ErrorKind::NotFound => return Err(VfsError::NotFound(dir.to_path_buf())),
                _ => return Err(VfsError::Io { source: e }),
            },
        };

        for dir_entry in read_dir {
            // Skip entries we can't read
            let dir_entry = match dir_entry {
                Ok(de) => de,
                Err(_) => continue,
            };
            let path = dir_entry.path();
            let name = dir_entry.file_name().to_string_lossy().into_owned();

            // Always skip .fracta directory
            if name == FRACTA_DIR {
                continue;
            }

            // Skip entries where we can't get metadata
            let metadata = match dir_entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let entry = self.build_entry(&path, &metadata);

            // Skip ignored entries unless explicitly requested
            if entry.scope == Scope::Ignored && !options.include_ignored {
                continue;
            }

            let should_recurse = entry.kind == EntryKind::Folder;
            results.push(entry);

            if should_recurse {
                // Continue walking even if a subdirectory fails
                let _ = self.walk_recursive(&path, options, depth + 1, results);
            }
        }

        Ok(())
    }
}

// ── CRUD operations ────────────────────────────────────────────────────

impl Location {
    /// Create a new file with the given content (atomic write).
    pub fn create_file(&self, path: &Path, content: &[u8]) -> VfsResult<()> {
        self.check_writable(path)?;
        if path.exists() {
            return Err(VfsError::AlreadyExists(path.to_path_buf()));
        }
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(VfsError::NotFound(parent.to_path_buf()));
            }
        }
        atomic_write(path, content)
    }

    /// Create a new directory.
    pub fn create_folder(&self, path: &Path) -> VfsResult<()> {
        self.check_writable(path)?;
        if path.exists() {
            return Err(VfsError::AlreadyExists(path.to_path_buf()));
        }
        std::fs::create_dir(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => VfsError::PermissionDenied(path.to_path_buf()),
            _ => VfsError::Io { source: e },
        })
    }

    /// Write content to an existing file (atomic overwrite).
    pub fn write_file(&self, path: &Path, content: &[u8]) -> VfsResult<()> {
        self.check_writable(path)?;
        if !path.exists() {
            return Err(VfsError::NotFound(path.to_path_buf()));
        }
        atomic_write(path, content)
    }

    /// Read a file's contents as bytes.
    pub fn read_file(&self, path: &Path) -> VfsResult<Vec<u8>> {
        if !self.contains(path) {
            return Err(VfsError::OutsideLocation(path.to_path_buf()));
        }
        std::fs::read(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => VfsError::NotFound(path.to_path_buf()),
            std::io::ErrorKind::PermissionDenied => VfsError::PermissionDenied(path.to_path_buf()),
            _ => VfsError::Io { source: e },
        })
    }

    /// Read a file's contents as a UTF-8 string.
    pub fn read_file_string(&self, path: &Path) -> VfsResult<String> {
        let bytes = self.read_file(path)?;
        String::from_utf8(bytes).map_err(|_| VfsError::Io {
            source: std::io::Error::new(std::io::ErrorKind::InvalidData, "file is not valid UTF-8"),
        })
    }

    /// Rename a file or folder (stays in the same parent directory).
    pub fn rename(&self, from: &Path, to: &Path) -> VfsResult<()> {
        self.check_writable(from)?;
        self.check_writable(to)?;
        if !from.exists() {
            return Err(VfsError::NotFound(from.to_path_buf()));
        }
        if to.exists() {
            return Err(VfsError::AlreadyExists(to.to_path_buf()));
        }
        std::fs::rename(from, to).map_err(|e| VfsError::Io { source: e })
    }

    /// Move a file or folder to a different directory. Returns the new path.
    pub fn move_entry(&self, from: &Path, to_dir: &Path) -> VfsResult<PathBuf> {
        self.check_writable(from)?;
        if !self.contains(to_dir) {
            return Err(VfsError::OutsideLocation(to_dir.to_path_buf()));
        }
        if !from.exists() {
            return Err(VfsError::NotFound(from.to_path_buf()));
        }
        if !to_dir.is_dir() {
            return Err(VfsError::NotFound(to_dir.to_path_buf()));
        }

        let file_name = from.file_name().ok_or_else(|| VfsError::Io {
            source: std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no file name"),
        })?;
        let dest = to_dir.join(file_name);

        if dest.exists() {
            return Err(VfsError::AlreadyExists(dest));
        }

        self.check_writable(&dest)?;
        std::fs::rename(from, &dest).map_err(|e| VfsError::Io { source: e })?;
        Ok(dest)
    }

    /// Delete a file.
    pub fn delete_file(&self, path: &Path) -> VfsResult<()> {
        self.check_writable(path)?;
        if !path.exists() {
            return Err(VfsError::NotFound(path.to_path_buf()));
        }
        std::fs::remove_file(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => VfsError::PermissionDenied(path.to_path_buf()),
            _ => VfsError::Io { source: e },
        })
    }

    /// Delete a folder and all its contents.
    pub fn delete_folder(&self, path: &Path) -> VfsResult<()> {
        self.check_writable(path)?;
        if !path.exists() {
            return Err(VfsError::NotFound(path.to_path_buf()));
        }
        std::fs::remove_dir_all(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => VfsError::PermissionDenied(path.to_path_buf()),
            _ => VfsError::Io { source: e },
        })
    }
}

// ── Internal helpers ───────────────────────────────────────────────────

impl Location {
    /// Build an Entry from a path and its filesystem metadata.
    fn build_entry(&self, path: &Path, metadata: &std::fs::Metadata) -> Entry {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();

        let kind = if metadata.is_dir() {
            EntryKind::Folder
        } else {
            EntryKind::File
        };

        let extension = if kind == EntryKind::File {
            path.extension().map(|e| e.to_string_lossy().to_lowercase())
        } else {
            None
        };

        let scope = self.scope_of(path).unwrap_or(Scope::Plain);

        Entry {
            path: path.to_path_buf(),
            kind,
            name,
            extension,
            size: metadata.len(),
            modified: metadata.modified().ok().map(DateTime::from),
            created: metadata.created().ok().map(DateTime::from),
            scope,
        }
    }

    /// Check that a path is within this Location and not inside `.fracta/`.
    fn check_writable(&self, path: &Path) -> VfsResult<()> {
        if !self.contains(path) {
            return Err(VfsError::OutsideLocation(path.to_path_buf()));
        }
        // Prevent CRUD operations inside .fracta/ (use init/writer directly for that)
        if let Some(rel) = self.relative_path(path) {
            if rel.starts_with(FRACTA_DIR) {
                return Err(VfsError::PermissionDenied(path.to_path_buf()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ── Basic Location tests ───────────────────────────────────────────

    #[test]
    fn test_location_contains() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir(root.join("subdir")).unwrap();
        std::fs::write(root.join("subdir/file.md"), "test").unwrap();

        let loc = Location::new("test", &root);

        // Existing file inside location
        assert!(loc.contains(&root.join("subdir/file.md")));
        // Non-existent but valid path inside location
        assert!(loc.contains(&root.join("subdir/new.md")));
        // Path outside location
        assert!(!loc.contains(Path::new("/tmp/other-location/file.md")));
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_escape_blocked() {
        use std::os::unix::fs::symlink;

        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Create a directory outside the location
        let outside = TempDir::new().unwrap();
        let secret_file = outside.path().join("secret.txt");
        std::fs::write(&secret_file, "sensitive data").unwrap();

        // Create a symlink inside the location pointing outside
        let evil_link = root.join("escape_link");
        symlink(outside.path(), &evil_link).unwrap();

        let loc = Location::new("test", &root);

        // The symlink itself exists inside the location
        assert!(evil_link.exists());
        // But contains() should return false because it resolves to outside
        assert!(!loc.contains(&evil_link));
        // And trying to access files through it should also fail
        assert!(!loc.contains(&evil_link.join("secret.txt")));
    }

    #[test]
    fn test_path_traversal_blocked() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        std::fs::create_dir(root.join("subdir")).unwrap();

        let loc = Location::new("test", &root);

        // Attempting path traversal with .. should fail
        // Even though subdir/../../../etc/passwd starts inside location,
        // it resolves to outside
        let traversal_path = root.join("subdir/../../../etc/passwd");
        assert!(!loc.contains(&traversal_path));
    }

    #[test]
    fn test_list_directory() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        std::fs::create_dir(root.join("folder_a")).unwrap();
        std::fs::create_dir(root.join("folder_b")).unwrap();
        std::fs::write(root.join("notes.md"), "# Hello").unwrap();
        std::fs::write(root.join("data.json"), "{}").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let entries = loc.list_directory(&root).unwrap();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].kind, EntryKind::Folder);
        assert_eq!(entries[1].kind, EntryKind::Folder);
        assert_eq!(entries[2].kind, EntryKind::File);
        assert_eq!(entries[3].kind, EntryKind::File);
    }

    #[test]
    fn test_fracta_dir_hidden_in_listing() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        std::fs::create_dir(root.join(".fracta")).unwrap();
        std::fs::write(root.join("visible.md"), "hello").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let entries = loc.list_directory(&root).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "visible.md");
    }

    // ── Init + ignore rules tests ──────────────────────────────────────

    #[test]
    fn test_init_and_open() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Init a new Location
        let mut loc = Location::new("test", &root);
        loc.init().unwrap();
        assert!(loc.managed);
        assert!(root.join(".fracta/config/ignore").exists());

        // Open the same Location
        let loc2 = Location::open("test", &root).unwrap();
        assert!(loc2.managed);

        // Default ignore rules should work
        std::fs::create_dir(root.join(".git")).unwrap();
        assert_eq!(loc2.scope_of(&root.join(".git")), Some(Scope::Ignored));
    }

    #[test]
    fn test_scope_with_ignore_rules() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        let mut loc = Location::new("test", &root);
        loc.init().unwrap();

        // Create test files
        std::fs::write(root.join("readme.md"), "hi").unwrap();
        std::fs::create_dir(root.join("node_modules")).unwrap();
        std::fs::write(root.join(".DS_Store"), "").unwrap();

        // readme.md → Managed
        assert_eq!(loc.scope_of(&root.join("readme.md")), Some(Scope::Managed));
        // node_modules/ → Ignored
        assert_eq!(
            loc.scope_of(&root.join("node_modules")),
            Some(Scope::Ignored)
        );
        // .DS_Store → Ignored
        assert_eq!(loc.scope_of(&root.join(".DS_Store")), Some(Scope::Ignored));
        // .fracta/ → Managed (always)
        assert_eq!(loc.scope_of(&root.join(".fracta")), Some(Scope::Managed));
    }

    #[test]
    fn test_scope_unmanaged_is_plain() {
        let tmp = TempDir::new().unwrap();
        let loc = Location::new("test", tmp.path());
        assert_eq!(loc.scope_of(tmp.path()), Some(Scope::Plain));
    }

    #[test]
    fn test_scope_outside_location() {
        let loc = Location::new("test", "/tmp/my-location");
        assert_eq!(loc.scope_of(Path::new("/tmp/other")), None);
    }

    // ── Walk tests ─────────────────────────────────────────────────────

    #[test]
    fn test_walk_basic() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
        std::fs::write(root.join("readme.md"), "# Hello").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let entries = loc.walk(&root, &WalkOptions::default()).unwrap();
        assert_eq!(entries.len(), 3); // src/, src/main.rs, readme.md
    }

    #[test]
    fn test_walk_skips_ignored() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        let mut loc = Location::new("test", &root);
        loc.init().unwrap();

        // Create managed and ignored content
        std::fs::write(root.join("readme.md"), "# Hello").unwrap();
        std::fs::create_dir(root.join("node_modules")).unwrap();
        std::fs::write(root.join("node_modules/pkg.json"), "{}").unwrap();

        // Default walk: ignored entries excluded
        let entries = loc.walk(&root, &WalkOptions::default()).unwrap();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"readme.md"));
        assert!(!names.contains(&"node_modules"));
        assert!(!names.contains(&"pkg.json"));

        // Walk with include_ignored: everything visible
        let opts = WalkOptions {
            include_ignored: true,
            max_depth: None,
        };
        let entries = loc.walk(&root, &opts).unwrap();
        let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"readme.md"));
        assert!(names.contains(&"node_modules"));
    }

    #[test]
    fn test_walk_max_depth() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        std::fs::create_dir_all(root.join("a/b/c")).unwrap();
        std::fs::write(root.join("a/b/c/deep.txt"), "deep").unwrap();

        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        // Depth 1: only immediate children
        let opts = WalkOptions {
            include_ignored: false,
            max_depth: Some(1),
        };
        let entries = loc.walk(&root, &opts).unwrap();
        assert_eq!(entries.len(), 1); // just "a/"
        assert_eq!(entries[0].name, "a");
    }

    // ── CRUD tests ─────────────────────────────────────────────────────

    #[test]
    fn test_create_and_read_file() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let path = root.join("test.md");
        loc.create_file(&path, b"# Hello").unwrap();
        assert_eq!(loc.read_file_string(&path).unwrap(), "# Hello");
    }

    #[test]
    fn test_create_file_already_exists() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let path = root.join("test.md");
        loc.create_file(&path, b"v1").unwrap();

        let err = loc.create_file(&path, b"v2").unwrap_err();
        assert!(matches!(err, VfsError::AlreadyExists(_)));
    }

    #[test]
    fn test_write_file_overwrites() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let path = root.join("test.md");
        loc.create_file(&path, b"v1").unwrap();
        loc.write_file(&path, b"v2").unwrap();
        assert_eq!(loc.read_file_string(&path).unwrap(), "v2");
    }

    #[test]
    fn test_create_and_delete_folder() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let folder = root.join("new_folder");
        loc.create_folder(&folder).unwrap();
        assert!(folder.is_dir());

        // Put a file inside
        loc.create_file(&folder.join("file.txt"), b"data").unwrap();

        // Delete folder recursively
        loc.delete_folder(&folder).unwrap();
        assert!(!folder.exists());
    }

    #[test]
    fn test_rename() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let old_path = root.join("old.md");
        let new_path = root.join("new.md");
        loc.create_file(&old_path, b"content").unwrap();
        loc.rename(&old_path, &new_path).unwrap();

        assert!(!old_path.exists());
        assert_eq!(loc.read_file_string(&new_path).unwrap(), "content");
    }

    #[test]
    fn test_move_entry() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        loc.create_folder(&root.join("dest")).unwrap();
        loc.create_file(&root.join("file.md"), b"data").unwrap();

        let new_path = loc
            .move_entry(&root.join("file.md"), &root.join("dest"))
            .unwrap();
        assert_eq!(new_path, root.join("dest/file.md"));
        assert!(!root.join("file.md").exists());
        assert_eq!(loc.read_file_string(&new_path).unwrap(), "data");
    }

    #[test]
    fn test_cannot_write_inside_fracta_dir() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let mut loc = Location::new("test", &root);
        loc.init().unwrap();

        let err = loc
            .create_file(&root.join(".fracta/hack.txt"), b"bad")
            .unwrap_err();
        assert!(matches!(err, VfsError::PermissionDenied(_)));
    }

    #[test]
    fn test_outside_location_rejected() {
        let tmp = TempDir::new().unwrap();
        let loc = Location::new("test", tmp.path());

        let err = loc
            .create_file(Path::new("/tmp/outside.txt"), b"bad")
            .unwrap_err();
        assert!(matches!(err, VfsError::OutsideLocation(_)));
    }

    #[test]
    fn test_delete_file() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let loc = Location {
            id: Uuid::now_v7(),
            label: "test".into(),
            root: root.clone(),
            managed: true,
            ignore_rules: IgnoreRules::empty(),
        };

        let path = root.join("delete_me.txt");
        loc.create_file(&path, b"gone soon").unwrap();
        assert!(path.exists());

        loc.delete_file(&path).unwrap();
        assert!(!path.exists());
    }
}
