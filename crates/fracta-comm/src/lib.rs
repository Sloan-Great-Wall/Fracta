//! # fracta-comm — Communication Protocols
//!
//! Protocol clients: IMAP/SMTP, CalDAV/ICS, RSS/Atom, HTTP/REST.
//!
//! Each protocol adapter fetches remote data and materializes it as
//! open-format files (EML, ICS, OPML) via VFS. Secrets are stored in
//! OS Keychain, never in files.
//!
//! Status: Phase 2 stub.
