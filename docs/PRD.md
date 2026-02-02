# Fracta PRD (Scope, Milestones, Roadmap)

This document is the canonical **product plan**:
- scope (what we build),
- milestones (what we build next),
- roadmap (how architecture layers and feature tracks interleave),
- backlog (unplanned ideas worth keeping).

System definition and constraints live in `docs/SPEC.md`. Architecture decisions live in `docs/adr/`.

---

## 0. One-liner

Fracta is a **local-first life operating system**: a modular personal data workbench with a file/note/communication engine at its core, a semantic framework for Library/Now/Past, and an application layer where methodologies like LIV, AI copilots, crypto incentives, and third-party integrations come together — all on open formats, without vendor lock-in.

---

## 1. Goals and success criteria

### 1.1 Long-term goals

- **Life operating system**: Fracta becomes the single tool for managing personal data, projects, time reflection, and growth — replacing the fragmented stack of file managers, note apps, task managers, calendars, and databases.
- **Low floor, high ceiling**: simple users get an out-of-the-box experience; power users get a scriptable, extensible framework.
- **Creator economy**: users publish and trade Profiles, Views, Schemas, Templates via the marketplace, powered by native crypto tokens.
- **AI-native**: AI automates mechanical work and surfaces creative insights across every layer.
- **Open infrastructure**: Engine + Framework are open-source; users never fear lock-in.

### 1.2 Initial success metrics (draft)

- Weekly active usage: users open Fracta ≥ 3 times/week (Past dashboard and/or Now projects).
- Behavior: at least one Now project maintained + one Past review per week.
- Performance floor (phase-dependent):
  - cold start < 2s
  - common search < 0.5s
  - view switch < 0.3s

---

## 2. Users and scenarios (Library / Now / Past)

### 2.1 Library (knowledge base)

- Public/semi-public resources: books, courses, movies/shows, templates, references.
- Key capabilities: typed fields, views, de-dup, packaging/sharing/trading under licensing constraints.
- The intake and exploration surface.

### 2.2 Now (active projects)

- Minimal active set: 1 health project + 1–3 career projects (LIV default).
- Key capabilities: quest/task chain, progress feedback, rewards, AI collaboration, optional agent actions.
- The execution surface.

### 2.3 Past (time diary)

- Multi-source event stream → AI-built timeline skeleton → user-authored story → daily Markdown.
- Key capabilities: timeline views, metrics cards, AI summaries, knowledge synthesis, optional Proof.
- The reflection and archive surface.

---

## 3. Feature scope

### 3.1 Finder-level file management (Engine)

- Finder-like browsing (list/column/gallery), drag & drop, keyboard shortcuts, contextual actions.
- Global View across multiple Locations.
- Managed Scope enables compute features (Folder Pages, metadata indexing, views).
- Ignore rules (gitignore-like) to keep large/noisy directories visible but un-managed.

### 3.2 Multidimensional metadata + views (Framework)

- Schema (field types) + Query (filter/sort/group/aggregate) + Views:
  - table / kanban / calendar / timeline / graph
- Virtual tables (default): table is derived from Items + metadata.
- Optional explicit table files (CSV/JSONL) for user-owned datasets.

### 3.3 AI system (pervasive, Engine + Framework)

- Mechanical automation: classify, tag, extract events, generate summaries, route Inbox items.
- Creative drafting: suggest quest priorities, draft narratives, prepare options — pending human review.
- Knowledge synthesis: topic clustering, thought-lineage tracking, open-question extraction.
- AI providers pluggable: on-device first; cloud optional.

### 3.4 Crypto / token economy (Framework)

- Attestation: sign Loot/Events → proof files; optional chain anchoring.
- Incentives: native token earned by achievements; trigger rules defined per Profile.
- Exchange: marketplace for Profiles, Views, Templates; token settlement.
- Three UX layers: passive (game-style rewards), active (marketplace), advanced (on-chain, hidden by default).

### 3.5 LIV default workflow (Application)

- Inbox, Quest Slots (cap=2), Dungeon Runs, Loot, HUD, Rulebook, Resupply.
- AI enforces Profile-defined guardrails (quest slot cap, loot requirement, budget protection, night rule).
- Reference: `docs/LIV_System_Manual.md`, `docs/adr/0011-liv-as-default-application-profile.md`.

---

## 4. Roadmap

### 4.1 Architecture phases (build order)

The three-layer architecture determines the build sequence: Engine first (infrastructure must exist), then Framework (semantics built on infrastructure), then Application (UI built on semantics).

Within each phase, **LIV's needs drive feature selection** — we build the Engine and Framework capabilities that LIV requires, validating generality as we go.

**Phase 0 — Foundation (current)**

Define the system. Establish constraints, formats, and architecture decisions.

- [x] Canonical docs + ADR set (0001–0009)
- [x] Three-layer architecture definition (ADR-0010)
- [x] LIV as default Profile decision (ADR-0011)
- [x] Past format decision (ADR-0012)
- [x] Open-source strategy (ADR-0013)
- [ ] Finalize Engine subsystem interface specs in ENGINEERING.md
- [ ] Finalize `.fracta/` directory layout in ENGINEERING.md
- [ ] Finalize Semantic Primitive on-disk schemas in ENGINEERING.md

**Phase 1 — Engine skeleton**

Build the minimal Engine: enough infrastructure to support one end-to-end workflow.

- [ ] **VFS**: file/folder CRUD + watch + atomic writes + Location abstraction + Managed/Ignored/Plain scope
- [ ] **Index**: SQLite index + FTS (full-text search) + incremental updates
- [ ] **Query**: basic filter/sort/group
- [ ] **Note**: Markdown parser + block model (read-only rendering first)
- [ ] **AI Primitives**: LLM call interface + embedding generation (provider-agnostic)
- [ ] **Platform Shell skeleton**: minimal SwiftUI app that can browse files and render Markdown

**Phase 2 — Framework skeleton**

Build the semantic layer: Primitives, Pipelines, and the first AI workflows.

- [ ] **Semantic Primitives**: Rust types for Item, Record, Event, Quest, Loot, Rule, Schedule, Metric, Claim, Token
- [ ] **Pipeline skeleton**: Capture → Route → Process → Materialize → Archive (configurable stages)
- [ ] **AI Orchestrator**: mechanical automation (classify, tag, extract events from Markdown)
- [ ] **Past pipeline**: multi-source ingestion → AI timeline skeleton → user enrichment → daily Markdown SOT
- [ ] **Crypto/Token Engine**: local ledger + signing + proof generation (no chain integration yet)
- [ ] **Knowledge Engine**: topic clustering + thought-lineage (basic)

**Phase 3 — Application skeleton**

Build the user-facing product: default UI, LIV workflow, and initial creator tools.

- [ ] **Default UI**: Library / Now / Past three-area layout
- [ ] **Past dashboard**: timeline view + metrics cards + AI summaries
- [ ] **Now workspace**: Quest Cards, Dungeon Run timer, Loot logger, HUD
- [ ] **Library browser**: Finder-like browsing + sidebar metadata + views
- [ ] **LIV Profile**: all invariants enforced (Quest Slots, budget, guardrails)
- [ ] **Profile system**: load/switch Profiles, custom Profile creation
- [ ] **Marketplace skeleton**: browse/publish Profiles and Templates

### 4.2 Feature tracks (parallel streams)

Within each phase, work proceeds along parallel feature tracks. Each track delivers incremental value.

```
          Phase 1 (Engine)    Phase 2 (Framework)    Phase 3 (Application)
          ───────────────     ──────────────────     ────────────────────
Past      VFS + Index         AI Pipeline +          Timeline view +
          + Note parser       Event extraction       Metrics dashboard +
                              + Past pipeline        AI summaries

Now       (not yet)           Quest/Loot/Rule        Quest Cards +
                              Primitives +           Dungeon Run UI +
                              Schedule               HUD + Guardrails

Library   VFS + Query         Schema + View          Finder-like browser +
                              Primitives             Sidebar + Search

AI        AI Primitives       AI Orchestrator +      AI Copilot UI +
          (LLM interface)     Knowledge Engine       Suggestion panel

Crypto    Crypto Primitives   Token Engine +         Reward display +
          (signing/hashing)   Ledger + Proofs        Marketplace UI
```

### 4.3 Milestones (delivery checkpoints)

Milestones are user-visible deliverables that combine work across phases and tracks.

**Milestone 0.0: Foundation** (Phase 0)

- All architecture decisions documented.
- Engine interface specs finalized.
- `.fracta/` layout and Primitive schemas finalized.
- Ready to write code.

**Milestone 0.1: First Light** (Phase 1 complete)

- Fracta app opens, browses files, renders Markdown.
- Index and search work on a managed Location.
- AI can be called (LLM interface functional).
- No semantic features yet — this is "a better Finder that can talk to AI".

**Milestone 0.2: Past Insight Dashboard** (Phase 2, Past track)

- At least 2 ingestors functional (manual + one automated source).
- AI builds timeline skeleton from data sources.
- User can enrich timeline with narrative.
- Daily Markdown SOT generated.
- Timeline view + metrics dashboard in UI.
- AI summaries (daily/weekly).

**Milestone 0.3: Now + LIV** (Phase 2–3, Now track)

- Quest/Loot/Rule/Schedule Primitives functional.
- LIV Profile active: Quest Slots, Dungeon Runs, HUD, Resupply workflow.
- AI guardrails enforce LIV invariants.
- Crypto: token rewards for quest completion (local ledger).

**Milestone 0.4: Library + Views** (Phase 2–3, Library track)

- Schema v1 + saved Views v1.
- Query engine: filter/sort/group + persisted view configs.
- Finder-like browsing with sidebar metadata and backlinks.
- Tags as first-class fields.

**Milestone 1.0: Life OS** (Phase 3 complete)

- All three areas (Library/Now/Past) functional and integrated.
- LIV is the default workflow with full guardrails.
- AI Copilot across all areas.
- Creator Marketplace: publish and browse Profiles/Templates.
- Crypto: attestation, incentives, and marketplace transactions.
- Performance targets met.

---

## 5. Backlog (unplanned ideas)

### 5.1 Finder-level enhancements

- Deep macOS integrations (Spotlight/Quick Look/Services/Shortcuts/Menu Bar/global hotkeys).
- Versioning / backups / corruption repair.

### 5.2 Library/view enhancements

- Rich formula fields, rollups, relation graphs, cross-table joins, advanced aggregates.
- View templates marketplace (licensed).

### 5.3 Notes and connections

- Block references, richer backlinks UI, graph controls.
- Rich editor features (WYSIWYG, tables, LaTeX, media embeds).

### 5.4 AI automation

- Semantic search and Q&A over local corpus.
- Automatic classification, quality scoring, anomaly detection.
- Multi-agent workflows for complex tasks.

### 5.5 Crypto / token / market

- Chain-agnostic anchoring strategies (Merkle root, batching).
- Advanced incentive mechanics and community value discovery.
- Licensed asset packaging and trading (templates/views/schemas/datasets).
- DAO-style governance for community Profiles.

### 5.6 Communication

- Email client (IMAP/SMTP) with AI triage.
- Calendar integration (CalDAV) with Dungeon Run scheduling.
- RSS reader with AI summarization and routing to Library/Inbox.

### 5.7 Non-functional

- Performance targets by dataset size.
- Privacy modes, export filters, security audits.
- Plugin system and scripting hooks.
- Multi-device sync.
- Windows/Linux/Android shells.
