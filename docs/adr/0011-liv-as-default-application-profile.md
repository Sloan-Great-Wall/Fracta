# ADR-0011: LIV as Default Application Profile

## Status

Accepted

## Context

LIV is a game-inspired personal execution methodology designed by the Fracta creator. It defines a complete system for lifelong learning and growth: a three-phase loop (Explore/Capture → Clear Challenges → Resupply/Upgrade), domain objects (Inbox, Quest, Dungeon Run, Loot, HUD, Rulebook), invariants (Quest Slots ≤ 2, no promotion outside Resupply, loot requirement), and built-in failure-recovery guardrails.

The Fracta product vision evolved from "file workbench" to "life operating system". This raises the question: what is LIV's role in the architecture?

Three options were considered:

1. **LIV concepts hard-coded into Framework**: Quest Slots, Dungeon Runs, etc. are Framework Primitives that all users must use.
2. **LIV as external reference**: Fracta's Framework is generic; LIV is a separate document that informs but doesn't constrain.
3. **LIV as a default Application Profile**: Framework provides generic Primitives (Quest, Loot, Rule, Schedule, Metric); LIV is a specific configuration of those Primitives, shipped as the default "gameplay" but replaceable.

## Decision

**LIV is the default Application Profile** (option 3).

### What this means

- **Framework level**: Semantic Primitives (Quest, Loot, Rule, Schedule, Metric, etc.) are generic. The Framework defines what these objects are and how they interact, but does not enforce specific constraints like "Quest Slots ≤ 2".

- **Application level**: LIV is a **Profile** — a JSON/YAML configuration that binds Framework Primitives into a specific workflow:
  - Quest Slots cap = 2 (default 1)
  - No promotion outside Resupply
  - Loot requirement per Dungeon Run
  - Budget protection 30/60/10
  - Night rule
  - Specific incentive triggers (complete Quest → earn tokens)
  - HUD metric definitions

- **User experience**: New users get LIV as the default out-of-the-box experience. Power users can modify LIV's parameters, create hybrid approaches, or build entirely different Profiles.

- **Creator economy**: Profile creators can design and sell alternative methodologies (GTD-based, OKR-based, domain-specific like "PhD Student Profile" or "Freelancer Profile") through the Creator Marketplace.

### Knowledge features as Framework foundation

AI-driven capabilities like topic clustering, thought-lineage tracking, and open-question extraction are **Framework-level features** (Knowledge Engine), not LIV-specific. LIV uses them (e.g., Side Quest emerging from fog = AI-discovered recurring topic), but they are available to any Profile.

### LIV System Manual

The `docs/LIV_System_Manual.md` document remains as the complete methodology reference. It is not absorbed into SPEC or PRD; instead, SPEC §4.3 and PRD reference it as the design source for the default Profile.

## Consequences

- Framework Primitives must be generic enough to support workflows beyond LIV.
- LIV's invariants (Quest Slots cap, etc.) are enforced by the LIV Profile configuration, not hard-coded in Framework logic.
- The Framework's AI Orchestrator and guardrail system must support Profile-defined rules, not just LIV-specific rules.
- Profile creators need a documented Profile schema and registration mechanism (part of Extension API).
- LIV becomes both the reference implementation (proving the Framework works) and the flagship product (attracting users).
