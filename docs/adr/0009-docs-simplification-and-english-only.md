# ADR-0009: Documentation Simplification & English-only Policy

## Context

The repository currently contains multiple “functional docs” (SPEC/PRD/ROADMAP/BACKLOG/etc.) plus ADRs.
At this early “software definition” stage, duplication and mixed-language docs increase maintenance cost and create ambiguity.

We also want the docs to be accessible to a broader set of future contributors, and to be consistent across the entire system.

## Decision

1. **English-only documentation**
   - All documentation content under `docs/` (including ADRs) must be written in **English**.
   - If an idea originates in Chinese, it should be translated and normalized into the canonical English docs.

2. **Simplified functional doc set**
   - Keep only these canonical docs:
     - `README.md` (repo entrypoint; should be a thin pointer to `docs/`)
     - `docs/README.md` (docs entrypoint + reading order + boundaries)
     - `docs/SPEC.md` (system definition: why/what/constraints/invariants)
     - `docs/PRD.md` (product scope + milestones + backlog)
     - `docs/ENGINEERING.md` (architecture + implementation guidance)
     - `docs/adr/` (decision history; `docs/adr/README.md` is the index)

3. **Merge ROADMAP/BACKLOG into PRD**
   - `docs/ROADMAP.md` and `docs/BACKLOG.md` are merged into `docs/PRD.md` to avoid duplication.
   - After merging, the standalone files are removed.

## Consequences

- The doc system becomes smaller, clearer, and cheaper to maintain.
- PRD becomes the single place where roadmap/milestones and backlog live.
- ADRs remain the source of “why” and are used to record any future doc-system changes.
 - Redundant split docs (e.g., separate “architecture” vs “dev guide” documents) should not coexist with `docs/ENGINEERING.md` (to avoid drift). Consolidate and remove duplicates.

## Alternatives

- Keep ROADMAP/BACKLOG as separate docs: clearer separation, but higher duplication and drift risk.
- Keep bilingual docs: lower translation overhead, but long-term consistency and onboarding suffer.

