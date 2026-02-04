# Fracta — Public Introduction (Draft)

This file is **external-facing**: a user/launch narrative and a lightweight introduction.

Canonical system truth lives in `docs/`:
- `docs/SPEC.md` — invariants (what must not break)
- `docs/PRD.md` — scope and milestones (what we build next)
- `docs/ENGINEERING.md` — implementation guide (how we build)
- `docs/adr/` — decision history (why)

Hard rule: if this file contradicts `docs/`, **`docs/` wins**. Link to `docs/` instead of duplicating implementation details.

---

## One sentence

Fracta is a **local-first personal data workbench** that helps you browse, understand, and evolve your files/notes/activity over time — on open formats, without lock-in.

## The user promise (non-negotiable)

- **Local-first**: works offline; your data stays on your storage.
- **Open formats**: primary data is readable without Fracta (Markdown/CSV/JSON/ICS/EML/OPML + normal folders).
- **File system as source of truth**: databases/indexes are derived caches, rebuildable.
- **Privacy by default**: no covert monitoring; no upload by default; secrets in OS secure storage.

## Narrow entry narrative (Phase 1)

Start small. Earn trust.

**First Light** (Milestone 0.1): “Finder, but it understands your files”
- browse files across Locations,
- preview/render Markdown,
- search in a Managed Location,
- optional AI call (provider-agnostic interface).

## What Fracta becomes (longer-term)

As capabilities grow, users may discover Fracta as a “life operating system”:
- **Library / Now / Past** as the default layout,
- **LIV** as a default workflow (optional and replaceable via Profiles),
- AI as infrastructure (mechanical automation + drafts pending human review),
- crypto as an optional economy layer (never required for core usability).

