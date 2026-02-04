# ADR-0014: Rename "Database" area to "Library"

## Status

Accepted

## Context

ADR-0002 established a three-area information architecture. Early drafts used the term **Database** to mean "stable libraries/resources" (books, courses, templates, public references).

In practice, "Database" is a high-friction word:
- It implies a technical, schema-first system and can intimidate mainstream users.
- It over-promises structure at the top level (even though Fracta stays file-first and progressively enhanced).
- It conflicts with the intended product narrative: *Library / Now / Past* is immediately legible.

Meanwhile, the rest of the canonical docs and roadmap have converged on **Library / Now / Past** as the default layout.

## Decision

Rename the top-level area formerly called **Database** to **Library**.

Rules:
- Canonical docs and UI use **Library / Now / Past**.
- The term **Database** may appear only as a historical alias (legacy references, migration notes).
- This change affects naming only; the underlying three-area mental model remains the same.

This ADR supersedes ADR-0002 **terminology** (not the three-area architecture itself).

## Consequences

- Documentation must be normalized to **Library / Now / Past**.
- If any on-disk layout, APIs, or UI labels previously used `database`, they should:
  - migrate to `library`, or
  - support `database` as a compatibility alias during transition (time-bounded).
- Future writing should avoid introducing new "Database" terminology drift.

## Alternatives considered

1. **Keep "Database"**
   - Rejected: too much user-facing friction; misaligned with product narrative.

2. **Support both "Database" and "Library" indefinitely**
   - Rejected: creates permanent ambiguity and documentation drift.

## Related updates (same cleanup cycle)

Alongside this terminology fix, we may update canonical docs to reduce contributor confusion during Phase 1:
- explicitly label "planned vs current repo structure" in engineering docs,
- normalize Profile on-disk paths (package vs active pointer),
- clarify open-source license selection as TBD until a dedicated license ADR is written.

