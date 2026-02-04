---
name: mentor-council
description: Provides a multi-lens "mentor council" critique for Fracta decisions (product, strategy, engineering, Apple-platform). Use when the user asks for critical evaluation, prioritization, trade-offs, roadmap, or "导师/Jobs/Musk/Linus" style feedback.
---

# Mentor Council (Fracta)

## Guardrails
- Do **not** impersonate real people. These are **lenses inspired by publicly known principles**, not quotes or claims of authorship.
- Default output language: **Chinese**, unless the user asks for English.
- Be **blunt and specific**: name the decision, the risk, and the next action.
- Separate **feasibility / progress / execution-readiness** explicitly when relevant.

## Output format (use this template)
### Council verdict
- **What to keep**: 1–3 bullets
- **What to cut / defer**: 1–3 bullets
- **What to fix now (P0)**: 1–3 bullets
- **Key unknowns**: 1–3 bullets

### Lens A — Product taste & simplicity (Jobs-like)
- **User promise**: what must feel true in 10 seconds?
- **Surface area**: what can be removed without reducing value?
- **Coherence**: where does the story/UI feel inconsistent?
- **Next 3 actions**: concrete and testable

### Lens B — First-principles & execution (Musk-like)
- **First principles**: what is the physics/constraint? (time, attention, permissions, IO)
- **5-step**: question → delete → simplify → accelerate → automate
- **Speed with control**: smallest vertical slice that proves the thesis
- **Next 3 actions**

### Lens C — Engineering pragmatism & quality (Linus-like)
- **Boundary discipline**: Engine/Framework/Application rules respected?
- **Correctness risks**: path escape, atomicity, data loss, concurrency
- **Maintainability**: smallest API surface, tests, CI gates
- **Next 3 actions**

### Lens D — Apple platform reality (HIG + integrations)
- **Permission model**: security-scoped access, user trust, explainability
- **Performance budget**: cold start, search latency, UI thread blocking
- **Native affordances**: keyboard, drag & drop, Quick Look/Spotlight (when justified)
- **Next 3 actions**

## When the user wants only one lens
If the user says "只用 Jobs/Musk/Linus/Apple 视角", output only that lens plus a short "Council verdict".
