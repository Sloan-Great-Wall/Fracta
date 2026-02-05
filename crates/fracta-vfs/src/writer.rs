//! Atomic file writer.
//!
//! Ensures crash-safe writes by writing to a temporary file first,
//! then atomically renaming to the target path.
//!
//! This is a core invariant from SPEC §7.2:
//! "All writes MUST use atomic write patterns (write temp → fsync if needed → atomic rename)."

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::error::{VfsError, VfsResult};

/// Write data to a file atomically.
///
/// 1. Write to a temporary file in the same directory
/// 2. Flush and sync to disk
/// 3. Atomically rename temp file to target path
///
/// If the process crashes at any point, the original file is untouched.
pub fn atomic_write(path: &Path, data: &[u8]) -> VfsResult<()> {
    let parent = path.parent().ok_or_else(|| VfsError::AtomicWriteFailed {
        path: path.to_path_buf(),
        reason: "path has no parent directory".into(),
    })?;

    // Create a temp file in the same directory (ensures same filesystem for rename)
    let mut temp =
        tempfile::NamedTempFile::new_in(parent).map_err(|e| VfsError::AtomicWriteFailed {
            path: path.to_path_buf(),
            reason: format!("failed to create temp file: {e}"),
        })?;

    // Write all data
    temp.write_all(data)
        .map_err(|e| VfsError::AtomicWriteFailed {
            path: path.to_path_buf(),
            reason: format!("failed to write data: {e}"),
        })?;

    // Flush to OS
    temp.flush().map_err(|e| VfsError::AtomicWriteFailed {
        path: path.to_path_buf(),
        reason: format!("failed to flush: {e}"),
    })?;

    // Sync to disk (fsync)
    temp.as_file()
        .sync_all()
        .map_err(|e| VfsError::AtomicWriteFailed {
            path: path.to_path_buf(),
            reason: format!("failed to sync: {e}"),
        })?;

    // Atomic rename (this is the commit point)
    temp.persist(path)
        .map_err(|e| VfsError::AtomicWriteFailed {
            path: path.to_path_buf(),
            reason: format!("failed to rename: {e}"),
        })?;

    Ok(())
}

/// Write a UTF-8 string to a file atomically.
pub fn atomic_write_string(path: &Path, content: &str) -> VfsResult<()> {
    atomic_write(path, content.as_bytes())
}

/// Ensure a directory exists, creating it and parents if necessary.
pub fn ensure_dir(path: &Path) -> VfsResult<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_atomic_write() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("test.md");

        atomic_write_string(&file, "# Hello, Fracta!").unwrap();

        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "# Hello, Fracta!");
    }

    #[test]
    fn test_atomic_write_overwrites() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("test.md");

        atomic_write_string(&file, "version 1").unwrap();
        atomic_write_string(&file, "version 2").unwrap();

        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "version 2");
    }
}
