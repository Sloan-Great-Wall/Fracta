# ADR-0001: Product name is Fracta

## Context

Older notes and drafts used multiple names / codenames. This creates:

- Inconsistent external messaging vs internal engineering communication
- Confusing naming across docs and (future) code, increasing coordination and maintenance cost

## Decision

- The product name is **`Fracta`** (used both externally and internally).
- Internal module/package/repo names may differ, but any difference MUST be mapped explicitly in `docs/ENGINEERING.md` and must not affect outward naming consistency.

## Consequences

- All canonical docs use **Fracta** as the name.
- Older names, if kept at all, appear only as historical aliases and are never the primary narrative.


