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
#![allow(unknown_lints, module_file_count, excessive_file_length)]
#![forbid(unsafe_code)]

/// Pure bounded session branch explorer reducer and host port.
#[path = "branchexplorer.rs"]
pub mod branch_explorer;
mod bridge;
mod frontend;
#[cfg(feature = "terminal-image")]
mod pixel;
mod runner;
#[cfg(feature = "terminal-video")]
mod video;
mod view;

#[cfg(test)]
mod tests;

pub use bridge::{
    ApprovalRequest, ToolCommandChoice, ToolCorrelationId, ToolHostCommand, ToolHostError,
    ToolHostEvent, ToolHostFailure, ToolHostFailureClass, ToolProgress, ToolRunId,
};
pub use frontend::CodingSessionRenderModel;
#[cfg(feature = "terminal-image")]
pub use pixel::{
    RATS_IMAGE_COHORT_REVISION, TerminalImageByteResolver, TerminalImageCapabilities,
    TerminalImageCapabilityState, TerminalImageDigest, TerminalImageError, TerminalImageExecution,
    TerminalImageFormat, TerminalImageGeometry, TerminalImageHandle, TerminalImageIntent,
    TerminalImageLifecycleService, TerminalImageMetadata, TerminalImageOperation,
    TerminalImageOperationStatus, TerminalImagePlan, TerminalImagePlanInput, TerminalImagePolicy,
    TerminalImageProtocol, TerminalImageReceipt, execute_terminal_image_plan,
    parse_kitty_capability_response, plan_terminal_image, project_terminal_image_receipt,
    validate_terminal_image_evidence,
};
/// Re-export of the Subwayrat terminal keymap package used by the optional TUI host.
#[cfg(feature = "rats-widgets")]
pub use rat_keymap;
/// Re-export of the Subwayrat terminal widget package used by the optional TUI host.
#[cfg(feature = "rats-widgets")]
pub use rats;
pub use runner::{
    GuardedToolRun, HostedToolOutcome, ToolHostCommandSource, ToolHostEventSink, run_guarded_tool,
    run_hosted_tool, run_simple_tool,
};
#[cfg(feature = "terminal-video")]
pub use video::{
    TerminalVideoAction, TerminalVideoBackendCall, TerminalVideoBackendMode,
    TerminalVideoBackendOperation, TerminalVideoBackendService, TerminalVideoCapabilities,
    TerminalVideoCapabilityState, TerminalVideoCommand, TerminalVideoDescriptor,
    TerminalVideoError, TerminalVideoFrameExecution, TerminalVideoFrameExecutionInput,
    TerminalVideoFrameFetch, TerminalVideoFrameMetadata, TerminalVideoFramePacket,
    TerminalVideoFrameSource, TerminalVideoFrameSourceRequest, TerminalVideoId,
    TerminalVideoPolicy, TerminalVideoPolicyDecision, TerminalVideoReceipt,
    TerminalVideoReduceInput, TerminalVideoReduction, TerminalVideoSourceClass, TerminalVideoState,
    TerminalVideoStatus, execute_terminal_video_backend_actions, execute_terminal_video_frame,
    fetch_terminal_video_frame, project_terminal_video_receipt, reduce_terminal_video,
    select_terminal_video_backend, validate_terminal_video_evidence,
};
pub use view::{
    AgentDashboardState, AgentStatus, AppShellConfig, AppShellState, CompletionCandidate,
    CompletionMenu, ControlCommand, ConversationRenderModel, DiffView, EntryMetadata, FileLine,
    ImageDisplay, LayoutDiagnostic, LayoutTree, PaneView, PromptInputState, QueueItem,
    QueueItemStatus, ScrollbackMetadata, ShellEvent, ShellEventOutcome, SlashCommand,
    SlashCommandDetector, StatusRailState, TerminalState, ToolApprovalPanel, ToolCallCard,
    ToolCardProgress, ToolPanel, ToolPanelState, ToolRenderModel, ToolSessionStatus,
    TranscriptEntry, TranscriptEntryKind, render_agent_dashboard_snapshot,
    render_app_shell_snapshot, render_conversation_snapshot, render_layout_tree_snapshot,
    render_status_rail_snapshot, render_streaming_snapshot, render_text_snapshot,
    render_tool_panel_snapshot, resolve_layout,
};

#[cfg(feature = "rats-widgets")]
pub use view::{InputAction, Keybinding, resolve_default_keybinding, resolve_keybinding};
