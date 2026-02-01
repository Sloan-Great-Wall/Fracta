# ADR-0008: Metadata persistence avoids `.DS_Store`-style design

## Context

Fracta needs a “compute layer” on top of the filesystem:

- Folder Pages (folder-as-page)
- Multidimensional attributes/fields and database-like views
- Full-text/semantic indexing, backlinks, stats, AI artifacts, and optional proofs

Users strongly dislike Finder’s `.DS_Store` (and similar hidden state files). Common pain points:

- **Everywhere**: one file per folder pollutes directories and creates huge version-control/sync noise
- **Fragile**: binary + high-frequency writes + concurrency/power loss/cloud-sync conflicts → corruption and weird states
- **Uncontrolled migration**: copies/archives carry lots of useless state, and devices diverge

Fracta aims to feel Finder-like, but must not fall into the same engineering and UX traps.

## Requirements

We must satisfy, simultaneously:

- **Local-first + portable**: copying/syncing a data directory should preserve Fracta experience as much as possible
- **Minimal intrusion**: do not write hidden files across the entire disk; writes must be explainable and reversible
- **Stability**: crashes/power loss/concurrent sync must not create corrupted or half-written state
- **Performance**: indexing/views/stats need caches, but caches must be safe to delete and rebuild

## Decision

Use a “three-layer + centralized system area” persistence strategy:

1. **Centralized system area**: create exactly one `.fracta/` at the **managed location root**, not hidden files in every subfolder.
2. **SOT / Config / Cache layering**:
   - **SOT (source of truth)**: user content + necessary portable metadata
   - **Config**: schema/view/ignore, etc. (versionable and portable)
   - **Cache**: indexes/materialized views/derived stats/vector stores (rebuildable)
3. **Text formats + atomic writes**:
   - Prefer YAML/JSON for config/metadata
   - Use temp file → (optional fsync) → atomic rename to avoid half-written state

And enforce the **Managed / Unmanaged (including Ignored)** boundary:

- Unmanaged directories: passthrough browsing only; do not write `.fracta/` or metadata
- Ignored directories: even inside managed scope, explicitly do not write, index, or materialize compute artifacts

## Consequences

- **No `.DS_Store`-style directory pollution** (system area is centralized).
- **More controlled migration**: copying a managed location can keep `.fracta/config` and `.fracta/meta`; `.fracta/cache` can be safely deleted and rebuilt.
- **Higher stability**: text + atomic writes reduce corruption risk; cache loss does not destroy user content.
- **Requires transparent UX/tools**:
  - clearly label directory state (Managed / Unmanaged / Ignored)
  - provide “clear cache” and “export package (config/meta)” actions

## Alternatives

- **One hidden file per directory (like `.DS_Store`)**: superficially simple, but too noisy, fragile, and unpopular with users.
- **Store all metadata in a database (SQLite)**: good performance, but poor portability/visibility and risks violating “files are the source of truth”.

