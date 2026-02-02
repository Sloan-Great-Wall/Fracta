//! # fracta-platform — Platform Adapter Traits
//!
//! Platform adapter trait definitions. Implementations live in platform
//! shells (Swift/SwiftUI on Apple, Kotlin on Android).
//!
//! Defines the boundary between portable Rust Core and platform-specific
//! capabilities: Keychain access, Secure Enclave, file-provider extensions,
//! push notifications, and native UI integration.
//!
//! Status: Phase 1 active.
