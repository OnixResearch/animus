//! Non-default live terminal client for Animus.
//!
//! The pure core owns client state, bounded event reduction, command descriptors,
//! and metadata-only receipts. The shell owns terminal I/O, signals, rendering,
//! and explicit daemon or injected direct connections.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod config;
mod connection;
mod core;
mod host;
mod lifecycle;
#[cfg(feature = "live-terminal")]
mod render;

pub use config::{
    ConnectionMode, TerminalConfigError, ValidatedTerminalConfig, parse_terminal_config,
};
pub use connection::{
    DaemonEvent, DaemonRequest, DaemonRequestDispatcher, DaemonResponse, DaemonSnapshot,
    DaemonTerminalConnection, DirectTerminalConnection, TerminalConnectionError,
    TerminalConnectionPort, connect_explicit_daemon,
};
pub use core::{
    ApprovalDecision, ChildAction, CleanupAction, CleanupObservation, ClientCommand,
    ClientSessionEvent, ClientSnapshot, ControlIntent, LiveTerminalReceipt, SessionAction,
    TerminalClientState, TerminalCoreError, TerminalExit, TerminalInput, TerminalIteration,
    TerminalIterationPlan, TerminalLimits, TerminalReceiptFacts, TerminalViewState,
    command_from_control, reduce_iteration,
};
#[cfg(feature = "live-terminal")]
pub use host::CrosstermTerminalHost;
pub use host::{LiveTerminalRequest, TerminalHostError, TerminalHostPort, run_attached_terminal};
#[cfg(feature = "live-terminal")]
pub use lifecycle::CrosstermTerminalLifecycle;
pub use lifecycle::{
    TerminalLifecycleError, TerminalLifecycleFailure, TerminalLifecycleGuard, TerminalLifecyclePort,
};
#[cfg(feature = "live-terminal")]
pub use render::{render_terminal_view, render_text_projection};
