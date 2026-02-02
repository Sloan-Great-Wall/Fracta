---
description:
alwaysApply: true
---

# CLAUDE.md

## Project Overview

Fracta is a **local-first life operating system**: a modular personal data workbench with a file/note/communication engine at its core, a semantic framework for organizing knowledge (Library), executing projects (Now), and reflecting on time (Past), and an application layer where methodologies like LIV, AI copilots, crypto incentives, and third-party integrations come together — all on open formats, without vendor lock-in.

Transitioning from **design phase** to **Phase 1 implementation** (Engine skeleton).

## Repository Structure

```
Cargo.toml                   — Rust workspace root
crates/
  fracta-vfs/                — Engine: file/folder CRUD, watching, atomic writes, scope
  fracta-index/              — Engine: SQLite index + FTS5
  fracta-query/              — Engine: filter/sort/group/aggregate
  fracta-note/               — Engine: Markdown parser + block model
  fracta-comm/               — Engine: IMAP/CalDAV/RSS/HTTP transports (stub)
  fracta-sync/               — Engine: conflict resolution, multi-device (stub)
  fracta-crypto/             — Engine: signing, hashing, key management (stub)
  fracta-ai/                 — Engine: LLM call interface, embeddings (stub)
  fracta-platform/           — Engine: platform adapter traits
  fracta-framework/          — Framework: Primitives, Pipelines, Orchestrator (stub)
  fracta-ffi/                — UniFFI bridge to Swift (stub)
docs/
  SPEC.md                    — System definition (invariants, architecture)
  PRD.md                     — Scope, milestones, roadmap, backlog
  ENGINEERING.md             — Architecture + implementation guidance + tech selections
  LIV_System_Manual.md       — Default methodology (Player Guide + System Spec)
  profiles/
    liv-profile-spec.md      — LIV Profile implementation spec (maps LIV → Fracta Primitives)
  adr/                       — Architecture Decision Records (13 ADRs)
```

## Document Boundaries

- **SPEC** — What must not break (principles, three-layer architecture, object model, invariants)
- **PRD** — What to build next (milestones, roadmap, scope, backlog)
- **ENGINEERING** — How to build it (architecture, storage, pipelines, tech selections, conventions)
- **LIV Manual** — Default methodology, user-facing (game-inspired execution system)
- **LIV Profile Spec** — Developer-facing: LIV objects → Fracta Primitives, config schema, enforcement
- **ADR** — Why we decided X (trade-offs, alternatives, consequences)

## Tech Stack

- **Rust** (stable) — Engine + Framework (cross-platform core)
- **Swift 6 + SwiftUI** — Application UI (Apple platforms)
- **UniFFI** — Rust ↔ Swift FFI bridge
- **Tokio** — Async runtime
- **rusqlite** (bundled) — SQLite + FTS5
- **comrak** — Markdown parsing (GFM, full mutable AST)
- **serde** — Serialization (JSON/YAML)
- **ed25519-dalek + blake3** — Crypto primitives

## Three-Layer Architecture

- **Engine Layer** — Generic infrastructure: VFS, Index, Query, Note, Comm (IMAP/CalDAV/RSS), Sync, Crypto Primitives, AI Primitives, Platform Adapters
- **Framework Layer** — Semantic layer: Primitives (Item/Record/Event/Quest/Loot/Claim/Token...), Pipelines, AI Orchestrator, Crypto/Token Engine, Knowledge Engine, Extension API
- **Application Layer** — User-facing: Default UI (Library/Now/Past), LIV Workflow, User Profiles, Third-party Connectors, Creator Marketplace

Implementation: Rust Core (Engine + Framework, cross-platform) + Platform Shell (Application UI, Swift/SwiftUI on Apple) + UniFFI bridge.

Reference: `docs/adr/0010-three-layer-architecture.md`

## Core Design Decisions

- **File system is source of truth** — Databases and indexes are derived caches, rebuildable
- **Three-level openness** — Data formats open; Engine+Framework open-source; Application layer commercial
- **Three-layer persistence** — SOT (user content, open formats) / Config (JSON/YAML) / Cache (SQLite, vectors, AI-derived)
- **Single `.fracta/` directory** per managed location root — no `.DS_Store`-style pollution
- **Lazy UID assignment** — Only assign stable IDs when semantics require it
- **Managed / Ignored / Plain** scope model with gitignore-like rules
- **Library / Now / Past** default information architecture (Application layer; customizable via Profiles)
- **LIV as default Profile** — game-inspired methodology; other Profiles possible
- **Past: Markdown SOT + AI cache** — AI builds timeline skeleton; user adds soul; structured data is derived cache
- **AI as infrastructure** — pervasive across all layers; mechanical automation + creative drafting
- **Crypto as native economy** — token incentives, attestation, marketplace; completely optional

## Key Invariants (from SPEC)

- Local-first: usable offline, data on user-owned storage
- Open formats: Markdown/CSV/JSON/ICS/EML/OPML + normal folders, readable without Fracta
- Low floor, high ceiling: works out-of-the-box for simple users; extensible for power users
- AI outputs materialized as open-format files, never hidden state
- No keylogging, no covert monitoring, no upload by default
- Core features never depend on crypto/tokens
- Secrets in OS Keychain/Secure Enclave, never in files
- Atomic writes everywhere (temp → fsync → rename)

## Current Milestones (from PRD)

- **0.0 Foundation** ← nearly complete — Architecture decisions, interface specs, tech selections, project scaffold
- **0.1 First Light** — App opens, browses files, renders Markdown, AI interface functional
- **0.2 Past Insight Dashboard** — Ingestors, AI timeline, daily Markdown SOT, metrics, summaries
- **0.3 Now + LIV** — Quest/Loot Primitives, LIV Profile, AI guardrails, token rewards
- **0.4 Library + Views** — Schema, query engine, Finder-like browsing, sidebar, tags
- **1.0 Life OS** — All areas integrated, Creator Marketplace, full AI + Crypto

## Conventions

- All docs are **English-only** (ADR-0009)
- Each topic has **one canonical home** — link, don't duplicate
- Any substantial change to invariants must be accompanied by an ADR
- Architecture layer boundary: Engine → Framework → Application (no skipping layers)
