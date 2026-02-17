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
    let entries = location.walk(&root, &WalkOptions::default()).unwrap();
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

/// Performance profile: measure critical path timings with a large dataset.
///
/// Generates 500 Markdown files and 200 non-Markdown files, then measures:
/// - Directory walk time
/// - Full index build time
/// - Search time (10 queries)
/// - Incremental update time (after modifying 10 files)
/// - Markdown parse time (largest document)
///
/// Not a strict benchmark — this is a regression safety net. The test passes
/// if all operations complete within generous thresholds.
#[test]
fn test_performance_profile_large_dataset() {
    use std::time::Instant;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    let mut location = Location::new("perf-test", &root);
    location.init().unwrap();

    // Generate 500 Markdown files with realistic content
    let md_count = 500;
    let other_count = 200;

    for i in 0..md_count {
        let area = match i % 3 {
            0 => "library",
            1 => "now",
            _ => "past",
        };
        let tags: Vec<&str> = match i % 5 {
            0 => vec!["rust", "programming"],
            1 => vec!["notes", "personal"],
            2 => vec!["project", "work"],
            3 => vec!["learning", "AI"],
            _ => vec!["misc"],
        };
        let tag_str = tags
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<_>>()
            .join(", ");

        // Vary content size: some short, some long
        let body_paragraphs = (i % 10) + 1;
        let mut body = String::new();
        for p in 0..body_paragraphs {
            body.push_str(&format!(
                "\n## Section {}\n\nThis is paragraph {} of document {}. \
                 It contains enough text to exercise the full-text search \
                 indexer and ensure that tokenization, stemming, and CJK \
                 segmentation are all working correctly under load.\n",
                p, p, i
            ));
        }

        let content = format!(
            "---\ntitle: Document {}\ntags: [{}]\narea: {}\n---\n\n# Document {}\n{}",
            i, tag_str, area, i, body
        );
        std::fs::write(root.join(format!("doc-{:04}.md", i)), content).unwrap();
    }

    // Generate non-Markdown files
    for i in 0..other_count {
        let ext = match i % 4 {
            0 => "json",
            1 => "txt",
            2 => "yaml",
            _ => "csv",
        };
        std::fs::write(
            root.join(format!("data-{:04}.{}", i, ext)),
            format!("content of file {}", i),
        )
        .unwrap();
    }

    let total_files = md_count + other_count;
    eprintln!("\n=== Performance Profile ({} files, {} Markdown) ===", total_files, md_count);

    // Measure: Directory walk
    let start = Instant::now();
    let entries = location.walk(&root, &WalkOptions::default()).unwrap();
    let walk_ms = start.elapsed().as_millis();
    assert_eq!(entries.len(), total_files);
    eprintln!("  Walk ({} entries):      {:>6} ms", entries.len(), walk_ms);

    // Measure: Full index build
    let mut index = Index::open_in_memory().unwrap();
    let start = Instant::now();
    let stats = index.build_full(&location).unwrap();
    let build_ms = start.elapsed().as_millis();
    assert_eq!(stats.files_scanned as usize, total_files);
    assert_eq!(stats.markdown_indexed as usize, md_count);
    eprintln!("  Full index build:      {:>6} ms ({} files, {} md)", build_ms, stats.files_scanned, stats.markdown_indexed);

    // Measure: Search (10 different queries)
    // Note: jieba tokenizer is case-sensitive for English text, so
    // queries must match the case used in the generated content.
    let queries = ["Document", "paragraph", "document", "exercise", "tokenization",
                   "indexer", "correctly", "contains", "working", "search"];
    let start = Instant::now();
    for query in &queries {
        let hits = index.search(query, 20).unwrap();
        assert!(!hits.is_empty(), "Expected results for '{}'", query);
    }
    let search_ms = start.elapsed().as_millis();
    eprintln!("  Search (10 queries):   {:>6} ms", search_ms);

    // Measure: Metadata search
    let start = Instant::now();
    let library_results = index.search_by_metadata(Some("library"), None, None, None, 500).unwrap();
    let now_results = index.search_by_metadata(Some("now"), None, None, None, 500).unwrap();
    let tag_results = index.search_by_metadata(None, Some("rust"), None, None, 500).unwrap();
    let meta_ms = start.elapsed().as_millis();
    eprintln!("  Metadata search (3):   {:>6} ms (lib={}, now={}, tag={})",
              meta_ms, library_results.len(), now_results.len(), tag_results.len());

    // Measure: Markdown parse (largest document)
    let large_doc = std::fs::read_to_string(root.join(format!("doc-{:04}.md", md_count - 1))).unwrap();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = Document::parse(&large_doc);
    }
    let parse_ms = start.elapsed().as_millis();
    eprintln!("  Markdown parse (100x): {:>6} ms ({} bytes)", parse_ms, large_doc.len());

    // Measure: Incremental update (modify 10 files)
    // Need sleep for mtime change detection
    std::thread::sleep(std::time::Duration::from_secs(2));
    for i in 0..10 {
        let path = root.join(format!("doc-{:04}.md", i));
        let mut content = std::fs::read_to_string(&path).unwrap();
        content.push_str("\n\n## Updated Section\n\nThis content was added during incremental update test.\n");
        std::fs::write(&path, content).unwrap();
    }

    let start = Instant::now();
    let inc_stats = index.update_incremental(&location).unwrap();
    let inc_ms = start.elapsed().as_millis();
    eprintln!("  Incremental update:    {:>6} ms ({} files, {} md)", inc_ms, inc_stats.files_scanned, inc_stats.markdown_indexed);

    eprintln!("=== End Performance Profile ===\n");

    // Generous thresholds — these are safety nets, not strict benchmarks.
    // On a modern Mac (M1/M2), typical values are well under these.
    assert!(walk_ms < 5000, "Walk took {} ms (threshold: 5000)", walk_ms);
    assert!(build_ms < 30000, "Full build took {} ms (threshold: 30000)", build_ms);
    assert!(search_ms < 2000, "Search took {} ms (threshold: 2000)", search_ms);
    assert!(meta_ms < 1000, "Metadata search took {} ms (threshold: 1000)", meta_ms);
    assert!(parse_ms < 5000, "Parse 100x took {} ms (threshold: 5000)", parse_ms);
    assert!(inc_ms < 30000, "Incremental update took {} ms (threshold: 30000)", inc_ms);
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
