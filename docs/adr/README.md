# ADR (Architecture Decision Records)

ADRs capture **why** we made a decision (trade-offs, constraints, alternatives), so future contributors do not have to re-litigate the same debates and so canonical docs do not drift into contradictions.

---

## Index

| ADR | Decision |
|-----|----------|
| [0001](0001-product-name-fracta.md) | Product name is Fracta |
| [0002](0002-information-architecture-database-now-past.md) | Information architecture: three areas (terminology updated in ADR-0014) |
| [0003](0003-core-architecture-rust-core-platform-shells.md) | Core architecture: Rust Core + Platform Shells (UniFFI) |
| [0004](0004-ingestion-strategy-platform-core-ingestors.md) | Ingestion strategy: Platform + Core Ingestors |
| [0005](0005-uid-strategy-lazy-assignment.md) | UID strategy: Lazy assignment |
| [0006](0006-locations-global-view.md) | Locations and Global View |
| [0007](0007-managed-scope-and-ignore-rules.md) | Managed Scope and Ignore Rules |
| [0008](0008-metadata-persistence-anti-dsstore.md) | Metadata persistence: Anti-.DS_Store |
| [0009](0009-docs-simplification-and-english-only.md) | Documentation simplification and English-only |
| [0010](0010-three-layer-architecture.md) | **Three-layer architecture: Engine / Framework / Application** |
| [0011](0011-liv-as-default-application-profile.md) | **LIV as default Application Profile** |
| [0012](0012-past-format-markdown-sot-ai-cache.md) | **Past format: Markdown SOT + AI-derived cache** |
| [0013](0013-open-source-engine-framework-commercial-application.md) | **Open-source Engine + Framework, commercial Application** |
| [0014](0014-rename-database-to-library.md) | Rename "Database" → "Library" terminology |
| [0015](0015-index-sqlite-metadata-tantivy-search.md) | **Index: SQLite metadata + Tantivy full-text search** |
| [0016](0016-apache-2.0-license.md) | **Apache-2.0 license selection** |

---

## When to write an ADR

- Any decision that affects long-term architecture, on-disk formats, security/privacy, or cross-platform strategy
- Any "small now, painful later" convention (e.g., UID rules, directory layout, what gets persisted to YAML)
- Any change to the three-layer boundary (Engine / Framework / Application)

---

## Suggested template

- Filename: `NNNN-topic-in-kebab-case.md` (example: `0001-local-first-sot.md`)
- Sections:
  - **Context**: background and constraints
  - **Decision**: what we decided
  - **Consequences**: impact and costs
  - **Alternatives**: considered options and why they were rejected

---

## Relationship to other docs

- `docs/SPEC.md` defines "what must not break" (system definition and invariants)
- `docs/PRD.md` defines "what to build next" (scope, milestones, roadmap)
- `docs/ENGINEERING.md` defines "how we build it" (architecture + implementation guidance)
- ADRs define "why we chose this approach"
