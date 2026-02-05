# ADR-0016: Apache-2.0 License Selection

**Status:** Accepted
**Date:** 2025-02-05
**Supersedes:** License TBD note in ADR-0013

## Context

ADR-0013 established the three-level openness strategy:
1. **Data formats** — Fully open (Markdown, JSON, CSV)
2. **Engine + Framework** — Open-source
3. **Application layer** — Commercial

The specific license for level 2 (Engine + Framework) was marked as TBD. We need to select a license that:

- Supports the "open foundation + commercial services" business model
- Provides patent protection for AI/crypto components
- Maximizes enterprise adoption potential
- Maintains compatibility with the Apple ecosystem (Swift/SwiftUI)
- Aligns with modern infrastructure project norms

## Decision

**We adopt Apache License 2.0** for the Fracta Engine and Framework layers.

## Rationale

### Why Apache-2.0 over alternatives

| License | Evaluation |
|---------|------------|
| **Apache-2.0** ✅ | Patent grant, enterprise-trusted, Rust ecosystem standard |
| MIT | No patent protection; too minimal for AI/crypto components |
| GPL v3 | Copyleft friction with Apple ecosystem; scares enterprise adoption |
| LGPL v3 | Static linking complexity in Rust; confusing for users |
| MPL 2.0 | File-level copyleft adds complexity; less familiar |
| Dual MIT/Apache-2.0 | Unnecessary complexity; Apache-2.0 alone is sufficient |

### Key benefits of Apache-2.0

1. **Explicit patent grant** (Section 3) — Contributors grant patent rights for their contributions
2. **Patent retaliation clause** — If someone sues over patents, their license terminates
3. **Clear attribution rules** — NOTICE file mechanism for proper credit
4. **Enterprise acceptance** — Legal teams worldwide are familiar with Apache-2.0
5. **OSI-approved** — Officially recognized as open source
6. **Permissive nature** — Application layer can remain proprietary

### Alignment with business model

```
┌─────────────────────────────────────────┐
│  Application Layer (Proprietary/SaaS)  │ ← Commercial
├─────────────────────────────────────────┤
│  Framework Layer (Apache-2.0)          │ ← Open Source
├─────────────────────────────────────────┤
│  Engine Layer (Apache-2.0)             │ ← Open Source
└─────────────────────────────────────────┘
```

Apache-2.0 allows:
- Community contributions to Engine/Framework
- Commercial differentiation in Application layer
- Third-party products built on Engine/Framework
- Enterprise adoption without legal concerns

## Consequences

### Positive
- Clear, single license (no dual-license confusion)
- Patent protection for all contributors and users
- Maximum enterprise adoption potential
- Compatible with Swift/Apple ecosystem
- Industry-standard for infrastructure projects (Kubernetes, TensorFlow, Rust itself)

### Negative
- No copyleft protection (others can fork and close source)
- Must maintain NOTICE file for third-party attributions
- Slightly more complex than MIT (but widely understood)

### Neutral
- Contributors must agree to Apache-2.0 terms (standard CLA optional)
- License header recommended in source files (not strictly required)

## Implementation

1. `LICENSE` file created at repository root with full Apache-2.0 text
2. `Cargo.toml` updated: `license = "Apache-2.0"`
3. ADR-0013 TBD note superseded by this ADR

## References

- [Apache License 2.0 Full Text](https://www.apache.org/licenses/LICENSE-2.0)
- [Choose a License: Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [Why Apache-2.0 for Rust](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive)
- ADR-0013: Open-source Engine + Framework, commercial Application layer
