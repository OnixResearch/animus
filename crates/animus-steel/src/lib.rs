//! Steel Scheme loop fabric for `animus` agent orchestration.
//!
//! This crate keeps the canonical scriptable agent loop separate from the
//! `no_std` core and adapter contracts. Steel programs decide the next loop
//! action while Rust validates every boundary decision before it reaches model
//! or tool adapters.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod action;
mod error;
mod fabric;
mod quoting;
mod turn;

/// Next action selected by the Steel loop fabric.
pub type LoopDecision = action::Decision;

/// Error returned by Steel loop fabric operations.
pub type LoopError = error::Failure;

/// Persistent Steel-backed loop fabric.
pub type SteelLoopFabric = fabric::Runtime;

/// Inputs made available to the Steel loop for one turn.
pub type LoopTurn = turn::Input;
