# ADR-0010: Three-Layer Architecture (Engine / Framework / Application)

## Status

Accepted

## Context

The original architecture (ADR-0003) defined a two-layer model: **Rust Core** (cross-platform logic) and **Platform Shell** (OS-specific UI and integrations). This is an implementation-level split — it describes how code is organized, but not how concepts are layered.

As the product vision expanded from "file workbench" to "life operating system", several needs emerged that the two-layer model cannot cleanly address:

1. **Semantic gap**: The Rust Core handles raw file operations, indexing, and queries, but there is no defined layer for semantic concepts like Quest, Loot, Pipeline, or Profile. These concepts float between Core and Shell with no clear home.

2. **Extensibility**: Users and third-party creators need to build custom workflows (Profiles) on top of Fracta's capabilities. A two-layer model forces them to either modify the Core (too low-level) or the Shell (too platform-specific).

3. **AI and Crypto as cross-cutting concerns**: AI orchestration and crypto/token economics need to integrate across file management, note-taking, task execution, and time journaling. They don't belong in either "Core" or "Shell" — they need a dedicated semantic layer.

4. **Commercial model**: The open-source commitment (Engine + Framework) and the commercial surface (Application) require a clear architectural boundary.

## Decision

Adopt a **three-layer architecture** inspired by game-engine design:

### Engine Layer ("physics engine")

Generic infrastructure with no knowledge of Fracta-specific workflows. Subsystems: VFS, Index, Query, Note, Comm (protocol transports), Sync, Crypto Primitives, AI Primitives, Platform Adapters.

Each subsystem exposes trait/protocol interfaces. Engine never depends on Framework or Application.

### Framework Layer ("game mechanics")

Fracta's core semantics. Components: Semantic Primitives (Item, Record, Event, Quest, Loot, Claim, etc.), Pipelines (Capture → Route → Process → Materialize → Archive), AI Orchestrator, Crypto/Token Engine, Knowledge Engine, Extension API.

Framework operates through Engine interfaces only. All AI-derived artifacts are cache.

### Application Layer ("gameplay")

What users see and interact with. Components: Default UI Shell (Library/Now/Past), LIV Workflow (default Profile), User Profiles, Third-party Connectors, Creator Marketplace.

Application uses Framework APIs only; it never calls Engine directly.

### Relationship to Rust Core + Platform Shell

The two-layer implementation model (ADR-0003) still holds for **code organization**:

- **Rust Core** implements Engine + Framework (both cross-platform).
- **Platform Shell** implements Application UI + Platform Adapters (OS-specific).
- **UniFFI** bridges the two.

The three-layer model is a **conceptual architecture** that sits above the implementation split. It defines dependency rules and API boundaries that the code must respect.

## Alternatives considered

1. **Keep two layers, add "plugin system"**: Would conflate infrastructure extensions (new file formats) with semantic extensions (new workflow Profiles). Rejected because it creates a flat extension model where everything is a "plugin" with no clear contracts.

2. **Four layers (Engine / Data / Logic / UI)**: More granular separation between data model and business logic. Rejected as premature — the Framework layer combines both, and splitting them now would add complexity without clear benefit at this stage.

3. **Microkernel / message-passing**: Each subsystem as an independent process communicating via messages. Rejected due to performance overhead and complexity for a local-first desktop application.

## Consequences

- SPEC must define the three-layer boundary and the interfaces between them.
- ADRs must specify which layer a decision applies to.
- The Extension API becomes a first-class design concern, not a future add-on.
- AI and Crypto are explicitly positioned as Framework-level cross-cutting systems, not optional Engine plugins.
- The open-source boundary (Engine + Framework = open; Application = commercial) has a clean architectural definition.
