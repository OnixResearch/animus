//! Optional terminal UI host shell for `animus`.
//!
//! This crate is the dependency boundary for terminal-facing UI integrations.
//! Reusable runtime and adapter crates stay independent of terminal rendering;
//! TUI hosts can build on this crate when they need Ratatui/Rats widgets.
//!
//! The public bridge types in this crate are UI-neutral data and thin host
//! orchestration helpers: tools report lifecycle/progress/output/approval events,
//! hosts answer with finite correlated commands, and the render model is reduced
//! from typed events without direct terminal I/O.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod bridge;
mod runner;
mod view;

#[cfg(test)]
mod tests;

pub use bridge::{
    ApprovalRequest, ToolCommandChoice, ToolCorrelationId, ToolHostCommand, ToolHostError,
    ToolHostEvent, ToolHostFailure, ToolHostFailureClass, ToolProgress, ToolRunId,
};
/// Re-export of the Rats terminal widget package used by the optional TUI host.
#[cfg(feature = "rats-widgets")]
pub use rats;
pub use runner::{
    GuardedToolRun, HostedToolOutcome, ToolHostCommandSource, ToolHostEventSink, run_guarded_tool,
    run_hosted_tool, run_simple_tool,
};
pub use view::{
    CompletionCandidate, CompletionMenu, ConversationRenderModel, EntryMetadata, ImageDisplay,
    PromptInputState, ScrollbackMetadata, SlashCommand, SlashCommandDetector, TranscriptEntry,
    TranscriptEntryKind, ToolApprovalPanel, ToolPanel, ToolRenderModel, ToolSessionStatus,
    render_conversation_snapshot, render_streaming_snapshot, render_text_snapshot,
};

#[cfg(feature = "rats-widgets")]
pub use view::{
    InputAction, Keybinding,
    resolve_default_keybinding, resolve_keybinding,
};
