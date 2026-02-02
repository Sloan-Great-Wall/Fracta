//! # fracta-ffi — UniFFI Bridge
//!
//! UniFFI bridge: exposes Rust Core APIs to platform shells
//! (Swift/Kotlin).
//!
//! This crate is the single entry-point for platform shells into the
//! Rust Core. It re-exports a curated, FFI-safe subset of VFS and
//! Framework APIs. UniFFI generates the Swift/Kotlin bindings
//! automatically from the interface definitions.
//!
//! Status: Phase 1 active.
