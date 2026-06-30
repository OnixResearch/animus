const MODEL_REQUEST_KIND: &str = "model_request";
const MODEL_RESPONSE_KIND: &str = "model_response";
const TOOL_REQUEST_KIND: &str = "tool_request";
const TOOL_RESPONSE_KIND: &str = "tool_response";
const RUNTIME_EVENT_KIND: &str = "runtime_event";
const RUNTIME_REPORT_KIND: &str = "runtime_report";
const RUNTIME_COMPONENT_FIELD: &str = "component";
const RUNTIME_BOUNDARY_FIELD: &str = "boundary";
const RUNTIME_DECISION_FIELD: &str = "decision";
const RUNTIME_TERMINAL_FIELD: &str = "terminal";
const RUNTIME_MAX_TURNS_FIELD: &str = "max_turns";
const RUNTIME_MAX_TURNS_ZERO_REASON: &str = "max turns must be at least one";
const MODEL_COMPONENT: &str = "model";
const TOOL_COMPONENT: &str = "tool";
const BOUNDARY_OBSERVE: &str = "observe";
const BOUNDARY_MODEL_DISPATCH: &str = "model_dispatch";
const BOUNDARY_TOOL_DISPATCH: &str = "tool_dispatch";
const BOUNDARY_COMPACTION: &str = "compaction";
const BOUNDARY_REPLAY: &str = "replay";
const BOUNDARY_SESSION_REPLACEMENT: &str = "session_replacement";
const DECISION_MODEL: &str = "model";
const DECISION_TOOL: &str = "tool";
const DECISION_RESPOND: &str = "respond";
const DECISION_STOP: &str = "stop";
const TERMINAL_FINAL_RESPONSE: &str = "final_response";
const TERMINAL_STOPPED: &str = "stopped";
const TERMINAL_MAX_TURNS_EXCEEDED: &str = "max_turns_exceeded";
const TERMINAL_FAILED: &str = "failed";
const UNKNOWN_COMPONENT_REASON: &str = "unknown runtime adapter component";
const UNKNOWN_BOUNDARY_REASON: &str = "unknown runtime hook boundary";
const UNKNOWN_DECISION_REASON: &str = "unknown runtime loop decision";
const UNKNOWN_TERMINAL_REASON: &str = "unknown runtime terminal status";

/// Boundary message kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Model request boundary payload.
    ModelRequest,
    /// Model response boundary payload.
    ModelResponse,
    /// Tool request boundary payload.
    ToolRequest,
    /// Tool response boundary payload.
    ToolResponse,
    /// Runtime event boundary payload.
    RuntimeEvent,
    /// Runtime report boundary payload.
    RuntimeReport,
}

impl Kind {
    /// Returns a stable string identifier for the message kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelRequest => MODEL_REQUEST_KIND,
            Self::ModelResponse => MODEL_RESPONSE_KIND,
            Self::ToolRequest => TOOL_REQUEST_KIND,
            Self::ToolResponse => TOOL_RESPONSE_KIND,
            Self::RuntimeEvent => RUNTIME_EVENT_KIND,
            Self::RuntimeReport => RUNTIME_REPORT_KIND,
        }
    }
}

/// Runtime failure payload encoded inside a runtime report status.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RuntimeFailureEnvelope {
    /// Loop fabric failed.
    Fabric {
        /// Human-readable failure message.
        message: String,
    },
    /// Model adapter failed.
    ModelAdapter {
        /// Human-readable failure message.
        message: String,
    },
    /// Tool adapter failed.
    ToolAdapter {
        /// Human-readable failure message.
        message: String,
    },
    /// Steel policy denied activation before fabric evaluation.
    SteelPolicy {
        /// Human-readable failure message.
        message: String,
    },
    /// Host middleware blocked or failed a guarded boundary.
    Middleware {
        /// Guarded boundary label.
        boundary: String,
        /// Human-readable failure message.
        message: String,
    },
}

impl RuntimeFailureEnvelope {
    fn from_runtime_failure(failure: &animus_runtime::RuntimeFailure) -> Self {
        match failure {
            animus_runtime::RuntimeFailure::Fabric { message } => Self::Fabric {
                message: message.clone(),
            },
            animus_runtime::RuntimeFailure::ModelAdapter { message } => Self::ModelAdapter {
                message: message.clone(),
            },
            animus_runtime::RuntimeFailure::ToolAdapter { message } => Self::ToolAdapter {
                message: message.clone(),
            },
            animus_runtime::RuntimeFailure::SteelPolicy { message } => Self::SteelPolicy {
                message: message.clone(),
            },
            animus_runtime::RuntimeFailure::Middleware { boundary, message } => Self::Middleware {
                boundary: boundary_to_string(*boundary),
                message: message.clone(),
            },
        }
    }

    fn into_runtime_failure(self) -> Result<animus_runtime::RuntimeFailure, crate::CodecError> {
        match self {
            Self::Fabric { message } => Ok(animus_runtime::RuntimeFailure::Fabric { message }),
            Self::ModelAdapter { message } => {
                Ok(animus_runtime::RuntimeFailure::ModelAdapter { message })
            }
            Self::ToolAdapter { message } => {
                Ok(animus_runtime::RuntimeFailure::ToolAdapter { message })
            }
            Self::SteelPolicy { message } => {
                Ok(animus_runtime::RuntimeFailure::SteelPolicy { message })
            }
            Self::Middleware { boundary, message } => {
                Ok(animus_runtime::RuntimeFailure::Middleware {
                    boundary: boundary_from_str(&boundary)?,
                    message,
                })
            }
        }
    }
}

/// Runtime report status payload.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RuntimeReportStatus {
    /// Runtime produced a final response.
    FinalResponse {
        /// Final response message.
        message: String,
    },
    /// Runtime stopped without a final response.
    Stopped,
    /// Runtime exhausted its max-turn limit.
    MaxTurnsExceeded {
        /// Configured maximum number of fabric turns.
        max_turns: usize,
    },
    /// Runtime failed through a typed boundary.
    Failed {
        /// Failure payload.
        failure: RuntimeFailureEnvelope,
    },
}

impl RuntimeReportStatus {
    fn from_runtime_status(status: &animus_runtime::RuntimeStatus) -> Self {
        match status {
            animus_runtime::RuntimeStatus::FinalResponse { response } => Self::FinalResponse {
                message: String::from(response.message()),
            },
            animus_runtime::RuntimeStatus::Stopped => Self::Stopped,
            animus_runtime::RuntimeStatus::MaxTurnsExceeded { max_turns } => {
                Self::MaxTurnsExceeded {
                    max_turns: *max_turns,
                }
            }
            animus_runtime::RuntimeStatus::Failed { failure } => Self::Failed {
                failure: RuntimeFailureEnvelope::from_runtime_failure(failure),
            },
        }
    }

    fn into_runtime_status(self) -> Result<animus_runtime::RuntimeStatus, crate::CodecError> {
        match self {
            Self::FinalResponse { message } => Ok(animus_runtime::RuntimeStatus::FinalResponse {
                response: animus_adapters::ModelResponse::new(&message)?,
            }),
            Self::Stopped => Ok(animus_runtime::RuntimeStatus::Stopped),
            Self::MaxTurnsExceeded { max_turns } => {
                let max_turns = max_turns_from_count(max_turns)?;
                Ok(animus_runtime::RuntimeStatus::MaxTurnsExceeded {
                    max_turns: max_turns.get(),
                })
            }
            Self::Failed { failure } => Ok(animus_runtime::RuntimeStatus::Failed {
                failure: failure.into_runtime_failure()?,
            }),
        }
    }
}

/// Typed Preserves boundary message.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Envelope {
    /// Model request boundary payload.
    ModelRequest {
        /// Prompt sent to a model adapter.
        prompt: String,
    },
    /// Model response boundary payload.
    ModelResponse {
        /// Message returned by a model adapter.
        message: String,
    },
    /// Tool request boundary payload.
    ToolRequest {
        /// Tool name selected by the harness.
        name: String,
        /// Tool input payload.
        input: String,
    },
    /// Tool response boundary payload.
    ToolResponse {
        /// Tool output payload.
        output: String,
    },
    /// Runtime-started event payload.
    RuntimeStarted {
        /// Runtime input summary.
        summary: String,
    },
    /// Runtime context-replayed event payload.
    RuntimeContextReplayed {
        /// Runtime context summary.
        summary: String,
    },
    /// Runtime loop-started event payload.
    RuntimeLoopStarted {
        /// Runtime input summary.
        summary: String,
    },
    /// Runtime turn-started event payload.
    RuntimeTurnStarted {
        /// Zero-based turn index.
        turn_index: usize,
    },
    /// Runtime loop-decision event payload.
    RuntimeLoopDecision {
        /// Zero-based turn index.
        turn_index: usize,
        /// Stable decision label.
        decision: String,
    },
    /// Runtime model-requested event payload.
    RuntimeModelRequested {
        /// Model prompt.
        prompt: String,
    },
    /// Runtime model-responded event payload.
    RuntimeModelResponded {
        /// Model response message.
        message: String,
    },
    /// Runtime model-denied event payload.
    RuntimeModelDenied {
        /// Denied model prompt.
        prompt: String,
        /// Safe denial reason.
        reason: String,
    },
    /// Runtime tool-requested event payload.
    RuntimeToolRequested {
        /// Tool name.
        name: String,
        /// Tool input.
        input: String,
    },
    /// Runtime tool-responded event payload.
    RuntimeToolResponded {
        /// Tool output.
        output: String,
    },
    /// Runtime tool-denied event payload.
    RuntimeToolDenied {
        /// Denied tool name.
        name: String,
        /// Denied tool input.
        input: String,
        /// Safe denial reason.
        reason: String,
    },
    /// Runtime final-response event payload.
    RuntimeFinalResponse {
        /// Final response message.
        message: String,
    },
    /// Runtime stopped event payload.
    RuntimeStopped,
    /// Runtime max-turn exhaustion event payload.
    RuntimeMaxTurnsExceeded {
        /// Configured maximum number of fabric turns.
        max_turns: usize,
    },
    /// Runtime adapter-error event payload.
    RuntimeAdapterError {
        /// Adapter component name.
        component: String,
        /// Human-readable adapter error message.
        message: String,
    },
    /// Runtime fabric-error event payload.
    RuntimeFabricError {
        /// Human-readable fabric error message.
        message: String,
    },
    /// Runtime hook-blocked event payload.
    RuntimeHookBlocked {
        /// Guarded boundary label.
        boundary: String,
        /// Hook label.
        hook: String,
        /// Safe block reason.
        reason: String,
    },
    /// Runtime hook-failed event payload.
    RuntimeHookFailed {
        /// Guarded boundary label.
        boundary: String,
        /// Hook label.
        hook: String,
        /// Safe failure message.
        message: String,
    },
    /// Runtime Steel policy-denied event payload.
    RuntimeSteelPolicyDenied {
        /// Safe selected Steel profile label.
        profile: String,
        /// BLAKE3 hash of observed script source.
        script_hash: String,
        /// BLAKE3 hash of safe receipt fields.
        receipt_hash: String,
        /// Stable denial reason label.
        reason: String,
    },
    /// Runtime cancellation event payload.
    RuntimeCancelled {
        /// Safe cancellation reason.
        reason: String,
    },
    /// Runtime compaction-requested event payload.
    RuntimeCompactionRequested {
        /// Safe compaction summary.
        summary: String,
    },
    /// Runtime shutdown event payload.
    RuntimeShutdown {
        /// Stable terminal status label.
        status: String,
    },
    /// Runtime report payload.
    RuntimeReport {
        /// Final runtime status.
        status: RuntimeReportStatus,
        /// Ordered runtime event envelopes.
        events: Vec<Envelope>,
    },
}

impl Envelope {
    /// Returns the boundary kind for this message.
    #[must_use]
    pub const fn kind(&self) -> Kind {
        match self {
            Self::ModelRequest { .. } => Kind::ModelRequest,
            Self::ModelResponse { .. } => Kind::ModelResponse,
            Self::ToolRequest { .. } => Kind::ToolRequest,
            Self::ToolResponse { .. } => Kind::ToolResponse,
            Self::RuntimeStarted { .. }
            | Self::RuntimeContextReplayed { .. }
            | Self::RuntimeLoopStarted { .. }
            | Self::RuntimeTurnStarted { .. }
            | Self::RuntimeLoopDecision { .. }
            | Self::RuntimeModelRequested { .. }
            | Self::RuntimeModelResponded { .. }
            | Self::RuntimeModelDenied { .. }
            | Self::RuntimeToolRequested { .. }
            | Self::RuntimeToolResponded { .. }
            | Self::RuntimeToolDenied { .. }
            | Self::RuntimeFinalResponse { .. }
            | Self::RuntimeStopped
            | Self::RuntimeMaxTurnsExceeded { .. }
            | Self::RuntimeAdapterError { .. }
            | Self::RuntimeFabricError { .. }
            | Self::RuntimeHookBlocked { .. }
            | Self::RuntimeHookFailed { .. }
            | Self::RuntimeSteelPolicyDenied { .. }
            | Self::RuntimeCancelled { .. }
            | Self::RuntimeCompactionRequested { .. }
            | Self::RuntimeShutdown { .. } => Kind::RuntimeEvent,
            Self::RuntimeReport { .. } => Kind::RuntimeReport,
        }
    }

    /// Builds a boundary message from a validated model request.
    #[must_use]
    pub fn from_model_request(request: &animus_adapters::ModelRequest) -> Self {
        Self::ModelRequest {
            prompt: String::from(request.prompt()),
        }
    }

    /// Builds a boundary message from a validated model response.
    #[must_use]
    pub fn from_model_response(response: &animus_adapters::ModelResponse) -> Self {
        Self::ModelResponse {
            message: String::from(response.message()),
        }
    }

    /// Builds a boundary message from a validated tool request.
    #[must_use]
    pub fn from_tool_request(request: &animus_adapters::ToolRequest) -> Self {
        Self::ToolRequest {
            name: String::from(request.name()),
            input: String::from(request.input()),
        }
    }

    /// Builds a boundary message from a validated tool response.
    #[must_use]
    pub fn from_tool_response(response: &animus_adapters::ToolResponse) -> Self {
        Self::ToolResponse {
            output: String::from(response.output()),
        }
    }

    /// Builds a boundary message from a runtime event.
    #[must_use]
    pub fn from_runtime_event(event: &animus_runtime::RuntimeEvent) -> Self {
        match runtime_event_group(event) {
            RuntimeEventGroup::Lifecycle => encode_lifecycle_event(event),
            RuntimeEventGroup::Adapter => encode_adapter_event(event),
            RuntimeEventGroup::Completion => encode_completion_event(event),
            RuntimeEventGroup::Failure => encode_failure_event(event),
        }
    }

    /// Builds a boundary message from a runtime report.
    #[must_use]
    pub fn from_runtime_report(report: &animus_runtime::RunReport) -> Self {
        Self::RuntimeReport {
            status: RuntimeReportStatus::from_runtime_status(report.status()),
            events: report
                .events()
                .iter()
                .map(Self::from_runtime_event)
                .collect(),
        }
    }

    /// Converts this decoded message into a validated model request.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_model_request(
        self,
    ) -> Result<animus_adapters::ModelRequest, crate::CodecError> {
        match self {
            Self::ModelRequest { prompt } => Ok(animus_adapters::ModelRequest::new(&prompt)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ModelRequest,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated model response.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_model_response(
        self,
    ) -> Result<animus_adapters::ModelResponse, crate::CodecError> {
        match self {
            Self::ModelResponse { message } => Ok(animus_adapters::ModelResponse::new(&message)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ModelResponse,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated tool request.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_tool_request(self) -> Result<animus_adapters::ToolRequest, crate::CodecError> {
        match self {
            Self::ToolRequest { name, input } => {
                Ok(animus_adapters::ToolRequest::new(&name, &input)?)
            }
            other => Err(crate::CodecError::wrong_kind(
                Kind::ToolRequest,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated tool response.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_tool_response(
        self,
    ) -> Result<animus_adapters::ToolResponse, crate::CodecError> {
        match self {
            Self::ToolResponse { output } => Ok(animus_adapters::ToolResponse::new(&output)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ToolResponse,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a runtime event.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload fields violate adapter/runtime contracts.
    pub fn try_into_runtime_event(self) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
        let actual = self.kind();
        let Some(group) = runtime_envelope_group(&self) else {
            return Err(crate::CodecError::wrong_kind(Kind::RuntimeEvent, actual));
        };

        match group {
            RuntimeEventGroup::Lifecycle => decode_lifecycle_event(self),
            RuntimeEventGroup::Adapter => decode_adapter_event(self),
            RuntimeEventGroup::Completion => decode_completion_event(self),
            RuntimeEventGroup::Failure => decode_failure_event(self),
        }
    }

    /// Converts this decoded message into a runtime report.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or any
    /// decoded status/event payload violates adapter/runtime contracts.
    pub fn try_into_runtime_report(self) -> Result<animus_runtime::RunReport, crate::CodecError> {
        match self {
            Self::RuntimeReport { status, events } => {
                let mut decoded_events = Vec::with_capacity(events.len());
                for event in events {
                    decoded_events.push(event.try_into_runtime_event()?);
                }
                Ok(animus_runtime::RunReport::new(
                    decoded_events,
                    status.into_runtime_status()?,
                ))
            }
            other => Err(crate::CodecError::wrong_kind(
                Kind::RuntimeReport,
                other.kind(),
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RuntimeEventGroup {
    Lifecycle,
    Adapter,
    Completion,
    Failure,
}

fn runtime_event_group(event: &animus_runtime::RuntimeEvent) -> RuntimeEventGroup {
    match event {
        animus_runtime::RuntimeEvent::RuntimeStarted { .. }
        | animus_runtime::RuntimeEvent::ContextReplayed { .. }
        | animus_runtime::RuntimeEvent::LoopStarted { .. }
        | animus_runtime::RuntimeEvent::TurnStarted { .. }
        | animus_runtime::RuntimeEvent::LoopDecision { .. } => RuntimeEventGroup::Lifecycle,
        animus_runtime::RuntimeEvent::ModelRequested { .. }
        | animus_runtime::RuntimeEvent::ModelResponded { .. }
        | animus_runtime::RuntimeEvent::ModelDenied { .. }
        | animus_runtime::RuntimeEvent::ToolRequested { .. }
        | animus_runtime::RuntimeEvent::ToolResponded { .. }
        | animus_runtime::RuntimeEvent::ToolDenied { .. } => RuntimeEventGroup::Adapter,
        animus_runtime::RuntimeEvent::FinalResponse { .. }
        | animus_runtime::RuntimeEvent::Stopped
        | animus_runtime::RuntimeEvent::MaxTurnsExceeded { .. }
        | animus_runtime::RuntimeEvent::Shutdown { .. } => RuntimeEventGroup::Completion,
        animus_runtime::RuntimeEvent::AdapterError { .. }
        | animus_runtime::RuntimeEvent::FabricError { .. }
        | animus_runtime::RuntimeEvent::HookBlocked { .. }
        | animus_runtime::RuntimeEvent::HookFailed { .. }
        | animus_runtime::RuntimeEvent::SteelPolicyDenied { .. }
        | animus_runtime::RuntimeEvent::Cancelled { .. }
        | animus_runtime::RuntimeEvent::CompactionRequested { .. } => RuntimeEventGroup::Failure,
    }
}

fn runtime_envelope_group(envelope: &Envelope) -> Option<RuntimeEventGroup> {
    match envelope {
        Envelope::RuntimeStarted { .. }
        | Envelope::RuntimeContextReplayed { .. }
        | Envelope::RuntimeLoopStarted { .. }
        | Envelope::RuntimeTurnStarted { .. }
        | Envelope::RuntimeLoopDecision { .. } => Some(RuntimeEventGroup::Lifecycle),
        Envelope::RuntimeModelRequested { .. }
        | Envelope::RuntimeModelResponded { .. }
        | Envelope::RuntimeModelDenied { .. }
        | Envelope::RuntimeToolRequested { .. }
        | Envelope::RuntimeToolResponded { .. }
        | Envelope::RuntimeToolDenied { .. } => Some(RuntimeEventGroup::Adapter),
        Envelope::RuntimeFinalResponse { .. }
        | Envelope::RuntimeStopped
        | Envelope::RuntimeMaxTurnsExceeded { .. }
        | Envelope::RuntimeShutdown { .. } => Some(RuntimeEventGroup::Completion),
        Envelope::RuntimeAdapterError { .. }
        | Envelope::RuntimeFabricError { .. }
        | Envelope::RuntimeHookBlocked { .. }
        | Envelope::RuntimeHookFailed { .. }
        | Envelope::RuntimeSteelPolicyDenied { .. }
        | Envelope::RuntimeCancelled { .. }
        | Envelope::RuntimeCompactionRequested { .. } => Some(RuntimeEventGroup::Failure),
        Envelope::ModelRequest { .. }
        | Envelope::ModelResponse { .. }
        | Envelope::ToolRequest { .. }
        | Envelope::ToolResponse { .. }
        | Envelope::RuntimeReport { .. } => None,
    }
}

fn encode_lifecycle_event(event: &animus_runtime::RuntimeEvent) -> Envelope {
    match event {
        animus_runtime::RuntimeEvent::RuntimeStarted { summary } => Envelope::RuntimeStarted {
            summary: summary.clone(),
        },
        animus_runtime::RuntimeEvent::ContextReplayed { summary } => {
            Envelope::RuntimeContextReplayed {
                summary: summary.clone(),
            }
        }
        animus_runtime::RuntimeEvent::LoopStarted { summary } => Envelope::RuntimeLoopStarted {
            summary: summary.clone(),
        },
        animus_runtime::RuntimeEvent::TurnStarted { turn_index } => Envelope::RuntimeTurnStarted {
            turn_index: *turn_index,
        },
        animus_runtime::RuntimeEvent::LoopDecision {
            turn_index,
            decision,
        } => Envelope::RuntimeLoopDecision {
            turn_index: *turn_index,
            decision: decision_to_string(*decision),
        },
        _other => unreachable!("runtime event group mismatch"),
    }
}

fn encode_adapter_event(event: &animus_runtime::RuntimeEvent) -> Envelope {
    match event {
        animus_runtime::RuntimeEvent::ModelRequested { request } => {
            Envelope::RuntimeModelRequested {
                prompt: String::from(request.prompt()),
            }
        }
        animus_runtime::RuntimeEvent::ModelResponded { response } => {
            Envelope::RuntimeModelResponded {
                message: String::from(response.message()),
            }
        }
        animus_runtime::RuntimeEvent::ModelDenied { request, reason } => {
            Envelope::RuntimeModelDenied {
                prompt: String::from(request.prompt()),
                reason: reason.clone(),
            }
        }
        animus_runtime::RuntimeEvent::ToolRequested { request } => Envelope::RuntimeToolRequested {
            name: String::from(request.name()),
            input: String::from(request.input()),
        },
        animus_runtime::RuntimeEvent::ToolResponded { response } => {
            Envelope::RuntimeToolResponded {
                output: String::from(response.output()),
            }
        }
        animus_runtime::RuntimeEvent::ToolDenied { request, reason } => {
            Envelope::RuntimeToolDenied {
                name: String::from(request.name()),
                input: String::from(request.input()),
                reason: reason.clone(),
            }
        }
        _other => unreachable!("runtime event group mismatch"),
    }
}

fn encode_completion_event(event: &animus_runtime::RuntimeEvent) -> Envelope {
    match event {
        animus_runtime::RuntimeEvent::FinalResponse { response } => {
            Envelope::RuntimeFinalResponse {
                message: String::from(response.message()),
            }
        }
        animus_runtime::RuntimeEvent::Stopped => Envelope::RuntimeStopped,
        animus_runtime::RuntimeEvent::MaxTurnsExceeded { max_turns } => {
            Envelope::RuntimeMaxTurnsExceeded {
                max_turns: *max_turns,
            }
        }
        animus_runtime::RuntimeEvent::Shutdown { status } => Envelope::RuntimeShutdown {
            status: terminal_to_string(*status),
        },
        _other => unreachable!("runtime event group mismatch"),
    }
}

fn encode_failure_event(event: &animus_runtime::RuntimeEvent) -> Envelope {
    match event {
        animus_runtime::RuntimeEvent::AdapterError { component, message } => {
            Envelope::RuntimeAdapterError {
                component: component_to_string(*component),
                message: message.clone(),
            }
        }
        animus_runtime::RuntimeEvent::FabricError { message } => Envelope::RuntimeFabricError {
            message: message.clone(),
        },
        animus_runtime::RuntimeEvent::HookBlocked {
            boundary,
            hook,
            reason,
        } => Envelope::RuntimeHookBlocked {
            boundary: boundary_to_string(*boundary),
            hook: hook.clone(),
            reason: reason.clone(),
        },
        animus_runtime::RuntimeEvent::HookFailed {
            boundary,
            hook,
            message,
        } => Envelope::RuntimeHookFailed {
            boundary: boundary_to_string(*boundary),
            hook: hook.clone(),
            message: message.clone(),
        },
        animus_runtime::RuntimeEvent::SteelPolicyDenied {
            profile,
            script_hash,
            receipt_hash,
            reason,
        } => Envelope::RuntimeSteelPolicyDenied {
            profile: profile.clone(),
            script_hash: script_hash.clone(),
            receipt_hash: receipt_hash.clone(),
            reason: reason.clone(),
        },
        animus_runtime::RuntimeEvent::Cancelled { reason } => Envelope::RuntimeCancelled {
            reason: reason.clone(),
        },
        animus_runtime::RuntimeEvent::CompactionRequested { summary } => {
            Envelope::RuntimeCompactionRequested {
                summary: summary.clone(),
            }
        }
        _other => unreachable!("runtime event group mismatch"),
    }
}

fn decode_lifecycle_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeStarted { summary } => {
            let input = animus_runtime::RuntimeInput::new(&summary)?;
            Ok(animus_runtime::RuntimeEvent::RuntimeStarted {
                summary: String::from(input.summary()),
            })
        }
        Envelope::RuntimeContextReplayed { summary } => {
            let input = animus_runtime::RuntimeInput::new(&summary)?;
            Ok(animus_runtime::RuntimeEvent::ContextReplayed {
                summary: String::from(input.summary()),
            })
        }
        Envelope::RuntimeLoopStarted { summary } => {
            let input = animus_runtime::RuntimeInput::new(&summary)?;
            Ok(animus_runtime::RuntimeEvent::LoopStarted {
                summary: String::from(input.summary()),
            })
        }
        Envelope::RuntimeTurnStarted { turn_index } => {
            Ok(animus_runtime::RuntimeEvent::TurnStarted { turn_index })
        }
        Envelope::RuntimeLoopDecision {
            turn_index,
            decision,
        } => Ok(animus_runtime::RuntimeEvent::LoopDecision {
            turn_index,
            decision: decision_from_str(&decision)?,
        }),
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

fn decode_adapter_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeModelRequested { prompt } => {
            Ok(animus_runtime::RuntimeEvent::ModelRequested {
                request: animus_adapters::ModelRequest::new(&prompt)?,
            })
        }
        Envelope::RuntimeModelResponded { message } => {
            Ok(animus_runtime::RuntimeEvent::ModelResponded {
                response: animus_adapters::ModelResponse::new(&message)?,
            })
        }
        Envelope::RuntimeModelDenied { prompt, reason } => {
            Ok(animus_runtime::RuntimeEvent::ModelDenied {
                request: animus_adapters::ModelRequest::new(&prompt)?,
                reason,
            })
        }
        Envelope::RuntimeToolRequested { name, input } => {
            Ok(animus_runtime::RuntimeEvent::ToolRequested {
                request: animus_adapters::ToolRequest::new(&name, &input)?,
            })
        }
        Envelope::RuntimeToolResponded { output } => {
            Ok(animus_runtime::RuntimeEvent::ToolResponded {
                response: animus_adapters::ToolResponse::new(&output)?,
            })
        }
        Envelope::RuntimeToolDenied {
            name,
            input,
            reason,
        } => Ok(animus_runtime::RuntimeEvent::ToolDenied {
            request: animus_adapters::ToolRequest::new(&name, &input)?,
            reason,
        }),
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

fn decode_completion_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeFinalResponse { message } => {
            Ok(animus_runtime::RuntimeEvent::FinalResponse {
                response: animus_adapters::ModelResponse::new(&message)?,
            })
        }
        Envelope::RuntimeStopped => Ok(animus_runtime::RuntimeEvent::Stopped),
        Envelope::RuntimeMaxTurnsExceeded { max_turns } => {
            let max_turns = max_turns_from_count(max_turns)?;
            Ok(animus_runtime::RuntimeEvent::MaxTurnsExceeded {
                max_turns: max_turns.get(),
            })
        }
        Envelope::RuntimeShutdown { status } => Ok(animus_runtime::RuntimeEvent::Shutdown {
            status: terminal_from_str(&status)?,
        }),
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

fn decode_failure_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeAdapterError { component, message } => {
            Ok(animus_runtime::RuntimeEvent::AdapterError {
                component: component_from_str(&component)?,
                message,
            })
        }
        Envelope::RuntimeFabricError { message } => {
            Ok(animus_runtime::RuntimeEvent::FabricError { message })
        }
        Envelope::RuntimeHookBlocked {
            boundary,
            hook,
            reason,
        } => Ok(animus_runtime::RuntimeEvent::HookBlocked {
            boundary: boundary_from_str(&boundary)?,
            hook,
            reason,
        }),
        Envelope::RuntimeHookFailed {
            boundary,
            hook,
            message,
        } => Ok(animus_runtime::RuntimeEvent::HookFailed {
            boundary: boundary_from_str(&boundary)?,
            hook,
            message,
        }),
        Envelope::RuntimeSteelPolicyDenied {
            profile,
            script_hash,
            receipt_hash,
            reason,
        } => Ok(animus_runtime::RuntimeEvent::SteelPolicyDenied {
            profile,
            script_hash,
            receipt_hash,
            reason,
        }),
        Envelope::RuntimeCancelled { reason } => {
            Ok(animus_runtime::RuntimeEvent::Cancelled { reason })
        }
        Envelope::RuntimeCompactionRequested { summary } => {
            let input = animus_runtime::RuntimeInput::new(&summary)?;
            Ok(animus_runtime::RuntimeEvent::CompactionRequested {
                summary: String::from(input.summary()),
            })
        }
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

fn max_turns_from_count(max_turns: usize) -> Result<core::num::NonZeroUsize, crate::CodecError> {
    core::num::NonZeroUsize::new(max_turns).ok_or(crate::CodecError::RuntimeContract {
        source: animus_runtime::RuntimeError::InvalidConfig {
            field: RUNTIME_MAX_TURNS_FIELD,
            reason: RUNTIME_MAX_TURNS_ZERO_REASON,
        },
    })
}

fn component_to_string(component: animus_runtime::AdapterComponent) -> String {
    match component {
        animus_runtime::AdapterComponent::Model => String::from(MODEL_COMPONENT),
        animus_runtime::AdapterComponent::Tool => String::from(TOOL_COMPONENT),
    }
}

fn component_from_str(
    component: &str,
) -> Result<animus_runtime::AdapterComponent, crate::CodecError> {
    match component {
        MODEL_COMPONENT => Ok(animus_runtime::AdapterComponent::Model),
        TOOL_COMPONENT => Ok(animus_runtime::AdapterComponent::Tool),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_COMPONENT_FIELD,
            reason: UNKNOWN_COMPONENT_REASON,
        }),
    }
}

fn boundary_to_string(boundary: animus_runtime::HookBoundary) -> String {
    String::from(boundary.as_str())
}

fn boundary_from_str(boundary: &str) -> Result<animus_runtime::HookBoundary, crate::CodecError> {
    match boundary {
        BOUNDARY_OBSERVE => Ok(animus_runtime::HookBoundary::Observe),
        BOUNDARY_MODEL_DISPATCH => Ok(animus_runtime::HookBoundary::ModelDispatch),
        BOUNDARY_TOOL_DISPATCH => Ok(animus_runtime::HookBoundary::ToolDispatch),
        BOUNDARY_COMPACTION => Ok(animus_runtime::HookBoundary::Compaction),
        BOUNDARY_REPLAY => Ok(animus_runtime::HookBoundary::Replay),
        BOUNDARY_SESSION_REPLACEMENT => Ok(animus_runtime::HookBoundary::SessionReplacement),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_BOUNDARY_FIELD,
            reason: UNKNOWN_BOUNDARY_REASON,
        }),
    }
}

fn decision_to_string(decision: animus_runtime::LoopDecisionKind) -> String {
    String::from(decision.as_str())
}

fn decision_from_str(
    decision: &str,
) -> Result<animus_runtime::LoopDecisionKind, crate::CodecError> {
    match decision {
        DECISION_MODEL => Ok(animus_runtime::LoopDecisionKind::Model),
        DECISION_TOOL => Ok(animus_runtime::LoopDecisionKind::Tool),
        DECISION_RESPOND => Ok(animus_runtime::LoopDecisionKind::Respond),
        DECISION_STOP => Ok(animus_runtime::LoopDecisionKind::Stop),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_DECISION_FIELD,
            reason: UNKNOWN_DECISION_REASON,
        }),
    }
}

fn terminal_to_string(status: animus_runtime::RuntimeTerminalKind) -> String {
    String::from(status.as_str())
}

fn terminal_from_str(
    status: &str,
) -> Result<animus_runtime::RuntimeTerminalKind, crate::CodecError> {
    match status {
        TERMINAL_FINAL_RESPONSE => Ok(animus_runtime::RuntimeTerminalKind::FinalResponse),
        TERMINAL_STOPPED => Ok(animus_runtime::RuntimeTerminalKind::Stopped),
        TERMINAL_MAX_TURNS_EXCEEDED => Ok(animus_runtime::RuntimeTerminalKind::MaxTurnsExceeded),
        TERMINAL_FAILED => Ok(animus_runtime::RuntimeTerminalKind::Failed),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_TERMINAL_FIELD,
            reason: UNKNOWN_TERMINAL_REASON,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const MODEL_MESSAGE: &str = "model answer";
    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const TOOL_OUTPUT: &str = "result";
    const BLANK_TEXT: &str = "   ";
    const UNKNOWN_COMPONENT: &str = "memory";
    const STEEL_PROFILE: &str = "default";
    const STEEL_SCRIPT_HASH: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const STEEL_RECEIPT_HASH: &str =
        "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";
    const STEEL_REASON: &str = "profile_mismatch";
    const STEEL_FAILURE_MESSAGE: &str = "steel policy denied: profile_mismatch";
    const MAX_TURNS: usize = 2;

    fn steel_policy_denied_event() -> animus_runtime::RuntimeEvent {
        animus_runtime::RuntimeEvent::SteelPolicyDenied {
            profile: String::from(STEEL_PROFILE),
            script_hash: String::from(STEEL_SCRIPT_HASH),
            receipt_hash: String::from(STEEL_RECEIPT_HASH),
            reason: String::from(STEEL_REASON),
        }
    }

    #[test]
    fn model_request_round_trips_through_contract() {
        let request =
            animus_adapters::ModelRequest::new(MODEL_PROMPT).expect("valid request should parse");
        let message = Envelope::from_model_request(&request);
        let decoded = message
            .try_into_model_request()
            .expect("boundary message should convert");

        assert_eq!(decoded.prompt(), MODEL_PROMPT);
    }

    #[test]
    fn model_response_round_trips_through_contract() {
        let response = animus_adapters::ModelResponse::new(MODEL_MESSAGE)
            .expect("valid response should parse");
        let message = Envelope::from_model_response(&response);
        let decoded = message
            .try_into_model_response()
            .expect("boundary message should convert");

        assert_eq!(decoded.message(), MODEL_MESSAGE);
    }

    #[test]
    fn tool_request_round_trips_through_contract() {
        let request = animus_adapters::ToolRequest::new(TOOL_NAME, TOOL_INPUT)
            .expect("valid request should parse");
        let message = Envelope::from_tool_request(&request);
        let decoded = message
            .try_into_tool_request()
            .expect("boundary message should convert");

        assert_eq!(decoded.name(), TOOL_NAME);
        assert_eq!(decoded.input(), TOOL_INPUT);
    }

    #[test]
    fn tool_response_round_trips_through_contract() {
        let response =
            animus_adapters::ToolResponse::new(TOOL_OUTPUT).expect("valid response should parse");
        let message = Envelope::from_tool_response(&response);
        let decoded = message
            .try_into_tool_response()
            .expect("boundary message should convert");

        assert_eq!(decoded.output(), TOOL_OUTPUT);
    }

    #[test]
    fn runtime_event_round_trips_through_contract() {
        let event = animus_runtime::RuntimeEvent::ToolRequested {
            request: animus_adapters::ToolRequest::new(TOOL_NAME, TOOL_INPUT)
                .expect("valid request should parse"),
        };
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("runtime event should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn steel_policy_denied_event_round_trips_through_contract() {
        let event = steel_policy_denied_event();
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("Steel policy event should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn runtime_report_round_trips_through_contract() {
        let response = animus_adapters::ModelResponse::new(MODEL_MESSAGE)
            .expect("valid response should parse");
        let event = animus_runtime::RuntimeEvent::FinalResponse {
            response: response.clone(),
        };
        let report = animus_runtime::RunReport::new(
            vec![event],
            animus_runtime::RuntimeStatus::FinalResponse { response },
        );
        let message = Envelope::from_runtime_report(&report);
        let decoded = message
            .try_into_runtime_report()
            .expect("runtime report should convert");

        assert_eq!(decoded, report);
    }

    #[test]
    fn runtime_report_text_round_trips() {
        let report = animus_runtime::RunReport::new(
            vec![animus_runtime::RuntimeEvent::MaxTurnsExceeded {
                max_turns: MAX_TURNS,
            }],
            animus_runtime::RuntimeStatus::MaxTurnsExceeded {
                max_turns: MAX_TURNS,
            },
        );
        let message = Envelope::from_runtime_report(&report);
        let text = crate::to_text(&message).expect("runtime report should encode");
        let decoded = crate::from_text(&text)
            .expect("runtime report should decode")
            .try_into_runtime_report()
            .expect("runtime report should validate");

        assert_eq!(decoded, report);
    }

    #[test]
    fn steel_policy_failure_status_round_trips_through_contract() {
        let report = animus_runtime::RunReport::new(
            vec![steel_policy_denied_event()],
            animus_runtime::RuntimeStatus::Failed {
                failure: animus_runtime::RuntimeFailure::SteelPolicy {
                    message: String::from(STEEL_FAILURE_MESSAGE),
                },
            },
        );
        let message = Envelope::from_runtime_report(&report);
        let decoded = message
            .try_into_runtime_report()
            .expect("Steel policy failure report should convert");

        assert_eq!(decoded, report);
    }

    #[test]
    fn wrong_kind_is_rejected() {
        let message = Envelope::ModelResponse {
            message: String::from(MODEL_MESSAGE),
        };
        let error = message
            .try_into_model_request()
            .expect_err("wrong kind should fail");

        assert!(matches!(
            error,
            crate::CodecError::WrongKind {
                expected: MODEL_REQUEST_KIND,
                actual: MODEL_RESPONSE_KIND,
            }
        ));
    }

    #[test]
    fn runtime_wrong_kind_is_rejected() {
        let message = Envelope::ModelRequest {
            prompt: String::from(MODEL_PROMPT),
        };
        let error = message
            .try_into_runtime_event()
            .expect_err("wrong kind should fail");

        assert!(matches!(
            error,
            crate::CodecError::WrongKind {
                expected: RUNTIME_EVENT_KIND,
                actual: MODEL_REQUEST_KIND,
            }
        ));
    }

    #[test]
    fn invalid_decoded_payload_is_revalidated() {
        let message = Envelope::ModelRequest {
            prompt: String::from(BLANK_TEXT),
        };
        let error = message
            .try_into_model_request()
            .expect_err("blank prompt should fail");

        assert!(matches!(
            error,
            crate::CodecError::AdapterContract {
                source: animus_adapters::AdapterContractError::EmptyField { .. },
            }
        ));
    }

    #[test]
    fn invalid_runtime_event_payload_is_revalidated() {
        let message = Envelope::RuntimeModelRequested {
            prompt: String::from(BLANK_TEXT),
        };
        let error = message
            .try_into_runtime_event()
            .expect_err("blank runtime request should fail");

        assert!(matches!(
            error,
            crate::CodecError::AdapterContract {
                source: animus_adapters::AdapterContractError::EmptyField { .. },
            }
        ));
    }

    #[test]
    fn invalid_runtime_component_is_rejected() {
        let message = Envelope::RuntimeAdapterError {
            component: String::from(UNKNOWN_COMPONENT),
            message: String::from(MODEL_MESSAGE),
        };
        let error = message
            .try_into_runtime_event()
            .expect_err("unknown component should fail");

        assert!(matches!(
            error,
            crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_COMPONENT_FIELD,
                reason: UNKNOWN_COMPONENT_REASON,
            }
        ));
    }
}
