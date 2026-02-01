# CLAUDE.md

## Project Overview

Fracta is a **local-first personal data workbench**: Finder-like global browsing, Notion-like multidimensional views over files, AI-driven automation, and optional cryptographic proof/incentives — without vendor lock-in.

Currently in the **software definition / design phase** (docs only, no implementation code yet).

## Repository Structure

```
README.md              — Repo entrypoint, links to docs/
docs/
  README.md            — Docs entrypoint + reading order
  SPEC.md              — System definition (why / what / constraints / invariants)
  PRD.md               — Product scope, milestones, backlog
  ENGINEERING.md       — Architecture + implementation guidance
  adr/                 — Architecture Decision Records (9 ADRs)
```

## Document Boundaries

- **SPEC** — What must not break (principles, object model, invariants)
- **PRD** — What to build next (milestones, scope, backlog)
- **ENGINEERING** — How to build it (architecture, storage, pipelines, conventions)
- **ADR** — Why we decided X (trade-offs, alternatives, consequences)

## Architecture (decided, not yet implemented)

- **Rust Core** (cross-platform) — Data model, indexing, search, query/view engine, event processing, crypto/signing
- **Platform Shell** (Swift/SwiftUI on Apple) — UI, OS permissions, Keychain, Spotlight, Apple AI frameworks
- **FFI** — UniFFI to expose Rust Core to platform shells

Reference: `docs/adr/0003-core-architecture-rust-core-platform-shells.md`

## Core Design Decisions

- **File system is source of truth** — Databases and indexes are derived caches, rebuildable
- **Three-layer persistence** — SOT (user content) / Config (schemas, views, ignore rules) / Cache (SQLite index, FTS, vectors)
- **Single `.fracta/` directory** per managed location root — no `.DS_Store`-style pollution
- **Lazy UID assignment** — Only assign stable IDs when semantics require it (linking, metadata, proofs)
- **Managed / Ignored / Plain** scope model with gitignore-like rules
- **Database / Now / Past** information architecture

## Key Invariants (from SPEC)

- Local-first: usable offline, data on user-owned storage
- Open formats: Markdown/CSV/JSON + normal folders, readable without Fracta
- No keylogging, no covert monitoring, no upload by default
- Secrets in OS Keychain/Secure Enclave, never in files
- Atomic writes everywhere (temp → fsync → rename)

## Current Milestones (from PRD)

- **0.0 Foundation** — Docs, ignore rules, Folder Page v1, minimal schemas, `.fracta/` layout
- **0.1 Past Insight Dashboard** — Ingestors, events.jsonl, timeline+table views, AI summaries
- **0.2 Finder replacement baseline** — Browsing, sidebar metadata, incremental indexing
- **0.3 Multidimensional database** — Schema v1, saved views, query engine
- **1.0 Now project system** — Quests, rewards, AI task chains

## Conventions

- All docs are **English-only** (ADR-0009)
- Each topic has **one canonical home** — link, don't duplicate
- Any substantial change to invariants must be accompanied by an ADR
