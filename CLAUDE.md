---
description:
alwaysApply: true
---

# CLAUDE.md

## Project Overview

Fracta is a **local-first life operating system**: a modular personal data workbench with a file/note/communication engine at its core, a semantic framework for organizing knowledge (Library), executing projects (Now), and reflecting on time (Past), and an application layer where methodologies like LIV, AI copilots, crypto incentives, and third-party integrations come together — all on open formats, without vendor lock-in.

Currently in the **software definition / design phase** (docs only, no implementation code yet).

## Repository Structure

```
README.md              — Repo entrypoint
docs/
  README.md            — Docs entrypoint + reading order
  SPEC.md              — System definition (why / what / constraints / architecture / invariants)
  PRD.md               — Product scope, milestones, roadmap, backlog
  ENGINEERING.md       — Architecture + implementation guidance
  LIV_System_Manual.md — Default execution methodology (Player Guide + System Spec)
  adr/                 — Architecture Decision Records (13 ADRs)
```

## Document Boundaries

- **SPEC** — What must not break (principles, three-layer architecture, object model, invariants)
- **PRD** — What to build next (milestones, roadmap, scope, backlog)
- **ENGINEERING** — How to build it (architecture, storage, pipelines, conventions)
- **LIV Manual** — Default methodology (game-inspired execution system)
- **ADR** — Why we decided X (trade-offs, alternatives, consequences)

## Three-Layer Architecture (decided, not yet implemented)

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

- **0.0 Foundation** ← current — Architecture decisions, interface specs, schemas, `.fracta/` layout
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
