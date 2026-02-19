//! Filesystem watcher.
//!
//! Watches a Location for file changes and emits events that other subsystems
//! (Index, Pipelines) can react to.
//!
//! Uses `notify-debouncer-mini` for cross-platform watching with debouncing
//! (coalesces rapid changes into single events). Events accumulate in a
//! thread-safe queue; consumers call `drain_events()` to retrieve them.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind, Debouncer};

use crate::error::{VfsError, VfsResult};

/// Events emitted by the filesystem watcher.
#[derive(Debug, Clone)]
pub enum FsEvent {
    /// A file or folder was created.
    Created(PathBuf),
    /// A file was modified.
    Modified(PathBuf),
    /// A file or folder was deleted.
    Deleted(PathBuf),
    /// A file or folder was renamed.
    Renamed { from: PathBuf, to: PathBuf },
}

/// Filesystem watcher for a Location root.
///
/// Accumulates debounced events in a queue. Call `drain_events()` to
/// consume them. Thread-safe — the watcher runs on a background thread.
pub struct LocationWatcher {
    _debouncer: Debouncer<RecommendedWatcher>,
    events: Arc<Mutex<Vec<FsEvent>>>,
    root: PathBuf,
}

impl LocationWatcher {
    /// Start watching a directory tree.
    ///
    /// Events are debounced with a 500ms window to coalesce rapid changes.
    pub fn start(root: &Path) -> VfsResult<Self> {
        let events: Arc<Mutex<Vec<FsEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        let root_buf = root.to_path_buf();

        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            move |result: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
                match result {
                    Ok(debounced_events) => {
                        let mut queue = events_clone.lock().unwrap();
                        for event in debounced_events {
                            // Skip .fracta/ internal changes
                            if event.path.components().any(|c| c.as_os_str() == ".fracta") {
                                continue;
                            }

                            let fs_event = match event.kind {
                                DebouncedEventKind::Any => {
                                    // notify-debouncer-mini coalesces all change types into Any
                                    if event.path.exists() {
                                        FsEvent::Modified(event.path)
                                    } else {
                                        FsEvent::Deleted(event.path)
                                    }
                                }
                                DebouncedEventKind::AnyContinuous | _ => {
                                    FsEvent::Modified(event.path)
                                }
                            };
                            queue.push(fs_event);
                        }
                    }
                    Err(_) => {
                        // Watcher errors are non-fatal — log and continue
                    }
                }
            },
        )
        .map_err(|e| VfsError::WatcherError(e.to_string()))?;

        debouncer
            .watcher()
            .watch(root, RecursiveMode::Recursive)
            .map_err(|e| VfsError::WatcherError(e.to_string()))?;

        Ok(LocationWatcher {
            _debouncer: debouncer,
            events,
            root: root_buf,
        })
    }

    /// Drain all accumulated events, returning them and clearing the queue.
    pub fn drain_events(&self) -> Vec<FsEvent> {
        let mut queue = self.events.lock().unwrap();
        std::mem::take(&mut *queue)
    }

    /// Check if there are pending events without consuming them.
    pub fn has_pending_events(&self) -> bool {
        let queue = self.events.lock().unwrap();
        !queue.is_empty()
    }

    /// Get the root path being watched.
    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;

    /// Canonicalize path to handle macOS /var -> /private/var symlink
    fn canon(p: &Path) -> PathBuf {
        p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
    }

    #[test]
    fn test_watcher_detects_file_change() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = canon(tmp.path());

        // Create a file before starting watcher
        let file_path = root.join("test.md");
        fs::write(&file_path, "initial").unwrap();

        let watcher = LocationWatcher::start(&root).unwrap();

        // Modify the file
        fs::write(&file_path, "modified").unwrap();

        // Wait for debounce window
        thread::sleep(Duration::from_millis(800));

        let events = watcher.drain_events();
        assert!(!events.is_empty(), "Expected at least one event");

        // Should be a Modified event
        let has_modify = events
            .iter()
            .any(|e| matches!(e, FsEvent::Modified(p) if p == &file_path));
        assert!(
            has_modify,
            "Expected Modified event for {:?}, got {:?}",
            file_path, events
        );
    }

    #[test]
    fn test_watcher_detects_file_delete() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = canon(tmp.path());

        let file_path = root.join("deleteme.md");
        fs::write(&file_path, "content").unwrap();

        let watcher = LocationWatcher::start(&root).unwrap();

        // Delete the file
        fs::remove_file(&file_path).unwrap();

        // Wait for debounce window
        thread::sleep(Duration::from_millis(800));

        let events = watcher.drain_events();
        assert!(!events.is_empty(), "Expected at least one event");

        let has_delete = events
            .iter()
            .any(|e| matches!(e, FsEvent::Deleted(p) if p == &file_path));
        assert!(
            has_delete,
            "Expected Deleted event for {:?}, got {:?}",
            file_path, events
        );
    }

    #[test]
    fn test_watcher_skips_fracta_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        // Create .fracta directory
        let fracta_dir = root.join(".fracta");
        fs::create_dir_all(&fracta_dir).unwrap();

        let watcher = LocationWatcher::start(root).unwrap();

        // Write inside .fracta/
        fs::write(fracta_dir.join("cache.db"), "data").unwrap();

        // Wait for debounce window
        thread::sleep(Duration::from_millis(800));

        let events = watcher.drain_events();
        // Events from .fracta/ should be filtered out
        let has_fracta = events.iter().any(|e| {
            let path = match e {
                FsEvent::Created(p) | FsEvent::Modified(p) | FsEvent::Deleted(p) => p,
                FsEvent::Renamed { to, .. } => to,
            };
            path.components().any(|c| c.as_os_str() == ".fracta")
        });
        assert!(!has_fracta, "Should not emit events for .fracta/ directory");
    }

    #[test]
    fn test_drain_events_clears_queue() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        let watcher = LocationWatcher::start(root).unwrap();

        fs::write(root.join("a.md"), "content").unwrap();
        thread::sleep(Duration::from_millis(800));

        let events1 = watcher.drain_events();
        assert!(!events1.is_empty());

        // Second drain should be empty (no new events)
        let events2 = watcher.drain_events();
        assert!(events2.is_empty(), "Queue should be empty after drain");
    }
}
