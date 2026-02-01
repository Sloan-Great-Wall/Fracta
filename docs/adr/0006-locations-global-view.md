# ADR-0006: Locations aggregation with default Global View (no mandatory vault management)

## Context

Fracta must be “file as source of truth + portable”. Under macOS/iOS sandbox permission models:

- The app can only access directories the user authorizes (e.g., security-scoped bookmarks).
- Users expect “open and see my files like Finder”, and do not want an Obsidian-style requirement to explicitly create/manage a vault.
- Users also want cloud-drive aggregation (Google Drive / OneDrive, etc.) to feel like native Finder folders.

We must balance:

- **User experience**: minimize “concept teaching”; do not force users to manage a vault concept.
- **Engineering reality**: we still need a concept of a root directory where Fracta can persist system data (for `.fracta/`).

## Decision

- Do not emphasize “vault” in the product UX. Use a single term:
  - **Location**: a directory tree that Fracta can manage (local folder, external disk, cloud-sync folder, etc.).
- Provide a default **Global View**:
  - Opening the app presents a Finder-like browsing experience (as broad as OS permissions allow).
  - For paths without permission, the UI may show structure, but reading/indexing/enhancement is constrained by OS permissions (prompt for authorization when needed).
- Enable Fracta’s **compute-layer** capabilities (Folder Pages, metadata indexing, multidimensional views, etc.) only for **Managed Locations**:
  - A Managed Location creates a single `.fracta/` system area at the location root.
  - Directories outside Managed Locations operate in **Plain Mode** (browse/open only; no Folder Pages or semantic compute layer).
- Support multiple Locations and automatic aggregation:
  - Auto-discover common candidate locations (home directory areas, common cloud-sync directories, external disks, etc.).
  - Users can enable/disable “managed” status for any location (no explicit vault creation required).
- Use a gitignore-like ignore/blacklist mechanism (see ADR-0007):
  - Large/noisy directories (e.g., `node_modules`) remain visible and openable, but do not enter the compute layer.

## Consequences

- Docs may mention “vault” as an internal synonym for Location, but the UI and product narrative should avoid pushing “vault” as a primary concept.
- We need to implement:
  - Location permissions + authorization prompts (especially on Apple platforms)
  - Clear boundaries in Global View (Managed vs Plain)
  - Ignore rules file + default ignore strategy (ADR-0007)
  - Cloud-drive aggregation:
    - If a cloud drive already appears as a directory (File Provider / sync folder), treat it as a Location.
    - If login is needed for discovery/status, OAuth can be an optional enhancement without breaking local-first.

Cloud “directory form” vs OAuth (not mutually exclusive):

- **Prefer directory form as the factual source**: if Google Drive/OneDrive already appears as folders, Fracta manages that one tree and does not index a second copy.
- **OAuth as an enhancement layer**: account-level capabilities (discover locations, status, remote metadata, sharing/permissions), bound to the same Location (avoid double indexing and duplicate UI).
- **For drives without good OS integration**: treat OAuth as a “Remote Location” (in-app browsing + on-demand download/cache), and only later consider deeper OS-level mounting.

## Alternatives

- **Mandatory vault mode (explicit multiple vaults)**: conceptually clean, but too much mental overhead for the target UX.
- **Full system-wide indexing**: not feasible or too risky under sandbox and privacy constraints.

