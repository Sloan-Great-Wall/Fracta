# LIV Profile Specification

> **Status**: Draft (Phase 0). To be refined and finalized during Phase 2–3 implementation.
>
> This document is the **developer-facing spec** for implementing LIV as a Fracta Application Profile. It translates LIV methodology concepts into Fracta's technical vocabulary and defines what the software must do.
>
> - **Methodology reference** (user-facing): `docs/LIV_System_Manual.md`
> - **Architecture context**: `docs/adr/0011-liv-as-default-application-profile.md`, `SPEC.md §4.3`

---

## 1. Overview

LIV is Fracta's **default Application Profile**. A Profile is a JSON/YAML configuration that customizes how Framework Primitives, Pipelines, and AI behaviors are presented and enforced in the Application layer.

LIV implements a game-inspired execution methodology with three phases:

| LIV phase | Maps to Fracta area | Primary Primitives |
|-----------|---------------------|--------------------|
| Explore/Capture | Library (intake) | Item, Record (InboxItem) |
| Clear Challenges | Now (execution) | Quest, Loot, Schedule (DungeonRun), Metric (HUD) |
| Resupply/Upgrade | Past (reflection) | Event, Rule, Claim, Token |

The Framework must be generic enough that replacing LIV with GTD, OKR, Scrum, or a custom Profile requires only a different config file — no Framework code changes.

---

## 2. LIV domain objects → Fracta Primitives

This table maps every LIV domain object (from `LIV_System_Manual.md §2`) to Framework Semantic Primitives (from `SPEC.md §9`).

| LIV object | Fracta Primitive | Type/Subtype | Notes |
|------------|-----------------|--------------|-------|
| **InboxItem** | Record | `subtype: inbox_item` | Append-fast capture. Fields: `content`, `source`, `captured_at`, `routed_to` (Quest UID or null). |
| **Quest** | Quest | `quest_type: main \| side \| micro \| experience \| season` | Status: `tracked \| untracked \| archived`. Side Quests default to `visibility: fog`. |
| **QuestCard** | View | `view_type: quest_card` | A derived 3-line view of a tracked Quest: Clear Check, HUD Top 3, Next Stage. Not a separate Primitive — it's a View over a Quest. |
| **QuestLog** | View | `view_type: quest_log` | Collection view of all Quests. Fog-of-war filter hides untracked Side Quests by default. |
| **QuestSlots** | *(Profile config)* | — | Not a Primitive. It's a Profile-enforced constraint: `max_tracked_quests: 2`. The Framework checks this value from Profile config. |
| **DungeonRun** | Schedule | `schedule_type: dungeon_run` | Calendar-locked time block. Links to exactly one tracked Quest via `quest_uid`. Fields: `start`, `end`, `quest_uid`, `loot_uids[]`, `clear_check`. |
| **Loot** | Loot | `loot_type: document \| commit \| decision \| card_update \| training \| other` | Proof of progress. Links to producing DungeonRun and parent Quest. Must be an identifiable artifact. |
| **HUD** | Metric | *(multiple instances)* | Each HUD entry is a Metric with a fixed definition. LIV default: small set (3–5 metrics). Profile config defines which Metrics are in the HUD. |
| **Rulebook** | Rule | `rule_type: if_then \| no_go_gate` | If-Then: `condition → action` (default move). No-Go gate: `condition → refuse` (hard boundary). |
| **ArchiveEntry** | Event + Loot | — | Not a separate Primitive. Archival is a Pipeline action: move Quest artifacts into Past, generate Event entries, preserve backlinks. |
| **Budget** | *(Profile config)* | — | Reference split: `explore: 0.30, challenge: 0.60, resupply: 0.10`. Used by AI Orchestrator for scheduling suggestions and budget warnings. |

---

## 3. Profile configuration schema

The LIV Profile **package** is stored at `.fracta/config/profiles/liv.json`. The currently active Profile is selected via `.fracta/config/profile.json` (pointer + user overrides). Below is the LIV package schema definition.

```jsonc
{
  "profile_id": "liv",
  "profile_version": "1.0.0",
  "display_name": "LIV",
  "description": "Game-inspired execution methodology: Explore → Clear → Resupply",

  // --- Invariant enforcement ---
  "invariants": {
    "max_tracked_quests": 2,           // I1: Quest Slots cap
    "promotion_only_in_resupply": true, // I2: No promotion outside Resupply
    "loot_required_per_dungeon": true,  // I3: Loot requirement
    "max_active_micro_quests": 1,       // I4: Micro-Quest limit
    "max_active_experiences": 1,        // I5: Serendipity lane
    "night_rule_enabled": true,         // I7: No new decisions at night
    "night_rule_start": "22:00",
    "night_rule_end": "06:00"
  },

  // --- Budget reference values ---
  "budget": {
    "explore": 0.30,
    "challenge": 0.60,
    "resupply": 0.10
  },

  // --- HUD configuration ---
  "hud": {
    "max_metrics": 5,
    "default_metrics": [
      // User defines these during setup; these are examples
      // { "metric_id": "dungeon_runs_week", "label": "Dungeon Runs / week", "type": "count" }
    ]
  },

  // --- Quest type definitions ---
  "quest_types": [
    { "id": "main",       "label": "Main Quest",    "can_track": true,  "uses_budget": "challenge" },
    { "id": "side",       "label": "Side Quest",    "can_track": true,  "uses_budget": "challenge", "default_visibility": "fog" },
    { "id": "micro",      "label": "Micro-Quest",   "can_track": false, "uses_budget": "explore",   "max_duration_weeks": 4 },
    { "id": "experience", "label": "Experience",     "can_track": false, "uses_budget": "explore" },
    { "id": "season",     "label": "Season Quest",  "can_track": true,  "uses_budget": "challenge", "temporary": true }
  ],

  // --- Loot type definitions ---
  "loot_types": [
    { "id": "document",    "label": "Document / draft" },
    { "id": "commit",      "label": "Code commit / shipped change" },
    { "id": "decision",    "label": "Decision memo" },
    { "id": "card_update", "label": "Quest Card update" },
    { "id": "training",    "label": "Training session completed" },
    { "id": "other",       "label": "Other artifact" }
  ],

  // --- Resupply schedule ---
  "resupply_cadence": {
    "post_dungeon":  { "duration_minutes": 2,   "scope": "log_loot_and_next_stage" },
    "end_of_day":    { "duration_minutes": 10,  "scope": "inbox_shutdown" },
    "weekly":        { "duration_minutes": 90,  "scope": "full_resupply" },
    "monthly":       { "duration_minutes": 90,  "scope": "quotas_and_compression" },
    "quarterly":     { "duration_minutes": 240, "scope": "stage_review" },
    "yearly":        { "duration_minutes": 480, "scope": "close_save_file" }
  },

  // --- Guardrail thresholds ---
  "guardrails": {
    "no_loot_warning_after_runs": 1,
    "thrash_detection_switches_per_week": 3,
    "drift_detection_rulebook_stale_weeks": 4,
    "collapse_detection_missed_floor_days": 3,
    "promotion_cooldown_days": 7,
    "side_quest_archive_after_months": 12
  },

  // --- AI behavior overrides ---
  "ai_overrides": {
    "inbox_routing": "mode_a",          // Auto-route Inbox items to Side Quests
    "hud_calculation": "mode_a",        // Auto-calculate HUD metrics
    "loot_linking": "mode_a",           // Auto-link Loot to Quest
    "quest_discovery": "mode_b",        // Suggest, don't auto-promote
    "resupply_template": "mode_b",      // Prepare template, user fills
    "guardrail_enforcement": "mode_a"   // Auto-warn on invariant violations
  },

  // --- Token incentive rules ---
  "incentives": {
    "enabled": true,
    "rules": [
      { "trigger": "dungeon_run_with_loot",      "reward": 10 },
      { "trigger": "weekly_resupply_completed",   "reward": 25 },
      { "trigger": "quest_completed",             "reward": 100 },
      { "trigger": "rulebook_rule_added",         "reward": 15 },
      { "trigger": "streak_7_day_dungeon",        "reward": 50 }
    ]
  }
}
```

---

## 4. Invariant enforcement

The Framework provides generic constraint-checking infrastructure. The LIV Profile specifies *which* constraints to enforce and *how* to respond.

| Invariant | Framework mechanism | LIV behavior |
|-----------|-------------------|--------------|
| **I1 — Quest Slots cap** | `max_tracked_quests` in Profile config | Block track action if `tracked_count >= max`. UI: gray out "Track" button, show explanation. |
| **I2 — No promotion outside Resupply** | `promotion_only_in_resupply` flag | Block status change `untracked → tracked` unless current session is tagged `resupply`. AI: suggest during Resupply only. |
| **I3 — Loot requirement** | `loot_required_per_dungeon` flag | When DungeonRun ends: if `loot_uids` is empty, show warning + Loot type selector. Don't auto-close the run until Loot is logged. |
| **I4 — Micro-Quest limit** | `max_active_micro_quests` count | Block creation of new Micro-Quest if one already exists with `status: active`. |
| **I5 — Serendipity lane** | `max_active_experiences` count | Same as I4 but for experience-type Quests. |
| **I6 — Budget protection** | `budget` ratios + AI scheduling | AI Orchestrator uses budget ratios when suggesting DungeonRun scheduling. Warn if challenge time exceeds 70% or explore time drops below 20%. |
| **I7 — Night rule** | `night_rule_*` config | During night window: block new Quest creation, block Quest promotion, allow only Inbox append and schedule cleanup. |

**Enforcement levels** (configurable per invariant):
- `hard`: Block the action. User cannot override.
- `soft` (default): Warn and require confirmation. User can override with explicit acknowledgment.
- `off`: No enforcement (for users who want to customize).

---

## 5. Workflows as Pipeline configurations

LIV's three phases map to Framework Pipeline configurations:

### 5.1 Capture Pipeline

```
Trigger: new item enters Inbox
Stages:
  1. Capture    → append to Inbox (Record, subtype: inbox_item)
  2. Route      → AI creates Side Quest in QuestLog (Quest, visibility: fog)
  3. (no Process/Materialize — routing is immediate)
```

### 5.2 Challenge Pipeline

```
Trigger: user starts a DungeonRun
Stages:
  1. Validate   → check Clear Check exists, Quest is tracked
  2. Execute    → timer starts, focus mode enabled
  3. Complete   → require Loot, update QuestCard (Next Stage), link Loot → Quest
  4. Materialize → if token incentives enabled, credit reward
```

### 5.3 Resupply Pipeline

```
Trigger: user opens Resupply session (or scheduled cadence fires)
Stages:
  Plan pass:
    1. Calculate  → update HUD metrics from recent Loot/Events
    2. Review     → present QuestCard summaries
    3. Schedule   → AI suggests DungeonRuns for next loop, respecting budget
    4. Commit     → user confirms Quest Slot selections and schedule

  Improve pass:
    5. Analyze    → AI clusters recent Loot themes, detects drift signals
    6. Suggest    → AI prepares Rulebook update suggestions (Mode B)
    7. Archive    → move completed Quest artifacts to Past, generate Events
    8. Discover   → Quest Discovery (Observe → Value → Experiment)
```

### 5.4 Guardrail Pipeline (background)

```
Trigger: continuous monitoring
Stages:
  1. Monitor    → check invariant conditions after each state change
  2. Detect     → identify failure patterns (no loot, thrash, drift, collapse)
  3. Respond    → generate appropriate warning/intervention per guardrails config
  4. Log        → record guardrail trigger for observability
```

---

## 6. On-disk storage mapping

LIV-specific data lives in the standard `.fracta/` structure:

```
LocationRoot/
├── .fracta/
│   ├── config/
│   │   ├── profile.json               ← Active Profile pointer + user overrides
│   │   ├── profiles/
│   │   │   └── liv.json               ← LIV Profile package (§3)
│   │   └── schemas/
│   │       ├── quest.schema.json     ← Quest field definitions
│   │       ├── loot.schema.json      ← Loot field definitions
│   │       └── rule.schema.json      ← Rule field definitions
│   ├── cache/
│   │   ├── hud/
│   │   │   └── current.json          ← Latest HUD metric values (rebuildable)
│   │   ├── guardrails/
│   │   │   └── state.json            ← Guardrail detection state (rebuildable)
│   │   └── ledger/
│   │       └── transactions.jsonl    ← Token incentive ledger (rebuildable from Loot)
│   └── state/
│       └── ai_queue.json             ← Pending AI suggestions (Mode B drafts)
│
├── Now/                               ← Active projects (user SOT)
│   ├── inbox.md                       ← Inbox (append-only Markdown)
│   ├── quest-log.md                   ← Quest Log (all quests, fog-of-war sections)
│   ├── rulebook.md                    ← Rulebook (If-Then + No-Go gates)
│   └── [project-folders]/             ← Active quest working directories
│       └── [loot artifacts]
│
└── Past/                              ← Archive (user SOT)
    └── YYYY/MM/
        └── YYYY-MM-DD.md             ← Daily story (includes archived quest events)
```

**Key principle**: User-authored files (`inbox.md`, `quest-log.md`, `rulebook.md`, daily stories) are **Markdown SOT** in open format. LIV-specific computed state (HUD values, guardrail state, token ledger) is **cache** under `.fracta/cache/`, always rebuildable.

---

## 7. AI Orchestrator behavior under LIV

The AI Orchestrator reads the active Profile to determine its behavior:

| AI task | Mode | What it does |
|---------|------|-------------|
| Inbox routing | A (auto) | New InboxItem → create Side Quest in quest-log.md, tag with detected theme |
| HUD calculation | A (auto) | Aggregate Loot/Event data → update `.fracta/cache/hud/current.json` |
| Loot linking | A (auto) | When Loot is logged → auto-link to active DungeonRun and parent Quest |
| Guardrail enforcement | A (auto) | Check invariants after state changes → generate warnings |
| Theme clustering | A (auto) | Analyze recent Loot → detect recurring themes for Quest Discovery |
| Schedule suggestion | B (draft) | Prepare next-loop DungeonRun schedule respecting budget → user approves |
| Resupply template | B (draft) | Pre-fill Resupply checklist with data → user completes |
| Quest Discovery | B (draft) | Observe → Value → Experiment suggestions → user decides |
| Rulebook suggestion | B (draft) | Suggest new If-Then or No-Go based on patterns → user writes |
| Narrative drafting | B (draft) | Draft daily/weekly Past summary with quest progress → user enriches |

---

## 8. Observability metrics

LIV requires the following metrics for guardrail detection and HUD display. These are computed from Framework Primitives and cached.

| Metric | Source | Update frequency |
|--------|--------|-----------------|
| Dungeon Runs / week | Count of Schedule (dungeon_run) completed this week | Real-time |
| Loot / week | Count of Loot created this week | Real-time |
| Quest switching frequency | Count of Quest Slot changes per week | Real-time |
| Reschedule count | Count of DungeonRun time changes per week | Real-time |
| Budget split | Time in DungeonRuns vs total available time | Weekly (Resupply) |
| Loot theme distribution | AI-classified themes of recent Loot | Weekly (Resupply) |
| Rulebook update cadence | Days since last Rule modification | Real-time |
| Life Base thresholds | User-defined health/maintenance metrics | Daily |
| Streak counters | Consecutive days with ≥1 DungeonRun | Real-time |

---

## 9. Open questions (to resolve during implementation)

- **Quest state machine**: Exact state transitions (created → tracked → completed → archived) and which transitions require Resupply context.
- **Fog-of-war rendering**: How to visually represent "fog" in the Quest Log UI — collapsed sections? Blur? Separate tab?
- **Multi-vault Quests**: Can a Quest span multiple Locations? Or must it live in one?
- **Collaborative LIV**: If two people share a Location, do they share Quest Slots? Separate Profiles?
- **Profile migration**: If a user switches from LIV to GTD Profile, what happens to Quest/Loot data?
- **Offline token ledger**: Conflict resolution strategy if the same vault is edited on two devices.
