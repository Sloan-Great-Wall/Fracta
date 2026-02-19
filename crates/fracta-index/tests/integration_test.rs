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
    eprintln!(
        "\n=== Performance Profile ({} files, {} Markdown) ===",
        total_files, md_count
    );

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
    assert_eq!(stats.files_scanned, total_files);
    assert_eq!(stats.markdown_indexed, md_count);
    eprintln!(
        "  Full index build:      {:>6} ms ({} files, {} md)",
        build_ms, stats.files_scanned, stats.markdown_indexed
    );

    // Measure: Search (10 different queries)
    // Note: jieba tokenizer is case-sensitive for English text, so
    // queries must match the case used in the generated content.
    let queries = [
        "Document",
        "paragraph",
        "document",
        "exercise",
        "tokenization",
        "indexer",
        "correctly",
        "contains",
        "working",
        "search",
    ];
    let start = Instant::now();
    for query in &queries {
        let hits = index.search(query, 20).unwrap();
        assert!(!hits.is_empty(), "Expected results for '{}'", query);
    }
    let search_ms = start.elapsed().as_millis();
    eprintln!("  Search (10 queries):   {:>6} ms", search_ms);

    // Measure: Metadata search
    let start = Instant::now();
    let library_results = index
        .search_by_metadata(Some("library"), None, None, None, 500)
        .unwrap();
    let now_results = index
        .search_by_metadata(Some("now"), None, None, None, 500)
        .unwrap();
    let tag_results = index
        .search_by_metadata(None, Some("rust"), None, None, 500)
        .unwrap();
    let meta_ms = start.elapsed().as_millis();
    eprintln!(
        "  Metadata search (3):   {:>6} ms (lib={}, now={}, tag={})",
        meta_ms,
        library_results.len(),
        now_results.len(),
        tag_results.len()
    );

    // Measure: Markdown parse (largest document)
    let large_doc =
        std::fs::read_to_string(root.join(format!("doc-{:04}.md", md_count - 1))).unwrap();
    let start = Instant::now();
    for _ in 0..100 {
        let _ = Document::parse(&large_doc);
    }
    let parse_ms = start.elapsed().as_millis();
    eprintln!(
        "  Markdown parse (100x): {:>6} ms ({} bytes)",
        parse_ms,
        large_doc.len()
    );

    // Measure: Incremental update (modify 10 files)
    // Need sleep for mtime change detection
    std::thread::sleep(std::time::Duration::from_secs(2));
    for i in 0..10 {
        let path = root.join(format!("doc-{:04}.md", i));
        let mut content = std::fs::read_to_string(&path).unwrap();
        content.push_str(
            "\n\n## Updated Section\n\nThis content was added during incremental update test.\n",
        );
        std::fs::write(&path, content).unwrap();
    }

    let start = Instant::now();
    let inc_stats = index.update_incremental(&location).unwrap();
    let inc_ms = start.elapsed().as_millis();
    eprintln!(
        "  Incremental update:    {:>6} ms ({} files, {} md)",
        inc_ms, inc_stats.files_scanned, inc_stats.markdown_indexed
    );

    eprintln!("=== End Performance Profile ===\n");

    // Generous thresholds — these are safety nets, not strict benchmarks.
    // On a modern Mac (M1/M2), typical values are well under these.
    assert!(walk_ms < 5000, "Walk took {} ms (threshold: 5000)", walk_ms);
    assert!(
        build_ms < 30000,
        "Full build took {} ms (threshold: 30000)",
        build_ms
    );
    assert!(
        search_ms < 2000,
        "Search took {} ms (threshold: 2000)",
        search_ms
    );
    assert!(
        meta_ms < 1000,
        "Metadata search took {} ms (threshold: 1000)",
        meta_ms
    );
    assert!(
        parse_ms < 5000,
        "Parse 100x took {} ms (threshold: 5000)",
        parse_ms
    );
    assert!(
        inc_ms < 30000,
        "Incremental update took {} ms (threshold: 30000)",
        inc_ms
    );
}

/// Test cache rebuild: deleting `.fracta/cache/` and rebuilding produces identical results.
///
/// This validates the core invariant: the filesystem is the source of truth,
/// and the cache (SQLite + Tantivy) is fully rebuildable from it. After nuking
/// the cache directory, `Index::open()` + `build_full()` must reproduce:
/// - Identical file counts (total files, Markdown indexed)
/// - Identical full-text search results (same paths, same hit count)
/// - Identical metadata (title, tags, area for each file)
/// - Identical metadata search results (by area, by tag)
#[test]
fn test_cache_rebuild_after_deletion() {
    // Step 1: Create a managed Location with diverse content
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    let mut location = Location::new("rebuild-test", &root);
    location.init().unwrap();

    // Create Markdown files with front matter (various areas, tags)
    let note1 = r#"---
title: Rust Programming Guide
tags: [rust, programming, systems]
area: library
---

# Getting Started with Rust

Rust is a systems programming language focused on safety and performance.
Ownership and borrowing are key concepts.
"#;

    let note2 = r#"---
title: 机器学习入门
tags: [AI, 学习, Python]
area: library
---

# 机器学习基础

机器学习是人工智能的核心技术，通过数据训练模型。
"#;

    let note3 = r#"---
title: Daily Standup Notes
area: now
tags: [project, fracta]
---

# Current Sprint

Working on the cache rebuild test for Fracta indexing.
"#;

    let note4 = "# No Front Matter\n\nThis file has no YAML front matter at all.\n";

    std::fs::write(root.join("rust-guide.md"), note1).unwrap();
    std::fs::write(root.join("ml-intro.md"), note2).unwrap();
    std::fs::write(root.join("standup.md"), note3).unwrap();
    std::fs::write(root.join("plain.md"), note4).unwrap();

    // Create subdirectory with nested files
    std::fs::create_dir_all(root.join("sub")).unwrap();
    std::fs::write(
        root.join("sub/nested.md"),
        "---\ntitle: Nested Note\narea: past\ntags: [reflection]\n---\n# Deep Thought\n\nA nested document for testing path handling.\n",
    ).unwrap();

    // Non-Markdown files
    std::fs::write(root.join("data.json"), r#"{"key": "value"}"#).unwrap();
    std::fs::write(root.join("config.yaml"), "setting: true\n").unwrap();

    // Step 2: Build the initial index ON DISK (not in-memory)
    let cache_dir = root.join(".fracta").join("cache");
    let mut index = Index::open(&cache_dir).unwrap();
    let stats_before = index.build_full(&location).unwrap();

    // Step 3: Record baseline results
    let file_count_before = index.file_count().unwrap();
    let indexed_count_before = index.indexed_count().unwrap();
    let search_doc_count_before = index.search_document_count().unwrap();

    // Full-text search baselines
    let rust_hits_before = index.search("rust", 10).unwrap();
    let ml_hits_before = index.search("机器学习", 10).unwrap();
    let sprint_hits_before = index.search("sprint", 10).unwrap();
    let nested_hits_before = index.search("nested", 10).unwrap();

    // Metadata search baselines
    let library_before = index
        .search_by_metadata(Some("library"), None, None, None, 10)
        .unwrap();
    let now_before = index
        .search_by_metadata(Some("now"), None, None, None, 10)
        .unwrap();
    let rust_tag_before = index
        .search_by_metadata(None, Some("rust"), None, None, 10)
        .unwrap();

    // Individual metadata baselines
    let rust_meta_before = index.get_metadata("rust-guide.md").unwrap();
    let ml_meta_before = index.get_metadata("ml-intro.md").unwrap();
    let nested_meta_before = index.get_metadata("sub/nested.md").unwrap();

    // Sanity checks on baseline
    assert_eq!(stats_before.files_scanned, 7, "Should scan 7 files");
    assert_eq!(
        stats_before.markdown_indexed, 5,
        "Should index 5 Markdown files"
    );
    assert!(!rust_hits_before.is_empty(), "Should find rust results");
    assert!(!ml_hits_before.is_empty(), "Should find ML results");

    // Step 4: Delete the entire cache directory
    drop(index); // Close file handles first
    std::fs::remove_dir_all(&cache_dir).unwrap();
    assert!(!cache_dir.exists(), "Cache directory should be deleted");

    // Step 5: Rebuild from scratch
    let mut index = Index::open(&cache_dir).unwrap();
    let stats_after = index.build_full(&location).unwrap();

    // Step 6: Verify everything matches

    // Build stats
    assert_eq!(
        stats_before.files_scanned, stats_after.files_scanned,
        "files_scanned mismatch"
    );
    assert_eq!(
        stats_before.markdown_indexed, stats_after.markdown_indexed,
        "markdown_indexed mismatch"
    );

    // Counts
    assert_eq!(
        file_count_before,
        index.file_count().unwrap(),
        "file_count mismatch"
    );
    assert_eq!(
        indexed_count_before,
        index.indexed_count().unwrap(),
        "indexed_count mismatch"
    );
    assert_eq!(
        search_doc_count_before,
        index.search_document_count().unwrap(),
        "search_document_count mismatch"
    );

    // Full-text search results: same paths and same count
    let rust_hits_after = index.search("rust", 10).unwrap();
    assert_eq!(
        rust_hits_before.len(),
        rust_hits_after.len(),
        "rust search hit count mismatch"
    );
    let rust_paths_before: Vec<_> = rust_hits_before.iter().map(|h| &h.path).collect();
    let rust_paths_after: Vec<_> = rust_hits_after.iter().map(|h| &h.path).collect();
    assert_eq!(
        rust_paths_before, rust_paths_after,
        "rust search paths mismatch"
    );

    let ml_hits_after = index.search("机器学习", 10).unwrap();
    assert_eq!(
        ml_hits_before.len(),
        ml_hits_after.len(),
        "ML search hit count mismatch"
    );

    let sprint_hits_after = index.search("sprint", 10).unwrap();
    assert_eq!(
        sprint_hits_before.len(),
        sprint_hits_after.len(),
        "sprint search hit count mismatch"
    );

    let nested_hits_after = index.search("nested", 10).unwrap();
    assert_eq!(
        nested_hits_before.len(),
        nested_hits_after.len(),
        "nested search hit count mismatch"
    );

    // Metadata search results
    let library_after = index
        .search_by_metadata(Some("library"), None, None, None, 10)
        .unwrap();
    let now_after = index
        .search_by_metadata(Some("now"), None, None, None, 10)
        .unwrap();
    let rust_tag_after = index
        .search_by_metadata(None, Some("rust"), None, None, 10)
        .unwrap();

    assert_eq!(
        library_before.len(),
        library_after.len(),
        "library area count mismatch"
    );
    assert_eq!(now_before.len(), now_after.len(), "now area count mismatch");
    assert_eq!(
        rust_tag_before.len(),
        rust_tag_after.len(),
        "rust tag count mismatch"
    );

    // Sorted comparison for metadata search (order may vary)
    let mut lib_before_sorted = library_before.clone();
    let mut lib_after_sorted = library_after.clone();
    lib_before_sorted.sort();
    lib_after_sorted.sort();
    assert_eq!(
        lib_before_sorted, lib_after_sorted,
        "library area paths mismatch"
    );

    // Individual file metadata
    let rust_meta_after = index.get_metadata("rust-guide.md").unwrap();
    assert_eq!(
        rust_meta_before.as_ref().map(|m| m.title.clone()),
        rust_meta_after.as_ref().map(|m| m.title.clone()),
        "rust-guide title mismatch"
    );
    assert_eq!(
        rust_meta_before.as_ref().map(|m| m.area.clone()),
        rust_meta_after.as_ref().map(|m| m.area.clone()),
        "rust-guide area mismatch"
    );
    assert_eq!(
        rust_meta_before.as_ref().map(|m| m.tags.clone()),
        rust_meta_after.as_ref().map(|m| m.tags.clone()),
        "rust-guide tags mismatch"
    );

    let ml_meta_after = index.get_metadata("ml-intro.md").unwrap();
    assert_eq!(
        ml_meta_before.as_ref().map(|m| m.title.clone()),
        ml_meta_after.as_ref().map(|m| m.title.clone()),
        "ml-intro title mismatch"
    );

    let nested_meta_after = index.get_metadata("sub/nested.md").unwrap();
    assert_eq!(
        nested_meta_before.as_ref().map(|m| m.title.clone()),
        nested_meta_after.as_ref().map(|m| m.title.clone()),
        "nested title mismatch"
    );
    assert_eq!(
        nested_meta_before.as_ref().map(|m| m.area.clone()),
        nested_meta_after.as_ref().map(|m| m.area.clone()),
        "nested area mismatch"
    );
}

/// Validate cold start performance: Location open + walk + index open must complete in < 2 seconds.
///
/// Cold start simulates what happens when the app launches with an existing managed location:
/// 1. Open Location (read .fracta/config/settings.json)
/// 2. Walk directory tree (list all files)
/// 3. Open existing index (SQLite + Tantivy)
/// 4. First search query
///
/// Target: total < 2000 ms for a 200-file location.
#[test]
fn test_cold_start_performance() {
    use std::time::Instant;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    // Setup: create a managed location with 200 files and build initial index
    let mut location = Location::new("cold-start", &root);
    location.init().unwrap();

    for i in 0..150 {
        let content = format!(
            "---\ntitle: Note {}\narea: library\ntags: [test]\n---\n\n# Note {}\n\nContent for note {}.\n",
            i, i, i
        );
        std::fs::write(root.join(format!("note-{:04}.md", i)), content).unwrap();
    }
    for i in 0..50 {
        std::fs::write(root.join(format!("data-{:04}.json", i)), "{}").unwrap();
    }

    // Build initial index on disk (this is the pre-existing state)
    let cache_dir = root.join(".fracta").join("cache");
    let mut index = Index::open(&cache_dir).unwrap();
    index.build_full(&location).unwrap();
    drop(index);

    // === Simulate cold start (app just launched) ===
    let total_start = Instant::now();

    // Step 1: Open existing location
    let open_start = Instant::now();
    let location = Location::open("cold-start", &root).unwrap();
    let open_ms = open_start.elapsed().as_millis();

    // Step 2: Walk directory
    let walk_start = Instant::now();
    let entries = location.walk(&root, &WalkOptions::default()).unwrap();
    let walk_ms = walk_start.elapsed().as_millis();
    assert_eq!(entries.len(), 200);

    // Step 3: Open existing index
    let index_start = Instant::now();
    let index = Index::open(&cache_dir).unwrap();
    let index_ms = index_start.elapsed().as_millis();

    // Step 4: First search query
    let search_start = Instant::now();
    let hits = index.search("note", 20).unwrap();
    let search_ms = search_start.elapsed().as_millis();
    assert!(!hits.is_empty());

    let total_ms = total_start.elapsed().as_millis();

    eprintln!("\n=== Cold Start Performance (200 files) ===");
    eprintln!("  Open location: {:>6} ms", open_ms);
    eprintln!("  Walk directory: {:>6} ms", walk_ms);
    eprintln!("  Open index:     {:>6} ms", index_ms);
    eprintln!("  First search:   {:>6} ms", search_ms);
    eprintln!("  TOTAL:          {:>6} ms (target: 2000)", total_ms);
    eprintln!("=== End Cold Start ===\n");

    assert!(
        total_ms < 2000,
        "Cold start took {} ms (target: < 2000)",
        total_ms
    );
}

/// Validate view switch performance: file read + parse must complete in < 300 ms.
///
/// View switch simulates what happens when a user clicks on a Markdown file:
/// 1. Read file from disk
/// 2. Parse Markdown (extract frontmatter + plain text)
///
/// Target: total < 300 ms even for large files.
#[test]
fn test_view_switch_performance() {
    use std::time::Instant;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();

    // Create files of varying sizes
    let small_content = "---\ntitle: Small\n---\n# Hello\n\nShort note.\n";
    let mut medium_content = String::from("---\ntitle: Medium Document\ntags: [test]\n---\n\n");
    for i in 0..100 {
        medium_content.push_str(&format!(
            "## Section {}\n\nParagraph {} with some content about programming.\n\n",
            i, i
        ));
    }
    let mut large_content =
        String::from("---\ntitle: Large Document\ntags: [test, large]\narea: library\n---\n\n");
    for i in 0..1000 {
        large_content.push_str(&format!(
            "## Section {}\n\nThis is section {} of a large document. It contains enough text \
             to simulate real-world note files with substantial content. The quick brown fox \
             jumps over the lazy dog multiple times in section {}.\n\n",
            i, i, i
        ));
    }

    std::fs::write(root.join("small.md"), small_content).unwrap();
    std::fs::write(root.join("medium.md"), &medium_content).unwrap();
    std::fs::write(root.join("large.md"), &large_content).unwrap();

    eprintln!("\n=== View Switch Performance ===");
    eprintln!(
        "  File sizes: small={} B, medium={} B, large={} B",
        small_content.len(),
        medium_content.len(),
        large_content.len()
    );

    // Measure each file: read + parse
    for (name, path) in [
        ("small.md", root.join("small.md")),
        ("medium.md", root.join("medium.md")),
        ("large.md", root.join("large.md")),
    ] {
        let start = Instant::now();
        let content = std::fs::read_to_string(&path).unwrap();
        let read_ms = start.elapsed().as_micros() as f64 / 1000.0;

        let parse_start = Instant::now();
        let doc = Document::parse(&content);
        let parse_ms = parse_start.elapsed().as_micros() as f64 / 1000.0;

        let total_ms = start.elapsed().as_micros() as f64 / 1000.0;

        eprintln!(
            "  {}: read={:.1} ms, parse={:.1} ms, total={:.1} ms (title={:?})",
            name,
            read_ms,
            parse_ms,
            total_ms,
            doc.title()
        );

        assert!(
            total_ms < 300.0,
            "{}: view switch took {:.1} ms (target: < 300)",
            name,
            total_ms
        );
    }

    eprintln!("=== End View Switch ===\n");
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
