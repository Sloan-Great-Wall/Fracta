//! # fracta-ffi — UniFFI Bridge
//!
//! UniFFI bridge: exposes Rust Core APIs to platform shells (Swift/Kotlin).
//!
//! This crate is the single entry-point for platform shells into the
//! Rust Core. It re-exports a curated, FFI-safe subset of VFS, Note,
//! Index, and AI APIs. UniFFI generates the Swift/Kotlin bindings
//! automatically from the proc-macro annotations.
//!
//! ## Architecture
//!
//! ```text
//! Swift/Kotlin App
//!       │
//!       ▼
//! ┌─────────────┐
//! │ fracta-ffi  │  ← This crate (UniFFI exports)
//! └──────┬──────┘
//!        │
//!   ┌────┴────┬────────────┬──────┐
//!   ▼         ▼            ▼      ▼
//! VFS       Note        Index    AI
//! ```

use std::path::PathBuf;
use std::sync::Mutex;

// Set up UniFFI scaffolding
uniffi::setup_scaffolding!();

// ═══════════════════════════════════════════════════════════════════════════
// Error Types
// ═══════════════════════════════════════════════════════════════════════════

/// FFI error type exposed to Swift/Kotlin.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiError {
    /// File or directory not found.
    #[error("Not found: {path}")]
    NotFound { path: String },

    /// Path is outside the managed Location.
    #[error("Path outside location: {path}")]
    OutsideLocation { path: String },

    /// Permission denied (e.g., writing to .fracta/).
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    /// File or directory already exists.
    #[error("Already exists: {path}")]
    AlreadyExists { path: String },

    /// IO error.
    #[error("IO error: {message}")]
    Io { message: String },

    /// Index error.
    #[error("Index error: {message}")]
    Index { message: String },

    /// Invalid argument.
    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    /// Internal error.
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl From<fracta_vfs::VfsError> for FfiError {
    fn from(e: fracta_vfs::VfsError) -> Self {
        match e {
            fracta_vfs::VfsError::NotFound(p) => FfiError::NotFound {
                path: p.display().to_string(),
            },
            fracta_vfs::VfsError::OutsideLocation(p) => FfiError::OutsideLocation {
                path: p.display().to_string(),
            },
            fracta_vfs::VfsError::PermissionDenied(p) => FfiError::PermissionDenied {
                path: p.display().to_string(),
            },
            fracta_vfs::VfsError::AlreadyExists(p) => FfiError::AlreadyExists {
                path: p.display().to_string(),
            },
            _ => FfiError::Io {
                message: e.to_string(),
            },
        }
    }
}

impl From<fracta_index::IndexError> for FfiError {
    fn from(e: fracta_index::IndexError) -> Self {
        FfiError::Index {
            message: e.to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// VFS Types
// ═══════════════════════════════════════════════════════════════════════════

/// Scope of an entry within a Location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FfiScope {
    /// Managed by Fracta (indexed, searchable, AI-accessible).
    Managed,
    /// Ignored (visible but not indexed).
    Ignored,
    /// Plain (Location is not managed).
    Plain,
}

impl From<fracta_vfs::Scope> for FfiScope {
    fn from(s: fracta_vfs::Scope) -> Self {
        match s {
            fracta_vfs::Scope::Managed => FfiScope::Managed,
            fracta_vfs::Scope::Ignored => FfiScope::Ignored,
            fracta_vfs::Scope::Plain => FfiScope::Plain,
        }
    }
}

/// Type of filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FfiEntryKind {
    File,
    Folder,
}

impl From<fracta_vfs::EntryKind> for FfiEntryKind {
    fn from(k: fracta_vfs::EntryKind) -> Self {
        match k {
            fracta_vfs::EntryKind::File => FfiEntryKind::File,
            fracta_vfs::EntryKind::Folder => FfiEntryKind::Folder,
        }
    }
}

/// A filesystem entry (file or folder).
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiEntry {
    /// Absolute path to the entry.
    pub path: String,
    /// Entry name (filename or folder name).
    pub name: String,
    /// File extension (lowercase, without dot), if any.
    pub extension: Option<String>,
    /// Entry kind (file or folder).
    pub kind: FfiEntryKind,
    /// Size in bytes.
    pub size: u64,
    /// Last modified time (ISO 8601).
    pub modified: String,
    /// Creation time (ISO 8601), if available.
    pub created: Option<String>,
    /// Scope within the Location.
    pub scope: FfiScope,
}

impl From<fracta_vfs::Entry> for FfiEntry {
    fn from(e: fracta_vfs::Entry) -> Self {
        FfiEntry {
            path: e.path.display().to_string(),
            name: e.name,
            extension: e.extension,
            kind: e.kind.into(),
            size: e.size,
            modified: e.modified.map(|m| m.to_rfc3339()).unwrap_or_default(),
            created: e.created.map(|c| c.to_rfc3339()),
            scope: e.scope.into(),
        }
    }
}

/// Options for directory traversal.
#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct FfiWalkOptions {
    /// Include ignored entries in results.
    pub include_ignored: bool,
    /// Maximum depth (None = unlimited).
    pub max_depth: Option<u32>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Location (VFS)
// ═══════════════════════════════════════════════════════════════════════════

/// A managed Location (directory tree).
///
/// Thread-safe wrapper around fracta_vfs::Location.
#[derive(uniffi::Object)]
pub struct FfiLocation {
    inner: Mutex<fracta_vfs::Location>,
}

#[uniffi::export]
impl FfiLocation {
    /// Create a new unmanaged Location.
    #[uniffi::constructor]
    pub fn new(label: String, root: String) -> Self {
        let location = fracta_vfs::Location::new(label, PathBuf::from(root));
        FfiLocation {
            inner: Mutex::new(location),
        }
    }

    /// Open an existing managed Location.
    #[uniffi::constructor]
    pub fn open(label: String, root: String) -> Result<Self, FfiError> {
        let location = fracta_vfs::Location::open(label, PathBuf::from(root))?;
        Ok(FfiLocation {
            inner: Mutex::new(location),
        })
    }

    /// Initialize this Location (create .fracta/ structure).
    pub fn init(&self) -> Result<(), FfiError> {
        let mut location = self.inner.lock().unwrap();
        location.init()?;
        Ok(())
    }

    /// Get the Location's root path.
    pub fn root(&self) -> String {
        let location = self.inner.lock().unwrap();
        location.root.display().to_string()
    }

    /// Get the Location's label.
    pub fn label(&self) -> String {
        let location = self.inner.lock().unwrap();
        location.label.clone()
    }

    /// Check if the Location is managed.
    pub fn is_managed(&self) -> bool {
        let location = self.inner.lock().unwrap();
        location.managed
    }

    /// List entries in a directory.
    pub fn list_directory(&self, path: String) -> Result<Vec<FfiEntry>, FfiError> {
        let location = self.inner.lock().unwrap();
        let entries = location.list_directory(&PathBuf::from(path))?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    /// Recursively walk a directory tree.
    pub fn walk(&self, path: String, options: FfiWalkOptions) -> Result<Vec<FfiEntry>, FfiError> {
        let location = self.inner.lock().unwrap();
        let opts = fracta_vfs::WalkOptions {
            include_ignored: options.include_ignored,
            max_depth: options.max_depth.map(|d| d as usize),
        };
        let entries = location.walk(&PathBuf::from(path), &opts)?;
        Ok(entries.into_iter().map(Into::into).collect())
    }

    /// Get the scope of a path.
    pub fn scope_of(&self, path: String) -> Option<FfiScope> {
        let location = self.inner.lock().unwrap();
        location.scope_of(&PathBuf::from(path)).map(Into::into)
    }

    /// Read a file as UTF-8 string.
    pub fn read_file(&self, path: String) -> Result<String, FfiError> {
        let location = self.inner.lock().unwrap();
        let content = location.read_file_string(&PathBuf::from(path))?;
        Ok(content)
    }

    /// Read a file as bytes.
    pub fn read_file_bytes(&self, path: String) -> Result<Vec<u8>, FfiError> {
        let location = self.inner.lock().unwrap();
        let content = location.read_file(&PathBuf::from(path))?;
        Ok(content)
    }

    /// Write a file (atomic write).
    pub fn write_file(&self, path: String, content: String) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.write_file(&PathBuf::from(path), content.as_bytes())?;
        Ok(())
    }

    /// Write bytes to a file (atomic write).
    pub fn write_file_bytes(&self, path: String, content: Vec<u8>) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.write_file(&PathBuf::from(path), &content)?;
        Ok(())
    }

    /// Create a new file.
    pub fn create_file(&self, path: String, content: String) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.create_file(&PathBuf::from(path), content.as_bytes())?;
        Ok(())
    }

    /// Create a new folder.
    pub fn create_folder(&self, path: String) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.create_folder(&PathBuf::from(path))?;
        Ok(())
    }

    /// Delete a file.
    pub fn delete_file(&self, path: String) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.delete_file(&PathBuf::from(path))?;
        Ok(())
    }

    /// Delete a folder and all its contents.
    pub fn delete_folder(&self, path: String) -> Result<(), FfiError> {
        let location = self.inner.lock().unwrap();
        location.delete_folder(&PathBuf::from(path))?;
        Ok(())
    }

    /// Rename an entry (new_name is just the filename, not a full path).
    pub fn rename(&self, path: String, new_name: String) -> Result<String, FfiError> {
        let location = self.inner.lock().unwrap();
        let from = PathBuf::from(&path);
        let parent = from.parent().ok_or_else(|| FfiError::InvalidArgument {
            message: "Path has no parent directory".to_string(),
        })?;
        let to = parent.join(&new_name);
        location.rename(&from, &to)?;
        Ok(to.display().to_string())
    }

    /// Move an entry to a new parent directory.
    pub fn move_entry(&self, path: String, new_parent: String) -> Result<String, FfiError> {
        let location = self.inner.lock().unwrap();
        let new_path = location.move_entry(&PathBuf::from(path), &PathBuf::from(new_parent))?;
        Ok(new_path.display().to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Note Types
// ═══════════════════════════════════════════════════════════════════════════

/// A parsed Markdown document.
#[derive(uniffi::Object)]
pub struct FfiDocument {
    inner: fracta_note::Document,
}

#[uniffi::export]
impl FfiDocument {
    /// Parse a Markdown string.
    #[uniffi::constructor]
    pub fn parse(markdown: String) -> Self {
        let doc = fracta_note::Document::parse(&markdown);
        FfiDocument { inner: doc }
    }

    /// Get the document title (from front matter or first heading).
    pub fn title(&self) -> Option<String> {
        self.inner.title()
    }

    /// Get plain text content (for indexing).
    pub fn plain_text(&self) -> String {
        self.inner.plain_text()
    }

    /// Check if the document has front matter.
    pub fn has_front_matter(&self) -> bool {
        self.inner.front_matter.is_some()
    }

    /// Get a string field from front matter.
    pub fn front_matter_string(&self, key: String) -> Option<String> {
        self.inner
            .front_matter
            .as_ref()?
            .get_str(&key)
            .map(String::from)
    }

    /// Get a string list field from front matter.
    pub fn front_matter_string_list(&self, key: String) -> Option<Vec<String>> {
        self.inner
            .front_matter
            .as_ref()?
            .get_string_list(&key)
            .map(|v| v.into_iter().map(String::from).collect())
    }

    /// Get an integer field from front matter.
    pub fn front_matter_int(&self, key: String) -> Option<i64> {
        self.inner.front_matter.as_ref()?.get_i64(&key)
    }

    /// Get a float field from front matter.
    pub fn front_matter_float(&self, key: String) -> Option<f64> {
        self.inner.front_matter.as_ref()?.get_f64(&key)
    }

    /// Get a boolean field from front matter.
    pub fn front_matter_bool(&self, key: String) -> Option<bool> {
        self.inner.front_matter.as_ref()?.get_bool(&key)
    }

    /// Get the number of blocks in the document.
    pub fn block_count(&self) -> u32 {
        self.inner.blocks.len() as u32
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Index Types
// ═══════════════════════════════════════════════════════════════════════════

/// A search hit from full-text search.
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiSearchHit {
    /// Relative path to the file.
    pub path: String,
    /// Document title (if available).
    pub title: Option<String>,
    /// Search relevance score.
    pub score: f32,
}

impl From<fracta_index::SearchHit> for FfiSearchHit {
    fn from(h: fracta_index::SearchHit) -> Self {
        FfiSearchHit {
            path: h.path,
            title: h.title,
            score: h.score,
        }
    }
}

/// Statistics from an index build operation.
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiBuildStats {
    /// Number of files scanned.
    pub files_scanned: u32,
    /// Number of Markdown files indexed for search.
    pub markdown_indexed: u32,
    /// Number of metadata entries updated.
    pub metadata_updated: u32,
}

impl From<fracta_index::BuildStats> for FfiBuildStats {
    fn from(s: fracta_index::BuildStats) -> Self {
        FfiBuildStats {
            files_scanned: s.files_scanned as u32,
            markdown_indexed: s.markdown_indexed as u32,
            metadata_updated: s.metadata_updated as u32,
        }
    }
}

/// Full-text search index.
#[derive(uniffi::Object)]
pub struct FfiIndex {
    inner: Mutex<fracta_index::Index>,
}

#[uniffi::export]
impl FfiIndex {
    /// Open or create an index at the Location's .fracta/cache/ directory.
    #[uniffi::constructor]
    pub fn open(location: &FfiLocation) -> Result<Self, FfiError> {
        let loc = location.inner.lock().unwrap();
        let cache_dir = loc.root.join(".fracta").join("cache");
        let index = fracta_index::Index::open(&cache_dir)?;
        Ok(FfiIndex {
            inner: Mutex::new(index),
        })
    }

    /// Open or create an index at a specific cache directory path.
    #[uniffi::constructor]
    pub fn open_at(cache_dir: String) -> Result<Self, FfiError> {
        let index = fracta_index::Index::open(&PathBuf::from(cache_dir))?;
        Ok(FfiIndex {
            inner: Mutex::new(index),
        })
    }

    /// Open an in-memory index (for testing).
    #[uniffi::constructor]
    pub fn open_in_memory() -> Result<Self, FfiError> {
        let index = fracta_index::Index::open_in_memory()?;
        Ok(FfiIndex {
            inner: Mutex::new(index),
        })
    }

    /// Build a full index from scratch.
    pub fn build_full(&self, location: &FfiLocation) -> Result<FfiBuildStats, FfiError> {
        let loc = location.inner.lock().unwrap();
        let mut index = self.inner.lock().unwrap();
        let stats = index.build_full(&loc)?;
        Ok(stats.into())
    }

    /// Incrementally update the index (only changed files).
    pub fn update_incremental(&self, location: &FfiLocation) -> Result<FfiBuildStats, FfiError> {
        let loc = location.inner.lock().unwrap();
        let mut index = self.inner.lock().unwrap();
        let stats = index.update_incremental(&loc)?;
        Ok(stats.into())
    }

    /// Search for documents matching the query.
    pub fn search(&self, query: String, limit: u32) -> Result<Vec<FfiSearchHit>, FfiError> {
        let index = self.inner.lock().unwrap();
        let hits = index.search(&query, limit as usize)?;
        Ok(hits.into_iter().map(Into::into).collect())
    }

    /// Search files by metadata.
    pub fn search_by_metadata(
        &self,
        area: Option<String>,
        tag: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        limit: u32,
    ) -> Result<Vec<String>, FfiError> {
        let index = self.inner.lock().unwrap();
        let paths = index.search_by_metadata(
            area.as_deref(),
            tag.as_deref(),
            date_from.as_deref(),
            date_to.as_deref(),
            limit as usize,
        )?;
        Ok(paths)
    }

    /// Get the number of indexed files.
    pub fn file_count(&self) -> Result<u32, FfiError> {
        let index = self.inner.lock().unwrap();
        let count = index.file_count()?;
        Ok(count as u32)
    }

    /// Get the number of files indexed for full-text search.
    pub fn indexed_count(&self) -> Result<u32, FfiError> {
        let index = self.inner.lock().unwrap();
        let count = index.indexed_count()?;
        Ok(count as u32)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AI Types
// ═══════════════════════════════════════════════════════════════════════════

/// Role of a participant in a chat conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FfiChatRole {
    /// System prompt — sets behavior and context.
    System,
    /// User message.
    User,
    /// Assistant (AI) response.
    Assistant,
}

impl From<FfiChatRole> for fracta_ai::ChatRole {
    fn from(r: FfiChatRole) -> Self {
        match r {
            FfiChatRole::System => fracta_ai::ChatRole::System,
            FfiChatRole::User => fracta_ai::ChatRole::User,
            FfiChatRole::Assistant => fracta_ai::ChatRole::Assistant,
        }
    }
}

impl From<fracta_ai::ChatRole> for FfiChatRole {
    fn from(r: fracta_ai::ChatRole) -> Self {
        match r {
            fracta_ai::ChatRole::System => FfiChatRole::System,
            fracta_ai::ChatRole::User => FfiChatRole::User,
            fracta_ai::ChatRole::Assistant => FfiChatRole::Assistant,
        }
    }
}

/// A single message in a chat conversation.
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiChatMessage {
    /// Role of the message sender.
    pub role: FfiChatRole,
    /// Message content.
    pub content: String,
}

/// Response from an AI completion request.
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiCompletionResponse {
    /// The generated text.
    pub content: String,
    /// Approximate tokens consumed (prompt + completion).
    pub tokens_used: u32,
    /// Model identifier that generated this response.
    pub model: String,
}

impl From<fracta_ai::AiError> for FfiError {
    fn from(e: fracta_ai::AiError) -> Self {
        match e {
            fracta_ai::AiError::ProviderNotConfigured => FfiError::Internal {
                message: "AI provider not configured".to_string(),
            },
            other => FfiError::Internal {
                message: other.to_string(),
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AI Engine
// ═══════════════════════════════════════════════════════════════════════════

/// AI engine wrapping a pluggable provider.
///
/// Use `new_echo()` for development/testing, or future constructors
/// for cloud and local providers.
#[derive(uniffi::Object)]
pub struct FfiAiEngine {
    provider: Box<dyn fracta_ai::AiProvider>,
}

#[uniffi::export]
impl FfiAiEngine {
    /// Create an AI engine with the echo provider (for testing/development).
    #[uniffi::constructor]
    pub fn new_echo() -> Self {
        FfiAiEngine {
            provider: Box::new(fracta_ai::EchoProvider),
        }
    }

    /// Send a completion request.
    pub fn complete(
        &self,
        messages: Vec<FfiChatMessage>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<FfiCompletionResponse, FfiError> {
        let request = fracta_ai::CompletionRequest {
            messages: messages
                .into_iter()
                .map(|m| fracta_ai::ChatMessage {
                    role: m.role.into(),
                    content: m.content,
                })
                .collect(),
            max_tokens,
            temperature,
        };

        let response = self.provider.complete(&request)?;

        Ok(FfiCompletionResponse {
            content: response.content,
            tokens_used: response.tokens_used,
            model: response.model,
        })
    }

    /// Get the model name of the active provider.
    pub fn model_name(&self) -> String {
        self.provider.model_name().to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Filesystem Watcher
// ═══════════════════════════════════════════════════════════════════════════

/// Type of filesystem event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FfiFsEventKind {
    /// A file or folder was created.
    Created,
    /// A file was modified.
    Modified,
    /// A file or folder was deleted.
    Deleted,
    /// A file or folder was renamed.
    Renamed,
}

/// A filesystem change event.
#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiFsEvent {
    /// Type of change.
    pub kind: FfiFsEventKind,
    /// Absolute path to the affected file/folder.
    pub path: String,
    /// For rename events, the original path (before rename).
    pub renamed_from: Option<String>,
}

impl From<fracta_vfs::FsEvent> for FfiFsEvent {
    fn from(e: fracta_vfs::FsEvent) -> Self {
        match e {
            fracta_vfs::FsEvent::Created(p) => FfiFsEvent {
                kind: FfiFsEventKind::Created,
                path: p.display().to_string(),
                renamed_from: None,
            },
            fracta_vfs::FsEvent::Modified(p) => FfiFsEvent {
                kind: FfiFsEventKind::Modified,
                path: p.display().to_string(),
                renamed_from: None,
            },
            fracta_vfs::FsEvent::Deleted(p) => FfiFsEvent {
                kind: FfiFsEventKind::Deleted,
                path: p.display().to_string(),
                renamed_from: None,
            },
            fracta_vfs::FsEvent::Renamed { from, to } => FfiFsEvent {
                kind: FfiFsEventKind::Renamed,
                path: to.display().to_string(),
                renamed_from: Some(from.display().to_string()),
            },
        }
    }
}

/// Filesystem watcher for a Location root.
///
/// Watches a directory tree for changes and accumulates events.
/// Call `drain_events()` periodically (e.g. from a Swift Timer) to
/// retrieve pending changes and trigger incremental index updates.
#[derive(uniffi::Object)]
pub struct FfiWatcher {
    inner: Mutex<Option<fracta_vfs::LocationWatcher>>,
}

#[uniffi::export]
impl FfiWatcher {
    /// Start watching a directory tree.
    #[uniffi::constructor]
    pub fn start(root: String) -> Result<Self, FfiError> {
        let watcher = fracta_vfs::LocationWatcher::start(&PathBuf::from(&root))
            .map_err(|e| FfiError::Io {
                message: e.to_string(),
            })?;
        Ok(FfiWatcher {
            inner: Mutex::new(Some(watcher)),
        })
    }

    /// Drain all pending filesystem events.
    ///
    /// Returns accumulated events since the last drain and clears the queue.
    /// Call this from a periodic Timer on the Swift side.
    pub fn drain_events(&self) -> Vec<FfiFsEvent> {
        let guard = self.inner.lock().unwrap();
        match guard.as_ref() {
            Some(watcher) => watcher.drain_events().into_iter().map(Into::into).collect(),
            None => Vec::new(),
        }
    }

    /// Check if there are pending events without consuming them.
    pub fn has_pending_events(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        match guard.as_ref() {
            Some(watcher) => watcher.has_pending_events(),
            None => false,
        }
    }

    /// Stop watching. After this call, no new events will be accumulated.
    pub fn stop(&self) {
        let mut guard = self.inner.lock().unwrap();
        *guard = None;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Convenience Functions
// ═══════════════════════════════════════════════════════════════════════════

/// Parse a Markdown string and return plain text (convenience function).
#[uniffi::export]
pub fn parse_markdown_to_plain_text(markdown: String) -> String {
    fracta_note::Document::parse(&markdown).plain_text()
}

/// Get the Fracta FFI version.
#[uniffi::export]
pub fn ffi_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_location_lifecycle() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_str().unwrap().to_string();

        // Create and init location
        let location = FfiLocation::new("test".to_string(), root.clone());
        assert!(!location.is_managed());

        location.init().unwrap();
        assert!(location.is_managed());

        // Create a file
        let file_path = format!("{}/test.md", root);
        location
            .create_file(file_path.clone(), "# Hello".to_string())
            .unwrap();

        // Read it back
        let content = location.read_file(file_path.clone()).unwrap();
        assert_eq!(content, "# Hello");

        // List directory
        let entries = location.list_directory(root.clone()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "test.md");
        assert_eq!(entries[0].kind, FfiEntryKind::File);
        assert_eq!(entries[0].scope, FfiScope::Managed);
    }

    #[test]
    fn test_document_parsing() {
        let markdown = r#"---
title: Test Document
tags: [rust, ffi]
---

# Hello World

This is a test document.
"#;

        let doc = FfiDocument::parse(markdown.to_string());
        assert_eq!(doc.title(), Some("Test Document".to_string()));
        assert!(doc.has_front_matter());
        assert_eq!(
            doc.front_matter_string("title".to_string()),
            Some("Test Document".to_string())
        );
        assert_eq!(
            doc.front_matter_string_list("tags".to_string()),
            Some(vec!["rust".to_string(), "ffi".to_string()])
        );

        let plain = doc.plain_text();
        assert!(plain.contains("Hello World"));
        assert!(plain.contains("test document"));
    }

    #[test]
    fn test_index_search() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_str().unwrap().to_string();

        // Set up location with content
        let location = FfiLocation::new("test".to_string(), root.clone());
        location.init().unwrap();

        location
            .create_file(
                format!("{}/rust.md", root),
                "---\ntitle: Rust Guide\ntags: [programming]\n---\n# Rust\n\nRust is awesome."
                    .to_string(),
            )
            .unwrap();

        location
            .create_file(
                format!("{}/python.md", root),
                "---\ntitle: Python Guide\ntags: [programming]\n---\n# Python\n\nPython is easy."
                    .to_string(),
            )
            .unwrap();

        // Build index
        let index = FfiIndex::open_in_memory().unwrap();
        let stats = index.build_full(&location).unwrap();
        assert_eq!(stats.files_scanned, 2);
        assert_eq!(stats.markdown_indexed, 2);

        // Search
        let hits = index.search("Rust".to_string(), 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].path, "rust.md");

        // Search by tag
        let paths = index
            .search_by_metadata(None, Some("programming".to_string()), None, None, 10)
            .unwrap();
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_ffi_version() {
        let version = ffi_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_parse_markdown_to_plain_text() {
        let md = "# Title\n\nSome **bold** text.";
        let plain = parse_markdown_to_plain_text(md.to_string());
        assert!(plain.contains("Title"));
        assert!(plain.contains("bold"));
    }

    #[test]
    fn test_watcher_lifecycle() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().canonicalize().unwrap();
        let root_str = root.to_str().unwrap().to_string();

        // Start watcher
        let watcher = FfiWatcher::start(root_str).unwrap();
        assert!(!watcher.has_pending_events());

        // Create a file
        std::fs::write(root.join("test.md"), "hello").unwrap();

        // Wait for debounce
        std::thread::sleep(std::time::Duration::from_millis(800));

        // Drain events
        let events = watcher.drain_events();
        assert!(!events.is_empty(), "Expected filesystem events");

        // Stop watcher
        watcher.stop();
        assert!(!watcher.has_pending_events());

        // Drain after stop should be empty
        let events2 = watcher.drain_events();
        assert!(events2.is_empty());
    }

    #[test]
    fn test_ai_engine_echo() {
        let engine = FfiAiEngine::new_echo();
        assert_eq!(engine.model_name(), "echo-v1");

        let messages = vec![
            FfiChatMessage {
                role: FfiChatRole::System,
                content: "You are helpful.".to_string(),
            },
            FfiChatMessage {
                role: FfiChatRole::User,
                content: "What is Fracta?".to_string(),
            },
        ];

        let response = engine.complete(messages, None, None).unwrap();
        assert!(response.content.contains("What is Fracta?"));
        assert_eq!(response.model, "echo-v1");
        assert!(response.tokens_used > 0);
    }
}
