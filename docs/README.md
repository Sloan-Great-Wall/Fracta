# Fracta Documentation

This repository is currently in the **software definition / design phase**.

**Policy**
- All documentation under `docs/` (including ADRs) is **English-only**.
- Each topic must have **one canonical home**. Other documents should link to it instead of duplicating it.

---

## Quick orientation

Fracta is a **local-first personal data workbench**:
- **Global View** feels Finder-like (browse everything).
- **Managed Scope** enables Fracta’s compute layer (Folder Pages, multidimensional metadata, views, indexing).
- Multidimensional “database views” are an **enhanced file-management mode**, not a separate world.
- Data stays in **open formats**; caches are rebuildable; sensitive secrets live in OS Keychain/Secure Enclave.

---

## Reading order

1. `docs/SPEC.md` — system definition (why / what / constraints)
2. `docs/PRD.md` — scope, milestones, and backlog (what to build)
3. `docs/ENGINEERING.md` — architecture + implementation guidance (how to build)

---

## Document boundaries (hard rules)

- **SPEC** answers: *Why? What is the system? What constraints must never break?*
- **PRD** answers: *What are we building next? What’s in/out? What are the milestones?*
- **ENGINEERING** answers: *How does it work end-to-end and how do we implement it (architecture, pipelines, storage, security, conventions)?*
- **ADR** answers: *Why did we decide X (trade-offs, alternatives, consequences)?*

---

## ADRs (Architecture Decision Records)

Directory: `docs/adr/`

Recommended starting ADRs:
- `0002-information-architecture-database-now-past.md`
- `0003-core-architecture-rust-core-platform-shells.md`
- `0005-uid-strategy-lazy-assignment.md`
- `0006-locations-global-view.md`
- `0007-managed-scope-and-ignore-rules.md`
- `0008-metadata-persistence-anti-dsstore.md`
- `0009-docs-simplification-and-english-only.md`

