//! Managed scope model.
//!
//! Every path within a Location has one of three scopes (SPEC §6.3):
//!
//! - **Managed**: compute layer enabled (indexing, metadata, views, Folder Pages)
//! - **Ignored**: inside a managed Location but explicitly excluded by rules
//! - **Plain**: not in any managed Location; browse/open only

use serde::{Deserialize, Serialize};

/// The scope of a path within a Location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// Compute layer enabled: indexing, metadata, views, Folder Pages.
    Managed,

    /// Inside a managed Location but excluded by ignore rules.
    /// Behaves like Plain — visible but not indexed or processed.
    Ignored,

    /// Not in any managed Location (or not yet enabled).
    /// Browse and open only.
    Plain,
}

impl Scope {
    /// Whether this scope allows indexing and compute features.
    pub fn is_managed(&self) -> bool {
        matches!(self, Scope::Managed)
    }
}
