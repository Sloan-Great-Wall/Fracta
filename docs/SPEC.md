# Fracta — System Specification (SPEC)

This document is the **canonical definition** of the Fracta system: what it is, why it exists, and the invariants that must not break.

- **Product scope and milestones** live in `docs/PRD.md`.
- **Engineering details** live in `docs/ENGINEERING.md`.
- **Decision rationale** lives in `docs/adr/` (ADRs).

---

## 1. Problem statement

People store their life and work across files, apps, browser histories, and services. The results are fragmented:

- Files are **hard to organize** without creating brittle folder hierarchies.
- Structured workflows (tables, databases, views) often become a **separate world** from the file system.
- Daily activity becomes a **lost stream** (watch history, browsing, health) that is hard to review or turn into narrative.
- AI can help, but it is usually **cloud-only**, opaque, and tends to lock users into proprietary storage.

Fracta aims to unify these into a single, local-first system with open formats.

---

## 2. Product definition (one sentence)

Fracta is a **local-first personal data workbench**: manage files like Finder, organize with database-like views on top of files, generate time-based “Past” narratives from multi-source activity, and use AI to assist—optionally supporting cryptographic / blockchain proofs for attribution and incentives.

---

## 3. Core principles (non-negotiable)

- **Local-first**: the system must be usable offline; user data stays on user-owned storage.
- **Open formats**: primary data MUST remain readable without Fracta (Markdown/CSV/JSON + normal folders).
- **File system as the source of truth (SOT)**: databases and indexes are derived; caches are rebuildable.
- **Progressive enhancement**: Fracta may run in “plain browsing” mode without changing user data.
- **Minimum intrusion by default**: do not write hidden files everywhere; do not assign IDs to everything up-front.
- **Explainable boundaries**: users must understand which folders are managed, ignored, or un-managed.
- **Security by default**: secrets never go into files; store credentials in OS secure storage (Keychain / Secure Enclave on Apple).

---

## 4. Information architecture (top-level mental model)

Within a single user space (internally: “vault”; externally: “location”), the top-level model is fixed:

- **Database**: relatively stable resource libraries (books, videos, templates, public references).
- **Now**: a small, focused set of active projects.
- **Past**: time-axis, already-happened event streams + narrative summaries (daily/weekly).

This model is chosen to unify Finder-like management, database views, and time journaling without competing “systems”.

Reference: `docs/adr/0002-information-architecture-database-now-past.md`.

---

## 5. Locations, Global View, and Managed Scope

### 5.1 Location

A **Location** is a directory tree that the user grants Fracta access to (local folder, external disk, cloud-sync folder, etc.).

Fracta must support multiple Locations and aggregate them.

Reference: `docs/adr/0006-locations-global-view.md`.

### 5.2 Global View

Fracta provides a **Global View** that feels Finder-like:

- Show what the OS allows Fracta to see.
- For paths without permission, Fracta may display structure but MUST NOT index or read content until authorized.

### 5.3 Managed / Ignored / Plain

Fracta has a compute layer (Folder Pages, metadata, views, indexing). That layer is **only enabled** inside **Managed Locations**.

Each directory must have an explainable state:

- **Managed**: compute layer enabled.
- **Ignored**: inside a managed location, but explicitly excluded by rules or heuristics; behaves like plain browsing.
- **Plain**: not in any managed location (or not yet enabled); browse/open only.

Ignore rules are gitignore-like and are stored under the managed location’s system area.

Reference: `docs/adr/0007-managed-scope-and-ignore-rules.md`.

---

## 6. Metadata persistence and system area

### 6.1 The `.fracta/` system area

To avoid `.DS_Store`-style pollution, Fracta must not create hidden files in every folder. Instead:

- A managed location has exactly one system directory at its root: `LocationRoot/.fracta/`
- Subfolders MUST remain clean unless the user explicitly creates content there.

### 6.2 Three-layer persistence model

Fracta splits persistence into:

- **SOT**: user content + minimal necessary metadata that must travel with the content.
- **Config**: schemas, views, ignore rules, settings (text-based, versionable).
- **Cache**: indexes, materialized views, derived stats, vectors (safe to delete and rebuild).

All writes MUST use atomic write patterns (write temp → fsync if needed → atomic rename).

Reference: `docs/adr/0008-metadata-persistence-anti-dsstore.md`.

---

## 7. Identity model (UIDs)

Fracta needs stable identities so that moves/renames do not break links, views, or proofs.

However, assigning IDs to everything is invasive and expensive. Therefore:

- Use **Lazy UID assignment**: only assign/persist a UID when stable semantics are required (e.g., adding metadata, linking, Folder Pages, proofs).
- Un-touched objects may use temporary identities for indexing only (path + timestamps + size), with no guarantee across moves.

Reference: `docs/adr/0005-uid-strategy-lazy-assignment.md`.

---

## 8. Core object model (shared across Database / Now / Past)

These conceptual objects must be consistent across the system:

- **Item**: a file-system entity (file or folder) that can be browsed and optionally “managed”.
- **Record**: a structured row (CSV/JSON-backed) that represents an entity in a database view.
- **Event**: a time-stamped activity unit used by Past (watch, visit, workout, sleep, manual note).
- **View**: a persisted representation of how to present/query items/records/events (table/kanban/timeline).
- **Asset**: a blob referenced by content (images, thumbnails, attachments).
- **Claim** (optional): a cryptographic / blockchain-attested statement about an item/event/asset.

The exact on-disk schema is defined in `docs/ENGINEERING.md`; SPEC only defines the invariants and boundaries.

---

## 9. Folder Pages (folder-as-page)

A **Folder Page** is a Markdown file that makes a folder behave like a “page”:

- It can contain narrative content, links, and embedded computed blocks (queries, summaries, charts).
- It must remain a valid Markdown file viewable in any editor.
- Computed blocks must be safely identifiable and re-materializable from cache.

Folder Pages only exist in Managed Scope.

---

## 10. Past subsystem (time journaling)

Past turns multi-source activity into daily narrative folders.

### 10.1 Output format (human-readable SOT)

Past outputs MUST be stored as normal folders with:

- A daily Markdown story (human narrative + summaries)
- A structured event table (`events.csv` or equivalent)
- An `assets/` folder for thumbnails/screenshots

### 10.2 Ingestion model

Ingestors are split by responsibility:

- **Platform ingestors** (Shell): handle OS permissions, WebView/OAuth flows, and secure credential storage.
- **Core ingestors** (Rust): handle cross-platform fetching/parsing/normalization and quality scoring.

Reference: `docs/adr/0004-ingestion-strategy-platform-core-ingestors.md`.

### 10.3 De-duplication and merging

Past needs deterministic merging to reduce noise:

- Dedup keys are based on platform identifiers + normalized IDs + time buckets.
- Merging prioritizes higher-quality sources (official APIs over scraping/estimation).

Implementation details live in `docs/ENGINEERING.md`.

---

## 11. AI integration (local-first)

AI is an assistant layer, not a storage system.

- Default should support **on-device** or **local** inference where possible.
- Cloud models may be optional, clearly disclosed, and user-controlled.
- AI outputs must be stored as derived artifacts (cache or explicit user-approved content), not as hidden state.

On Apple platforms, Apple AI frameworks must be called from the platform shell (Swift), not from Rust.

---

## 12. Optional proofs / blockchain

Fracta may support proofs for attribution, incentives, or transactions. This is optional and must follow:

- No blockchain requirement for core usability.
- Proofs must reference stable UIDs and open-format artifacts.
- Private material must never be published accidentally; privacy filters must be explicit.

---

## 13. Non-goals and red lines

Fracta MUST NOT do the following:

- Keylogging, screen recording, or covert monitoring.
- Uploading user content by default.
- Requiring an account to browse local files.
- Writing hidden metadata files into every directory.

---

## 14. Document control

- This SPEC is the canonical “what must not break”.
- Any substantial change to the invariants must be accompanied by an ADR.

