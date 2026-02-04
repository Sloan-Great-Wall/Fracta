//! Location initialization.
//!
//! Creates the `.fracta/` directory structure within a Location root.
//! This is called once when the user first adds a Location to Fracta.

use std::path::Path;

use crate::error::VfsResult;
use crate::ignore::DEFAULT_IGNORE;
use crate::location::FRACTA_DIR;
use crate::writer::{atomic_write_string, ensure_dir};

/// Subdirectories created during initialization.
const INIT_DIRS: &[&str] = &[
    "config",
    "config/schemas",
    "config/views",
    "meta",
    "cache",
    "state",
];

/// Initialize the `.fracta/` directory structure at the given Location root.
///
/// Creates the directory tree and default configuration files.
/// Safe to call on an already-initialized Location (idempotent — does not
/// overwrite existing files).
pub fn init_fracta_dir(root: &Path) -> VfsResult<()> {
    let fracta = root.join(FRACTA_DIR);

    // Create all subdirectories
    for subdir in INIT_DIRS {
        ensure_dir(&fracta.join(subdir))?;
    }

    // Write default ignore file (only if it doesn't exist)
    let ignore_path = fracta.join("config").join("ignore");
    if !ignore_path.exists() {
        atomic_write_string(&ignore_path, DEFAULT_IGNORE)?;
    }

    // Write default settings.json (only if it doesn't exist)
    let settings_path = fracta.join("config").join("settings.json");
    if !settings_path.exists() {
        atomic_write_string(&settings_path, "{}\n")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_structure() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        init_fracta_dir(root).unwrap();

        assert!(root.join(".fracta").is_dir());
        assert!(root.join(".fracta/config").is_dir());
        assert!(root.join(".fracta/config/schemas").is_dir());
        assert!(root.join(".fracta/config/views").is_dir());
        assert!(root.join(".fracta/meta").is_dir());
        assert!(root.join(".fracta/cache").is_dir());
        assert!(root.join(".fracta/state").is_dir());

        assert!(root.join(".fracta/config/ignore").is_file());
        assert!(root.join(".fracta/config/settings.json").is_file());
    }

    #[test]
    fn test_init_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // Write a custom ignore file before init
        std::fs::create_dir_all(root.join(".fracta/config")).unwrap();
        std::fs::write(root.join(".fracta/config/ignore"), "custom_rule\n").unwrap();

        // Init should NOT overwrite existing files
        init_fracta_dir(root).unwrap();

        let content = std::fs::read_to_string(root.join(".fracta/config/ignore")).unwrap();
        assert_eq!(content, "custom_rule\n");
    }

    #[test]
    fn test_default_ignore_content() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        init_fracta_dir(root).unwrap();

        let content = std::fs::read_to_string(root.join(".fracta/config/ignore")).unwrap();
        assert!(content.contains(".git/"));
        assert!(content.contains(".DS_Store"));
        assert!(content.contains("node_modules/"));
    }
}
