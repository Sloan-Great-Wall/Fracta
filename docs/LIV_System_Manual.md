## ENGLISH VERSION

> This English section is canonical. The Chinese section below is a translation; if anything differs, follow the English version.

## LIV Player Guide (EN)

Most people don’t fail because they are lazy. They fail because they are running modern life with the wrong model.

The brain is great at judgement, taste, and meaning. It is not a hard drive, and it is not a reliable scheduler. When you keep everything “open” in your head, unfinished tasks become background processes: attention fragments, anxiety rises, and output drops. Kahneman’s work helps name part of the mechanism (planning fallacy, cognitive limits). Taleb helps name the failure mode (fragility: one surprise collapses the plan).

LIV is a counter-move. Instead of demanding more willpower, it changes the operating environment: externalize memory, constrain attention, calendar-lock deep work, measure a few numbers, and upgrade the system regularly. Over time you become a cyborg: the machine does the mechanical loops; the human does judgement and correction.

## The lean loop (3 steps)

The whole system is one loop with three phases:

Explore/Capture → Clear Challenges → Resupply/Upgrade

Explore/Capture is free roam + intake. Clearing Challenges is focused play. Resupply/Upgrade is recovery + reward + correction.

This loop is also a game loop. Nintendo’s best design keeps the player in a rhythm: discover points of interest, face a challenge, earn a reward, resupply, and go again. LIV borrows the same structure for real life: explore and capture signals, clear the right challenges, then resupply and upgrade.

### Player view (human-only): how it runs

From your side, LIV is intentionally boring. You roam, you fight, you resupply. Explore is not “planning time”; it’s freedom and slack. You can do anything you want there, as long as you capture inputs into one Inbox and keep moving. Challenges are where you commit: you look at the calendar, you pick one tracked quest, and you run one Dungeon Run without switching. Resupply is where you change the map: you open the HUD and the tracked quest card, you decide what to track next, you schedule the next dungeons, you write one rule, and you archive the loot.

Here’s the interaction pattern in one story. You notice an idea and drop it into the Inbox. You don’t decide. The system routes it into the Quest Log as a Side Quest under fog of war. Later, you follow the calendar and clear a challenge, producing loot (a doc, a commit, a decision, a card update). During Resupply, you notice that your recent loot and captured ideas keep pointing to the same theme. You run the Regret Test and check your No-Go gate. If it passes, you turn that Side Quest into a Micro-Quest (2–4 weeks) that spends Explore budget, so it doesn’t steal a quest slot. If the Micro-Quest keeps producing repeatable loot, then in the next Resupply you promote it into Quest Slots=2 by replacing one tracked quest.

This story scales. Day / week / month / quarter / year / multi-year are all the same interaction, just with different aggregation windows. What changes is the *size* of the Resupply decision. What must stay invariant is: you don’t promote outside Resupply, Quest Slots stay capped, and every Dungeon Run leaves loot.

If you’re curious about how the system works under the hood (data objects, guardrails, recovery, automation, storage), see **LIV System Spec (EN)** below. You don’t need those details to use the system day-to-day.

## Explore/Capture: build one Inbox and only append

Pick one place as your Inbox. A note file, a doc, a tool—anything that opens fast. The rule is simple: if it matters, it goes in the Inbox. Not “remember it”; not “decide now”; just capture.

If the item is heavy media stored elsewhere (iCloud / home NAS / OneDrive), do not duplicate. Capture a pointer instead: a small note that says what it is, where it lives, and which project it belongs to. Google Drive stays your searchable index without forcing all bytes into one place.

The “explore” part is psychological: you are allowed to notice interesting signals without committing to them immediately. The Inbox is how curiosity becomes safe. You can capture an idea, a risk, a question, a person to talk to, or an opportunity, without forcing a decision on the spot.

Explore also plays another role: it is your protected slack. As a reference, give it ~30% of your loop budget. Most of the time this is where you rest, wander, try new experiences, and leave room for black swans. On bad weeks, this is also your emergency buffer: you use it to repair and reset without stealing from challenge time.

The key is separation of concerns. Explore/Capture is intake + freedom: append fast, keep it messy, do not sort, do not schedule. Decisions belong to Resupply. If you mix the two, you either stop exploring (because every signal demands effort), or you stop deciding (because the Inbox becomes a swamp).

Default routing is what keeps the system light: everything you capture becomes a Side Quest first. You do not “delete dreams”; you store them. Side Quests live in the Quest Log but stay under fog of war (not visible day-to-day). Promotion is allowed only during Resupply: to track a new quest, you must replace one of the tracked quests (Quest Slots=2). A simple cooldown helps: wait 7–14 days before promoting a fresh idea, so hype has time to die and signal can stay.

New Experiences are Side Quests too, but with their own lane. Treat them as a separate “Serendipity queue”: at most 1 active experience at a time, and it must spend Explore budget (it may not steal from Challenge time). Capture an experience as a short quest story (time / place / people / event / budget). During Resupply, extract “recovery factors” and write them into the Rulebook.

## Clear Challenges: Quest Slots=2 (quest tracker), calendar, Dungeon Runs

Clearing challenges begins with a brutal constraint: attention is the bottleneck. LIV enforces Quest Slots=2: at most two quests can be tracked at once. A single Dungeon Run tracks only one quest—no branching inside the run. Everything else waits in your quest log.

In practice, “2” is a ceiling, not a goal. Default to tracking 1 quest. Track 2 only when you accept the cost (more switching pressure). If you notice switching spikes, treat it as a red flag and drop back to tracking 1 in the next Resupply.

Then comes the physical constraint: time is real only when it is on the calendar. So challenges happen as calendar-locked, distraction-free single-task blocks. LIV calls them Dungeon Runs. A Dungeon Run should drop something you can point to: a shipped change, a draft paragraph, a decision note, a measurable training session completed. If you can’t point to a “drop”, the run was probably consumed by switching.

This is the first built-in failure mode and its fix. When the horizon is short (a day), your brain can feel “busy” while nothing advances. LIV’s rule is deliberately brutal: **no loot, no clear**. If you keep finishing days without loot, your Clear Check is too big or too vague. Shrink it until one Dungeon Run can realistically drop loot again.

Across any loop length you choose (days are flexible), keep one budget guideline (reference values): ~30% Explore, ~60% Clear Challenges, ~10% Resupply. This is not a moral rule; it is an anti-fragility rule. Without protected slack, reality will break the system.

One practical rule keeps a challenge honest: define a Clear Check before you start a battle. Clear Check is your one-sentence win condition. If you can’t write it, you don’t yet have a task—you have fog.

If you want the smallest possible Dungeon Run protocol, it is this: decide one “loot drop” before you start, protect the block like an appointment, and write one line after you finish (what changed, what is the next concrete step). The system is simple; the discipline is avoiding the invisible enemy: switching.

To keep tracked quests lightweight, use Quest Cards. For each tracked quest, keep only three lines visible: Clear Check / HUD Top 3 / Next Stage. Everything else can live in an “expand” section. Resupply updates the card; the card drives the next Dungeon Run.

## Resupply/Upgrade: one session, two passes (Plan + Improve)

Resupply is the small slice (often ~10%) that makes the next ~60% possible. It prevents drift.

But Resupply is not one action. It is two passes that must cooperate.

The Plan pass makes the next loop runnable: update a minimal HUD, choose what to track (Quest Slots=2), schedule the next loop’s Dungeon Runs (~60%), and protect Explore (~30%) + Resupply (~10%) so they won’t be eaten.

The Improve pass makes the next loop cheaper: run Musk’s 5-step method (question → delete → simplify → accelerate → automate only after stable), write or upgrade one If-Then rule (behavior code), pick one automation target (biggest friction), and do the archival move: finished or inactive outputs go into `lifelong_story` with a timestamp. Don’t edit history; append a new entry when you learn later.

Resupply is also where top-down and bottom-up finally touch. You take the week’s loot (what you actually did), and you update the world map (what you will do next) so the next loop feels lighter.

If you want a discovery method that is grounded in reality, use Quest Discovery inside Resupply:

- Observe: scan Inbox + loot. What themes keep reappearing? Where does energy rise? What drains you?
- Value: apply your Rulebook. Run the Regret Test (“if I don’t do this, will future me regret it?”) and check No-Go gates (anti-goals).
- Experiment: run a Micro-Quest (2–4 weeks) using Explore budget. Only 1 experiment can be active. Promote only if the experiment produces repeatable loot and a clearer map.

This is why Side Quests are not a distraction: they are a candidate pool. But the system stays focused because promotion is rate-limited (only in Resupply, with cooldown), and tracked quests are hard-capped (Quest Slots=2).

This is the second built-in failure mode and its fix. When the horizon is medium (a week), the risk is thrash: tracking 2 quests turns into constant switching. LIV’s fix is baked into the rules: keep Quest Slots capped, default to tracking 1, and allow promotions only during Resupply (with cooldown). If you’re switching, you don’t need more discipline — you need fewer tracked quests.

This is the third built-in failure mode and its fix. When the horizon is long (a quarter/year), the risk is drift: the world changes but your map stays the same, or you keep rewriting the map without evidence. LIV’s fix is Micro-Quests: use Explore budget to test direction changes safely. Promote only after repeatable loot appears.

Finally, make “hidden vs archived” explicit. Hidden means “not now but still alive” (Quest Log under fog of war). Archived means “past save file”: move it into `lifelong_story` and keep only a pointer plus one-line summary in the Quest Log. A practical garbage-collection rule: if a Side Quest has not been promoted or reviewed for ~12 months, compress it and archive the details. The world stays explorable; the HUD stays light.

Life Base is monitored the same way. It usually should not compete for quest slots; it should be maintained by thresholds and automation. When it breaks (sleep collapses, home becomes chaos, cash buffer is unsafe, relationships drift), treat it as a temporary season quest: promote it for one loop, repair, then demote back to the base system.

Calibration is not a separate ceremony. It is just Resupply at a different intensity. On a normal week you do both passes. When you are drifting, you do both passes more ruthlessly. When you are drowning, you run Resupply in “rescue mode”: temporarily reduce to Quest Slots=1, schedule the next single Dungeon Run, and do Plan first; Improve comes later after motion returns.

Treat “Resupply” as one action with multiple sizes, so it stays consistent across time scales:

- 2 minutes (after a Dungeon Run): log loot + write “Next Stage” (one sentence).
- 10 minutes (end-of-day): Inbox shutdown (move items into “tomorrow/this loop”; no new decisions at night).
- 60–120 minutes (weekly): full Resupply (HUD + schedule + Rulebook + promotions + archive).
- 60–120 minutes (monthly): quotas + Side Quest compression + Life Base thresholds.
- half-day (quarterly): stage review + major swaps + deeper archival.
- 1 day (yearly): close the save file (compile the Rulebook, archive big artifacts, reset the map).

This is also where the filesystem contract becomes real. Resupply is the moment you clean the working set: you move finished artifacts out of working directories and into the time-ordered archive (`lifelong_story`). You keep the present light, and you keep history searchable. In practice, it becomes both a diary and a source of reusable knowledge.

## Tutorial mode (new player): one week to install and calibrate

A system is not proven by how pretty it is. It is proven by whether it runs on a bad day. So week one is a tutorial, not a perfection contest.

Before you build a loop, you need a map: your Main Quests.

One practical method is brutally simple. Write down 100 (or just “a lot”) life-scale things you want to do. Then delete, delete, delete until you have fewer than five—ideally three. Name them. This is your top-down direction: it tells you what matters and what you will refuse.

Then the loop gives you the bottom-up engine: you explore and capture daily signals, turn them into challenges, clear them in Dungeon Runs, and upgrade the rules. The top-down map and the bottom-up engine are both necessary: one without the other either drifts or stalls.

In the first hour, set up only the minimum: create one Inbox; pick two quests you can actually move in seven days; write one Clear Check sentence for each quest (“what counts as shipped/cleared”); pick a tiny HUD you can measure; and put time on the calendar using the 30/60/10 budget (Explore / Challenges / Resupply). Then run one Dungeon Run within 24 hours. Do not wait to “feel ready”.

Week one should also install the “focus locks”: create a place for Side Quests (Quest Log under fog of war), write one No-Go gate (one thing you will refuse this week), and decide your default tracking mode (track 1 quest unless a strong reason forces 2). You’re not building a perfect plan. You’re building a loop that keeps working when you’re tired.

For the rest of week one, don’t optimize. Just run the loop and log honestly. At the end of the week, do a Resupply and write your first If-Then rule. That is the moment life becomes a system instead of wishful thinking.

## LIV System Spec (EN)

> Audience: a small team building a LIV software assistant.  
> This spec defines the system model, the core workflows, and the built-in guardrails. The goal is not “more features” — it is a lighter player experience.

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

These are not “tips”. They are invariants the software must enforce or strongly warn on:

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
  - End Dungeon Run → require Loot → update QuestCard (“Next Stage”)

- **Resupply flow (Plan + Improve)**
  - Plan: update HUD → pick Quest Slots (≤2) → schedule challenge blocks (~60%) → protect Explore (~30%) + Resupply (~10%)
  - Improve: write/upgrade Rulebook (1 If-Then + 1 No-Go gate) → pick one automation target → archive Loot → run Quest Discovery

- **Quest Discovery (inside Resupply)**
  - Observe: aggregate Inbox + Loot by theme and by energy signal
  - Value: apply Rulebook + Regret Test + No-Go gates
  - Experiment: start one Micro-Quest (2–4 weeks) using Explore budget; promote only after repeatable Loot

- **Archival & compression**
  - “Hidden” (fog) = still alive in Quest Log
  - “Archived” = move details to `lifelong_story`, keep only a pointer + one-line summary in Quest Log
  - Garbage collection rule: if a Side Quest is untouched for ~12 months, compress + archive details

### 5) Guardrails & recovery (built-in, not optional)

These are the system’s stability mechanisms. They should be treated as first-class features.

- **Short horizon (day) failure: busy but no Loot**
  - Detection: 1+ Dungeon Runs without Loot, or repeated “wrap” notes with no artifact
  - Response: force smaller Clear Check suggestions; downgrade scope; prompt Loot type selection

- **Medium horizon (week) failure: thrash**
  - Detection: high quest switching frequency, repeated rescheduling, low completion
  - Response: recommend tracking 1 quest; add friction to switching; require replacement to track a new quest

- **Long horizon (quarter/year) failure: drift**
  - Detection: Loot themes diverge from tracked quests; Rulebook not updated; HUD trends degrade
  - Response: run Micro-Quest probes; schedule a longer Resupply; allow a major swap only with evidence

- **Overload failure: collapse**
  - Detection: missing “floor 1 dungeon” streaks + Life Base thresholds broken (sleep/training/maintenance)
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

## Appendix (optional, EN)

### Appendix A: scientific anchors (short)

Attention residue; Zeigarnik effect; deep work; implementation intentions; decision fatigue; Gall’s law; negative feedback loops; antifragility; planning fallacy.

### Appendix B: glossary (EN)

Inbox: single capture entry for everything.  
Quest Slots=2: quest tracker limit; at most two quests tracked at once.  
Quest Log: the full list of quests (Main Quests + Side Quests).  
Side Quest: a stored “maybe later” quest; not tracked by default.  
Rulebook: your global principles (If-Then rules + No-Go gates).  
No-Go gate: a hard refusal rule (an anti-goal turned into behavior code).  
Regret Test: “If I don’t do this, will future me regret it?” (a prioritization filter).  
Micro-Quest: a 2–4 week experiment to test a candidate quest before promotion.  
Dungeon Run: calendar-locked, distraction-free single-task block.  
HUD: a small scoreboard (feedback, not goals).  
Resupply/Upgrade: the camp session (Plan + Improve).  
If-Then: “If situation, then action” rule (behavior code).  
`lifelong_story`: time-ordered archive; append-only save files.  

### Appendix C: filesystem reference (practical)

Minimal roles (Google Drive-first):

- `Lifelong_Vision/` (control plane): `Inbox.*`, `HUD.*`, `QuestLog.*`, `Rulebook.*`, templates
- Project folder (working directory): active quests + loot (outputs)
- `lifelong_story/` (archive): `YYYY/YYYY-MM/YYYY-MM-DD - ...` (append-only)

Pointer idea (for off-Drive media): keep the heavy asset in iCloud/NAS/OneDrive, and store a small pointer note in Drive (name/id, real location path/link, date, related project).

---

## 中文版

> 本部分为英文版翻译。如有不一致，以英文版为准。

## LIV 玩家指南（中文）

多数人失败，不是因为懒，而是因为用了错误的模型在运行现代生活。

大脑擅长判断、品味与意义，但它不是硬盘，也不是可靠的调度器。当你把所有事都“开在脑子里”，未完成的任务会变成后台进程：注意力被切碎，焦虑上升，进度下降。卡尼曼的研究能帮助我们命名其中一部分机制（规划谬误、认知上限），塔勒布能帮助我们命名结果（脆弱：现实一个小意外就能击穿计划）。

LIV 的反击不是要求你更自律，而是改变运行环境：外置记忆、限制注意力、让日历承载现实、用少数指标做反馈、定期升级系统。长期来看，你会变成半人半机：机器做机械循环，人做判断与纠偏。

## 精简主线（3 步）

整套系统只有三步：

探索收集 → 完成挑战 → 整备升级

探索收集负责自由探索并进水，完成挑战负责通关与掉落，整备升级负责回血、整理、纠偏与再出发。

这也是一条游戏叙事。任天堂的哲学是让玩家始终处在一个清晰节奏里：发现兴趣点、面对难题、解决难题获得奖励与整备，然后再出发。LIV 借用同样结构：先探索并收集信号，再选择并攻克两条主线的难题，最后通过整备升级把系统变得更省力。

### 玩家视角（只包含你要做的）：系统怎么跑起来

从你的视角看，LIV 故意做得很“无聊”。你漫游，你通关，你整备。探索不是“规划时间”，它是自由与留白。你可以做任何想做的事，但只要把所有输入丢进收集箱，然后继续走。挑战才是承诺发生的地方：你看日历，从追踪主线里选一条，打一把副本，不切换。整备是你“改地图”的唯一时刻：你打开状态栏与追踪主线任务卡，决定下一轮追踪什么、把副本排进日历、写一条规则、归档战利品、把“下一关”写清楚。

把它想成一条故事线。你看到一个灵感，把它丢进收集箱，不做决定。系统会把它记进任务面板，先当支线（默认迷雾）。过几天你照日历打一把副本，产出战利品（文档/改动/决策/卡片更新）。到了整备，你会发现：最近这段时间的战利品 + 想法，反复指向同一个主题。你跑一次遗憾测试，再看禁行门。如果通过，你就把那条支线变成一个微型任务（2–4 周），但它只花探索预算，不立刻抢主线槽位。等微型任务持续产出稳定战利品，下一次整备你才把它晋升进任务槽位=2（替换掉一个旧追踪）。

这个故事能在天/周/月/季度/年/几年上重复。变化的只是你“看战利品”的时间窗口，以及整备时决策的尺度。不变的硬约束是：不在整备之外晋升，任务槽位必须封顶，每次副本必须掉落战利品。

如果你想看“系统内部如何互动”（状态对象、不变式、护栏与复活、自动化职责、存储契约），请看后面的 **《LIV 系统工程规格（中文）》**。使用系统时，你不需要先理解这些。

## 探索收集：建立一个收集箱，只追加

选一个入口当收集箱：一份文档、一个工具都可以，关键是打开要快。规则只有一条：重要输入都追加进去。不是“记住它”，也不是“立刻决定”，先收集。

如果输入是大体积媒体，原件在 iCloud / 家庭 NAS / OneDrive，就不要复制。只写“指针”：一条小记录，说明它是什么、在哪里、关联哪个项目。这样 Google Drive 继续充当可搜索的全局索引，而你不必把所有字节强行搬家。

“探索”指的是心理许可：你可以先对世界保持敏感，把兴趣点、安全风险、疑问、机会、人脉线索先收下，而不必立刻承诺。收集箱的价值就在于：让好奇心变得安全。

探索也有一个更现实的作用：它是你被保护的留白（参考值约 30%）。平稳时你用它休息、散步、尝试新体验、捕捉黑天鹅；崩溃时你用它做紧急修复与重置，而不是偷走挑战时间。

关键在“职责分离”。探索收集只负责进水口 + 自由留白：先追加，先保持凌乱，不排序，不排程。决策与排程属于整备升级。如果把两者混在一起，你要么不敢探索（因为每个信号都要你立刻处理），要么无法决策（因为收集箱最终会变成沼泽）。

为了让系统轻，你必须默认路由：所有输入先进入支线任务。你不是删除梦想，而是收起来。支线任务存在于任务面板，但默认在“战争迷雾”里（平时不展开、不维护）。只有在整备升级时允许“晋升”：要追踪一个新任务，必须替换掉当前追踪槽位中的一个（任务槽位=2）。建议加一个冷却：新想法至少放 7–14 天再晋升，让兴奋退潮、信号留下。

新体验也是支线，但要有独立车道：把它当作“机缘队列”。同时只允许 1 个活跃体验，并且只能消耗探索预算（不得挤占挑战副本）。记录时把体验写成一条短任务故事（时间 / 地点 / 人物 / 事件 / 预算）。之后在整备升级里提炼“回血要素”，再写进规则书。

## 完成挑战：任务槽位=2（任务追踪栏）+ 日历 + 副本

完成挑战的起点是一个残酷事实：注意力是瓶颈。所以 LIV 强制任务槽位=2：最多同时追踪两条主线。一次副本只追踪一条，不在副本里分叉。其余的先进入候选/以后再说。

实践中，“2”是上限，不是目标。默认只追踪 1 条。只有在你接受切换压力时才追踪 2 条。如果你发现切换频率在上升，把它当作红旗：下次整备就降回只追踪 1 条。

然后是物理事实：不进日历就等于不存在。于是挑战必须以“副本”的形式发生：锁进日历的单核无干扰时间块。副本里只做一件事，不切换，不混搭，通关。

循环长度可变（几天一轮由你决定），但预算保持简单（参考值）：约 30% 用于探索留白（休息/机缘/新体验/缓冲），约 60% 用于挑战副本，约 10% 用于整备升级（排程/规则/归档/系统修复）。这不是道德规则，而是反脆弱规则：没有被保护的留白，现实会击穿系统。

副本必须掉落能“指认”的战利品：一段可发表的文字、一段能运行的改动、一张清晰的决策备忘录、一次可测量的训练完成。如果你无法指向一个战利品，那这一段时间大概率被切换与漂移吃掉了。

这是第一类内建崩点与修复：短周期（天）的常见风险是“忙但没有战利品”。LIV 的规则很硬：**不掉落就不算通关**。如果你连续几天打完副本却拿不出战利品，说明通关判定太大或太虚。把通关判定缩到“一次副本就能掉落”的程度，系统才会重新跑起来。

一个很实用的规则能让挑战变诚实：开打前先写一句通关判定。写不出来，说明你手里不是挑战，而是一团雾。

最小可用的副本协议也可以压成一句话：开打前写清一个战利品，像预约一样保护时间块，结束后写一行“我改变了什么/下一步是什么”。系统很简单，难点只有一个：抵抗看不见的敌人——切换。

为了让追踪主线保持轻，用“任务卡”。对每条被追踪的主线，只把三行放在主界面：通关判定 / 状态栏前三 / 下一关。其余细节都放到“扩展（可选）”。每次整备回写任务卡；任务卡驱动下一次副本。

## 整备升级：一次会话，两段协同（计划 + 改进）

整备升级是那一小块（常见参考值约 10%），用来保证下一轮 60% 能跑起来，并防止系统漂移。

但升级不是一个动作，而是两段，必须协同。

计划段让下一轮能跑起来：更新一个极简状态栏；选择要追踪什么（任务槽位=2）；按约 60% 把挑战副本排进日历；把探索留白（约 30%）和整备时间（约 10%）提前保护住，避免被吃掉。

改进段让下一轮更省力：用马斯克五步法（质疑→删除→简化→加速→最后才自动化）；写/升级一条如果-那么（行为代码）；选一个最大摩擦点准备自动化；并执行归档——完成或不再活跃的战利品（产物），带时间戳移动进 `lifelong_story`。不改历史，未来补充时新增一条存档。

整备升级也是“自上而下”和“自下而上”真正握手的地方。你拿这一周的战利品（你实际做了什么），去更新世界地图（你下一轮要做什么），让下一轮更省力。

如果你想用更“贴地”的方式发现方向，可以在整备里运行“主线发现（Quest Discovery）”：

- 观察：扫一遍收集箱 + 战利品。哪些主题反复出现？哪里让你更有能量？哪里在耗你？
- 价值：用规则书做过滤。跑一次遗憾测试（“如果我不做，未来的我会后悔吗？”），同时检查禁行门（反向清单）。
- 实验：用探索预算跑一个微型任务（2–4 周）。同一时间只允许 1 个实验。只有当实验持续产出可复用战利品，并让地图更清晰，才允许晋升为追踪主线。

这就是为什么支线不会分散注意力：它们只是候选池。但系统仍然专注，因为晋升被限流（只在整备发生 + 有冷却），追踪槽位被硬封顶（任务槽位=2）。

这是第二类内建崩点与修复：中周期（周）的常见风险是“追踪两条导致切换过频”。解法不是更自律，而是更少追踪：任务槽位封顶、默认只追踪 1 条、晋升只在整备发生（并带冷却）。

这是第三类内建崩点与修复：长周期（季/年）的常见风险是“地图漂移”（环境变了你不变，或你频繁换方向但没证据）。解法是微型任务：用探索预算做安全探针，只有稳定产出可复用战利品才允许晋升换挡。

最后，把“隐藏 vs 归档”讲清楚。隐藏＝“不是现在但仍活着”（任务面板里，迷雾下）。归档＝“过去存档”：把细节移进 `lifelong_story`，在任务面板里只留一个指针 + 一行摘要。一个可行的垃圾回收规则：如果一条支线 ~12 个月都没被晋升或复盘过，就压缩并归档细节。世界仍可探索，主界面仍很轻。

生活底盘也用同样思路监控。多数时候它不该和主线抢槽位，而该用阈值 + 自动化维持稳定。只有当它崩坏（睡眠崩/家里乱/现金不安全/关系漂移）时，才把它当作一个“赛季任务”临时晋升：先修复，再降回底盘。

校准并不是另一套仪式，它只是整备升级在不同强度下的表现。平稳时两段都做；跑偏时两段做得更狠；快淹死时就用“救援模式”整备：临时降级为任务槽位=1，只排下一个挑战副本，先做计划段把循环拉回轨道；等运动恢复后再补改进段。

把“整备”当作同一个动作的不同尺寸，你就能在不同时间尺度保持一致：

- 2 分钟（每次副本后）：记录战利品 + 写一句“下一关”。
- 10 分钟（每天收工）：收集箱关机（清到“明天/本轮”，夜里不做新决策）。
- 60–120 分钟（每周）：完整整备（状态栏 + 排程 + 规则书 + 晋升 + 归档）。
- 60–120 分钟（每月）：配额检查 + 支线压缩 + 底盘阈值检查。
- 半天（每季）：关卡复盘 + 大换挡 + 深度归档。
- 1 天（每年）：关档（编译规则书、归档大产物、重置地图）。

这也是文件系统契约真正落地的时刻：整备升级就是清理工作集（背包整理）。你把已完成产物从工作目录移动进时间线归档（`lifelong_story`），让“现在”保持轻，让“历史”可追溯。长期看，它既是日记，也是可复用的知识资产库。

## 新手村（第一周教程）：安装 + 校准

系统的价值不在于“看起来多漂亮”，而在于它能不能在坏日子里跑起来。所以第一周不比优化，比“跑通”。

在你搭建行动系统之前，你需要先确定自己的长期主线任务（人生级目标）。

一个很实用的方法是：写出 100 个（或者很多就行）你想做的人生目标级别的事情，然后删除删除删除，直到剩 5 个以下——最好是 3 个以下。把它们命名为你的主线任务。这个自上而下的过程给你“世界地图”：你知道自己要去哪里，也知道哪些路你要拒绝走。

然后，探索收集→完成挑战→整备升级这个自下而上的循环，给你“行动引擎”：每天从现实输入出发，把它们变成挑战，在副本中通关，然后升级规则。上层目标和下层引擎缺一不可：只有目标会空转，只有引擎会迷路。

第一小时只做最小安装：建一个收集箱；选两条你能在 7 天内推进的主线（最多追踪两条）；每条主线写一句通关判定；选一个你能测的极简状态栏；按 30/60/10 的参考值把时间放进日历（探索留白/挑战副本/整备升级）。然后在 24 小时内跑一次副本，再做一次整备，不要等“准备好了”。

第一周还要装好“专注锁”：给支线任务准备一个地方（任务面板，默认迷雾），写下 1 条禁行门（这周你拒绝的一件事），并确定默认追踪模式（默认只追踪 1 条，除非强理由必须 2 条）。你不是在做完美计划，而是在做一个你累了也能跑的循环。

接下来的一周不要优化，只要运行并诚实记录。周末做一次整备，写下第一条如果-那么。那一刻，你的人生从愿望清单变成可运行的系统。

## LIV 系统工程规格（中文）

> 面向：开发团队（LIV 软件助手）。  
> 本规格定义系统模型、核心工作流与内建护栏。目标不是“堆功能”，而是让玩家侧更省力。

### 1）设计目标（我们优化什么）

- 最小化执行期认知负荷（人累了也能跑）。
- 保护探索预算与黑天鹅：新想法默认进支线，迷雾下不打扰。
- 用硬封顶稳定专注：任务槽位=2，且晋升只能发生在整备。
- 用微型任务安全换挡：先用证据跑出来，再晋升。
- 存储要简单可检视：类文件系统契约 + `lifelong_story` 追加写入。

非目标：
- “预测未来”。这是一套反馈系统，不是预言机。
- 做一个巨型待办数据库。任务面板允许很大，因为只有追踪任务才重要。

### 2）核心模型（状态对象）

- **收集箱条目**：一个输入（想法/风险/机会/人脉/媒体指针），必须追加很快。
- **任务（Quest）**：一条任务线，带类型（主线/支线/微型任务/新体验/赛季修复）。
- **任务卡（Quest Card）**：追踪主线的 3 行接口（通关判定 / 状态栏前三 / 下一关）。
- **任务面板（Quest Log）**：所有任务集合（大多数支线在迷雾下）。
- **任务槽位（Quest Slots）**：正在追踪的任务集合，上限=2（默认追踪 1）。
- **副本（Dungeon Run）**：日历锁定的专注块，只能绑定 1 条追踪任务。
- **战利品（Loot）**：副本产出的可指认成果（文档/改动/决策/卡片更新），可回链可归档。
- **状态栏（HUD）**：少量指标（反馈，不是目标），口径固定。
- **规则书（Rulebook）**：全局行为代码（如果-那么 + 禁行门）。
- **存档（ArchiveEntry）**：`lifelong_story` 里的时间线存档（尽量只读、追加写入、带回链）。

### 3）不变式（硬规则）

这些不是建议，而是系统必须强制/强提醒的硬约束：

- **I1 — 任务槽位封顶**：`追踪任务 ≤ 2`（默认 1）。
- **I2 — 不在整备之外晋升**：支线不能在整备之外直接变成追踪主线。
- **I3 — 战利品要求**：副本结束必须生成 ≥1 个战利品，否则视为未通关。
- **I4 — 微型任务限流**：同一时间 `活跃微型任务 ≤ 1`（消耗探索预算）。
- **I5 — 机缘队列独立**：`活跃新体验 ≤ 1` 且只能花探索预算。
- **I6 — 预算保护**：默认维持 30/60/10（探索/挑战/整备），不是道德，而是反脆弱。
- **I7 — 夜间规则**：夜里不做新决策，只做收集箱关机与排程收尾。

### 4）核心工作流（状态转移）

- **收集流**
  - 输入 → 追加到收集箱
  - 路由 → 进入任务面板成为支线（默认迷雾）
  - 此阶段禁止排程

- **挑战流**
  - 选追踪任务 → 开副本（锁日历）
  - 开打前必须写通关判定
  - 副本结束 → 必须生成战利品 → 回写任务卡“下一关”

- **整备流（计划 + 改进）**
  - 计划：更新 HUD → 选任务槽位（≤2）→ 排挑战副本（约 60%）→ 保护探索（约 30%）+ 整备（约 10%）
  - 改进：更新规则书（1 条如果-那么 + 1 条禁行门）→ 选 1 个自动化摩擦点 → 归档战利品 → 运行主线发现

- **主线发现（整备内）**
  - 观察：收集箱 + 战利品做主题聚合，并识别能量信号
  - 价值：规则书过滤 + 遗憾测试 + 禁行门
  - 实验：开 1 个微型任务（2–4 周），只花探索预算；只有持续产出可复用战利品才允许晋升

- **归档与压缩**
  - 隐藏（迷雾）＝仍活着，留在任务面板
  - 归档（过去存档）＝细节移进 `lifelong_story`，任务面板只留指针 + 一行摘要
  - 垃圾回收：支线 ~12 个月未触达 → 压缩并归档细节

### 5）护栏与复活（内建，不是外挂）

这些是系统稳定性的第一等特性：

- **短周期（天）失败：忙但没战利品**
  - 识别：副本结束没有战利品，或连续“收尾”但没有可指向产物
  - 处理：强制缩小通关判定；提示选择战利品类型；降级范围

- **中周期（周）失败：切换过频**
  - 识别：频繁换追踪、反复改日历、低通关率
  - 处理：建议回到默认追踪 1 条；对切换增加阻力；新增追踪必须用“替换”而不是“叠加”

- **长周期（季/年）失败：地图漂移**
  - 识别：战利品主题与追踪主线偏离；规则书长期不更新；HUD 趋势变差
  - 处理：用微型任务做安全探针；安排更长整备；大换挡必须有证据

- **崩溃失败：系统熄火**
  - 识别：连续做不到最低副本 + 底盘阈值破裂（睡眠/训练/维护）
  - 处理：救援模式（任务槽位=1，只排下一个副本，先做计划段）或复活协议（宣布破产/冷启动）

### 6）人与 AI：半人半机协作契约

机器负责机械循环：收集、去重、生成指针、主题聚类（发现重复主题）、把输入默认路由到支线任务面板、汇总状态栏、提出排程建议与冲突检测、在你追踪过多/切换过频时增加阻力与提醒、模板与提醒、自动归档到 `lifelong_story` 并回链。机器维护管道。

人负责判断：定义通关判定、做取舍、诚实解释反馈、决定下一条如果-那么要改哪里，以及处理不确定性、风险与黑天鹅机会。人决定系统的目的。

把判断交给机器会变脆弱；把机械循环留在脑子里会把你耗干。

### 7）存储契约（文件系统）

- `Lifelong_Vision/`（控制面）：`Inbox.*`、`HUD.*`、`QuestLog.*`、`Rulebook.*`、模板
- 项目文件夹（工作目录）：正在进行的挑战与战利品
- `lifelong_story/`（归档日志）：按时间追加写入的存档（带回链）

跨云大媒体依旧留在原处（iCloud/NAS/OneDrive）。Drive 放指针（名称/标识、原始位置链接/路径、日期、关联任务）。

### 8）可观测性（给开发团队）

最小可观测指标：

- 次数：每周副本数、每周战利品数、切换频率、改日历次数
- 预算：探索/挑战/整备 时间占比（近似即可）
- 漂移：战利品主题重复、规则书更新频率
- 底盘：睡眠/训练/维护阈值是否破裂

---

## 附录（可追求大而全）

### 附录 1：科学内核与全景研究笔记

LIV 系统不是凭空发明的，它是基于以下经过实证的认知科学、行为学与管理学原理组装而成的。我们在这里列出它的“源代码”，供你查阅。

#### 1. 认知负荷与注意力

**注意力残留 (Attention Residue)**  
Sophie Leroy (2009) 的研究发现，当你从任务 A 切换到任务 B 时，你的注意力并不会立刻跟过来，一部分认知资源会“残留”在任务 A 上。频繁切换会导致“认知带宽”急剧下降。  
*LIV 应用*：这就是为什么我们强制使用一段无干扰的 **副本 (Dungeon Run)**，并在块内禁止切换。

**蔡格尼克效应 (Zeigarnik Effect)**  
Bluma Zeigarnik 发现，人对“未完成的任务”记忆深刻，这会占用大量工作记忆（RAM）。一旦任务完成或被妥善记录，大脑就会将其“卸载”。  
*LIV 应用*：**收集箱 (Inbox)** 的存在就是为了利用这一效应，把所有想法从大脑移出，释放工作记忆。

**深度工作 (Deep Work)**  
Cal Newport 提出，在 AI 时代，稀缺的不是信息，而是“在无干扰状态下进行职业活动的能力”。  
*LIV 应用*：**日历优先**。主动型工作必须优先排入日历，被动型工作（邮件/会议）只能填缝。

#### 2. 决策科学与行为设计

**实施意图 (Implementation Intentions)**  
Peter Gollwitzer (1999) 证明，把目标写成 `If [情境] Then [行动]` 的格式，能显著提高达成率。这利用了大脑的自动化机制，绕过了对“意志力”的依赖。  
*LIV 应用*：这就是我们的 **如果-那么规则库**。

**心理对比 (WOOP / MCII)**  
Gabriele Oettingen 发现，单纯的“正向幻想”反而会降低行动力。有效的策略是先想愿景，再想障碍。  
*LIV 应用*：**整备中的改进段**。在推进前直面障碍，在事后直面失败。

**福格行为模型 (Fogg Behavior Model)**  
BJ Fogg 提出 `B=MAP`（行为 = 动机 + 能力 + 提示）。动机是波动的，能力是缓慢增长的，只有提示是可控的。  
*LIV 应用*：**机器外骨骼**。通过环境场与自动化脚本降低行为门槛，不依赖不稳定的动机。

**决策疲劳 (Decision Fatigue)**  
Baumeister 提出，做决策会消耗心理能量。随着一天中决策次数增加，决策质量会下降。  
*LIV 应用*：**任务槽位=2**。减少当下的决策量，把决策前置到整备集中处理。

#### 3. 系统论与控制论

**盖尔定律 (Gall’s Law)**  
John Gall 指出，一个切实可行的复杂系统，势必是从一个切实可行的简单系统发展而来的。从头设计的复杂系统根本不工作。  
*LIV 应用*：**先跑通再优化**。第一周教程只追求“可运行”。

**负反馈调节 (Negative Feedback Loop)**  
控制论的核心。系统通过感知偏差，调整输入，使输出回归目标值。  
*LIV 应用*：**状态栏 (HUD) + 整备**。每次整备就是一次纠偏。

**反脆弱 (Antifragility)**  
Nassim Taleb 提出，有些系统能从冲击和混乱中获益。  
*LIV 应用*：**探索留白（参考值约 30%）** + **复活协议**。允许系统在遇到黑天鹅机会或现实冲击时熔断、修复与重启，拥抱随机性。

#### 4. 管理学与工程思维

**规划谬误 (Planning Fallacy)**  
Kahneman & Tversky (1979) 发现，人类倾向于用“最佳情况”来预测未来，而忽略历史上的失败概率。  
*LIV 应用*：**冲突检测**。日历的物理空间是有限的，这强制你面对现实。

**帕金森定律 (Parkinson’s Law)**  
工作会自动膨胀，占满所有可用的时间。  
*LIV 应用*：**时间盒/副本**。用明确的时间块边界对抗无限膨胀。

**限制理论 (Theory of Constraints)**  
Goldratt 提出，任何系统的产出都受限于其瓶颈。  
*LIV 应用*：注意力是瓶颈，所以必须限流（任务槽位=2）。

### 附录 2：术语表 (Glossary)

| 术语 | 解释 | 来源 |
|---|---|---|
| **Inbox** | 收集箱，所有想法的唯一入口 | GTD |
| **Quest Slots** | 任务槽位（最多同时追踪的主线数量，上限=2） | LIV（类游戏） |
| **Quest Log** | 任务面板/任务日志（主线 + 支线的全集） | LIV（类游戏） |
| **Side Quest** | 支线任务（候选任务，默认迷雾/不追踪） | 游戏通用 |
| **Rulebook** | 规则书/原则库（全局如果-那么 + 禁行门） | LIV（类游戏） |
| **No-Go gate** | 禁行门（硬拒绝规则，把反向清单写成脚本） | LIV（类游戏） |
| **Regret Test** | 遗憾测试（“如果不做，未来会后悔吗？”） | Decision science |
| **Micro-Quest** | 微型任务（2–4 周试运行，用来验证是否值得晋升） | LIV（类游戏） |
| **WIP** | Work In Progress（同义：Quest Slots） | Kanban |
| **DoD** | Definition of Done，完成的定义 | Agile / Scrum |
| **Dungeon Run** | 副本（无干扰专注时间，原 Deep Block） | Deep Work |
| **HUD** | 状态栏（原 Scoreboard/记分牌） | 4DX / OKR |
| **If-Then** | 实施意图，预设的条件反射脚本 | Psychology |
| **Resupply/Upgrade** | 整备升级（计划 + 改进 的整备会话） | LIV / inspired by GTD |

### 附录 3：文件系统参考（建议结构）

最小三层（类 Unix）：

- `Lifelong_Vision/`（控制面）：`Inbox.*`、`HUD.*`、`QuestLog.*`、`Rulebook.*`、模板与清单
- 项目文件夹（工作目录）：正在进行的挑战与战利品（产物）
- `lifelong_story/`（归档日志）：`YYYY/YYYY-MM/YYYY-MM-DD - ...`（追加写入，尽量只读）

跨云大媒体（iCloud/NAS/OneDrive）：不强制复制，优先在 Drive 放“指针文件”（类似软链接），写清名称/标识、原始位置（路径/链接）、日期、关联项目。

