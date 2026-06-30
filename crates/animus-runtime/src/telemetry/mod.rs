mod dto;

pub use dto::RuntimeEvent;

/// Adapter component kind used in runtime events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterComponent {
    /// Model adapter component.
    Model,
    /// Tool adapter component.
    Tool,
}

/// Loop decision class selected by the fabric.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoopDecisionKind {
    /// Model dispatch decision.
    Model,
    /// Tool dispatch decision.
    Tool,
    /// Final response decision.
    Respond,
    /// Stop decision.
    Stop,
}

impl LoopDecisionKind {
    /// Returns the stable decision label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Tool => "tool",
            Self::Respond => "respond",
            Self::Stop => "stop",
        }
    }
}

/// Terminal runtime status class emitted at shutdown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeTerminalKind {
    /// Run completed with a final response.
    FinalResponse,
    /// Run stopped without a final response.
    Stopped,
    /// Run exhausted its configured turn budget.
    MaxTurnsExceeded,
    /// Run failed closed.
    Failed,
}

impl RuntimeTerminalKind {
    /// Returns the stable terminal status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalResponse => "final_response",
            Self::Stopped => "stopped",
            Self::MaxTurnsExceeded => "max_turns_exceeded",
            Self::Failed => "failed",
        }
    }

    pub(crate) fn from_status(status: &crate::RuntimeStatus) -> Self {
        match status {
            crate::RuntimeStatus::FinalResponse { .. } => Self::FinalResponse,
            crate::RuntimeStatus::Stopped => Self::Stopped,
            crate::RuntimeStatus::MaxTurnsExceeded { .. } => Self::MaxTurnsExceeded,
            crate::RuntimeStatus::Failed { .. } => Self::Failed,
        }
    }
}

/// Stable runtime event kind for headless consumers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeEventKind {
    /// Runtime start event.
    RuntimeStarted,
    /// Compatibility loop-start event.
    LoopStarted,
    /// Context replay event.
    ContextReplayed,
    /// Turn start event.
    TurnStarted,
    /// Loop decision event.
    LoopDecision,
    /// Model request event.
    ModelRequested,
    /// Model response event.
    ModelResponded,
    /// Model denial event.
    ModelDenied,
    /// Tool request event.
    ToolRequested,
    /// Tool response event.
    ToolResponded,
    /// Tool denial event.
    ToolDenied,
    /// Final response event.
    FinalResponse,
    /// Stop event.
    Stopped,
    /// Max-turn exhaustion event.
    MaxTurnsExceeded,
    /// Adapter error event.
    AdapterError,
    /// Fabric error event.
    FabricError,
    /// Hook block event.
    HookBlocked,
    /// Hook failure event.
    HookFailed,
    /// Steel policy denial event.
    SteelPolicyDenied,
    /// Cancellation event.
    Cancelled,
    /// Compaction event.
    CompactionRequested,
    /// Shutdown event.
    Shutdown,
}

impl RuntimeEventKind {
    /// Returns the stable event-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeStarted => "runtime_started",
            Self::LoopStarted => "loop_started",
            Self::ContextReplayed => "context_replayed",
            Self::TurnStarted => "turn_started",
            Self::LoopDecision => "loop_decision",
            Self::ModelRequested => "model_requested",
            Self::ModelResponded => "model_responded",
            Self::ModelDenied => "model_denied",
            Self::ToolRequested => "tool_requested",
            Self::ToolResponded => "tool_responded",
            Self::ToolDenied => "tool_denied",
            Self::FinalResponse => "final_response",
            Self::Stopped => "stopped",
            Self::MaxTurnsExceeded => "max_turns_exceeded",
            Self::AdapterError => "adapter_error",
            Self::FabricError => "fabric_error",
            Self::HookBlocked => "hook_blocked",
            Self::HookFailed => "hook_failed",
            Self::SteelPolicyDenied => "steel_policy_denied",
            Self::Cancelled => "cancelled",
            Self::CompactionRequested => "compaction_requested",
            Self::Shutdown => "shutdown",
        }
    }
}
