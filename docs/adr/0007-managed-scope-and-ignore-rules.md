# ADR-0007: Managed Scope and ignore rules (gitignore-like)

## Context

Fracta aims to provide Finder-like global browsing while enabling compute-layer capabilities (Folder Pages, metadata indexing, multidimensional views) only where appropriate.

Without an ignore mechanism:

- `node_modules`, build artifacts, and cache directories will drag down indexing/view performance
- Users will worry that Fracta “invades” all directories (writing `.fracta/` or metadata)
- Very large folders create UI/index noise, violating “minimal by default”

## Decision

Introduce **Managed Scope** and **Plain Mode**:

- **Global View**: browse the whole visible filesystem
- **Managed Scope**: compute-layer features only apply within user-enabled managed Locations
- **Plain Mode**: Finder-style passthrough (visible + openable), but no compute layer (no Folder Pages, no metadata indexing, no writes)

Directory states (UI must be able to explain):

- **Managed**: compute layer enabled (Folder Pages / metadata indexing / multidimensional views, etc.)
- **Ignored**: still within a managed location’s path range, but excluded by ignore rules or size heuristics; behaves like Plain (still visible/openable) and MUST NOT be “implicitly managed” by a managed parent
- **Plain**: not inside any managed location (or not yet enabled); behaves like Finder passthrough browsing

Key distinction:

- **Plain** = not managed (outside the compute layer scope)
- **Ignored** = within the managed scope but explicitly excluded to prevent accidental management and performance collapse

Optional UI simplification:

- UI may merge **Plain + Ignored** into a single state: **Unmanaged (compute layer disabled)**
- But it must still expose a reason:
  - `not_managed` (Plain)
  - `ignored_by_rule` / `ignored_by_heuristic` (Ignored)
  - `no_permission` (no authorization; cannot read/index)

Ignore rules for Managed Scope (gitignore-like):

- Recommended rules file: `LocationRoot/.fracta/config/ignore` (text; versionable)
- Optional alias: `LocationRoot/.fractaignore`
  - Goal: let users edit rules like `.gitignore` (high readability)
  - Not required: users can rely on built-in defaults + `.fracta/config/ignore` without adding more root files
- Effect: exclude paths inside a managed location, downgrading them to Unmanaged behavior (still visible/openable, but no compute layer)
- Syntax: glob-style (close to gitignore; exact parsing can evolve)

Default ignore strategy (built-in + configurable):

- Directory patterns: `**/node_modules/**`, `**/.git/**`, `**/DerivedData/**`, `**/__pycache__/**`, `**/.venv/**`
- Size heuristics (tunable): if directory depth or file count is too large, do not enter the compute layer by default (but allow an explicit “force manage” override)

Rule layering (to avoid “global vs per-location” confusion):

- **Built-in defaults**: always present (e.g., `node_modules`, `.git`)
- **App-level global ignore (optional)**: applies across all locations (preferences/policy)
- **Location-level ignore (recommended)**: travels with the location (in `.fracta/config/ignore` or optional `.fractaignore`)

## Consequences

- Managed features (Folder Pages/views/indexing) will not pollute or slow down irrelevant directories.
- Users can progressively enable management; the system expands capabilities in a “safe by default / minimal by default” way.
- The UI must clearly show directory state and offer quick actions:
  - Manage / Ignore / Unignore
- Migration and stability:
  - `.fracta/` appears only at the managed location root (not “one hidden file per folder”)
  - Config/metadata should be text-based (YAML/JSON), written via temp file → atomic rename
  - Caches are always rebuildable; ignored directories are not written to, indexed, or materialized

## Alternatives

- **Enable compute layer everywhere**: aggressive UX, but too risky in privacy and performance.
- **Only allow manual folder selection (no Global View)**: simpler implementation but fails the “global aggregation” goal.

