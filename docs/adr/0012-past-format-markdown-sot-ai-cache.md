# ADR-0012: Past Format — Markdown as SOT, Structured Data as AI-Derived Cache

## Status

Accepted

## Context

The Past subsystem turns multi-source activity into daily narrative stories. The original ENGINEERING.md design placed both human-written Markdown and a machine-generated `events.csv` in the same user directory, side by side.

Analysis of real-world usage (a 16-year, 31GB personal archive spanning 137 time-period folders) revealed:

1. **Users naturally write in Markdown**: time-stamped entries, free-form text, inline cost tracking, wiki-style links, photos alongside text. No rigid schema.
2. **A machine-generated CSV alongside user Markdown creates friction**: it pollutes the user's file structure, creates ambiguity about which file is authoritative, and requires dual maintenance.
3. **The structured data users need (timeline views, spending analysis, sleep stats) can be derived from the Markdown + raw data sources by AI**.

## Decision

### 1. User's Markdown is the sole SOT for Past

Each time period or each day produces a Markdown file written by the user (with AI assistance). This file is the source of truth. It lives in the user's directory alongside their photos and assets.

The daily Markdown file contains:
- Optional YAML front matter for structured metadata (date, sources, mood, summary metrics).
- A Timeline section with time-stamped entries (AI can pre-populate a skeleton from data sources; user edits and enriches).
- A Story section for narrative, reflection, and emotion (human-authored).
- Inline references to assets, people, places, and other content.

### 2. Structured event data is cache

AI extracts structured events from:
- The user's Markdown SOT.
- Raw data sources (browser history, HealthKit, calendar, spending APIs, etc.).

This structured data is stored in `.fracta/cache/events/` as JSONL files. It powers views, metrics, summaries, and analytics. It is always rebuildable — deleting `.fracta/cache/` and re-running the pipeline regenerates everything.

### 3. Export on demand

If users want structured data in their own files (e.g., `events.csv` for Excel analysis, `spending.json` for budgeting), Fracta provides an explicit export function. This is a user-initiated action, not an automatic side-effect.

### 4. AI builds skeleton, human fills soul

The intended workflow:
```
Multi-source data → AI auto-constructs timeline skeleton
                            ↓
User reviews skeleton, adds narrative + emotion + corrections
                            ↓
Complete daily Markdown file (SOT)
```

AI's role is to reduce the mechanical work of logging (what happened, where, for how long, at what cost) so the user can focus on the meaningful work of reflection (what it meant, how it felt, what to do next).

## Alternatives considered

1. **Markdown + CSV co-located (original design)**: User writes Markdown, system maintains events.csv next to it. Rejected because it pollutes user directories and creates dual-authority confusion.

2. **Database as SOT**: All events stored in SQLite; Markdown generated from database on demand. Rejected because it violates "file system as SOT" and "open formats" principles.

3. **Structured-first with optional Markdown export**: Events entered via forms; Markdown exported for reading. Rejected because form-based input is unnatural for journaling and discourages the narrative/reflective writing that gives Past its value.

## Consequences

- The AI pipeline for Past must be robust enough to extract structured events from free-form Markdown. This is a non-trivial NLP/LLM task.
- Users who prefer highly structured logging (spreadsheet-like) can still do so — their structured Markdown will be easier for AI to parse. The system does not force a style.
- The `.fracta/cache/events/` schema must be documented so that CLI tools and third-party integrations can consume it.
- Backward compatibility with existing Markdown journals (various personal formats) is a nice-to-have for onboarding, but not a design constraint — migration tools can be provided separately.
