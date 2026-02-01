# ADR-0005: UID strategy uses lazy assignment

## Context

Fracta needs stable identities (UIDs) to support:

- Links, views, backlinks, and proofs that survive file/folder moves and renames
- Stable association between database records (Record) and filesystem entities (Item)

However, assigning UIDs to every file/folder up front is problematic:

- High generation and write cost (a lot of low-value writes)
- “Pollutes” user files, especially for content the user never touches
- Erodes user trust (“Fracta modified all my files”)

## Decision

Use **Lazy UID assignment**:

- Only create and persist a UID when the user performs an operation that requires stable semantics, for example:
  - writing multidimensional metadata (entering a database view)
  - creating bidirectional links (references/backlinks)
  - generating or referencing an object from a Folder Page
  - participating in proof/incentive/transaction features
- For untouched objects, the system may use temporary identities (path + mtime + size, etc.) for short-term indexing, with **no guarantee** of semantic continuity across moves.

## Consequences

- Fracta’s writes to user data are fewer and more controlled, aligning with “minimal by default” and trust.
- Untouched files moved outside Fracta may require re-indexing to restore temporary associations (an acceptable boundary).
- We should provide tools (or UI) such as:
  - `fracta doctor`: detect missing UIDs, broken links, orphan metadata
  - `fracta assign-uid`: optionally assign UIDs in bulk for a selected scope

## Alternatives

- **Eager UID assignment (assign to everything)**: strongest semantic continuity, but too intrusive and expensive for the product stage and minimalism goals.

