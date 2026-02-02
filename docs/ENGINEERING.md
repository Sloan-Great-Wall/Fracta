# Fracta — Engineering (Architecture + Implementation Guide)

This document is the canonical **technical** reference (architecture + implementation guidance).

- Canonical system definition: `docs/SPEC.md`
- Canonical scope/milestones: `docs/PRD.md`
- Decision history: `docs/adr/`

---

## 1. High-level architecture

### 1.1 Two models coexist

Fracta has two complementary architecture models:

- **Conceptual architecture (ADR-0010)**: Engine / Framework / Application — three layers defining dependency rules and API boundaries.
- **Implementation architecture (ADR-0003)**: Rust Core / Platform Shell — two code-organization units defining what language runs where.

The Rust Core implements both the Engine and Framework layers (cross-platform logic). The Platform Shell implements Application UI and Platform Adapters (OS-specific code). UniFFI bridges the two.

```
Conceptual layers          Implementation
─────────────────          ──────────────
Application  ─────────→    Platform Shell (Swift/SwiftUI)
Framework    ─────────→    Rust Core (via UniFFI)
Engine       ─────────→    Rust Core + Platform Adapters
```

See SPEC §4 for the full three-layer specification.

### 1.2 Rust Core + Platform Shells

- **Rust Core (cross-platform)**: Engine subsystems (VFS, Index, Query, Note, Comm, Sync, Crypto Primitives, AI Primitives) + Framework components (Semantic Primitives, Pipelines, AI Orchestrator, Crypto/Token Engine, Knowledge Engine, Extension API).
- **Platform Shell (platform-specific)**:
  - Apple: Swift + SwiftUI for UI, OS permissions, Keychain/Secure Enclave, Spotlight/QuickLook, Apple AI frameworks.
  - Future: Windows/Linux (Tauri or native), Android (Kotlin/Compose).
- **FFI boundary**: UniFFI exposes Rust Core APIs to platform shells.

Reference: `docs/adr/0003-core-architecture-rust-core-platform-shells.md`.

### 1.3 Why AI lives in the shell (on Apple)

On Apple platforms, Apple AI frameworks must be called from Swift. Rust should not depend on those APIs; instead, Rust defines interfaces (AI Primitives in Engine) and consumes results.

---

## 2. Storage model (SOT vs Config vs Cache)

Fracta is file-based. The source of truth is user-readable content in normal folders:

- **SOT**: Markdown/CSV/JSON/ICS/EML/OPML + assets folders. Open formats only.
- **Config**: schemas/views/ignore rules/Profile configs (text files in JSON/YAML; can be versioned).
- **Cache**: indexes, materialized views, derived stats, vectors, AI-derived artifacts (safe to delete and rebuild).

### 2.1 `.fracta/` directory layout

For each managed Location, Fracta creates exactly one system folder:

```
LocationRoot/
  .fracta/
    config/
      ignore                  # gitignore-like rules
      settings.json           # Location-level settings
      profile.json            # Active Profile config (e.g., LIV)
      schemas/                # User/Profile-defined schemas
      views/                  # Saved view configurations
    meta/
      uids.jsonl              # Lazy-assigned UIDs
      links.jsonl             # Bidirectional links
    cache/
      index.sqlite            # File index + metadata
      fts/                    # Full-text search index
      vectors/                # Vector embeddings
      events/                 # AI-extracted structured events (JSONL per day)
      summaries/              # AI-generated summaries (Markdown)
      metrics/                # Aggregated metrics (JSON)
      topics/                 # Topic maps, thought lineage (JSON)
      ledger/                 # Local token ledger (JSON)
      proofs/                 # Generated proof files
    state/
      last_runs.json          # Ingestor/pipeline run timestamps
      ai_queue.json           # Pending AI tasks
```

### 2.2 Storage contracts and Profiles

The three-layer persistence model (SOT / Config / Cache) is **Framework-mandated**. What is mandated:

- SOT must be in open formats (Markdown/CSV/JSON/ICS/EML/OPML).
- Config must be in human-readable JSON/YAML.
- Cache must be under `.fracta/cache/` and must be rebuildable.
- All writes must be atomic (temp file → fsync → rename).

What is **Profile-specific**: directory names within the user's Location, subfolder organization, naming conventions. For example:

- LIV Profile uses `Lifelong_Vision/` (control plane), project folders (working directories), and `lifelong_story/` (archive).
- A different Profile might organize these differently.

References:

- `docs/adr/0008-metadata-persistence-anti-dsstore.md`
- `docs/adr/0007-managed-scope-and-ignore-rules.md`

---

## 3. Past subsystem (time journaling)

Past is a pipeline that turns multi-source activity into daily narrative stories. AI builds the timeline skeleton; the user fills in story, emotion, and reflection.

Reference: `docs/adr/0012-past-format-markdown-sot-ai-cache.md`.

### 3.1 Pipeline stages

1. **Ingest** events from multiple sources (Platform + Core Ingestors)
2. **Normalize** into a shared event model
3. **De-duplicate and merge** using deterministic rules
4. **AI constructs** timeline skeleton (structured events → Markdown draft)
5. **User enriches** with narrative, emotion, reflection
6. **Materialize** as daily Markdown file (SOT)
7. **Extract** structured data into `.fracta/cache/events/` (JSONL)
8. **Aggregate** metrics and generate AI summaries

### 3.2 On-disk structure (human-readable SOT)

```
My Drive/
  lifelong_story/
    2025/
      2025-10-01 — Travel prep/
        2025-10-01 — Travel prep.md    ← SOT (Markdown with YAML front matter)
        assets/
          cover.jpg
          thumb_001.jpg
```

Note: There is no `events.csv` in the user's directory. Structured event data lives in `.fracta/cache/events/` and is AI-derived. Users who want CSV/JSON exports can use an explicit export function.

### 3.3 Story Markdown front matter (example)

```yaml
---
uid: "story_2025-10-01_9f2c1a"
title: "2025-10-01 — Travel prep"
date: "2025-10-01"
status: archived            # draft | reviewed | archived
area: past                  # library | now | past
sources: [bilibili, youtube, chrome, healthkit]
privacy: internal           # public | internal | sensitive
mood: 7                     # 1-10, user-set emotional temperature
assets_dir: "./assets/"
summary:
  ai: ""                    # AI-generated summary (cache-derived, user-approved)
  manual: ""                # User-written summary
metrics:
  focus_minutes: 0
  video_minutes: 120
  sleep_hours: 7.5
  spending_cny: 234
tags: [journal, travel, health]
version: "1.0"
last_updated: "2025-10-01T22:30:00+08:00"
---
```

Key changes from earlier design:
- `area` field uses `library | now | past` (not the old "database" term).
- `mood` field added for user emotional temperature.
- `files.events_csv` removed — structured events are cache, not SOT.
- `metrics.spending_*` fields support multi-currency.

### 3.4 Cached event schema (`.fracta/cache/events/`)

Structured events extracted by AI live in `.fracta/cache/events/YYYY-MM-DD.jsonl`. Each line is a JSON object:

| Field | Type | Meaning | Example |
|---|---|---|---|
| `event_id` | string | Unique event id | `bili_2025100112345` |
| `platform` | string | Source system | `bilibili`, `youtube`, `chrome`, `healthkit`, `manual` |
| `event_type` | string | Event kind | `video_watch`, `web_visit`, `sleep`, `meal`, `exercise`, `work`, `travel` |
| `title` | string | Title (UTF-8) | Video or page title |
| `normalized_id` | string | Cross-run stable content id | `bili:BV1xx...` |
| `link` | url | Original link | `https://...` |
| `pic` | path | Relative asset path | `./assets/thumb_001.jpg` |
| `desc` | string | Summary/description (UTF-8) | `...` |
| `start_datetime` | ISO8601 | Start time (with timezone) | `2025-10-01T14:30:00+08:00` |
| `end_datetime` | ISO8601 | End time (with timezone) | `2025-10-01T15:15:00+08:00` |
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

Note: This schema is preliminary. The `event_type` and `platform` fields will have defined enum values during implementation. All string fields are UTF-8. Datetime fields should always include timezone information.

### 3.5 De-duplication and merging (deterministic)

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
| Manual input | User writes in Markdown | Always available; highest priority |
| Bilibili history | WebView cookie capture → official APIs | Cookies expire; refresh flow required |
| YouTube history | OAuth → YouTube Data API | Progress is often unavailable; set `progress_seconds = -1` |
| Chrome history | User-authorized copy of History DB → parse | Avoid fragile automation |
| Safari history | Safari Extension (optional) | Higher complexity under sandboxing |
| HealthKit | HealthKit framework | Steps/sleep/workouts; privacy-sensitive |
| Calendar | CalDAV / ICS import | Schedule events as timeline entries |

Ingestors are split by responsibility (ADR-0004):

- **Platform ingestors** (Shell): handle OS permissions, WebView/OAuth flows, and secure credential storage.
- **Core ingestors** (Rust): handle cross-platform fetching/parsing/normalization and quality scoring.

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
    case public_
    case internal_
    case sensitive
}
```

Exports and chain-anchoring must filter by privacy level. Sensitive content is never published or uploaded.

---

## 6. Technology selections

### 6.1 Core language and runtime

| Choice | Decision | Rationale |
|--------|----------|-----------|
| **Core language** | Rust (stable toolchain) | Cross-platform, memory-safe, no GC pauses, good FFI story |
| **Async runtime** | Tokio | Industry standard, largest async ecosystem, required by many Rust libraries |
| **UI language** | Swift 6 + SwiftUI | Apple-native, modern concurrency model, required for platform APIs |
| **FFI bridge** | UniFFI (mozilla/uniffi-rs) | Generates Swift/Kotlin bindings from Rust, handles type conversion automatically |

### 6.2 Key dependencies

| Subsystem | Crate | Why |
|-----------|-------|-----|
| **SQLite** | `rusqlite` (bundled) | Most mature Rust SQLite binding; bundling avoids system version issues |
| **Full-text search** | SQLite FTS5 (via rusqlite) | Ships with SQLite, zero extra binary size; adequate for personal-data scale |
| **Markdown parser** | `comrak` | GFM superset (tables, task lists, strikethrough, footnotes); full mutable AST for custom block model |
| **File watching** | `notify` | Cross-platform filesystem watcher; debounced events |
| **Serialization** | `serde` + `serde_json` + `serde_yaml` | Universal in Rust; needed for all config/cache/schema files |
| **UUID** | `uuid` (v7) | Time-ordered UUIDs; monotonic within a millisecond; sortable |
| **Crypto signing** | `ed25519-dalek` | Ed25519 as specified in SPEC §13 |
| **Hashing** | `blake3` | Fast, parallelizable; used for content-addressing and dedup keys |
| **Date/time** | `chrono` | Full timezone support; ISO 8601 parsing |
| **Error handling** | `thiserror` + `anyhow` | `thiserror` for library errors; `anyhow` for application-level |
| **Logging** | `tracing` | Structured logging with spans; async-aware |
| **HTTP client** | `reqwest` | Async HTTP; needed for API ingestors and AI provider calls |
| **Vector embeddings** | *(deferred to Phase 2)* | Options: sqlite-vss, qdrant-client, or custom HNSW |

### 6.3 Cargo workspace structure

The Rust code is organized as a Cargo workspace with one crate per Engine subsystem, plus a Framework crate and FFI bridge:

```
fracta/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── fracta-vfs/               # Engine: file/folder CRUD, watching, atomic writes, scope
│   ├── fracta-index/             # Engine: SQLite index + FTS5
│   ├── fracta-query/             # Engine: filter/sort/group/aggregate
│   ├── fracta-note/              # Engine: Markdown parser + block model
│   ├── fracta-comm/              # Engine: IMAP/CalDAV/RSS/HTTP transports
│   ├── fracta-sync/              # Engine: conflict resolution, multi-device
│   ├── fracta-crypto/            # Engine: signing, hashing, key management
│   ├── fracta-ai/                # Engine: LLM call interface, embeddings, prompts
│   ├── fracta-platform/          # Engine: platform adapter trait definitions
│   ├── fracta-framework/         # Framework: Primitives, Pipelines, Orchestrator
│   └── fracta-ffi/               # UniFFI bridge: exposes Rust Core to Swift
├── apple/
│   └── Fracta/                   # Xcode project: SwiftUI app shell
├── docs/                         # (existing documentation)
└── tests/                        # Workspace-level integration tests
```

**Phase 1 active crates**: `fracta-vfs`, `fracta-index`, `fracta-note`, `fracta-ai`, `fracta-ffi`. Other crates have stub `lib.rs` files and will be developed in later phases.

**Dependency rules** (enforced by crate boundaries):
- Engine crates depend only on other Engine crates and external libraries.
- `fracta-framework` depends on Engine crates.
- `fracta-ffi` depends on everything it exposes to Swift.
- No crate may have circular dependencies (enforced by Cargo).

---

## 7. Engineering conventions (initial)

- **Atomic writes** everywhere for SOT/config/meta.
- **Caches must be rebuildable**; never store irreplaceable truth only in SQLite or AI-derived caches.
- **Lazy UID**: do not assign IDs until necessary (ADR-0005).
- **Docs are English-only** (ADR-0009).
- **Layer boundary**: Engine code must not import Framework types; Framework code must not import Application types (ADR-0010).
- **AI outputs are files**: all AI-generated artifacts must be materialized as open-format files, not hidden in databases.
- **Profile-configurable**: behavior constraints (like Quest Slots cap) are defined in Profile configs, not hard-coded.
- **Error pattern**: Engine crates define typed errors with `thiserror`; application code uses `anyhow` for convenience.
- **Logging**: Use `tracing` spans to track operations across async boundaries.
- **Testing**: Each crate has unit tests; `tests/` directory has integration tests that span multiple crates.
