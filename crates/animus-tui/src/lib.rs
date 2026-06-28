//! Optional terminal UI host shell for `animus`.
//!
//! This crate is the dependency boundary for terminal-facing UI integrations.
//! Reusable runtime and adapter crates stay independent of terminal rendering;
//! TUI hosts can build on this crate when they need Ratatui/Rats widgets.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

/// Re-export of the Rats terminal widget package used by the optional TUI host.
pub use rats;
