//! Thin compatibility crate for the `animus` workspace.
//!
//! New business logic belongs in `animus-core`; swappable component contracts
//! belong in `animus-adapters`; runtime orchestration belongs in `animus-std`.
//! This crate re-exports the workspace crates for downstream users that want one
//! dependency while keeping crate barriers explicit in-tree.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

pub use animus_adapters::{
    AdapterContractError, ComponentAdapter, MAX_ADAPTER_FIELD_CHARS, MIN_ADAPTER_FIELD_CHARS,
    ModelAdapter, ModelRequest, ModelResponse, ToolAdapter, ToolRequest, ToolResponse,
};
pub use animus_core::{AnimusState, CoreError, Intent, MAX_INTENT_CHARS, MIN_INTENT_CHARS};
pub use animus_std::{
    ConfigError, INTENT_ENV_KEY, RunError, RunReport, RuntimeConfig, SummaryModelAdapter,
    load_runtime, run, run_with_model,
};

/// Re-export of the optional Basalt UCAN contract-boundary dependency.
#[cfg(feature = "basalt-reference")]
pub use basalt;

/// Re-export of the optional Cairn lifecycle-engine core dependency.
#[cfg(feature = "cairn-reference")]
pub use cairn_core;

/// Re-export of the optional Trellis verified-logic dependency.
#[cfg(feature = "trellis-reference")]
pub use trellis;

/// Re-export of the optional Preserves boundary serialization bridge.
#[cfg(feature = "preserves")]
pub use animus_preserves;

/// Re-export of the optional Steel Scheme loop fabric.
#[cfg(feature = "steel")]
pub use animus_steel;

/// Re-export of the optional terminal UI host shell.
#[cfg(feature = "tui")]
pub use animus_tui;
