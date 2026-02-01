# Fracta — Engineering (Architecture + Implementation Guide)

This document is the canonical **technical** reference (architecture + implementation guidance).

- Canonical system definition: `docs/SPEC.md`
- Canonical scope/milestones: `docs/PRD.md`
- Decision history: `docs/adr/`

---

## 1. High-level architecture

### 1.1 One core, multiple shells

Fracta uses **one shared core** and **multiple platform shells**:

- **Rust Core (cross-platform)**: data model, indexing, search, query/view engine, event processing, cryptography/signing primitives, sync primitives.
- **Platform Shell (platform-specific)**:
  - Apple: Swift + SwiftUI for UI, OS permissions, Keychain/Secure Enclave, Spotlight/QuickLook, Apple AI frameworks.
  - Future: Windows/Linux (Tauri or native), Android (Kotlin/Compose).
- **FFI boundary**: UniFFI exposes Rust Core APIs to platform shells.

Reference: `docs/adr/0003-core-architecture-rust-core-platform-shells.md`.

### 1.2 Why AI lives in the shell (on Apple)

On Apple platforms, Apple AI frameworks must be called from Swift. Rust should not depend on those APIs; instead, Rust defines interfaces and consumes results.

---

## 2. Storage model (SOT vs Config vs Cache)

Fracta is file-based. The source of truth is user-readable content in normal folders:

- **SOT**: Markdown/CSV/JSON + assets folders.
- **Config**: schemas/views/ignore rules (text files; can be versioned).
- **Cache**: indexes, materialized views, derived stats, vectors (safe to delete).

For each managed location, Fracta creates exactly one system folder:

```
LocationRoot/
  .fracta/
    config/
      ignore
      settings.json
    meta/
      uids.jsonl
      links.jsonl
    cache/
      index.sqlite
      fts/
      vectors/
    state/
      last_runs.json
```

Notes:

- The concrete filenames are implementation details, but the **layering invariant** must hold (SPEC).
- Writes must be atomic (temp file → rename) to survive cloud-sync/concurrency.

References:

- `docs/adr/0008-metadata-persistence-anti-dsstore.md`
- `docs/adr/0007-managed-scope-and-ignore-rules.md`

---

## 3. Past subsystem (time journaling)

Past is a pipeline:

1. Ingest events from multiple sources
2. Normalize into a shared event model
3. De-duplicate and merge
4. Materialize daily folders (Markdown + CSV + assets)
5. Build derived stats and summaries (optional AI)

### 3.1 On-disk structure (human-readable)

Example:

```
My Drive/
  lifelong_story/
    2025/
      2025-10-01 — Travel prep/
        2025-10-01 — Travel prep.md
        events.csv
        assets/
          cover.jpg
          thumb_001.jpg
```

### 3.2 Story Markdown front matter (example)

```yaml
---
uid: "story_2025-10-01_9f2c1a"
title: "2025-10-01 — Travel prep"
date: "2025-10-01"
status: archived            # active | archived
area: lifelong_story        # database | now | past (conceptual grouping)
sources: [bilibili, youtube, chrome, healthkit]
privacy: internal           # public | internal | sensitive
files:
  events_csv: "./events.csv"
  assets_dir: "./assets/"
summary:
  ai: ""
  manual: ""
metrics:
  focus_minutes: 0
  video_minutes: 120
  sleep_hours: 7.5
tags: [journal, travel, health]
version: "1.0"
deduplicated: true
last_updated: "2025-10-01T22:30:00+08:00"
---
```

### 3.3 `events.csv` schema (initial)

`events.csv` is the primary structured artifact for Past.

| Field | Type | Meaning | Example |
|---|---|---|---|
| `event_id` | string | Unique event id | `bili_2025100112345` |
| `platform` | string | Source system | `bilibili`, `youtube`, `chrome` |
| `event_type` | string | Event kind | `video_watch`, `web_visit`, `sleep` |
| `title` | string | Title | Video or page title |
| `normalized_id` | string | Cross-run stable content id | `bili:BV1xx...` |
| `link` | url | Original link | `https://...` |
| `pic` | path | Relative asset path | `./assets/thumb_001.jpg` |
| `desc` | string | Summary/description | `...` |
| `start_datetime` | ISO8601 | Start time (with tz) | `2025-10-01T14:30:00+08:00` |
| `end_datetime` | ISO8601 | End time (with tz) | `2025-10-01T15:15:00+08:00` |
| `duration_seconds` | int | Duration in seconds | `2700` |
| `progress_seconds` | int | Progress, `-1` = unknown | `1800` |
| `device` | string | Device label | `MacBook Pro` |
| `confidence` | float | Ingest confidence (0..1) | `1.0` |
| `tags` | json array | User/AI tags | `["learning","tech"]` |
| `metadata_json` | json | Source-specific fields | `{...}` |
| `privacy` | string | Privacy label | `public`, `internal`, `sensitive` |
| `dedup_key` | string | Debug-only dedup string | `...` |
| `merged_from` | json array | Source event ids merged into this | `["...","..."]` |
| `quality_score` | float | Quality score | `1.0` |

### 3.4 De-duplication and merging (deterministic)

Initial rules:

- **Dedup key**: `platform + normalized_id + time_bucket`
- `time_bucket`: treat repeats within a small window (e.g., 1 hour) as the same logical event.

Merging (conceptual):

```
- Sort events within a day by start_datetime
- If gap < min(600s, duration/2), merge adjacent segments
```

Quality priority guideline:

`official_api (1.0) > cookie_scrape (0.8) > selenium (0.5) > estimation (0.3)`

---

## 4. Ingestion sources (initial targets)

| Source | Approach | Notes |
|---|---|---|
| Bilibili history | WebView cookie capture → official APIs | Cookies expire; refresh flow required |
| YouTube history | OAuth → YouTube Data API | Progress is often unavailable; set `progress_seconds = -1` |
| Chrome history | User-authorized copy of History DB → parse | Avoid fragile automation |
| Safari history | Safari Extension (optional) | Higher complexity under sandboxing |
| HealthKit | HealthKit framework | Steps/sleep/workouts; privacy-sensitive |

Reference: `docs/adr/0004-ingestion-strategy-platform-core-ingestors.md`.

---

## 5. Security and privacy

### 5.1 Credentials

Secrets must not be written into the filesystem. On Apple platforms, store secrets in Keychain:

```swift
KeychainManager.save(
    service: "com.fracta.bilibili",
    account: "cookie",
    data: cookieData,
    accessibility: .whenUnlockedThisDeviceOnly
)
```

### 5.2 Privacy levels

Use a small, explicit set of privacy labels:

```swift
enum PrivacyLevel: String {
    case public
    case internal
    case sensitive
}
```

Exports must filter by privacy level.

---

## 6. Engineering conventions (initial)

- **Atomic writes** everywhere for SOT/config/meta.
- **Caches must be rebuildable**; never store irreplaceable truth only in SQLite.
- **Lazy UID**: do not assign IDs until necessary (ADR-0005).
- **Docs are English-only** (ADR-0009).

