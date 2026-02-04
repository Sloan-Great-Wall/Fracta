# ADR-0002: Information architecture uses Database / Now / Past

## Status

Superseded (terminology) by ADR-0014. The three-area architecture remains, but the canonical naming is now **Library / Now / Past**.

## Context

We need a long-lived mental model that can simultaneously support:

- A Finder-level file manager (large numbers of files/folders/assets)
- Notion-like multidimensional databases and views
- Automated time journaling and review (multi-source event streams)
- AI automation and agents
- Optional cryptographic/blockchain attribution, incentives, and transactions

Earlier drafts explored multiple top-level taxonomies (domains/projects/tasks, “introduction/projects”, “workspace/lifelong_story”, etc.). Divergent top-level models tend to cause:

- Semantic shifts when moving/archiving data (forcing refactors)
- Duplicate modeling of the same content in multiple “systems”
- Unclear boundaries between product narrative and implementation

## Decision

Inside each user space (internally: “vault”; externally: “location”), the top-level information architecture is fixed to three areas:

- **Database**: relatively stable libraries/resources (books, videos, templates, public references).
- **Now**: a minimal, focused set of active projects (e.g., 1 health project + 1–3 work projects).
- **Past**: a time-axis stream of what already happened (multi-source ingestion + narrative + stats/review).

Constraints:

- All three areas share the same underlying object model and metadata rules (Item/Record/Event/View/Claim/Asset).
- Moving between areas is a semantic transition (Now → Past for archiving; Database → Now for turning a resource into a project), but the underlying formats remain consistent.

## Consequences

- `docs/SPEC.md` is the canonical definition; PRD/engineering/implementation must follow this information architecture.
- Older naming schemes (e.g., “introduction/projects”, “workspace/lifelong_story”) may survive only as compatibility naming or UI views, not as the top-level truth structure.

## Alternatives

- **Domain/Project/Task hierarchy**: good for productivity workflows, but it does not unify a time-axis stream and resource libraries well, and hierarchies tend to balloon.
- **Pure timeline**: strong for Past, but weak for long-running projects and resource libraries.


