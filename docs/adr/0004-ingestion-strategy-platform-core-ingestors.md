# ADR-0004: Ingestion strategy uses Platform/Core Ingestors

## Context

The Past subsystem requires multi-source ingestion (video, browser, health, manual, etc.).
If we implement ingestion as an extra runtime/CLI shipped with the app, we typically incur:

- Larger binaries and worse startup performance
- Higher sandbox permission and code-signing/distribution complexity (especially on mobile)
- Violations of the “local-first, minimal dependency” engineering principle

## Decision

Implement ingestion as “ingestors” in two layers:

- **Platform Ingestors (platform layer)**: implement platform capabilities in the shell (WebView/OAuth flows, permission model, Keychain, etc.).
- **Core Ingestors (core layer)**: implement cross-platform fetch/parse/normalize/quality-score logic in Rust Core.

## Consequences

- Technical docs should describe ingestion as Platform/Core Ingestors (canonical: `docs/ENGINEERING.md`).
- The event/object model must be versioned and stabilized so different ingestors produce mergeable outputs.

## Alternatives

- **Ship an extra runtime/CLI with the app**: faster to prototype, but too risky for distribution/sandboxing and creates long-term maintenance complexity.

