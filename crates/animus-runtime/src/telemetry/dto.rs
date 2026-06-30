/// Ordered runtime event emitted by the agent loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeEvent {
    /// Runtime started for a validated runtime input summary.
    RuntimeStarted {
        /// Deterministic input summary made available to the runtime.
        summary: String,
    },
    /// The loop started for a validated runtime input summary.
    LoopStarted {
        /// Deterministic input summary made available to the loop fabric.
        summary: String,
    },
    /// Context replay boundary was observed.
    ContextReplayed {
        /// Deterministic context summary for headless replay consumers.
        summary: String,
    },
    /// A loop-fabric turn started.
    TurnStarted {
        /// Zero-based deterministic turn index.
        turn_index: usize,
    },
    /// The loop fabric selected a decision for the current turn.
    LoopDecision {
        /// Zero-based deterministic turn index.
        turn_index: usize,
        /// Decision class selected by the fabric.
        decision: super::LoopDecisionKind,
    },
    /// A validated model request is about to cross the model adapter boundary.
    ModelRequested {
        /// Validated model request.
        request: animus_adapters::ModelRequest,
    },
    /// The model adapter returned a validated response.
    ModelResponded {
        /// Validated model response.
        response: animus_adapters::ModelResponse,
    },
    /// A model guard denied dispatch before adapter execution.
    ModelDenied {
        /// Denied model request.
        request: animus_adapters::ModelRequest,
        /// Safe denial reason.
        reason: String,
    },
    /// A validated tool request is about to cross the tool adapter boundary.
    ToolRequested {
        /// Validated tool request.
        request: animus_adapters::ToolRequest,
    },
    /// The tool adapter returned a validated response.
    ToolResponded {
        /// Validated tool response.
        response: animus_adapters::ToolResponse,
    },
    /// A tool guard denied dispatch before adapter execution.
    ToolDenied {
        /// Denied tool request.
        request: animus_adapters::ToolRequest,
        /// Safe denial reason.
        reason: String,
    },
    /// The loop fabric selected a final response.
    FinalResponse {
        /// Validated final model response.
        response: animus_adapters::ModelResponse,
    },
    /// The loop fabric selected a stop decision without a final response.
    Stopped,
    /// The configured maximum-turn limit was reached before completion.
    MaxTurnsExceeded {
        /// Configured maximum number of fabric turns.
        max_turns: usize,
    },
    /// An adapter returned an error and the runtime halted.
    AdapterError {
        /// Adapter component that failed.
        component: super::AdapterComponent,
        /// Human-readable adapter error message.
        message: String,
    },
    /// The loop fabric returned an error and the runtime halted.
    FabricError {
        /// Human-readable fabric error message.
        message: String,
    },
    /// Host middleware blocked a guarded boundary.
    HookBlocked {
        /// Boundary that was blocked.
        boundary: crate::HookBoundary,
        /// Middleware hook label.
        hook: String,
        /// Safe block reason.
        reason: String,
    },
    /// Host middleware failed at a guarded boundary.
    HookFailed {
        /// Boundary that failed.
        boundary: crate::HookBoundary,
        /// Middleware hook label.
        hook: String,
        /// Safe failure message.
        message: String,
    },
    /// Steel policy denied activation before Steel evaluation.
    SteelPolicyDenied {
        /// Safe selected profile label.
        profile: String,
        /// BLAKE3 hash of observed script source.
        script_hash: String,
        /// BLAKE3 hash of safe receipt fields.
        receipt_hash: String,
        /// Typed denial reason label.
        reason: String,
    },
    /// A host cancellation boundary was observed.
    Cancelled {
        /// Safe cancellation reason.
        reason: String,
    },
    /// A compaction boundary was requested.
    CompactionRequested {
        /// Safe compaction summary.
        summary: String,
    },
    /// Runtime completed and emitted a terminal status class.
    Shutdown {
        /// Terminal status class.
        status: super::RuntimeTerminalKind,
    },
}

impl RuntimeEvent {
    /// Returns the stable kind for this event.
    #[must_use]
    pub fn kind(&self) -> super::RuntimeEventKind {
        match self {
            Self::RuntimeStarted { .. } => super::RuntimeEventKind::RuntimeStarted,
            Self::LoopStarted { .. } => super::RuntimeEventKind::LoopStarted,
            Self::ContextReplayed { .. } => super::RuntimeEventKind::ContextReplayed,
            Self::TurnStarted { .. } => super::RuntimeEventKind::TurnStarted,
            Self::LoopDecision { .. } => super::RuntimeEventKind::LoopDecision,
            Self::ModelRequested { .. } => super::RuntimeEventKind::ModelRequested,
            Self::ModelResponded { .. } => super::RuntimeEventKind::ModelResponded,
            Self::ModelDenied { .. } => super::RuntimeEventKind::ModelDenied,
            Self::ToolRequested { .. } => super::RuntimeEventKind::ToolRequested,
            Self::ToolResponded { .. } => super::RuntimeEventKind::ToolResponded,
            Self::ToolDenied { .. } => super::RuntimeEventKind::ToolDenied,
            Self::FinalResponse { .. } => super::RuntimeEventKind::FinalResponse,
            Self::Stopped => super::RuntimeEventKind::Stopped,
            Self::MaxTurnsExceeded { .. } => super::RuntimeEventKind::MaxTurnsExceeded,
            Self::AdapterError { .. } => super::RuntimeEventKind::AdapterError,
            Self::FabricError { .. } => super::RuntimeEventKind::FabricError,
            Self::HookBlocked { .. } => super::RuntimeEventKind::HookBlocked,
            Self::HookFailed { .. } => super::RuntimeEventKind::HookFailed,
            Self::SteelPolicyDenied { .. } => super::RuntimeEventKind::SteelPolicyDenied,
            Self::Cancelled { .. } => super::RuntimeEventKind::Cancelled,
            Self::CompactionRequested { .. } => super::RuntimeEventKind::CompactionRequested,
            Self::Shutdown { .. } => super::RuntimeEventKind::Shutdown,
        }
    }
}
