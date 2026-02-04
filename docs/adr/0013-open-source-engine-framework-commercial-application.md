# ADR-0013: Open-Source Engine + Framework, Commercial Application Layer

## Status

Accepted

## Context

Fracta's core promise is that users own their data and are never locked in. This raises a strategic question: how much of the system should be open-source, and where does the business model live?

Users of productivity tools carry deep fear of vendor lock-in. They have experienced:
- Evernote degrading and making export painful.
- Notion owning all data in a proprietary cloud database.
- Apps shutting down with short notice and inadequate export tools.
- Switching costs that make them stay in tools they've outgrown.

Fracta aims to eliminate this fear entirely.

## Decision

### Three-level openness

**Level 1 — Data formats (open)**

All user-created data uses standard open formats: Markdown, CSV, JSON, ICS (calendar), EML (email), OPML (feeds), and normal folders. Users can read, edit, and process their data with any tool at any time.

**Level 2 — Engine + Framework (open-source)**

The Engine layer (VFS, Index, Query, Note, Comm, Sync, Crypto Primitives, AI Primitives) and the Framework layer (Semantic Primitives, Pipelines, AI Orchestrator, Crypto/Token Engine, Knowledge Engine, Extension API) are released as open-source software.

This means:
- If Fracta the company ceases to exist, the community can fork and maintain the core.
- Third-party developers can build alternative Application layers.
- CLI tools can operate on `.fracta/` directories without the GUI app.
- The `.fracta/` directory structure specification is public.
- Users can inspect, audit, and extend the code.

**Level 3 — Application layer (commercial)**

Revenue comes from the Application layer:
- **Default UI Shell**: the polished macOS/iOS app with Library/Now/Past layout.
- **Premium Profiles**: LIV and other professionally designed workflow methodologies.
- **Advanced AI features**: cloud-based AI models, advanced knowledge synthesis, multi-device AI sync.
- **Creator Marketplace**: platform fees on Profile/Template/View trades.
- **Team/Organization features**: shared Locations, collaborative workflows, admin controls.
- **Cloud sync service**: optional Fracta-hosted sync for users who don't want to self-host.

### The user promise

Even if you stop paying (or Fracta disappears):
- Your **data** (Level 1) is intact and usable with any tool.
- Your **tools** (Level 2) are open-source and can be community-maintained.
- You lose premium **UI** and **marketplace** access, not your data or your ability to process it.

## Alternatives considered

1. **Fully open-source with SaaS revenue**: Everything is open-source; revenue comes from hosted services only. Rejected because a local-first product has limited SaaS surface area, and this model undervalues the Application layer.

2. **Fully proprietary with open data formats**: Only the file formats are open; all code is proprietary. Rejected because it doesn't provide the "framework survives" guarantee that users need for true lock-in elimination.

3. **Open-core with "Community Edition" vs "Enterprise Edition"**: Common in B2B. Rejected because Fracta is B2C/prosumer — the Community/Enterprise split creates confusing messaging for individual users.

4. **Open everything, monetize support/consulting**: Works for infrastructure (Red Hat model), not for consumer products. Rejected.

## Consequences

- The codebase must maintain a clean boundary between open (Engine + Framework) and commercial (Application) code.
- Open-source licensing must be chosen carefully (**TBD**). Selection criteria should include: ecosystem friendliness, contribution incentives, protection against closed-source competitors, and a clean legal boundary to the commercial Application layer. The final choice will be captured in a follow-up ADR; the current `Cargo.toml` license field is a provisional placeholder until that decision is made.
- The Extension API is the critical interface: it must be stable, documented, and powerful enough that third-party Application layers are viable.
- Marketing must clearly communicate the three-level openness to build user trust.
- The Creator Marketplace becomes a key revenue driver and must be designed into the Framework from the start (not bolted on later).
