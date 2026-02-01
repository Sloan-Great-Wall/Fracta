# Fracta PRD (Scope, Milestones, Backlog)

This document is the canonical **product plan**:
- scope (what we build),
- milestones (what we build next),
- backlog (unplanned ideas worth keeping).

System definition and constraints live in `docs/SPEC.md`. Architecture decisions live in `docs/adr/`.

---

## 0. One-liner

Fracta is a **local-first personal data workbench**: Finder-like global browsing, Notion-like multidimensional views, AI-driven automation, and optional cryptographic proof/incentives—without vendor lock-in.

---

## 1. Goals and success criteria

### 1.1 Long-term goals

- **Finder replacement (macOS)**: Fracta becomes a primary file-management workflow.
- **Notion-like multidimensional views**: open-file-based “database views” (table/kanban/calendar/timeline/graph) as an enhancement to file management.
- **AI + incentives**:
  - Past: summaries/insights over an event stream
  - Now: project execution support (quests/tasks + feedback + rewards)
  - Proof: local signatures and optional chain anchoring

### 1.2 Initial success metrics (draft)

- Weekly active usage: users open Fracta ≥ 3 times/week (Past dashboard and/or Now projects).
- Behavior: at least one Now project maintained + one Past review per week.
- Performance floor (phase-dependent):
  - cold start < 2s
  - common search < 0.5s
  - view switch < 0.3s

---

## 2. Users and scenarios (Database / Now / Past)

### 2.1 Database (library)

- Public/semi-public resources: books, courses, movies/shows, templates, references.
- Key capabilities: typed fields, views, de-dup, packaging/sharing/trading under licensing constraints.

### 2.2 Now (active projects)

- Minimal active set: 1 health project + 1–3 career projects.
- Key capabilities: quest/task chain, progress feedback, rewards, AI collaboration, optional agent actions.

### 2.3 Past (time diary)

- Multi-source event stream → structured events → daily storage → review/insight.
- Key capabilities: timeline filtering, metrics cards, AI summaries, optional Proof.

---

## 3. Feature scope

### 3.1 Finder-level file management (core capability)

- Finder-like browsing (list/column/gallery), drag & drop, keyboard shortcuts, contextual actions.
- Global View across multiple Locations.
- Managed Scope enables compute features (Folder Pages, metadata indexing, views).
- Ignore rules (gitignore-like) to keep large/noisy directories visible but un-managed.

### 3.2 Multidimensional metadata + views (core capability)

- Schema (field types) + Query (filter/sort/group/aggregate) + Views:
  - table / kanban / calendar / timeline / graph
- Virtual tables (default): table is derived from Items + metadata.
- Optional explicit table files (CSV/JSONL) for user-owned datasets.

### 3.3 AI automation + proof/incentives (core capability)

- AI providers are pluggable: on-device first; cloud optional.
- Past: metrics + AI review summaries/insight cards (with provenance).
- Now: AI-generated quest/task chains and feedback loops.
- Proof: local signing + optional anchoring (chain-agnostic).

---

## 4. Milestones (linear roadmap)

> We do not “remove big goals”; instead we deliver **slices** while keeping the core constraints intact (SOT/UID/ignore/atomic writes/rebuildable cache).

### 0.0 Foundation (definition → engineering base)

- [x] Canonical docs + ADR set established
- [x] Lazy UID policy
- [x] Locations + Global View model
- [x] Managed/Unmanaged(reason) + ignore strategy
- [ ] Ignore v1: rule format + defaults + heuristic thresholds
- [ ] Folder Page v1: placeholder syntax + YAML pointer conventions
- [ ] Minimal schemas: Event / Record / View (v1)
- [ ] `.fracta/` layout finalized in code: config/meta/cache/state

### 0.1 Slice: Past Insight Dashboard

- [ ] Ingestors: pick 1–2 minimal sources (e.g. manual + one browser/video source)
- [ ] `events.jsonl` v1 + basic de-dup/merge + metrics aggregation (Rust Core)
- [ ] Views: timeline + table (SwiftUI Shell)
- [ ] AI: summary/insight cards (on-device placeholder OK, provenance required)
- [ ] Proof (optional): local signature file emitted for a day/package

### 0.2 Slice: Finder replacement baseline

- [ ] Finder-like browsing + keyboard shortcuts
- [ ] Sidebar: YAML metadata, backlinks, view entry points
- [ ] Incremental indexing (rebuildable cache)

### 0.3 Slice: Multidimensional database baseline

- [ ] Schema v1 + saved Views v1
- [ ] Query v1 (filter/sort/group) + persisted view configs
- [ ] Treat “classic tags” as a first-class field (bridge file-management and database views)

### 1.0 Slice: Now project system (quests + rewards)

- [ ] Project model constraints (health + 1–3 career projects) + UI guidance
- [ ] AI quest/task chain generation
- [ ] Rewards ledger (offline first) + optional on-chain claiming later
- [ ] Optional agent actions (behind explicit user consent and audit trail)

---

## 5. Backlog (unplanned ideas)

### 5.1 Finder-level enhancements

- Deep macOS integrations (Spotlight/Quick Look/Services/Shortcuts/Menu Bar/global hotkeys).
- Versioning / backups / corruption repair.

### 5.2 Database/view enhancements

- Rich formula fields, rollups, relation graphs, cross-table joins, advanced aggregates.
- View templates marketplace (licensed).

### 5.3 Notes and connections

- Block references, richer backlinks UI, graph controls.
- Rich editor features (WYSIWYG, tables, LaTeX, media embeds).

### 5.4 AI automation

- Semantic search and Q&A over local corpus.
- Automatic classification, quality scoring, anomaly detection.

### 5.5 Proof / token / market

- Chain-agnostic anchoring strategies (Merkle root, batching).
- Incentive mechanics and community value discovery.
- Licensed asset packaging and trading (templates/views/schemas/datasets).

### 5.6 Non-functional

- Performance targets by dataset size.
- Privacy modes, export filters, security audits.
- Plugin system and scripting hooks.

