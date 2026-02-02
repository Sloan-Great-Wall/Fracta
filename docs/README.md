# Fracta Documentation

This repository is currently in the **software definition / design phase**.

**Policy**
- All documentation under `docs/` (including ADRs) is **English-only**.
- Each topic must have **one canonical home**. Other documents should link to it instead of duplicating it.

---

## Quick orientation

Fracta is a **local-first life operating system**:
- **Three-layer architecture**: Engine (generic infrastructure) → Framework (semantic layer) → Application (user-facing workflows).
- **Global View** feels Finder-like (browse everything). **Managed Scope** enables compute features.
- **Library / Now / Past** is the default Application layout: organize knowledge, execute projects, reflect on time.
- **LIV** is the default execution methodology (game-inspired: Quest Slots, Dungeon Runs, Loot, HUD, Resupply).
- **AI** permeates every layer: mechanical automation + creative drafting pending human review.
- **Crypto** provides a native token economy: incentives, attestation, and creator marketplace — completely optional.
- Data stays in **open formats** (Markdown/CSV/JSON/ICS/EML/OPML); Engine + Framework are **open-source**; Application layer is commercial.

---

## Reading order

1. `docs/SPEC.md` — system definition (why / what / constraints / architecture / invariants)
2. `docs/PRD.md` — scope, milestones, roadmap, and backlog (what to build)
3. `docs/ENGINEERING.md` — architecture + implementation guidance (how to build)
4. `docs/LIV_System_Manual.md` — the default execution methodology (Player Guide + System Spec)
5. `docs/profiles/liv-profile-spec.md` — LIV Profile implementation spec (maps methodology → Fracta Primitives)

---

## Document boundaries (hard rules)

- **SPEC** answers: *Why? What is the system? What constraints must never break?*
- **PRD** answers: *What are we building next? What's in/out? What are the milestones?*
- **ENGINEERING** answers: *How does it work end-to-end and how do we implement it (architecture, pipelines, storage, security, conventions)?*
- **LIV Manual** answers: *What is the default methodology and how does it work as a system?* (user-facing)
- **LIV Profile Spec** answers: *How does LIV map to Fracta Primitives, Pipelines, and config?* (developer-facing)
- **ADR** answers: *Why did we decide X (trade-offs, alternatives, consequences)?*

---

## ADRs (Architecture Decision Records)

Directory: `docs/adr/`

Key ADRs:
- `0002` — Information architecture: Library / Now / Past
- `0003` — Core architecture: Rust Core + Platform Shells
- `0010` — **Three-layer architecture: Engine / Framework / Application**
- `0011` — **LIV as default Application Profile**
- `0012` — **Past format: Markdown SOT + AI-derived cache**
- `0013` — **Open-source Engine + Framework, commercial Application**

Full index: `docs/adr/README.md`
