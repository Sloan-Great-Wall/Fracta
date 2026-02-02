# Fracta — System Specification (SPEC)

This document is the **canonical definition** of the Fracta system: what it is, why it exists, and the invariants that must not break.

- **Product scope and milestones** live in `docs/PRD.md`.
- **Engineering details** live in `docs/ENGINEERING.md`.
- **Decision rationale** lives in `docs/adr/` (ADRs).

---

## 1. Problem statement

People who want to learn, grow, and build throughout their lives are forced to assemble a fragile stack of disconnected tools: a file manager, a note-taking app, a task manager, a calendar, an email client, an RSS reader, a habit tracker, a database, and more. The results are:

- **Tool fragmentation**: each tool is a silo; data and workflows don't compose across them. Users endlessly switch between apps hoping the next one will "finally work".
- **File chaos**: files are hard to organize without creating brittle folder hierarchies.
- **Disconnected structure**: structured workflows (tables, databases, views) become a separate world from the file system.
- **Lost activity stream**: daily activity (watch history, browsing, health, communication) is scattered and hard to review or turn into narrative.
- **AI lock-in**: AI can help, but it is usually cloud-only, opaque, and tends to lock users into proprietary storage.
- **No life operating system**: there is no single system that unifies personal data management, project execution, time reflection, and lifelong growth into a coherent whole.

Fracta aims to be that system: a **life operating system** for lifelong learners — so users never need to choose between tools again.

---

## 2. Product definition (one sentence)

Fracta is a **local-first life operating system**: a modular personal data workbench with a file/note/communication engine at its core, a semantic framework for organizing knowledge (Library), executing projects (Now), and reflecting on time (Past), and an application layer where methodologies like LIV, AI copilots, crypto incentives, and third-party integrations come together — all on open formats, without vendor lock-in.

---

## 3. Core principles (non-negotiable)

1. **Local-first**: the system must be usable offline; user data stays on user-owned storage.
2. **Open formats**: primary data MUST remain readable without Fracta (Markdown/CSV/JSON/ICS/EML/OPML + normal folders).
3. **File system as the source of truth (SOT)**: databases and indexes are derived; caches are rebuildable. The file system is the canonical state.
4. **Three-level openness**: data formats are open; Engine and Framework are open-source; the Application layer (default UI, premium workflows, creator marketplace) is the commercial surface. If Fracta the company disappears, the open Engine + Framework + user data survive.
5. **Low floor, high ceiling**: a new user opens the app and it works; a power user can script, customize, and extend every layer. AI bridges the gap.
6. **Progressive enhancement**: Fracta may run in "plain browsing" mode without changing user data. Features activate only when the user opts in.
7. **Minimum intrusion by default**: do not write hidden files everywhere; do not assign IDs to everything up-front.
8. **Explainable boundaries**: users must understand which folders are managed, ignored, or un-managed.
9. **Security by default**: secrets never go into files; store credentials in OS secure storage (Keychain / Secure Enclave on Apple).
10. **AI as infrastructure, not feature**: AI permeates every layer — automating mechanical work, surfacing insights, and preparing creative options for human decision. AI outputs are always materialized as open-format files, never hidden state.
11. **Crypto as native economy**: token incentives and cryptographic proofs are built into the framework, but completely optional for users who don't want them. Core usability never depends on crypto.

---

## 4. System architecture (three layers)

Fracta is organized into three layers, inspired by game-engine architecture: a generic Engine, a semantic Framework, and a customizable Application layer.

Reference: `docs/adr/0010-three-layer-architecture.md`.

### 4.1 Engine layer ("physics engine")

The Engine provides **generic infrastructure** that any data-intensive desktop application could use. It has no knowledge of Library/Now/Past or any specific workflow.

| Subsystem | Responsibility |
|-----------|---------------|
| **VFS** | File/folder CRUD, file-system watching, atomic writes, Location abstraction, Managed/Ignored/Plain scope |
| **Index** | SQLite indexing, full-text search (FTS), vector embeddings, incremental updates |
| **Query** | Filter/sort/group/aggregate engine, view materialization |
| **Note** | Markdown parsing/rendering, block model, computed-block placeholders |
| **Comm** | Protocol clients: IMAP/SMTP (email transport), CalDAV/ICS (calendar transport), RSS/Atom (feed transport), HTTP/REST (API transport) |
| **Sync** | Conflict resolution, cloud-drive awareness, multi-device primitives |
| **Crypto Primitives** | Signing (Ed25519/secp256k1), hashing (SHA-256/Blake3), key-management interfaces |
| **AI Primitives** | LLM call interface, embedding generation, prompt-template management, context-window management |
| **Platform Adapters** | OS-specific integrations: Apple (Swift/SwiftUI, Keychain, Spotlight, HealthKit, Apple Intelligence, Notifications); future: Windows, Linux, Android |

**Design rules**:

- Each subsystem exposes a **trait/protocol interface**. Engine never depends on Framework or Application.
- Engine is implemented primarily in **Rust** (cross-platform core) with **platform adapters** in the native language of each OS (Swift for Apple).
- FFI boundary: **UniFFI** exposes Rust Core to platform shells.
- Comm subsystem provides **transport only** (connect, fetch, send). Semantic interpretation (email → Event, calendar entry → Schedule) belongs to Framework.

Reference: `docs/adr/0003-core-architecture-rust-core-platform-shells.md`.

### 4.2 Framework layer ("game mechanics")

The Framework defines Fracta's **core semantics**. It uses Engine interfaces to implement meaningful concepts, but does not prescribe any specific user workflow.

| Component | Responsibility |
|-----------|---------------|
| **Semantic Primitives** | Unified domain objects: Item, Record, Event, View, Quest, Loot, Claim, Asset, Rule, Schedule, Metric. All share UID, metadata, privacy, and lifecycle semantics. |
| **Pipelines** | Composable data-flow stages: Capture → Route → Process → Materialize → Archive. Any workflow (LIV, GTD, custom) is a Pipeline configuration. |
| **AI Orchestrator** | Coordinates AI across all pipelines: mechanical automation (classify, tag, summarize, route) and creative drafting (suggest plans, prepare options, draft narratives — always pending human review). |
| **Crypto/Token Engine** | Signing workflows, local ledger, incentive-rule evaluation, proof generation, optional chain anchoring. |
| **Knowledge Engine** | Topic clustering, thought-lineage tracking, open-question extraction, cross-time opinion-evolution analysis — all derived from user content via AI. |
| **Extension API** | Registration points for new Primitive types, Pipeline stages, AI strategies, Comm connectors, and Application Profiles. |

**Design rules**:

- Framework operates through Engine interfaces; it never touches the file system directly.
- All AI-derived artifacts (topic maps, summaries, classifications) are **cache**: stored under `.fracta/cache/`, deletable and rebuildable.
- AI writes to user SOT only with explicit user approval (e.g., user confirms an AI-drafted summary, which is then saved as Markdown).
- Semantic Primitives can be extended by Application Profiles without modifying Framework code.

### 4.3 Application layer ("gameplay")

The Application layer is what users see and interact with. It is built entirely on Framework APIs.

| Component | Description |
|-----------|-------------|
| **Default UI Shell** | Library / Now / Past three-area layout, Finder-like browsing, sidebar, search, dashboard |
| **LIV Workflow** | The default execution methodology: Inbox, Quest Slots, Dungeon Runs, Loot, HUD, Rulebook, Resupply — implemented as a Framework Profile. Reference: `docs/LIV_System_Manual.md`. |
| **User Profiles** | Custom workflow configurations (JSON/YAML). Users can modify LIV or create entirely different methodologies. |
| **Third-party Connectors** | Google Drive, Slack, Notion import, and other integrations — registered through the Framework Extension API. |
| **Creator Marketplace** | Users can publish and trade Profiles, Views, Schemas, Templates — powered by the Crypto/Token Engine. |

**Design rules**:

- Application only uses Framework APIs; it never calls Engine directly.
- LIV is the **default Profile**, not the only one. The Framework must be generic enough that other methodologies (GTD, OKR, Scrum, custom) can be implemented as Profiles.
- Third-party connectors register through the Extension API, not through hard-coded integrations.

Reference: `docs/adr/0011-liv-as-default-application-profile.md`.

---

## 5. Information architecture (top-level mental model)

Within a single user space (internally: "vault"; externally: "location"), the **default** top-level model has three areas:

- **Library**: relatively stable resource collections (books, videos, templates, public references). The intake and exploration surface.
- **Now**: a small, focused set of active projects. The execution surface.
- **Past**: time-axis event streams + narrative summaries (daily/weekly). The reflection and archive surface.

This three-area model is the **default Application layout**. It is not hard-coded into the Framework — a custom Profile could reorganize or rename these areas. However, the underlying Framework Primitives (Item, Record, Event, Quest, Loot, etc.) are shared regardless of layout.

Reference: `docs/adr/0002-information-architecture-database-now-past.md`.

---

## 6. Locations, Global View, and Managed Scope

### 6.1 Location

A **Location** is a directory tree that the user grants Fracta access to (local folder, external disk, cloud-sync folder, etc.).

Fracta must support multiple Locations and aggregate them.

Reference: `docs/adr/0006-locations-global-view.md`.

### 6.2 Global View

Fracta provides a **Global View** that feels Finder-like:

- Show what the OS allows Fracta to see.
- For paths without permission, Fracta may display structure but MUST NOT index or read content until authorized.

### 6.3 Managed / Ignored / Plain

Fracta has a compute layer (Folder Pages, metadata, views, indexing). That layer is **only enabled** inside **Managed Locations**.

Each directory must have an explainable state:

- **Managed**: compute layer enabled.
- **Ignored**: inside a managed location, but explicitly excluded by rules or heuristics; behaves like plain browsing.
- **Plain**: not in any managed location (or not yet enabled); browse/open only.

Ignore rules are gitignore-like and are stored under the managed location's system area.

Reference: `docs/adr/0007-managed-scope-and-ignore-rules.md`.

---

## 7. Metadata persistence and system area

### 7.1 The `.fracta/` system area

To avoid `.DS_Store`-style pollution, Fracta must not create hidden files in every folder. Instead:

- A managed location has exactly one system directory at its root: `LocationRoot/.fracta/`
- Subfolders MUST remain clean unless the user explicitly creates content there.

### 7.2 Three-layer persistence model

Fracta splits persistence into:

- **SOT**: user content + minimal necessary metadata that must travel with the content. Open formats only (Markdown/CSV/JSON/ICS/EML/OPML).
- **Config**: schemas, views, ignore rules, settings (text-based, versionable, human-readable JSON/YAML).
- **Cache**: indexes, materialized views, derived stats, vectors, AI-derived artifacts (safe to delete and rebuild).

All writes MUST use atomic write patterns (write temp → fsync if needed → atomic rename).

Reference: `docs/adr/0008-metadata-persistence-anti-dsstore.md`.

---

## 8. Identity model (UIDs)

Fracta needs stable identities so that moves/renames do not break links, views, or proofs.

However, assigning IDs to everything is invasive and expensive. Therefore:

- Use **Lazy UID assignment**: only assign/persist a UID when stable semantics are required (e.g., adding metadata, linking, Folder Pages, proofs, crypto claims).
- Un-touched objects may use temporary identities for indexing only (path + timestamps + size), with no guarantee across moves.

Reference: `docs/adr/0005-uid-strategy-lazy-assignment.md`.

---

## 9. Core object model

These are the **Framework-level Semantic Primitives** shared across the entire system:

| Primitive | Description |
|-----------|-------------|
| **Item** | A file-system entity (file or folder) that can be browsed and optionally managed. |
| **Record** | A structured row (CSV/JSON-backed) representing an entity in a database view. |
| **Event** | A time-stamped activity unit: watch, visit, workout, sleep, meal, work session, communication, manual note. Used by Past. |
| **View** | A persisted representation of how to present/query items/records/events (table/kanban/timeline/calendar/graph). |
| **Quest** | A task-line with a type (main/side/micro/experience/season) and a status. Used by Now. |
| **Loot** | Proof of progress produced by completing a quest stage: a document, commit, decision, card update. Linkable and archivable. |
| **Rule** | A behavior directive: If-Then rules and No-Go gates. Stored in the Rulebook. |
| **Schedule** | A calendar-bound time block (e.g., Dungeon Run). Links to one Quest. |
| **Metric** | A named numeric measurement with a fixed definition, used for feedback (HUD). Not a goal. |
| **Asset** | A blob referenced by content (images, thumbnails, attachments). |
| **Claim** | A cryptographic statement about any Primitive: signature, proof, attestation, token reward. |
| **Token** | A unit of the native crypto economy: earned through achievements, spent on marketplace transactions. |

All Primitives share: optional UID (lazy-assigned), metadata (YAML), privacy level, lifecycle state, and timestamps.

The on-disk schema is defined in `docs/ENGINEERING.md`; SPEC only defines the invariants and boundaries.

---

## 10. Folder Pages (folder-as-page)

A **Folder Page** is a Markdown file that makes a folder behave like a "page":

- It can contain narrative content, links, and embedded computed blocks (queries, summaries, charts).
- It must remain a valid Markdown file viewable in any editor.
- Computed blocks must be safely identifiable and re-materializable from cache.

Folder Pages only exist in Managed Scope.

---

## 11. Past subsystem (time journaling)

Past turns multi-source activity into daily narrative stories. It is the reflection and archive surface of the system.

### 11.1 Core flow

```
Multi-source data (browser, calendar, health, spending, manual input, ...)
        ↓  AI auto-constructs
Timeline skeleton (who / where / what / how much / how long)
        ↓  User manually adds
Story + emotion + reflection
        ↓  Merged
Complete daily Markdown file (SOT)
```

AI builds the skeleton; the human fills in the soul.

### 11.2 Output format (human-readable SOT)

Each day produces a **Markdown file** as the source of truth. The file contains:

- YAML front matter with structured metadata (date, sources, mood, metrics summary).
- A Timeline section with time-stamped entries (auto-generated by AI from data sources, editable by user).
- A Story section for user-authored narrative, reflection, and emotional content.
- Inline references to assets, people, places, and other Primitives.

The Markdown file MUST be readable and meaningful in any text editor, even without Fracta.

### 11.3 Structured data as cache

AI extracts structured event data from the Markdown SOT and from raw data sources. This structured data lives in `.fracta/cache/events/` as JSONL files. It powers:

- Timeline views, table views, calendar views.
- Metrics aggregation (spending, sleep, exercise, screen time, etc.).
- AI summaries (daily/weekly/monthly/quarterly/yearly).
- Topic maps and thought-lineage tracking.

The cache is **always rebuildable** from SOT + data sources. If `.fracta/cache/` is deleted, re-running the AI pipeline regenerates everything.

Users who want structured data in their own files (e.g., `events.csv` for Excel analysis) can use an **export function**. The export is a user action, not an automatic side-effect.

### 11.4 Ingestion model

Ingestors are split by responsibility:

- **Platform ingestors** (Shell): handle OS permissions, WebView/OAuth flows, and secure credential storage.
- **Core ingestors** (Rust): handle cross-platform fetching/parsing/normalization and quality scoring.

Reference: `docs/adr/0004-ingestion-strategy-platform-core-ingestors.md`.

### 11.5 De-duplication and merging

Past needs deterministic merging to reduce noise:

- Dedup keys are based on platform identifiers + normalized IDs + time buckets.
- Merging prioritizes higher-quality sources (official APIs over scraping/estimation).

Reference: `docs/adr/0012-past-format-markdown-sot-ai-cache.md`.

---

## 12. AI system (pervasive)

AI is not a feature module — it is **infrastructure that permeates every layer**.

### 12.1 Two operating modes

**Mode A — Mechanical automation** (auto-execute):

- Classify and tag new files entering Managed Scope.
- Extract structured events from Markdown and data sources.
- Generate daily/weekly Past summaries.
- Route Inbox items to Side Quests.
- Calculate HUD metrics.
- Detect pattern changes and anomalies.

**Mode B — Creative drafting** (prepare-and-suggest):

- During Resupply: analyze loot themes, suggest which Quests to track next.
- Quarterly review: draft narrative with key inflection points.
- When a Quest stalls: prepare 3 possible breakthrough directions.
- When a Side Quest recurs: prepare a Micro-Quest proposal.
- Knowledge synthesis: "You've been thinking about X for 3 months. Your view evolved from A to B. Here are 3 unresolved tensions."

Mode A runs automatically. Mode B produces **drafts pending human review**. Users can configure which tasks belong to which mode.

### 12.2 AI invariants

1. AI outputs MUST be materialized as **open-format files** (Markdown, JSON, YAML). Never hidden state.
2. AI-derived artifacts are **cache** unless the user explicitly approves them into SOT.
3. AI providers are **pluggable**: on-device (Apple Intelligence, local models via Ollama/llama.cpp), cloud (OpenAI, Anthropic), or custom. All providers must implement the Engine's unified AI Primitives interface.
4. On Apple platforms, Apple AI frameworks are called from the platform shell (Swift), not from Rust.
5. AI MUST NOT make irreversible changes to user SOT without explicit user consent.
6. AI MUST NOT upload user content unless the user explicitly opts into a cloud AI provider.
7. AI classification/tagging is Mode A (auto-execute) by default; any content generation is Mode B (draft-pending-review) by default. Users can reconfigure.
8. All AI actions must be **auditable**: the system must be able to show what AI did, when, and why.

### 12.3 Knowledge synthesis

AI continuously analyzes user content to build derived knowledge structures:

- **Topic Map**: clusters of related ideas, projects, and interests.
- **Thought Lineage**: how opinions on a topic evolved over time.
- **Open Questions**: unresolved tensions and contradictions across notes.

These structures are cache (stored in `.fracta/cache/`). They power the Application layer's insight views (e.g., "What have you been thinking about?" in Past review, or "Side Quest emerging from fog" in LIV).

This is the **functional foundation**; the Application layer (LIV or custom) provides the **gameplay** on top of it.

---

## 13. Crypto / Token system (native economy)

The crypto system provides an in-game economy for Fracta. It is native to the framework but **completely optional** for users who don't want it.

### 13.1 Three functions

**Attestation (proof)**: cryptographic evidence that something happened.

- Sign Loot/Event/Story → generate a `.proof.json` file.
- Optional on-chain anchoring (Merkle-root batching, chain-agnostic).
- Use cases: personal achievement proof, learning records, work-output verification.
- AI-generated content is marked `ai_generated`; human content is marked `human_authored`.

**Incentives (rewards)**: tokens earned by achieving milestones.

- Native token tracked in a local ledger (JSON/CSV) — offline-first.
- Trigger conditions defined by the active Profile (e.g., LIV: complete Quest, maintain Dungeon Run streak, hit HUD targets).
- Profile creators can design their own incentive rules using the Framework API.
- Optional on-chain minting for portability.

**Exchange (marketplace)**: trade valuable creations between users.

- Profiles, Views, Schemas, Templates, curated Libraries can be priced and sold.
- Transactions settle in native tokens.
- Copyright and licensing enforced via Claims.

### 13.2 Crypto UX layers

| Layer | Who sees it | Experience |
|-------|------------|------------|
| **Passive** | All users of a Profile with incentives | "I completed a quest and earned 50 coins" — game-style reward display |
| **Active** | Users who want attestation or marketplace | App-Store-like purchase UI, certificate viewer — not crypto-native |
| **Advanced** | Crypto-savvy users who want on-chain features | Chain selection, gas settings, key export, wallet interop — hidden by default |

### 13.3 Crypto invariants

- Core usability MUST NOT depend on crypto. A user who never touches crypto must have the full Fracta experience.
- Local ledger is the default. On-chain operations are always opt-in.
- Attestation never publishes content — only hashes. Privacy filters are explicit.
- Proofs must reference stable UIDs (see §8).
- Private material must never be published accidentally.

Reference: `docs/adr/0003-core-architecture-rust-core-platform-shells.md` (crypto signing in Rust Core).

---

## 14. Openness commitment

Fracta's openness operates at three levels:

### Level 1 — Data formats (open)

All user-created data uses standard open formats: Markdown, CSV, JSON, ICS (calendar), EML (email), OPML (feeds), and normal folders. Users can read, edit, and process their data with any tool — Finder, VS Code, Excel, Obsidian, git, Python scripts, etc.

### Level 2 — Engine + Framework (open-source)

The Engine layer and the Framework layer are open-source. This means:

- If Fracta the company disappears, the community can fork and maintain the core.
- Third-party developers can build alternative Application layers on top of the Framework.
- CLI tools can operate on `.fracta/` directories without the GUI app.
- The `.fracta/` directory structure is publicly documented; other tools can read it.

### Level 3 — Application layer (commercial)

The default UI, premium Profiles (like LIV), advanced AI features, cloud sync, team collaboration, and the Creator Marketplace are commercial products. This is where the business model lives.

**The user's promise**: even if you stop paying, your data (Level 1) and your tools (Level 2) still work. You lose premium UI and marketplace access, not your data or your ability to process it.

Reference: `docs/adr/0013-open-source-engine-framework-commercial-application.md`.

---

## 15. Non-goals and red lines

Fracta MUST NOT do the following:

- Keylogging, screen recording, or covert monitoring.
- Uploading user content by default.
- Requiring an account to browse local files.
- Writing hidden metadata files into every directory.
- Making any core feature depend on blockchain or token ownership.
- Making AI changes to user SOT without explicit consent.

---

## 16. Document control

- This SPEC is the canonical "what must not break".
- Any substantial change to the invariants must be accompanied by an ADR.
