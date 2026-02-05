//! SQLite metadata store.
//!
//! Stores file registry and extracted metadata (from front matter).
//! Used for structural queries: list files, filter by tags/date/area, etc.

use std::path::Path;

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// SQLite metadata store.
pub struct MetadataStore {
    conn: Connection,
}

/// File entry in the metadata store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Relative path from Location root.
    pub path: String,
    /// File modification time.
    pub mtime: DateTime<Utc>,
    /// File size in bytes.
    pub size: u64,
    /// Content hash (blake3, hex).
    pub content_hash: Option<String>,
    /// Whether this file is indexed for full-text search.
    pub indexed: bool,
}

/// Extracted metadata from front matter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Document title (from front matter or first h1).
    pub title: Option<String>,
    /// Tags list.
    pub tags: Vec<String>,
    /// Date (from front matter).
    pub date: Option<String>,
    /// Area: library, now, past.
    pub area: Option<String>,
}

/// Statistics from a metadata operation.
#[derive(Debug, Clone, Default)]
pub struct MetadataStats {
    pub files_added: usize,
    pub files_updated: usize,
    pub files_removed: usize,
}

impl MetadataStore {
    /// Open or create a metadata store at the given path.
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// Open an in-memory metadata store (for testing).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// Initialize the database schema.
    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- File registry: tracks all files in the Location
            CREATE TABLE IF NOT EXISTS files (
                path TEXT PRIMARY KEY,
                mtime TEXT NOT NULL,
                size INTEGER NOT NULL,
                content_hash TEXT,
                indexed INTEGER NOT NULL DEFAULT 0
            );

            -- Metadata: extracted from front matter
            CREATE TABLE IF NOT EXISTS metadata (
                path TEXT PRIMARY KEY REFERENCES files(path) ON DELETE CASCADE,
                title TEXT,
                tags TEXT,  -- JSON array
                date TEXT,
                area TEXT
            );

            -- Indexes for common queries
            CREATE INDEX IF NOT EXISTS idx_files_mtime ON files(mtime);
            CREATE INDEX IF NOT EXISTS idx_metadata_area ON metadata(area);
            CREATE INDEX IF NOT EXISTS idx_metadata_date ON metadata(date);
            "#,
        )?;
        Ok(())
    }

    /// Insert or update a file entry.
    pub fn upsert_file(&self, entry: &FileEntry) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO files (path, mtime, size, content_hash, indexed)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(path) DO UPDATE SET
                mtime = excluded.mtime,
                size = excluded.size,
                content_hash = excluded.content_hash,
                indexed = excluded.indexed
            "#,
            params![
                entry.path,
                entry.mtime.to_rfc3339(),
                entry.size as i64,
                entry.content_hash,
                entry.indexed,
            ],
        )?;
        Ok(())
    }

    /// Insert or update metadata for a file.
    pub fn upsert_metadata(&self, path: &str, metadata: &FileMetadata) -> Result<()> {
        let tags_json = serde_json::to_string(&metadata.tags).unwrap_or_default();
        self.conn.execute(
            r#"
            INSERT INTO metadata (path, title, tags, date, area)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(path) DO UPDATE SET
                title = excluded.title,
                tags = excluded.tags,
                date = excluded.date,
                area = excluded.area
            "#,
            params![path, metadata.title, tags_json, metadata.date, metadata.area],
        )?;
        Ok(())
    }

    /// Get a file entry by path.
    pub fn get_file(&self, path: &str) -> Result<Option<FileEntry>> {
        let entry = self
            .conn
            .query_row(
                "SELECT path, mtime, size, content_hash, indexed FROM files WHERE path = ?1",
                params![path],
                |row| {
                    let mtime_str: String = row.get(1)?;
                    let mtime = DateTime::parse_from_rfc3339(&mtime_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    Ok(FileEntry {
                        path: row.get(0)?,
                        mtime,
                        size: row.get::<_, i64>(2)? as u64,
                        content_hash: row.get(3)?,
                        indexed: row.get(4)?,
                    })
                },
            )
            .optional()?;
        Ok(entry)
    }

    /// Get metadata for a file.
    pub fn get_metadata(&self, path: &str) -> Result<Option<FileMetadata>> {
        let meta = self
            .conn
            .query_row(
                "SELECT title, tags, date, area FROM metadata WHERE path = ?1",
                params![path],
                |row| {
                    let tags_json: Option<String> = row.get(1)?;
                    let tags: Vec<String> = tags_json
                        .and_then(|s| serde_json::from_str(&s).ok())
                        .unwrap_or_default();
                    Ok(FileMetadata {
                        title: row.get(0)?,
                        tags,
                        date: row.get(2)?,
                        area: row.get(3)?,
                    })
                },
            )
            .optional()?;
        Ok(meta)
    }

    /// List all indexed file paths.
    pub fn list_indexed_paths(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path FROM files WHERE indexed = 1")?;
        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
        Ok(paths)
    }

    /// List all file paths.
    pub fn list_all_paths(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT path FROM files")?;
        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
        Ok(paths)
    }

    /// Remove a file entry and its metadata.
    pub fn remove_file(&self, path: &str) -> Result<bool> {
        let deleted = self
            .conn
            .execute("DELETE FROM files WHERE path = ?1", params![path])?;
        Ok(deleted > 0)
    }

    /// Remove files that no longer exist in the given set of paths.
    pub fn remove_stale_files(&self, current_paths: &[String]) -> Result<usize> {
        if current_paths.is_empty() {
            let deleted = self.conn.execute("DELETE FROM files", [])?;
            return Ok(deleted);
        }

        // Build a temporary table for efficient comparison
        self.conn.execute(
            "CREATE TEMP TABLE IF NOT EXISTS current_paths (path TEXT PRIMARY KEY)",
            [],
        )?;
        self.conn.execute("DELETE FROM current_paths", [])?;

        let mut stmt = self
            .conn
            .prepare("INSERT INTO current_paths (path) VALUES (?1)")?;
        for path in current_paths {
            stmt.execute(params![path])?;
        }

        let deleted = self.conn.execute(
            "DELETE FROM files WHERE path NOT IN (SELECT path FROM current_paths)",
            [],
        )?;

        self.conn.execute("DROP TABLE IF EXISTS current_paths", [])?;

        Ok(deleted)
    }

    /// Get total number of files.
    pub fn file_count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    /// Get number of indexed files.
    pub fn indexed_count(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files WHERE indexed = 1",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    /// List files in a directory (direct children only).
    pub fn list_directory(&self, dir: &str) -> Result<Vec<FileEntry>> {
        let pattern = if dir.is_empty() {
            // Root directory: match paths without '/'
            "%".to_string()
        } else {
            format!("{}/%", dir)
        };

        let mut stmt = self.conn.prepare(
            r#"
            SELECT path, mtime, size, content_hash, indexed FROM files
            WHERE path LIKE ?1
            AND path NOT LIKE ?2
            "#,
        )?;

        // Exclude nested paths (more than one level deep)
        let exclude_pattern = if dir.is_empty() {
            "%/%".to_string()
        } else {
            format!("{}/%/%", dir)
        };

        let entries = stmt
            .query_map(params![pattern, exclude_pattern], |row| {
                let mtime_str: String = row.get(1)?;
                let mtime = DateTime::parse_from_rfc3339(&mtime_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                Ok(FileEntry {
                    path: row.get(0)?,
                    mtime,
                    size: row.get::<_, i64>(2)? as u64,
                    content_hash: row.get(3)?,
                    indexed: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Search files by metadata criteria.
    pub fn search_by_metadata(
        &self,
        area: Option<&str>,
        tag: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        limit: usize,
    ) -> Result<Vec<String>> {
        let mut sql = String::from(
            r#"
            SELECT f.path FROM files f
            LEFT JOIN metadata m ON f.path = m.path
            WHERE 1=1
            "#,
        );
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(a) = area {
            sql.push_str(" AND m.area = ?");
            params_vec.push(Box::new(a.to_string()));
        }
        if let Some(t) = tag {
            sql.push_str(" AND m.tags LIKE ?");
            params_vec.push(Box::new(format!("%\"{}%", t)));
        }
        if let Some(df) = date_from {
            sql.push_str(" AND m.date >= ?");
            params_vec.push(Box::new(df.to_string()));
        }
        if let Some(dt) = date_to {
            sql.push_str(" AND m.date <= ?");
            params_vec.push(Box::new(dt.to_string()));
        }

        sql.push_str(" ORDER BY f.mtime DESC LIMIT ?");
        params_vec.push(Box::new(limit as i64));

        let mut stmt = self.conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let paths = stmt
            .query_map(params_refs.as_slice(), |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_query() {
        let store = MetadataStore::open_in_memory().unwrap();

        let entry = FileEntry {
            path: "notes/test.md".to_string(),
            mtime: Utc::now(),
            size: 1024,
            content_hash: Some("abc123".to_string()),
            indexed: true,
        };
        store.upsert_file(&entry).unwrap();

        let retrieved = store.get_file("notes/test.md").unwrap().unwrap();
        assert_eq!(retrieved.path, "notes/test.md");
        assert_eq!(retrieved.size, 1024);
        assert!(retrieved.indexed);
    }

    #[test]
    fn test_metadata() {
        let store = MetadataStore::open_in_memory().unwrap();

        let entry = FileEntry {
            path: "notes/test.md".to_string(),
            mtime: Utc::now(),
            size: 100,
            content_hash: None,
            indexed: true,
        };
        store.upsert_file(&entry).unwrap();

        let metadata = FileMetadata {
            title: Some("Test Note".to_string()),
            tags: vec!["rust".to_string(), "fracta".to_string()],
            date: Some("2025-01-15".to_string()),
            area: Some("library".to_string()),
        };
        store.upsert_metadata("notes/test.md", &metadata).unwrap();

        let retrieved = store.get_metadata("notes/test.md").unwrap().unwrap();
        assert_eq!(retrieved.title, Some("Test Note".to_string()));
        assert_eq!(retrieved.tags, vec!["rust", "fracta"]);
        assert_eq!(retrieved.area, Some("library".to_string()));
    }

    #[test]
    fn test_list_indexed() {
        let store = MetadataStore::open_in_memory().unwrap();

        store
            .upsert_file(&FileEntry {
                path: "a.md".to_string(),
                mtime: Utc::now(),
                size: 100,
                content_hash: None,
                indexed: true,
            })
            .unwrap();

        store
            .upsert_file(&FileEntry {
                path: "b.txt".to_string(),
                mtime: Utc::now(),
                size: 50,
                content_hash: None,
                indexed: false,
            })
            .unwrap();

        let indexed = store.list_indexed_paths().unwrap();
        assert_eq!(indexed, vec!["a.md"]);
    }

    #[test]
    fn test_remove_stale() {
        let store = MetadataStore::open_in_memory().unwrap();

        for name in ["a.md", "b.md", "c.md"] {
            store
                .upsert_file(&FileEntry {
                    path: name.to_string(),
                    mtime: Utc::now(),
                    size: 100,
                    content_hash: None,
                    indexed: true,
                })
                .unwrap();
        }

        assert_eq!(store.file_count().unwrap(), 3);

        // Only a.md and c.md still exist
        let current = vec!["a.md".to_string(), "c.md".to_string()];
        let removed = store.remove_stale_files(&current).unwrap();
        assert_eq!(removed, 1);
        assert_eq!(store.file_count().unwrap(), 2);
    }

    #[test]
    fn test_search_by_metadata() {
        let store = MetadataStore::open_in_memory().unwrap();

        // Create files with metadata
        for (path, area, tag) in [
            ("lib/a.md", "library", "rust"),
            ("lib/b.md", "library", "python"),
            ("now/c.md", "now", "rust"),
        ] {
            store
                .upsert_file(&FileEntry {
                    path: path.to_string(),
                    mtime: Utc::now(),
                    size: 100,
                    content_hash: None,
                    indexed: true,
                })
                .unwrap();
            store
                .upsert_metadata(
                    path,
                    &FileMetadata {
                        title: None,
                        tags: vec![tag.to_string()],
                        date: None,
                        area: Some(area.to_string()),
                    },
                )
                .unwrap();
        }

        // Search by area
        let results = store
            .search_by_metadata(Some("library"), None, None, None, 10)
            .unwrap();
        assert_eq!(results.len(), 2);

        // Search by tag
        let results = store
            .search_by_metadata(None, Some("rust"), None, None, 10)
            .unwrap();
        assert_eq!(results.len(), 2);

        // Search by area + tag
        let results = store
            .search_by_metadata(Some("library"), Some("rust"), None, None, 10)
            .unwrap();
        assert_eq!(results.len(), 1);
    }
}
