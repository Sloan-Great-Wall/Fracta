# ADR (Architecture Decision Records)

ADRs capture **why** we made a decision (trade-offs, constraints, alternatives), so future contributors do not have to re-litigate the same debates and so canonical docs do not drift into contradictions.

---

## When to write an ADR

- Any decision that affects long-term architecture, on-disk formats, security/privacy, or cross-platform strategy
- Any “small now, painful later” convention (e.g., UID rules, directory layout, what gets persisted to YAML)

---

## Suggested template

- Filename: `NNNN-topic-in-kebab-case.md` (example: `0001-local-first-sot.md`)
- Sections:
  - **Context**: background and constraints
  - **Decision**: what we decided
  - **Consequences**: impact and costs
  - **Alternatives**: considered options and why they were rejected

---

## Relationship to other docs

- `docs/SPEC.md` defines “what must not break” (system definition and invariants)
- `docs/ENGINEERING.md` defines “how we build it” (architecture + implementation guidance)
- ADRs define “why we chose this approach”


