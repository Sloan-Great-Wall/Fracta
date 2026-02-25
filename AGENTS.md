# AGENTS.md

## Cursor Cloud specific instructions

### Project overview

Fracta is a Rust workspace (11 crates) implementing a local-first life operating system. There is no runnable binary or server — the codebase is a library consumed via UniFFI by a Swift/SwiftUI app (Apple platforms only). On Linux, development consists of building, linting, and testing the Rust core.

### System dependency

`libssl-dev` (Ubuntu) is required for the `reqwest` crate used by `fracta-comm`. This is pre-installed in the VM environment.

### Key commands

All commands from the repo root (`/workspace`):

| Task | Command |
|------|---------|
| Build | `cargo build --workspace` |
| Test | `cargo test --workspace --all-features --locked` |
| Lint (clippy) | `cargo clippy --workspace --all-targets --locked -- -D warnings` |
| Format check | `cargo fmt --all -- --check` |
| Docs | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked` |

These mirror the CI configuration in `.github/workflows/ci.yml`.

### Crate status

Active crates with real implementations and tests: `fracta-vfs`, `fracta-note`, `fracta-index`, `fracta-ai`, `fracta-ffi`. The remaining crates (`fracta-query`, `fracta-platform`, `fracta-comm`, `fracta-sync`, `fracta-crypto`, `fracta-framework`) are stubs.

### Gotchas

- The Rust toolchain is pinned to `stable` via `rust-toolchain.toml` with `rustfmt` and `clippy` components. `rustup` will auto-install this on first cargo invocation.
- `rusqlite` uses `features = ["bundled"]` so it compiles SQLite from C source — a C compiler (`gcc`/`cc`) must be present.
- All tests create temp dirs dynamically; no fixtures or test data directories exist.
- There is no runnable application binary on Linux. The FFI bridge (`fracta-ffi`) produces a static library for Swift consumption. Testing the full product end-to-end requires macOS with Swift/Xcode.
- `Cargo.lock` is committed; use `--locked` to ensure reproducible builds matching CI.
