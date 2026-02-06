//! Location settings persistence.
//!
//! Handles reading and writing `.fracta/config/settings.json`, which stores
//! Location-level configuration including the persistent Location ID.

use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{VfsError, VfsResult};
use crate::location::FRACTA_DIR;
use crate::writer::{atomic_write_string, ensure_dir};

/// Location settings stored in `.fracta/config/settings.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocationSettings {
    /// Persistent Location ID (survives across sessions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    /// Location label (user-friendly name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl LocationSettings {
    /// Load settings from a Location root directory.
    ///
    /// Returns default settings if the file doesn't exist or is invalid.
    pub fn load(root: &Path) -> VfsResult<Self> {
        let path = root.join(FRACTA_DIR).join("config").join("settings.json");

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path).map_err(|e| VfsError::Io { source: e })?;

        serde_json::from_str(&content).map_err(|e| VfsError::Io {
            source: std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
        })
    }

    /// Save settings to a Location root directory.
    pub fn save(&self, root: &Path) -> VfsResult<()> {
        let config_dir = root.join(FRACTA_DIR).join("config");
        let path = config_dir.join("settings.json");

        // Ensure the config directory exists before writing
        ensure_dir(&config_dir)?;

        let content = serde_json::to_string_pretty(self).map_err(|e| VfsError::Io {
            source: std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()),
        })?;

        atomic_write_string(&path, &content)
    }

    /// Get or create a persistent ID.
    ///
    /// If no ID exists, generates a new one and returns it.
    /// The caller should save the settings after calling this if the ID was new.
    pub fn get_or_create_id(&mut self) -> Uuid {
        if let Some(id) = self.id {
            id
        } else {
            let id = Uuid::now_v7();
            self.id = Some(id);
            id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_missing_file() {
        let tmp = TempDir::new().unwrap();
        let settings = LocationSettings::load(tmp.path()).unwrap();
        assert!(settings.id.is_none());
    }

    #[test]
    fn test_save_and_load() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // Create .fracta/config directory
        std::fs::create_dir_all(root.join(".fracta/config")).unwrap();

        let mut settings = LocationSettings::default();
        let id = settings.get_or_create_id();
        settings.label = Some("Test Location".to_string());
        settings.save(root).unwrap();

        // Load and verify
        let loaded = LocationSettings::load(root).unwrap();
        assert_eq!(loaded.id, Some(id));
        assert_eq!(loaded.label, Some("Test Location".to_string()));
    }

    #[test]
    fn test_get_or_create_id_idempotent() {
        let mut settings = LocationSettings::default();
        let id1 = settings.get_or_create_id();
        let id2 = settings.get_or_create_id();
        assert_eq!(id1, id2);
    }
}
