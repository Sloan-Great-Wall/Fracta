# LIV System Manual

> **Role in Fracta**: This document is the **methodology reference** for LIV — the default Application Profile in Fracta. It describes *how to use LIV as a human*, not how to implement it in code.
>
> - **Fracta technical mapping**: see `docs/adr/0011-liv-as-default-application-profile.md` and `SPEC.md §4.3`.
> - **Implementation spec**: see `docs/profiles/liv-profile-spec.md` (created during Phase 2–3).
> - **Language**: English only per ADR-0009.

---

## LIV Player Guide

Most people don't fail because they are lazy. They fail because they are running modern life with the wrong model.

The brain is great at judgement, taste, and meaning. It is not a hard drive, and it is not a reliable scheduler. When you keep everything "open" in your head, unfinished tasks become background processes: attention fragments, anxiety rises, and output drops. Kahneman's work helps name part of the mechanism (planning fallacy, cognitive limits). Taleb helps name the failure mode (fragility: one surprise collapses the plan).

LIV is a counter-move. Instead of demanding more willpower, it changes the operating environment: externalize memory, constrain attention, calendar-lock deep work, measure a few numbers, and upgrade the system regularly. Over time you become a cyborg: the machine does the mechanical loops; the human does judgement and correction.

## The lean loop (3 steps)

The whole system is one loop with three phases:

Explore/Capture → Clear Challenges → Resupply/Upgrade

Explore/Capture is free roam + intake. Clearing Challenges is focused play. Resupply/Upgrade is recovery + reward + correction.

This loop is also a game loop. Nintendo's best design keeps the player in a rhythm: discover points of interest, face a challenge, earn a reward, resupply, and go again. LIV borrows the same structure for real life: explore and capture signals, clear the right challenges, then resupply and upgrade.

### Player view (human-only): how it runs

From your side, LIV is intentionally boring. You roam, you fight, you resupply. Explore is not "planning time"; it's freedom and slack. You can do anything you want there, as long as you capture inputs into one Inbox and keep moving. Challenges are where you commit: you look at the calendar, you pick one tracked quest, and you run one Dungeon Run without switching. Resupply is where you change the map: you open the HUD and the tracked quest card, you decide what to track next, you schedule the next dungeons, you write one rule, and you archive the loot.

Here's the interaction pattern in one story. You notice an idea and drop it into the Inbox. You don't decide. The system routes it into the Quest Log as a Side Quest under fog of war. Later, you follow the calendar and clear a challenge, producing loot (a doc, a commit, a decision, a card update). During Resupply, you notice that your recent loot and captured ideas keep pointing to the same theme. You run the Regret Test and check your No-Go gate. If it passes, you turn that Side Quest into a Micro-Quest (2–4 weeks) that spends Explore budget, so it doesn't steal a quest slot. If the Micro-Quest keeps producing repeatable loot, then in the next Resupply you promote it into Quest Slots=2 by replacing one tracked quest.

This story scales. Day / week / month / quarter / year / multi-year are all the same interaction, just with different aggregation windows. What changes is the *size* of the Resupply decision. What must stay invariant is: you don't promote outside Resupply, Quest Slots stay capped, and every Dungeon Run leaves loot.

If you're curious about how the system works under the hood (data objects, guardrails, recovery, automation, storage), see **LIV System Spec** below. You don't need those details to use the system day-to-day.

## Explore/Capture: build one Inbox and only append

Pick one place as your Inbox. A note file, a doc, a tool—anything that opens fast. The rule is simple: if it matters, it goes in the Inbox. Not "remember it"; not "decide now"; just capture.

If the item is heavy media stored elsewhere (iCloud / home NAS / OneDrive), do not duplicate. Capture a pointer instead: a small note that says what it is, where it lives, and which project it belongs to. Google Drive stays your searchable index without forcing all bytes into one place.

The "explore" part is psychological: you are allowed to notice interesting signals without committing to them immediately. The Inbox is how curiosity becomes safe. You can capture an idea, a risk, a question, a person to talk to, or an opportunity, without forcing a decision on the spot.

Explore also plays another role: it is your protected slack. As a reference, give it ~30% of your loop budget. Most of the time this is where you rest, wander, try new experiences, and leave room for black swans. On bad weeks, this is also your emergency buffer: you use it to repair and reset without stealing from challenge time.

The key is separation of concerns. Explore/Capture is intake + freedom: append fast, keep it messy, do not sort, do not schedule. Decisions belong to Resupply. If you mix the two, you either stop exploring (because every signal demands effort), or you stop deciding (because the Inbox becomes a swamp).

Default routing is what keeps the system light: everything you capture becomes a Side Quest first. You do not "delete dreams"; you store them. Side Quests live in the Quest Log but stay under fog of war (not visible day-to-day). Promotion is allowed only during Resupply: to track a new quest, you must replace one of the tracked quests (Quest Slots=2). A simple cooldown helps: wait 7–14 days before promoting a fresh idea, so hype has time to die and signal can stay.

New Experiences are Side Quests too, but with their own lane. Treat them as a separate "Serendipity queue": at most 1 active experience at a time, and it must spend Explore budget (it may not steal from Challenge time). Capture an experience as a short quest story (time / place / people / event / budget). During Resupply, extract "recovery factors" and write them into the Rulebook.

## Clear Challenges: Quest Slots=2 (quest tracker), calendar, Dungeon Runs

Clearing challenges begins with a brutal constraint: attention is the bottleneck. LIV enforces Quest Slots=2: at most two quests can be tracked at once. A single Dungeon Run tracks only one quest—no branching inside the run. Everything else waits in your quest log.

In practice, "2" is a ceiling, not a goal. Default to tracking 1 quest. Track 2 only when you accept the cost (more switching pressure). If you notice switching spikes, treat it as a red flag and drop back to tracking 1 in the next Resupply.

Then comes the physical constraint: time is real only when it is on the calendar. So challenges happen as calendar-locked, distraction-free single-task blocks. LIV calls them Dungeon Runs. A Dungeon Run should drop something you can point to: a shipped change, a draft paragraph, a decision note, a measurable training session completed. If you can't point to a "drop", the run was probably consumed by switching.

This is the first built-in failure mode and its fix. When the horizon is short (a day), your brain can feel "busy" while nothing advances. LIV's rule is deliberately brutal: **no loot, no clear**. If you keep finishing days without loot, your Clear Check is too big or too vague. Shrink it until one Dungeon Run can realistically drop loot again.

Across any loop length you choose (days are flexible), keep one budget guideline (reference values): ~30% Explore, ~60% Clear Challenges, ~10% Resupply. This is not a moral rule; it is an anti-fragility rule. Without protected slack, reality will break the system.

One practical rule keeps a challenge honest: define a Clear Check before you start a battle. Clear Check is your one-sentence win condition. If you can't write it, you don't yet have a task—you have fog.

If you want the smallest possible Dungeon Run protocol, it is this: decide one "loot drop" before you start, protect the block like an appointment, and write one line after you finish (what changed, what is the next concrete step). The system is simple; the discipline is avoiding the invisible enemy: switching.

To keep tracked quests lightweight, use Quest Cards. For each tracked quest, keep only three lines visible: Clear Check / HUD Top 3 / Next Stage. Everything else can live in an "expand" section. Resupply updates the card; the card drives the next Dungeon Run.

## Resupply/Upgrade: one session, two passes (Plan + Improve)

Resupply is the small slice (often ~10%) that makes the next ~60% possible. It prevents drift.

But Resupply is not one action. It is two passes that must cooperate.

The Plan pass makes the next loop runnable: update a minimal HUD, choose what to track (Quest Slots=2), schedule the next loop's Dungeon Runs (~60%), and protect Explore (~30%) + Resupply (~10%) so they won't be eaten.

The Improve pass makes the next loop cheaper: run Musk's 5-step method (question → delete → simplify → accelerate → automate only after stable), write or upgrade one If-Then rule (behavior code), pick one automation target (biggest friction), and do the archival move: finished or inactive outputs go into `lifelong_story` with a timestamp. Don't edit history; append a new entry when you learn later.

Resupply is also where top-down and bottom-up finally touch. You take the week's loot (what you actually did), and you update the world map (what you will do next) so the next loop feels lighter.

If you want a discovery method that is grounded in reality, use Quest Discovery inside Resupply:

- Observe: scan Inbox + loot. What themes keep reappearing? Where does energy rise? What drains you?
- Value: apply your Rulebook. Run the Regret Test ("if I don't do this, will future me regret it?") and check No-Go gates (anti-goals).
- Experiment: run a Micro-Quest (2–4 weeks) using Explore budget. Only 1 experiment can be active. Promote only if the experiment produces repeatable loot and a clearer map.

This is why Side Quests are not a distraction: they are a candidate pool. But the system stays focused because promotion is rate-limited (only in Resupply, with cooldown), and tracked quests are hard-capped (Quest Slots=2).

This is the second built-in failure mode and its fix. When the horizon is medium (a week), the risk is thrash: tracking 2 quests turns into constant switching. LIV's fix is baked into the rules: keep Quest Slots capped, default to tracking 1, and allow promotions only during Resupply (with cooldown). If you're switching, you don't need more discipline — you need fewer tracked quests.

This is the third built-in failure mode and its fix. When the horizon is long (a quarter/year), the risk is drift: the world changes but your map stays the same, or you keep rewriting the map without evidence. LIV's fix is Micro-Quests: use Explore budget to test direction changes safely. Promote only after repeatable loot appears.

Finally, make "hidden vs archived" explicit. Hidden means "not now but still alive" (Quest Log under fog of war). Archived means "past save file": move it into `lifelong_story` and keep only a pointer plus one-line summary in the Quest Log. A practical garbage-collection rule: if a Side Quest has not been promoted or reviewed for ~12 months, compress it and archive the details. The world stays explorable; the HUD stays light.

Life Base is monitored the same way. It usually should not compete for quest slots; it should be maintained by thresholds and automation. When it breaks (sleep collapses, home becomes chaos, cash buffer is unsafe, relationships drift), treat it as a temporary season quest: promote it for one loop, repair, then demote back to the base system.

Calibration is not a separate ceremony. It is just Resupply at a different intensity. On a normal week you do both passes. When you are drifting, you do both passes more ruthlessly. When you are drowning, you run Resupply in "rescue mode": temporarily reduce to Quest Slots=1, schedule the next single Dungeon Run, and do Plan first; Improve comes later after motion returns.

Treat "Resupply" as one action with multiple sizes, so it stays consistent across time scales:

- 2 minutes (after a Dungeon Run): log loot + write "Next Stage" (one sentence).
- 10 minutes (end-of-day): Inbox shutdown (move items into "tomorrow/this loop"; no new decisions at night).
- 60–120 minutes (weekly): full Resupply (HUD + schedule + Rulebook + promotions + archive).
- 60–120 minutes (monthly): quotas + Side Quest compression + Life Base thresholds.
- half-day (quarterly): stage review + major swaps + deeper archival.
- 1 day (yearly): close the save file (compile the Rulebook, archive big artifacts, reset the map).

This is also where the filesystem contract becomes real. Resupply is the moment you clean the working set: you move finished artifacts out of working directories and into the time-ordered archive (`lifelong_story`). You keep the present light, and you keep history searchable. In practice, it becomes both a diary and a source of reusable knowledge.

## Tutorial mode (new player): one week to install and calibrate

A system is not proven by how pretty it is. It is proven by whether it runs on a bad day. So week one is a tutorial, not a perfection contest.

Before you build a loop, you need a map: your Main Quests.

One practical method is brutally simple. Write down 100 (or just "a lot") life-scale things you want to do. Then delete, delete, delete until you have fewer than five—ideally three. Name them. This is your top-down direction: it tells you what matters and what you will refuse.

Then the loop gives you the bottom-up engine: you explore and capture daily signals, turn them into challenges, clear them in Dungeon Runs, and upgrade the rules. The top-down map and the bottom-up engine are both necessary: one without the other either drifts or stalls.

In the first hour, set up only the minimum: create one Inbox; pick two quests you can actually move in seven days; write one Clear Check sentence for each quest ("what counts as shipped/cleared"); pick a tiny HUD you can measure; and put time on the calendar using the 30/60/10 budget (Explore / Challenges / Resupply). Then run one Dungeon Run within 24 hours. Do not wait to "feel ready".

Week one should also install the "focus locks": create a place for Side Quests (Quest Log under fog of war), write one No-Go gate (one thing you will refuse this week), and decide your default tracking mode (track 1 quest unless a strong reason forces 2). You're not building a perfect plan. You're building a loop that keeps working when you're tired.

For the rest of week one, don't optimize. Just run the loop and log honestly. At the end of the week, do a Resupply and write your first If-Then rule. That is the moment life becomes a system instead of wishful thinking.

## LIV System Spec

> Audience: a small team building a LIV software assistant.
> This spec defines the system model, the core workflows, and the built-in guardrails. The goal is not "more features" — it is a lighter player experience.

### 1) Design goals (what we optimize for)

- Minimize cognitive load during execution (the player should be able to run the system when tired).
- Preserve curiosity and black swans by protecting Explore budget and by keeping ideas as Side Quests under fog of war.
- Keep focus stable via a hard cap (Quest Slots=2) and by making promotion a Resupply-only action.
- Make direction changes safe via Micro-Quests (evidence before promotion).
- Keep storage simple and inspectable (filesystem contract; append-only `lifelong_story`).

Non-goals:
- Perfect prediction. LIV is a feedback system, not an oracle.
- A giant task database. The Quest Log is allowed to be messy, because only tracked quests matter.

### 2) Core model (domain objects)

The system maintains a small set of stateful objects:

- **InboxItem**: a captured input (idea/risk/opportunity/person/media pointer). Must be append-fast.
- **Quest**: a task-line with a type (`main`, `side`, `micro`, `experience`, `season/base-repair`) and a status (`tracked` or not).
- **QuestCard** (UI compression): the 3-line interface for a tracked quest:
  - Clear Check (win condition)
  - HUD Top 3 (weekly numbers)
  - Next Stage (one sentence)
- **QuestLog**: collection of all quests (mostly Side Quests under fog of war).
- **QuestSlots**: the set of tracked quests. Hard cap: 2 (default track 1).
- **DungeonRun**: a calendar-locked focus block linked to exactly one tracked quest.
- **Loot**: the proof of progress produced by a Dungeon Run (doc/commit/decision/card update). Loot is linkable and archivable.
- **HUD**: a small set of metrics (feedback, not goals) with fixed definitions.
- **Rulebook**: global behavior code:
  - If-Then rules (default moves)
  - No-Go gates (hard refusal rules)
- **ArchiveEntry**: a save file in `lifelong_story` (append-only, time-ordered) with backlinks to the quest/loot.

### 3) Invariants (hard rules)

These are not "tips". They are invariants the software must enforce or strongly warn on:

- **I1 — Quest Slots cap**: `tracked_quests ≤ 2` (default 1).
- **I2 — No promotion outside Resupply**: a Side Quest cannot become tracked outside a Resupply session.
- **I3 — Loot requirement**: a Dungeon Run is invalid unless it produces at least one Loot item.
- **I4 — Micro-Quest limit**: `active_micro_quests ≤ 1` (runs on Explore budget).
- **I5 — Serendipity lane**: `active_experience ≤ 1` and it must spend Explore budget.
- **I6 — Budget protection**: preserve 30/60/10 (Explore/Challenges/Resupply) as default, not as a moral rule.
- **I7 — Night rule**: no new decisions at night; only Inbox shutdown and scheduling cleanup.

### 4) Primary workflows (state transitions)

- **Capture flow**
  - Input → append to Inbox
  - Route → create/attach a Side Quest in Quest Log (under fog of war by default)
  - Do not schedule here

- **Challenge flow**
  - Select tracked quest → start Dungeon Run (calendar-locked)
  - Require Clear Check before start
  - End Dungeon Run → require Loot → update QuestCard ("Next Stage")

- **Resupply flow (Plan + Improve)**
  - Plan: update HUD → pick Quest Slots (≤2) → schedule challenge blocks (~60%) → protect Explore (~30%) + Resupply (~10%)
  - Improve: write/upgrade Rulebook (1 If-Then + 1 No-Go gate) → pick one automation target → archive Loot → run Quest Discovery

- **Quest Discovery (inside Resupply)**
  - Observe: aggregate Inbox + Loot by theme and by energy signal
  - Value: apply Rulebook + Regret Test + No-Go gates
  - Experiment: start one Micro-Quest (2–4 weeks) using Explore budget; promote only after repeatable Loot

- **Archival & compression**
  - "Hidden" (fog) = still alive in Quest Log
  - "Archived" = move details to `lifelong_story`, keep only a pointer + one-line summary in Quest Log
  - Garbage collection rule: if a Side Quest is untouched for ~12 months, compress + archive details

### 5) Guardrails & recovery (built-in, not optional)

These are the system's stability mechanisms. They should be treated as first-class features.

- **Short horizon (day) failure: busy but no Loot**
  - Detection: 1+ Dungeon Runs without Loot, or repeated "wrap" notes with no artifact
  - Response: force smaller Clear Check suggestions; downgrade scope; prompt Loot type selection

- **Medium horizon (week) failure: thrash**
  - Detection: high quest switching frequency, repeated rescheduling, low completion
  - Response: recommend tracking 1 quest; add friction to switching; require replacement to track a new quest

- **Long horizon (quarter/year) failure: drift**
  - Detection: Loot themes diverge from tracked quests; Rulebook not updated; HUD trends degrade
  - Response: run Micro-Quest probes; schedule a longer Resupply; allow a major swap only with evidence

- **Overload failure: collapse**
  - Detection: missing "floor 1 dungeon" streaks + Life Base thresholds broken (sleep/training/maintenance)
  - Response: Rescue Mode (Quest Slots=1, schedule one next Dungeon Run, Plan only first), or Respawn (declare bankruptcy on backlog, cold start today)

### 6) Human + AI contract (responsibilities)

The contract is a separation of concerns:

- **Human responsibilities**
  - Define Clear Checks
  - Choose what to track (Quest Slots)
  - Interpret feedback honestly (HUD + Loot)
  - Decide Rulebook changes (what behavior to encode)
  - Handle ambiguity, risk, and black swans

- **Machine responsibilities**
  - Capture plumbing: append-fast, dedupe, pointer generation
  - Routing: Inbox → Side Quest creation (fog by default)
  - Aggregation: HUD calculation, Loot linking, theme clustering
  - Scheduling: suggestions + conflict detection + budget protection
  - Guardrails: friction/warnings when invariants are violated
  - Templates: Resupply templates, checklists, Quest Card scaffolds
  - Archival: write save files into `lifelong_story` with backlinks

Give judgement to the machine and you become fragile. Keep mechanical loops in your head and you burn out.

### 7) Storage contract (filesystem)

Treat the filesystem as the ground truth:

- `Lifelong_Vision/` (control plane): `Inbox.*`, `HUD.*`, `QuestLog.*`, `Rulebook.*`, templates
- Project folder (working directory): active quests + loot artifacts
- `lifelong_story/` (archive): time-ordered, append-only save files with backlinks

Large media stays where it is (iCloud/NAS/OneDrive). Drive holds pointers (name/id, location link/path, date, related quest).

### 8) Observability (for builders)

Minimum telemetry (even for a single user):

- Counts: Dungeon Runs/week, Loot/week, switching frequency, reschedules
- Budget: Explore/Challenge/Resupply time split (approximate is fine)
- Drift: repeated Loot themes, Rulebook update cadence
- Base System: sleep/training/maintenance thresholds

## Appendix

### Appendix A: scientific anchors (short)

Attention residue; Zeigarnik effect; deep work; implementation intentions; decision fatigue; Gall's law; negative feedback loops; antifragility; planning fallacy.

### Appendix B: glossary

Inbox: single capture entry for everything.
Quest Slots=2: quest tracker limit; at most two quests tracked at once.
Quest Log: the full list of quests (Main Quests + Side Quests).
Side Quest: a stored "maybe later" quest; not tracked by default.
Rulebook: your global principles (If-Then rules + No-Go gates).
No-Go gate: a hard refusal rule (an anti-goal turned into behavior code).
Regret Test: "If I don't do this, will future me regret it?" (a prioritization filter).
Micro-Quest: a 2–4 week experiment to test a candidate quest before promotion.
Dungeon Run: calendar-locked, distraction-free single-task block.
HUD: a small scoreboard (feedback, not goals).
Resupply/Upgrade: the camp session (Plan + Improve).
If-Then: "If situation, then action" rule (behavior code).
`lifelong_story`: time-ordered archive; append-only save files.

### Appendix C: filesystem reference (practical)

Minimal roles (Google Drive-first):

- `Lifelong_Vision/` (control plane): `Inbox.*`, `HUD.*`, `QuestLog.*`, `Rulebook.*`, templates
- Project folder (working directory): active quests + loot (outputs)
- `lifelong_story/` (archive): `YYYY/YYYY-MM/YYYY-MM-DD - ...` (append-only)

Pointer idea (for off-Drive media): keep the heavy asset in iCloud/NAS/OneDrive, and store a small pointer note in Drive (name/id, real location path/link, date, related project).
