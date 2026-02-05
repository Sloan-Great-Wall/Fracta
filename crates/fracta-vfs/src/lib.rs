//! # fracta-vfs — Virtual File System
//!
//! Engine-layer subsystem providing file/folder operations, filesystem watching,
//! atomic writes, and managed scope (Managed / Ignored / Plain).
//!
//! This is the foundation layer. Every other Engine and Framework crate depends
//! on VFS for file access. VFS has no knowledge of Framework concepts (Quests,
//! Events, etc.) — it only understands files, folders, and Locations.
//!
//! ## Architecture
//!
//! - `Location`: a user-granted directory tree (local folder, cloud-sync folder, etc.)
//! - `IgnoreRules`: gitignore-style patterns that determine Managed vs Ignored scope
//! - `Scope`: each path within a Location is Managed, Ignored, or Plain
//! - `Entry`: metadata about a file or folder (name, size, timestamps, scope)
//! - `Watcher`: observes filesystem changes and emits events
//! - `AtomicWriter`: ensures crash-safe writes (temp → fsync → rename)
//!
//! ## Design rules (from SPEC §4.1)
//!
//! - VFS exposes a trait interface. Engine never depends on Framework or Application.
//! - All writes use atomic patterns (temp file → fsync → rename).
//! - No `.DS_Store`-style pollution: system data lives in `.fracta/` at Location root.

pub mod entry;
pub mod error;
pub mod ignore;
pub mod init;
pub mod location;
pub mod scope;
pub mod settings;
pub mod watcher;
pub mod writer;

pub use entry::{Entry, EntryKind};
pub use error::{VfsError, VfsResult};
pub use ignore::IgnoreRules;
pub use init::init_fracta_dir;
pub use location::{Location, WalkOptions, FRACTA_DIR};
pub use scope::Scope;
pub use settings::LocationSettings;
