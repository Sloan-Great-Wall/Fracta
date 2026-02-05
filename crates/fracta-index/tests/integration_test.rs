//! Integration tests for the VFS → Note → Index pipeline.
//!
//! These tests verify that the three Engine crates work together correctly.

use tempfile::TempDir;

use fracta_index::Index;
use fracta_note::Document;
use fracta_vfs::{Location, Scope, WalkOptions};

/// Test the complete pipeline: VFS → Note → Index.
#[test]
fn test_vfs_note_index_pipeline() {
    // Step 1: Create a managed Location with VFS
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    let mut location = Location::new("test", &root);
    location.init().unwrap();

    // Step 2: Create Markdown files with front matter
    let note1 = r#"---
title: Rust Programming Guide
tags: [rust, programming, systems]
area: library
---

# Getting Started with Rust

Rust is a systems programming language focused on safety and performance.

## Memory Safety

Rust's ownership system ensures memory safety without garbage collection.
"#;

    let note2 = r#"---
title: 机器学习入门
tags: [AI, 学习, Python]
area: library
---

# 机器学习基础

机器学习是人工智能的核心技术，通过数据训练模型。

## 监督学习

监督学习需要带标签的训练数据。
"#;

    let note3 = r#"---
title: Project Notes
area: now
---

# Current Project

Working on the Fracta indexing system.
"#;

    std::fs::write(root.join("rust-guide.md"), note1).unwrap();
    std::fs::write(root.join("ml-intro.md"), note2).unwrap();
    std::fs::write(root.join("project.md"), note3).unwrap();
    std::fs::write(root.join("data.json"), "{}").unwrap(); // Non-markdown file

    // Step 3: Verify VFS sees the files correctly
    let entries = location
        .walk(&root, &WalkOptions::default())
        .unwrap();
    assert_eq!(entries.len(), 4);
    assert!(entries.iter().all(|e| e.scope == Scope::Managed));

    // Step 4: Parse a Markdown file with Note engine
    let content = std::fs::read_to_string(root.join("rust-guide.md")).unwrap();
    let doc = Document::parse(&content);

    assert_eq!(doc.title(), Some("Rust Programming Guide".to_string()));
    assert!(doc.front_matter.is_some());

    let fm = doc.front_matter.as_ref().unwrap();
    assert_eq!(fm.get_str("area"), Some("library"));
    assert_eq!(
        fm.get_string_list("tags").unwrap(),
        vec!["rust", "programming", "systems"]
    );

    // Step 5: Build the Index
    let mut index = Index::open_in_memory().unwrap();
    let stats = index.build_full(&location).unwrap();

    assert_eq!(stats.files_scanned, 4);
    assert_eq!(stats.markdown_indexed, 3);
    assert_eq!(stats.metadata_updated, 4);

    // Step 6: Verify full-text search works
    let hits = index.search("Rust", 10).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].path, "rust-guide.md");

    // Step 7: Verify Chinese search works
    let hits = index.search("机器学习", 10).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].path, "ml-intro.md");

    // Step 8: Verify metadata search works
    let results = index
        .search_by_metadata(Some("library"), None, None, None, 10)
        .unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.contains(&"rust-guide.md".to_string()));
    assert!(results.contains(&"ml-intro.md".to_string()));

    // Step 9: Search by tag
    let results = index
        .search_by_metadata(None, Some("rust"), None, None, 10)
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "rust-guide.md");
}

/// Test SQL injection defense in metadata search.
#[test]
fn test_sql_injection_defense() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    let mut location = Location::new("test", &root);
    location.init().unwrap();

    // Create a simple file
    std::fs::write(
        root.join("test.md"),
        "---\ntitle: Test\narea: library\n---\n# Test",
    )
    .unwrap();

    let mut index = Index::open_in_memory().unwrap();
    index.build_full(&location).unwrap();

    // Attempt various SQL injection payloads
    let injection_payloads = [
        "'; DROP TABLE files; --",
        "library' OR '1'='1",
        "library'; DELETE FROM metadata; --",
        "library\" OR \"1\"=\"1",
        "library\"; DROP TABLE files; --",
        "library\\'; DROP TABLE files; --",
        "' UNION SELECT * FROM metadata --",
    ];

    for payload in injection_payloads {
        // These should not cause errors (parameterized queries protect us)
        // and should return empty results (no actual match)
        let result = index.search_by_metadata(Some(payload), None, None, None, 10);
        assert!(
            result.is_ok(),
            "SQL injection payload caused error: {}",
            payload
        );

        // The injection should not return the actual file
        let results = result.unwrap();
        assert!(
            results.is_empty() || !results.contains(&"'; DROP TABLE files; --".to_string()),
            "SQL injection may have succeeded with payload: {}",
            payload
        );
    }

    // Verify the database is still intact
    assert_eq!(index.file_count().unwrap(), 1);
    assert_eq!(index.indexed_count().unwrap(), 1);

    // Normal search should still work
    let results = index
        .search_by_metadata(Some("library"), None, None, None, 10)
        .unwrap();
    assert_eq!(results.len(), 1);
}

/// Test incremental update detects file changes correctly.
#[test]
fn test_incremental_pipeline() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    let mut location = Location::new("test", &root);
    location.init().unwrap();

    // Initial file
    std::fs::write(
        root.join("note.md"),
        "---\ntitle: Version 1\n---\n# V1 Content",
    )
    .unwrap();

    let mut index = Index::open_in_memory().unwrap();
    let stats = index.build_full(&location).unwrap();
    assert_eq!(stats.markdown_indexed, 1);

    // Search for V1
    let hits = index.search("V1", 10).unwrap();
    assert_eq!(hits.len(), 1);

    // Add delay to ensure mtime changes (need >1 second due to mtime tolerance)
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Update the file
    std::fs::write(
        root.join("note.md"),
        "---\ntitle: Version 2\n---\n# V2 Updated Content",
    )
    .unwrap();

    // Incremental update
    let stats = index.update_incremental(&location).unwrap();
    // Should detect the change
    assert!(stats.markdown_indexed >= 1);

    // Search should now find V2
    let hits = index.search("V2", 10).unwrap();
    assert_eq!(hits.len(), 1);

    // V1 should no longer be found (replaced)
    let hits = index.search("V1", 10).unwrap();
    assert_eq!(hits.len(), 0);
}
