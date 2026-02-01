# ADR-0003: Core architecture is Rust Core + platform shells (UniFFI)

## Context

Fracta’s core capabilities (Finder replacement, AI, optional blockchain) impose requirements we must satisfy at the same time:

- **Performance**: indexing/search/querying, view materialization, event merging, cryptographic signing, etc.
- **Cross-platform**: long-term goal is full multi-platform support; short-term focus is macOS/iOS.
- **Platform integration**: filesystem permission models, Keychain/Secure Enclave, Spotlight/Quick Look, Apple AI frameworks, etc.

If we put “thick logic” in platform languages (Swift/Kotlin/JS), we will eventually rewrite and drift across platforms.

## Decision

Adopt “one core, multiple shells”:

- **Rust Core (cross-platform)**: data model, indexing, search, query/view engine, event processing (dedup/merge/stats), crypto/signing, chain integration abstractions.
- **Platform Shell (platform-specific)**:
  - Apple: Swift/SwiftUI handles UI, OS integrations and permissions, Keychain/Secure Enclave, and Apple AI framework calls.
  - Future: Windows/Linux (Tauri or native), Android (Kotlin/Compose).
- **FFI**: expose Rust Core APIs to shells via UniFFI.

## Consequences

- Architecture and on-disk formats should be stabilized first in `docs/SPEC.md` and ADRs to reduce cross-language API churn.
- System-level technical documentation should treat event processing as part of Rust Core; the shell focuses on UI and platform integrations.
  - Canonical technical doc: `docs/ENGINEERING.md`

## Alternatives

- **All Swift (Apple-first)**: best short-term UX on Apple, but high cross-platform rewrite risk and long-term logic divergence.
- **Electron / all-web**: faster cross-platform UI, but poor fit for Finder replacement, deep system integration, performance, privacy, and local AI.

