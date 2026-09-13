/// Terminal classification for one parsed provider stream.
///
/// `octet::sealed_enum`: this closed classification covers success, cancellation,
/// and failure. New provider terminal events must map to these outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiStreamTerminal {
    /// The provider completed normally.
    Completed,
    /// The caller cancelled parsing.
    Cancelled,
    /// The provider emitted a failure or incomplete terminal event.
    Failed,
}

/// Safe stream progress for existing provider retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiRetryStreamProgress {
    /// Transport started but no semantic frame was emitted.
    TransportStarted,
    /// Assistant content was emitted.
    Content,
    /// Provider-exposed reasoning summary was emitted.
    Reasoning,
    /// A function-call delta was emitted.
    ToolCall,
    /// Usage or terminal metadata was emitted without earlier semantic text.
    Other,
}

/// Bounded result from the production `OpenAI` Responses SSE parser.
#[derive(Clone)]
pub struct OpenAiStreamOutcome {
    pub(super) provider_response_id: String,
    pub(super) frames: Vec<animus_adapters::ModelStreamFrame>,
    pub(super) tool_calls: Vec<crate::OpenAiCompletedToolCall>,
    pub(super) terminal: OpenAiStreamTerminal,
    pub(super) retry_progress: OpenAiRetryStreamProgress,
}

impl OpenAiStreamOutcome {
    /// Returns the host-local provider response identity.
    #[must_use]
    pub fn provider_response_id(&self) -> &str {
        self.provider_response_id.as_str()
    }

    /// Returns ordered canonical model stream frames.
    #[must_use]
    pub fn frames(&self) -> &[animus_adapters::ModelStreamFrame] {
        self.frames.as_slice()
    }

    /// Returns completed correlated provider function calls.
    #[must_use]
    pub fn tool_calls(&self) -> &[crate::OpenAiCompletedToolCall] {
        self.tool_calls.as_slice()
    }

    /// Returns the terminal stream classification.
    #[must_use]
    pub const fn terminal(&self) -> OpenAiStreamTerminal {
        self.terminal
    }

    /// Returns the safe retry progress classification.
    #[must_use]
    pub const fn retry_progress(&self) -> OpenAiRetryStreamProgress {
        self.retry_progress
    }

    /// Reduces accepted frames into the existing runtime replay-safety state.
    #[must_use]
    pub fn retry_stream_state(&self) -> animus_runtime::provider_retry::ProviderRetryStreamState {
        self.frames.iter().fold(
            animus_runtime::provider_retry::ProviderRetryStreamState::TransportStarted,
            |state, frame| {
                animus_runtime::provider_retry::observe_provider_retry_stream(
                    state,
                    animus_runtime::provider_retry::ProviderRetryStreamObservation::Semantic(
                        semantic_kind(frame.kind()),
                    ),
                )
            },
        )
    }
}

const fn semantic_kind(
    kind: animus_adapters::ModelStreamFrameKind,
) -> animus_runtime::provider_retry::ProviderRetrySemanticFrame {
    match kind {
        animus_adapters::ModelStreamFrameKind::ContentDelta => {
            animus_runtime::provider_retry::ProviderRetrySemanticFrame::Content
        }
        animus_adapters::ModelStreamFrameKind::ThoughtDelta => {
            animus_runtime::provider_retry::ProviderRetrySemanticFrame::Reasoning
        }
        animus_adapters::ModelStreamFrameKind::ToolCallDelta => {
            animus_runtime::provider_retry::ProviderRetrySemanticFrame::ToolCall
        }
        animus_adapters::ModelStreamFrameKind::Usage
        | animus_adapters::ModelStreamFrameKind::Finish
        | animus_adapters::ModelStreamFrameKind::Cancelled
        | animus_adapters::ModelStreamFrameKind::Failure => {
            animus_runtime::provider_retry::ProviderRetrySemanticFrame::Other
        }
    }
}

/// Renders metadata-only evidence for one tool-capable stream outcome.
#[must_use]
pub fn render_openai_tool_stream_evidence(outcome: &OpenAiStreamOutcome) -> String {
    let terminal = match outcome.terminal {
        OpenAiStreamTerminal::Completed => "completed",
        OpenAiStreamTerminal::Cancelled => "cancelled",
        OpenAiStreamTerminal::Failed => "failed",
    };
    let provider_response_identity = blake3::hash(outcome.provider_response_id.as_bytes());
    format!(
        "LIVE_OPENAI_TOOL_STREAM status={terminal}\nframe_count={}\ntool_call_count={}\nprovider_response_b3={}\nraw_response=omitted\ntool_arguments=omitted\ntool_output=omitted\ncredential=omitted\nreasoning=omitted",
        outcome.frames.len(),
        outcome.tool_calls.len(),
        provider_response_identity.to_hex(),
    )
}

impl std::fmt::Debug for OpenAiStreamOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiStreamOutcome")
            .field("provider_response_id", &"[REDACTED]")
            .field("frame_count", &self.frames.len())
            .field("tool_call_count", &self.tool_calls.len())
            .field("terminal", &self.terminal)
            .field("retry_progress", &self.retry_progress)
            .finish()
    }
}
