const MODEL_REQUEST_KIND: &str = "model_request";
const MODEL_RESPONSE_KIND: &str = "model_response";
const TOOL_REQUEST_KIND: &str = "tool_request";
const TOOL_RESPONSE_KIND: &str = "tool_response";
const RUNTIME_EVENT_KIND: &str = "runtime_event";
const RUNTIME_REPORT_KIND: &str = "runtime_report";
const RUNTIME_COMPONENT_FIELD: &str = "component";
const RUNTIME_OUTCOME_CLASS_FIELD: &str = "adapter_outcome_class";
const RUNTIME_BOUNDARY_FIELD: &str = "boundary";
const RUNTIME_DECISION_FIELD: &str = "decision";
const RUNTIME_SANDBOX_FIELD: &str = "sandbox";
const RUNTIME_TERMINAL_FIELD: &str = "terminal";
const RUNTIME_MAX_TURNS_FIELD: &str = "max_turns";
const RUNTIME_REDACTION_FIELD: &str = "redaction_marker";
const RUNTIME_LIFECYCLE_FIELD: &str = "lifecycle_hook_outcome";
const RUNTIME_LIFECYCLE_TRUST_FIELD: &str = "lifecycle_trust_status";
const RUNTIME_LIFECYCLE_SCOPE_FIELD: &str = "lifecycle_trust_scope";
const RUNTIME_LIFECYCLE_OUTCOME_FIELD: &str = "lifecycle_outcome_class";
const RUNTIME_LIFECYCLE_FAILURE_FIELD: &str = "lifecycle_failure_class";
const RUNTIME_LIFECYCLE_REDACTION_FIELD: &str = "lifecycle_redaction";
const RUNTIME_STREAM_FIELD: &str = "model_stream";
const RUNTIME_STREAM_KIND_FIELD: &str = "model_stream_kind";
const RUNTIME_STREAM_VISIBILITY_FIELD: &str = "model_stream_visibility";
const RUNTIME_PROVIDER_RETRY_FIELD: &str = "provider_retry_progress";
const RUNTIME_COMPLETION_CANDIDATE_FIELD: &str = "completion_candidate_request_hash";
const INVALID_COMPLETION_CANDIDATE_HASH_REASON: &str = "expected lowercase BLAKE3 hex";
const INVALID_COMPLETION_CANDIDATE_REQUEST_REASON: &str = "expected exact COMPLETE request";
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
const STREAM_CONTENT_DELTA: &str = "content_delta";
const STREAM_THOUGHT_DELTA: &str = "thought_delta";
const STREAM_TOOL_CALL_DELTA: &str = "tool_call_delta";
const STREAM_USAGE: &str = "usage";
const STREAM_FINISH: &str = "finish";
const STREAM_CANCELLED: &str = "cancelled";
const STREAM_FAILURE: &str = "failure";
const STREAM_ASSISTANT_VISIBLE: &str = "assistant_visible";
const STREAM_POLICY_GATED: &str = "policy_gated";
const STREAM_REDACTED: &str = "redacted";
const EXEC_DECISION_ALLOW: &str = "allow";
const EXEC_DECISION_PROMPT: &str = "prompt";
const EXEC_DECISION_FORBID: &str = "forbid";
const COMMAND_HASH_ONLY_REDACTION: &str = "command_hash_only";
const TERMINAL_FINAL_RESPONSE: &str = "final_response";
const TERMINAL_COMPLETION_CANDIDATE: &str = "completion_candidate";
const TERMINAL_STOPPED: &str = "stopped";
const TERMINAL_MAX_TURNS_EXCEEDED: &str = "max_turns_exceeded";
const TERMINAL_FAILED: &str = "failed";
const UNKNOWN_COMPONENT_REASON: &str = "unknown runtime adapter component";
const UNKNOWN_OUTCOME_CLASS_REASON: &str = "unknown runtime adapter outcome class";
const UNKNOWN_BOUNDARY_REASON: &str = "unknown runtime hook boundary";
const UNKNOWN_DECISION_REASON: &str = "unknown runtime loop decision";
const UNKNOWN_STREAM_KIND_REASON: &str = "unknown model stream kind";
const UNKNOWN_STREAM_VISIBILITY_REASON: &str = "unknown model stream visibility";
const INVALID_STREAM_USAGE_REASON: &str =
    "model stream usage requires both input and output counts";
const INVALID_STREAM_FRAME_REASON: &str = "invalid model stream frame";
const UNKNOWN_PROVIDER_RETRY_REASON: &str = "unknown provider retry reason";
const UNKNOWN_PROVIDER_RETRY_STATUS: &str = "unknown provider retry status";
const INVALID_PROVIDER_RETRY_EVENT: &str = "invalid provider retry progress event";
const UNKNOWN_EXEC_DECISION_REASON: &str = "unknown exec policy decision";
const UNKNOWN_REDACTION_REASON: &str = "unknown runtime redaction marker";
const UNKNOWN_TERMINAL_REASON: &str = "unknown runtime terminal status";
const UNKNOWN_LIFECYCLE_TRUST_REASON: &str = "unknown lifecycle trust status";
const UNKNOWN_LIFECYCLE_SCOPE_REASON: &str = "unknown lifecycle trust scope";
const UNKNOWN_LIFECYCLE_OUTCOME_REASON: &str = "unknown lifecycle outcome class";
const UNKNOWN_LIFECYCLE_FAILURE_REASON: &str = "unknown lifecycle failure class";
const UNKNOWN_LIFECYCLE_REDACTION_REASON: &str = "unknown lifecycle redaction";
const UNKNOWN_TOOL_OUTPUT_STATUS_REASON: &str = "unknown tool output status";
const UNKNOWN_TOOL_REDACTION_REASON: &str = "unknown tool redaction class";
const COMPLETE_TOOL_OUTPUT: &str = "complete";
const TRUNCATED_TOOL_OUTPUT: &str = "truncated";
const INVALID_LIFECYCLE_SCOPE_REASON: &str = "lifecycle trust status requires matching scope";
const INVALID_LIFECYCLE_HOOK_REASON: &str = "invalid lifecycle hook outcome";
const INVALID_SANDBOX_CONTEXT_REASON: &str = "invalid sandbox escalation context";
const LIFECYCLE_TRUSTED: &str = "trusted";
const LIFECYCLE_DENIED: &str = "denied";
const LIFECYCLE_UNTRUSTED: &str = "untrusted";
const LIFECYCLE_SCOPE_MANAGED: &str = "managed";
const LIFECYCLE_SCOPE_USER: &str = "user";
const LIFECYCLE_SCOPE_PROJECT: &str = "project";
const LIFECYCLE_OUTCOME_APPLIED: &str = "applied";
const LIFECYCLE_OUTCOME_SKIPPED: &str = "skipped";
const LIFECYCLE_OUTCOME_UNTRUSTED: &str = "untrusted";
const LIFECYCLE_OUTCOME_FAILED: &str = "failed";
const LIFECYCLE_OUTCOME_DENIED: &str = "denied";
const LIFECYCLE_OUTCOME_MALFORMED: &str = "malformed";
const LIFECYCLE_OUTCOME_UNSUPPORTED: &str = "unsupported";
const LIFECYCLE_FAILURE_REQUIRED_HOOK_UNAVAILABLE: &str = "required_hook_unavailable";
const LIFECYCLE_FAILURE_HOOK_EXECUTION_FAILED: &str = "hook_execution_failed";
const LIFECYCLE_FAILURE_HOOK_DENIED_BY_TRUST: &str = "hook_denied_by_trust";
const LIFECYCLE_FAILURE_MALFORMED_OUTPUT: &str = "malformed_output";
const LIFECYCLE_FAILURE_UNSUPPORTED_EFFECT: &str = "unsupported_effect";
const LIFECYCLE_FAILURE_POLICY_BYPASS: &str = "policy_bypass";
const LIFECYCLE_REDACTION_NO_SENSITIVE_BODY: &str = "no_sensitive_body";
const LIFECYCLE_REDACTION_BODY_REDACTED: &str = "body_redacted";

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

/// One current-shape tool argument field.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ToolArgumentEnvelope {
    /// Current argument name.
    name: String,
    /// Validated argument value.
    value: String,
}

/// Current-shape tool request payload.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ToolRequestEnvelope {
    /// Tool name selected by the harness.
    name: String,
    /// Current parameter schema identity.
    schema_identity: String,
    /// Validated current argument fields.
    arguments: Vec<ToolArgumentEnvelope>,
    /// Tool-call correlation identity.
    correlation_id: String,
    /// Model-visible output character budget.
    visible_char_budget: u32,
    /// Declared opaque mutation targets.
    mutation_targets: Vec<String>,
    /// Exact required capability labels.
    required_capability_labels: Vec<String>,
    /// Approval posture label.
    approval_posture: String,
}

/// Bounded tool result payload without raw full output bodies.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ToolResponseEnvelope {
    /// Bounded model-visible text.
    visible_text: String,
    /// Complete/truncated status label.
    status: String,
    /// Original output byte count.
    byte_count: u64,
    /// Original output character count.
    char_count: u64,
    /// Original output line count.
    line_count: u64,
    /// Optional BLAKE3 content hash.
    content_hash: Option<String>,
    /// Optional host-owned full-output handle.
    full_output_reference: Option<String>,
    /// Redaction class label.
    redaction_class: String,
}

impl ToolRequestEnvelope {
    fn from_tool_request(request: &animus_adapters::ToolRequest) -> Self {
        Self {
            name: String::from(request.name()),
            schema_identity: String::from(request.schema_identity()),
            arguments: request
                .arguments()
                .iter()
                .map(|argument| ToolArgumentEnvelope {
                    name: String::from(argument.name()),
                    value: String::from(argument.value()),
                })
                .collect(),
            correlation_id: String::from(request.correlation_id()),
            visible_char_budget: request.output_policy().visible_char_budget,
            mutation_targets: request.mutation_targets().to_vec(),
            required_capability_labels: request.required_capability_labels().to_vec(),
            approval_posture: String::from(request.approval_posture().as_str()),
        }
    }

    fn into_tool_request(self) -> Result<animus_adapters::ToolRequest, crate::CodecError> {
        let argument_names: Vec<&str> = self
            .arguments
            .iter()
            .map(|argument| argument.name.as_str())
            .collect();
        let argument_fields: Vec<animus_adapters::tool::ArgumentFieldDraft<'_>> = self
            .arguments
            .iter()
            .map(|argument| animus_adapters::tool::ArgumentFieldDraft {
                name: Some(argument.name.as_str()),
                value: Some(argument.value.as_str()),
            })
            .collect();
        let prepared = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: self.schema_identity.as_str(),
                allowed_field_names: argument_names.as_slice(),
                compatibility_rules: &[],
                fields: argument_fields.as_slice(),
            },
        )?;
        let mutation_targets: Vec<&str> =
            self.mutation_targets.iter().map(String::as_str).collect();
        let capability_labels: Vec<&str> = self
            .required_capability_labels
            .iter()
            .map(String::as_str)
            .collect();
        Ok(animus_adapters::ToolRequest::from_draft(
            &animus_adapters::tool::RequestDraft {
                name: Some(self.name.as_str()),
                arguments: Some(&prepared),
                correlation_id: Some(self.correlation_id.as_str()),
                output_policy: animus_adapters::tool::OutputPolicy {
                    visible_char_budget: self.visible_char_budget,
                },
                mutation_targets: mutation_targets.as_slice(),
                required_capability_labels: capability_labels.as_slice(),
                approval_posture: self.approval_posture.as_str(),
            },
        )?)
    }
}

impl ToolResponseEnvelope {
    fn from_tool_response(response: &animus_adapters::ToolResponse) -> Self {
        Self {
            visible_text: String::from(response.model_visible_text()),
            status: output_status_to_string(response.status()),
            byte_count: response.byte_count(),
            char_count: response.char_count(),
            line_count: response.line_count(),
            content_hash: response.content_hash().map(String::from),
            full_output_reference: response.full_output_reference().map(String::from),
            redaction_class: String::from(response.redaction_class().as_str()),
        }
    }

    fn into_tool_response(self) -> Result<animus_adapters::ToolResponse, crate::CodecError> {
        let status = output_status_from_string(self.status.as_str())?;
        let redaction_class = output_redaction_from_string(self.redaction_class.as_str())?;
        let bounded = animus_adapters::tool::BoundedOutput::from_metadata(
            &animus_adapters::tool::BoundedOutputMetadataDraft {
                visible_text: self.visible_text.as_str(),
                status,
                byte_count: self.byte_count,
                char_count: self.char_count,
                line_count: self.line_count,
                content_hash: self.content_hash.as_deref(),
                full_output_reference: self.full_output_reference.as_deref(),
            },
        )?;
        Ok(animus_adapters::ToolResponse::from_bounded(
            bounded,
            redaction_class,
        ))
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
    /// Runtime requested host-owned completion evaluation.
    CompletionCandidate {
        /// Exact admitted compatibility request.
        request: String,
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
            animus_runtime::RuntimeStatus::CompletionCandidate { candidate } => {
                Self::CompletionCandidate {
                    request: String::from(candidate.request()),
                }
            }
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
            Self::CompletionCandidate { request } => {
                Ok(animus_runtime::RuntimeStatus::CompletionCandidate {
                    candidate: completion_candidate_from_request(&request)?,
                })
            }
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
        /// Structured current-shape tool request.
        request: ToolRequestEnvelope,
    },
    /// Tool response boundary payload.
    ToolResponse {
        /// Bounded tool result metadata.
        response: ToolResponseEnvelope,
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
    /// Runtime model-stream-started event payload.
    RuntimeModelStreamStarted {
        /// Model-call correlation id.
        call_id: String,
    },
    /// Runtime model-stream-frame event payload.
    RuntimeModelStreamFrame {
        /// Model-call correlation id.
        call_id: String,
        /// Strict per-call stream order.
        order: u64,
        /// Stable stream frame kind label.
        frame_kind: String,
        /// Optional stream frame body.
        body: Option<String>,
        /// Optional streamed tool-call id.
        tool_call_id: Option<String>,
        /// Optional input-token usage count.
        usage_input_tokens: Option<u32>,
        /// Optional output-token usage count.
        usage_output_tokens: Option<u32>,
        /// Default visibility label.
        visibility: String,
    },
    /// Runtime model-stream-ended event payload.
    RuntimeModelStreamEnded {
        /// Model-call correlation id.
        call_id: String,
        /// Terminal stream status label.
        status: String,
    },
    /// Runtime model-stream-failed event payload.
    RuntimeModelStreamFailed {
        /// Model-call correlation id.
        call_id: String,
        /// Safe failure reason.
        reason: String,
    },
    /// Runtime provider-retry progress payload.
    RuntimeProviderRetryProgress {
        /// Stable run identity.
        run_id: String,
        /// Current or next attempt count.
        attempt_count: u32,
        /// Maximum total attempt count.
        max_attempt_count: u32,
        /// Planned delay in milliseconds when applicable.
        delay_ms: Option<u64>,
        /// Canonical provider-neutral reason class.
        reason: String,
        /// Provider-neutral progress status.
        status: String,
        /// Exact invocation identity.
        exact_plan_hash: String,
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
        /// Structured current-shape tool request.
        request: ToolRequestEnvelope,
    },
    /// Runtime tool-responded event payload.
    RuntimeToolResponded {
        /// Bounded tool result metadata.
        response: ToolResponseEnvelope,
    },
    /// Runtime tool-denied event payload.
    RuntimeToolDenied {
        /// Denied current-shape tool request.
        request: ToolRequestEnvelope,
        /// Safe denial reason.
        reason: String,
    },
    /// Runtime final-response event payload.
    RuntimeFinalResponse {
        /// Final response message.
        message: String,
    },
    /// Runtime completion-candidate request event payload.
    RuntimeCompletionCandidateRequested {
        /// BLAKE3 identity of the exact request spelling.
        request_hash: String,
    },
    /// Runtime stopped event payload.
    RuntimeStopped,
    /// Runtime max-turn exhaustion event payload.
    RuntimeMaxTurnsExceeded {
        /// Configured maximum number of fabric turns.
        max_turns: usize,
    },
    /// Runtime typed adapter outcome classification event payload.
    RuntimeAdapterOutcomeClassified {
        /// Adapter component name.
        component: String,
        /// Adapter outcome class label.
        class: String,
        /// Bounded safe adapter outcome message.
        message: String,
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
    /// Runtime lifecycle hook outcome event payload.
    RuntimeLifecycleHookOutcome {
        /// Stable hook identifier.
        hook_id: String,
        /// BLAKE3 hash of the reviewed hook definition.
        definition_hash: String,
        /// Stable lifecycle event kind label.
        event_kind: String,
        /// Trust status label.
        trust_status: String,
        /// Trust scope label when status is scoped.
        trust_scope: Option<String>,
        /// Outcome class label.
        outcome_class: String,
        /// Optional failure class label.
        failure_class: Option<String>,
        /// Stable lifecycle event identifier.
        event_id: String,
        /// Safe resource class.
        resource_class: String,
        /// Safe redacted resource label.
        safe_label: String,
        /// Safe bounded input summary.
        summary: String,
        /// Sensitive body byte count.
        body_size_bytes: usize,
        /// BLAKE3 sensitive body hash or no-body marker hash.
        body_hash: String,
        /// Input redaction label.
        redaction: String,
        /// Optional safe input status.
        status: Option<String>,
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
    /// Runtime sandbox-escalation policy event payload.
    RuntimeSandboxEscalation {
        /// Exec-policy decision label.
        decision: String,
        /// Matched rule identities.
        rule_ids: Vec<String>,
        /// BLAKE3 command fingerprints only.
        command_fingerprints: Vec<String>,
        /// Active sandbox label before escalation.
        active_sandbox: String,
        /// Requested capability outside the active sandbox.
        requested_capability: String,
        /// Safe escalation reason label.
        escalation_reason: String,
        /// Redaction marker proving command bodies were omitted.
        redaction_marker: String,
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
            | Self::RuntimeModelStreamStarted { .. }
            | Self::RuntimeModelStreamFrame { .. }
            | Self::RuntimeModelStreamEnded { .. }
            | Self::RuntimeModelStreamFailed { .. }
            | Self::RuntimeProviderRetryProgress { .. }
            | Self::RuntimeModelResponded { .. }
            | Self::RuntimeModelDenied { .. }
            | Self::RuntimeToolRequested { .. }
            | Self::RuntimeToolResponded { .. }
            | Self::RuntimeToolDenied { .. }
            | Self::RuntimeFinalResponse { .. }
            | Self::RuntimeCompletionCandidateRequested { .. }
            | Self::RuntimeStopped
            | Self::RuntimeMaxTurnsExceeded { .. }
            | Self::RuntimeAdapterOutcomeClassified { .. }
            | Self::RuntimeAdapterError { .. }
            | Self::RuntimeFabricError { .. }
            | Self::RuntimeHookBlocked { .. }
            | Self::RuntimeHookFailed { .. }
            | Self::RuntimeLifecycleHookOutcome { .. }
            | Self::RuntimeSteelPolicyDenied { .. }
            | Self::RuntimeSandboxEscalation { .. }
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
            request: ToolRequestEnvelope::from_tool_request(request),
        }
    }

    /// Builds a boundary message from a validated tool response.
    #[must_use]
    pub fn from_tool_response(response: &animus_adapters::ToolResponse) -> Self {
        Self::ToolResponse {
            response: ToolResponseEnvelope::from_tool_response(response),
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
            Self::ToolRequest { request } => request.into_tool_request(),
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
            Self::ToolResponse { response } => response.into_tool_response(),
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
        | animus_runtime::RuntimeEvent::LoopDecision { .. }
        | animus_runtime::RuntimeEvent::LifecycleHookOutcome { .. } => RuntimeEventGroup::Lifecycle,
        animus_runtime::RuntimeEvent::ModelRequested { .. }
        | animus_runtime::RuntimeEvent::ModelStreamStarted { .. }
        | animus_runtime::RuntimeEvent::ModelStreamFrame { .. }
        | animus_runtime::RuntimeEvent::ModelStreamEnded { .. }
        | animus_runtime::RuntimeEvent::ModelStreamFailed { .. }
        | animus_runtime::RuntimeEvent::ProviderRetryProgress { .. }
        | animus_runtime::RuntimeEvent::ModelResponded { .. }
        | animus_runtime::RuntimeEvent::ModelDenied { .. }
        | animus_runtime::RuntimeEvent::ToolRequested { .. }
        | animus_runtime::RuntimeEvent::ToolResponded { .. }
        | animus_runtime::RuntimeEvent::ToolDenied { .. }
        | animus_runtime::RuntimeEvent::SandboxEscalation { .. } => RuntimeEventGroup::Adapter,
        animus_runtime::RuntimeEvent::FinalResponse { .. }
        | animus_runtime::RuntimeEvent::CompletionCandidateRequested { .. }
        | animus_runtime::RuntimeEvent::Stopped
        | animus_runtime::RuntimeEvent::MaxTurnsExceeded { .. }
        | animus_runtime::RuntimeEvent::Shutdown { .. } => RuntimeEventGroup::Completion,
        animus_runtime::RuntimeEvent::AdapterOutcomeClassified { .. }
        | animus_runtime::RuntimeEvent::AdapterError { .. }
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
        | Envelope::RuntimeLoopDecision { .. }
        | Envelope::RuntimeLifecycleHookOutcome { .. } => Some(RuntimeEventGroup::Lifecycle),
        Envelope::RuntimeModelRequested { .. }
        | Envelope::RuntimeModelStreamStarted { .. }
        | Envelope::RuntimeModelStreamFrame { .. }
        | Envelope::RuntimeModelStreamEnded { .. }
        | Envelope::RuntimeModelStreamFailed { .. }
        | Envelope::RuntimeProviderRetryProgress { .. }
        | Envelope::RuntimeModelResponded { .. }
        | Envelope::RuntimeModelDenied { .. }
        | Envelope::RuntimeToolRequested { .. }
        | Envelope::RuntimeToolResponded { .. }
        | Envelope::RuntimeToolDenied { .. }
        | Envelope::RuntimeSandboxEscalation { .. } => Some(RuntimeEventGroup::Adapter),
        Envelope::RuntimeFinalResponse { .. }
        | Envelope::RuntimeCompletionCandidateRequested { .. }
        | Envelope::RuntimeStopped
        | Envelope::RuntimeMaxTurnsExceeded { .. }
        | Envelope::RuntimeShutdown { .. } => Some(RuntimeEventGroup::Completion),
        Envelope::RuntimeAdapterOutcomeClassified { .. }
        | Envelope::RuntimeAdapterError { .. }
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
        animus_runtime::RuntimeEvent::LifecycleHookOutcome { outcome } => {
            encode_lifecycle_hook_outcome(outcome)
        }
        _other => unreachable!("runtime event group mismatch"),
    }
}

fn encode_lifecycle_hook_outcome(outcome: &animus_runtime::LifecycleHookOutcome) -> Envelope {
    let input = outcome.input();
    Envelope::RuntimeLifecycleHookOutcome {
        hook_id: String::from(outcome.hook_id()),
        definition_hash: String::from(outcome.definition_hash()),
        event_kind: String::from(outcome.event_kind().as_str()),
        trust_status: String::from(outcome.trust_status().as_str()),
        trust_scope: lifecycle_trust_scope(outcome.trust_status()),
        outcome_class: String::from(outcome.outcome_class().as_str()),
        failure_class: outcome.failure_class().map(lifecycle_failure_to_string),
        event_id: String::from(input.event_id()),
        resource_class: String::from(input.resource_class()),
        safe_label: String::from(input.safe_label()),
        summary: String::from(input.summary()),
        body_size_bytes: input.body_size_bytes(),
        body_hash: String::from(input.body_hash()),
        redaction: String::from(input.redaction().as_str()),
        status: input.status().map(String::from),
    }
}

fn lifecycle_trust_scope(status: animus_runtime::LifecycleHookTrustStatus) -> Option<String> {
    match status {
        animus_runtime::LifecycleHookTrustStatus::Trusted { scope }
        | animus_runtime::LifecycleHookTrustStatus::Denied { scope } => {
            Some(String::from(scope.as_str()))
        }
        animus_runtime::LifecycleHookTrustStatus::Untrusted => None,
    }
}

fn lifecycle_failure_to_string(failure: animus_runtime::LifecycleHookFailureClass) -> String {
    String::from(failure.as_str())
}

fn encode_adapter_event(event: &animus_runtime::RuntimeEvent) -> Envelope {
    match event {
        animus_runtime::RuntimeEvent::ModelRequested { request } => {
            Envelope::RuntimeModelRequested {
                prompt: String::from(request.prompt()),
            }
        }
        animus_runtime::RuntimeEvent::ModelStreamStarted { call_id } => {
            Envelope::RuntimeModelStreamStarted {
                call_id: String::from(call_id.as_str()),
            }
        }
        animus_runtime::RuntimeEvent::ModelStreamFrame { frame } => {
            let usage = frame.usage();
            Envelope::RuntimeModelStreamFrame {
                call_id: String::from(frame.call_id().as_str()),
                order: frame.order().value(),
                frame_kind: stream_kind_to_string(frame.kind()),
                body: frame.body().map(String::from),
                tool_call_id: frame
                    .tool_call_id()
                    .map(|call_id| String::from(call_id.as_str())),
                usage_input_tokens: usage.map(animus_adapters::ModelStreamUsage::input_tokens),
                usage_output_tokens: usage.map(animus_adapters::ModelStreamUsage::output_tokens),
                visibility: stream_visibility_to_string(frame.visibility()),
            }
        }
        animus_runtime::RuntimeEvent::ModelStreamEnded { call_id, status } => {
            Envelope::RuntimeModelStreamEnded {
                call_id: String::from(call_id.as_str()),
                status: stream_kind_to_string(*status),
            }
        }
        animus_runtime::RuntimeEvent::ModelStreamFailed { call_id, reason } => {
            Envelope::RuntimeModelStreamFailed {
                call_id: String::from(call_id.as_str()),
                reason: reason.clone(),
            }
        }
        animus_runtime::RuntimeEvent::ProviderRetryProgress { progress } => {
            Envelope::RuntimeProviderRetryProgress {
                run_id: progress.run_id.clone(),
                attempt_count: progress.attempt_count,
                max_attempt_count: progress.max_attempt_count,
                delay_ms: progress.delay_ms,
                reason: String::from(progress.reason.as_str()),
                status: String::from(progress.status.as_str()),
                exact_plan_hash: progress.exact_plan_hash.clone(),
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
            request: ToolRequestEnvelope::from_tool_request(request),
        },
        animus_runtime::RuntimeEvent::ToolResponded { response } => {
            Envelope::RuntimeToolResponded {
                response: ToolResponseEnvelope::from_tool_response(response),
            }
        }
        animus_runtime::RuntimeEvent::ToolDenied { request, reason } => {
            Envelope::RuntimeToolDenied {
                request: ToolRequestEnvelope::from_tool_request(request),
                reason: reason.clone(),
            }
        }
        animus_runtime::RuntimeEvent::SandboxEscalation {
            decision,
            rule_ids,
            command_fingerprints,
            active_sandbox,
            requested_capability,
            escalation_reason,
            redaction_marker,
        } => Envelope::RuntimeSandboxEscalation {
            decision: exec_decision_to_string(*decision),
            rule_ids: rule_ids.clone(),
            command_fingerprints: command_fingerprints.clone(),
            active_sandbox: active_sandbox.clone(),
            requested_capability: requested_capability.clone(),
            escalation_reason: escalation_reason.clone(),
            redaction_marker: redaction_marker.clone(),
        },
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
        animus_runtime::RuntimeEvent::CompletionCandidateRequested { request_hash } => {
            Envelope::RuntimeCompletionCandidateRequested {
                request_hash: request_hash.clone(),
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
        animus_runtime::RuntimeEvent::AdapterOutcomeClassified {
            component,
            class,
            message,
        } => Envelope::RuntimeAdapterOutcomeClassified {
            component: component_to_string(*component),
            class: outcome_class_to_string(*class),
            message: message.clone(),
        },
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
        Envelope::RuntimeLifecycleHookOutcome {
            hook_id,
            definition_hash,
            event_kind,
            trust_status,
            trust_scope,
            outcome_class,
            failure_class,
            event_id,
            resource_class,
            safe_label,
            summary,
            body_size_bytes,
            body_hash,
            redaction,
            status,
        } => decode_lifecycle_hook_outcome(LifecycleHookOutcomeParts {
            hook_id: &hook_id,
            definition_hash: &definition_hash,
            event_kind: &event_kind,
            trust_status: &trust_status,
            trust_scope: trust_scope.as_deref(),
            outcome_class: &outcome_class,
            failure_class: failure_class.as_deref(),
            event_id: &event_id,
            resource_class: &resource_class,
            safe_label: &safe_label,
            summary: &summary,
            body_size_bytes,
            body_hash: &body_hash,
            redaction: &redaction,
            status: status.as_deref(),
        }),
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

#[derive(Clone, Copy)]
struct LifecycleHookOutcomeParts<'a> {
    hook_id: &'a str,
    definition_hash: &'a str,
    event_kind: &'a str,
    trust_status: &'a str,
    trust_scope: Option<&'a str>,
    outcome_class: &'a str,
    failure_class: Option<&'a str>,
    event_id: &'a str,
    resource_class: &'a str,
    safe_label: &'a str,
    summary: &'a str,
    body_size_bytes: usize,
    body_hash: &'a str,
    redaction: &'a str,
    status: Option<&'a str>,
}

fn decode_lifecycle_hook_outcome(
    parts: LifecycleHookOutcomeParts<'_>,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    let input = animus_runtime::LifecycleHookInput::from_redacted_report(
        &animus_runtime::LifecycleHookInputReportDraft {
            event_id: parts.event_id,
            event_kind: parts.event_kind,
            resource_class: parts.resource_class,
            safe_label: parts.safe_label,
            summary: parts.summary,
            body_size_bytes: parts.body_size_bytes,
            body_hash: parts.body_hash,
            redaction: lifecycle_redaction_from_str(parts.redaction)?,
            status: parts.status,
        },
    )
    .map_err(lifecycle_error)?;
    let outcome = animus_runtime::LifecycleHookOutcome::from_redacted_report(
        &animus_runtime::LifecycleHookOutcomeReportDraft {
            hook_id: parts.hook_id,
            definition_hash: parts.definition_hash,
            trust_status: lifecycle_trust_status_from_parts(parts.trust_status, parts.trust_scope)?,
            outcome_class: lifecycle_outcome_from_str(parts.outcome_class)?,
            failure_class: parts
                .failure_class
                .map(lifecycle_failure_from_str)
                .transpose()?,
            input: &input,
        },
    )
    .map_err(lifecycle_error)?;
    Ok(animus_runtime::RuntimeEvent::LifecycleHookOutcome { outcome })
}

fn lifecycle_error(_error: animus_runtime::LifecycleHookError) -> crate::CodecError {
    crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_LIFECYCLE_FIELD,
        reason: INVALID_LIFECYCLE_HOOK_REASON,
    }
}

fn decode_adapter_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        model_envelope @ (Envelope::RuntimeModelRequested { .. }
        | Envelope::RuntimeModelStreamStarted { .. }
        | Envelope::RuntimeModelStreamFrame { .. }
        | Envelope::RuntimeModelStreamEnded { .. }
        | Envelope::RuntimeModelStreamFailed { .. }
        | Envelope::RuntimeProviderRetryProgress { .. }
        | Envelope::RuntimeModelResponded { .. }
        | Envelope::RuntimeModelDenied { .. }) => decode_model_adapter_event(model_envelope),
        tool_envelope @ (Envelope::RuntimeToolRequested { .. }
        | Envelope::RuntimeToolResponded { .. }
        | Envelope::RuntimeToolDenied { .. }
        | Envelope::RuntimeSandboxEscalation { .. }) => decode_tool_adapter_event(tool_envelope),
        _other => unreachable!("runtime envelope group mismatch"),
    }
}

fn decode_model_adapter_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeModelRequested { prompt } => {
            Ok(animus_runtime::RuntimeEvent::ModelRequested {
                request: animus_adapters::ModelRequest::new(&prompt)?,
            })
        }
        Envelope::RuntimeModelStreamStarted { call_id } => {
            Ok(animus_runtime::RuntimeEvent::ModelStreamStarted {
                call_id: animus_adapters::ModelStreamCallId::new(&call_id)
                    .map_err(model_stream_error)?,
            })
        }
        Envelope::RuntimeModelStreamFrame {
            call_id,
            order,
            frame_kind,
            body,
            tool_call_id,
            usage_input_tokens,
            usage_output_tokens,
            visibility,
        } => Ok(animus_runtime::RuntimeEvent::ModelStreamFrame {
            frame: animus_adapters::ModelStreamFrame::from_draft(
                animus_adapters::ModelStreamFrameDraft {
                    call_id: &call_id,
                    order,
                    kind: stream_kind_from_str(&frame_kind)?,
                    body: body.as_deref(),
                    tool_call_id: tool_call_id.as_deref(),
                    usage: stream_usage(usage_input_tokens, usage_output_tokens)?,
                    visibility: stream_visibility_from_str(&visibility)?,
                },
            )
            .map_err(model_stream_error)?,
        }),
        Envelope::RuntimeModelStreamEnded { call_id, status } => {
            Ok(animus_runtime::RuntimeEvent::ModelStreamEnded {
                call_id: animus_adapters::ModelStreamCallId::new(&call_id)
                    .map_err(model_stream_error)?,
                status: stream_kind_from_str(&status)?,
            })
        }
        Envelope::RuntimeModelStreamFailed { call_id, reason } => {
            Ok(animus_runtime::RuntimeEvent::ModelStreamFailed {
                call_id: animus_adapters::ModelStreamCallId::new(&call_id)
                    .map_err(model_stream_error)?,
                reason,
            })
        }
        Envelope::RuntimeProviderRetryProgress {
            run_id,
            attempt_count,
            max_attempt_count,
            delay_ms,
            reason,
            status,
            exact_plan_hash,
        } => decode_provider_retry_progress(ProviderRetryProgressParts {
            run_id,
            attempt_count,
            max_attempt_count,
            delay_ms,
            reason,
            status,
            exact_plan_hash,
        }),
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
        _other => unreachable!("runtime model envelope group mismatch"),
    }
}

fn decode_tool_adapter_event(
    envelope: Envelope,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    match envelope {
        Envelope::RuntimeToolRequested { request } => {
            Ok(animus_runtime::RuntimeEvent::ToolRequested {
                request: request.into_tool_request()?,
            })
        }
        Envelope::RuntimeToolResponded { response } => {
            Ok(animus_runtime::RuntimeEvent::ToolResponded {
                response: response.into_tool_response()?,
            })
        }
        Envelope::RuntimeToolDenied { request, reason } => {
            Ok(animus_runtime::RuntimeEvent::ToolDenied {
                request: request.into_tool_request()?,
                reason,
            })
        }
        Envelope::RuntimeSandboxEscalation {
            decision,
            rule_ids,
            command_fingerprints,
            active_sandbox,
            requested_capability,
            escalation_reason,
            redaction_marker,
        } => decode_sandbox_escalation(SandboxEscalationParts {
            decision: &decision,
            rule_ids,
            command_fingerprints,
            active_sandbox: &active_sandbox,
            requested_capability: &requested_capability,
            escalation_reason: &escalation_reason,
            redaction_marker: &redaction_marker,
        }),
        _other => unreachable!("runtime tool envelope group mismatch"),
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
        Envelope::RuntimeCompletionCandidateRequested { request_hash } => {
            validate_completion_candidate_hash(&request_hash)?;
            Ok(animus_runtime::RuntimeEvent::CompletionCandidateRequested { request_hash })
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
        Envelope::RuntimeAdapterOutcomeClassified {
            component,
            class,
            message,
        } => Ok(animus_runtime::RuntimeEvent::AdapterOutcomeClassified {
            component: component_from_str(&component)?,
            class: outcome_class_from_str(&class)?,
            message,
        }),
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

fn completion_candidate_from_request(
    request: &str,
) -> Result<animus_runtime::RuntimeCompletionCandidate, crate::CodecError> {
    let decision = animus_runtime::RuntimeLoopDecision::from_draft(
        animus_runtime::RuntimeLoopDecisionDraft::completion_candidate(request),
    )
    .map_err(|_error| crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_COMPLETION_CANDIDATE_FIELD,
        reason: INVALID_COMPLETION_CANDIDATE_REQUEST_REASON,
    })?;
    match decision {
        animus_runtime::RuntimeLoopDecision::CompletionCandidate { candidate } => Ok(candidate),
        _other => unreachable!("completion-candidate draft must preserve its decision kind"),
    }
}

fn validate_completion_candidate_hash(request_hash: &str) -> Result<(), crate::CodecError> {
    if request_hash.len() == animus_adapters::SAFE_HASH_HEX_CHARS
        && request_hash
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Ok(());
    }
    Err(crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_COMPLETION_CANDIDATE_FIELD,
        reason: INVALID_COMPLETION_CANDIDATE_HASH_REASON,
    })
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

fn outcome_class_to_string(class: animus_adapters::AdapterCallFailureClass) -> String {
    String::from(class.as_str())
}

fn outcome_class_from_str(
    class: &str,
) -> Result<animus_adapters::AdapterCallFailureClass, crate::CodecError> {
    match class {
        "absent" => Ok(animus_adapters::AdapterCallFailureClass::Absent),
        "invalid_input" => Ok(animus_adapters::AdapterCallFailureClass::InvalidInput),
        "missing_executor" => Ok(animus_adapters::AdapterCallFailureClass::MissingExecutor),
        "denied" => Ok(animus_adapters::AdapterCallFailureClass::Denied),
        "denied_capability" => Ok(animus_adapters::AdapterCallFailureClass::DeniedCapability),
        "approval_denied" => Ok(animus_adapters::AdapterCallFailureClass::ApprovalDenied),
        "unavailable" => Ok(animus_adapters::AdapterCallFailureClass::Unavailable),
        "malformed_output" => Ok(animus_adapters::AdapterCallFailureClass::MalformedOutput),
        "unsafe_output_reference" => {
            Ok(animus_adapters::AdapterCallFailureClass::UnsafeOutputReference)
        }
        "unbounded_oversized_output" => {
            Ok(animus_adapters::AdapterCallFailureClass::UnboundedOversizedOutput)
        }
        "mutation_conflict" => Ok(animus_adapters::AdapterCallFailureClass::MutationConflict),
        "fallback_denied" => Ok(animus_adapters::AdapterCallFailureClass::FallbackDenied),
        "cancelled" => Ok(animus_adapters::AdapterCallFailureClass::Cancelled),
        "timed_out" => Ok(animus_adapters::AdapterCallFailureClass::TimedOut),
        "execution_failure" => Ok(animus_adapters::AdapterCallFailureClass::ExecutionFailure),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_OUTCOME_CLASS_FIELD,
            reason: UNKNOWN_OUTCOME_CLASS_REASON,
        }),
    }
}

fn output_status_to_string(status: animus_adapters::tool::OutputStatus) -> String {
    match status {
        animus_adapters::tool::OutputStatus::Complete => String::from(COMPLETE_TOOL_OUTPUT),
        animus_adapters::tool::OutputStatus::Truncated => String::from(TRUNCATED_TOOL_OUTPUT),
    }
}

fn output_status_from_string(
    status: &str,
) -> Result<animus_adapters::tool::OutputStatus, crate::CodecError> {
    match status {
        COMPLETE_TOOL_OUTPUT => Ok(animus_adapters::tool::OutputStatus::Complete),
        TRUNCATED_TOOL_OUTPUT => Ok(animus_adapters::tool::OutputStatus::Truncated),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_REDACTION_FIELD,
            reason: UNKNOWN_TOOL_OUTPUT_STATUS_REASON,
        }),
    }
}

fn output_redaction_from_string(
    redaction: &str,
) -> Result<animus_adapters::tool::OutputRedactionClass, crate::CodecError> {
    match redaction {
        "safe" => Ok(animus_adapters::tool::OutputRedactionClass::Safe),
        "redacted" => Ok(animus_adapters::tool::OutputRedactionClass::Redacted),
        "secret_adjacent" => Ok(animus_adapters::tool::OutputRedactionClass::SecretAdjacent),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_REDACTION_FIELD,
            reason: UNKNOWN_TOOL_REDACTION_REASON,
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

fn lifecycle_trust_status_from_parts(
    status: &str,
    scope: Option<&str>,
) -> Result<animus_runtime::LifecycleHookTrustStatus, crate::CodecError> {
    match status {
        LIFECYCLE_TRUSTED => Ok(animus_runtime::LifecycleHookTrustStatus::Trusted {
            scope: required_lifecycle_trust_scope(scope)?,
        }),
        LIFECYCLE_DENIED => Ok(animus_runtime::LifecycleHookTrustStatus::Denied {
            scope: required_lifecycle_trust_scope(scope)?,
        }),
        LIFECYCLE_UNTRUSTED => {
            if scope.is_some() {
                Err(crate::CodecError::InvalidRuntimePayload {
                    field: RUNTIME_LIFECYCLE_SCOPE_FIELD,
                    reason: INVALID_LIFECYCLE_SCOPE_REASON,
                })
            } else {
                Ok(animus_runtime::LifecycleHookTrustStatus::Untrusted)
            }
        }
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_LIFECYCLE_TRUST_FIELD,
            reason: UNKNOWN_LIFECYCLE_TRUST_REASON,
        }),
    }
}

fn required_lifecycle_trust_scope(
    scope: Option<&str>,
) -> Result<animus_runtime::LifecycleHookTrustScope, crate::CodecError> {
    scope.map_or_else(
        || {
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_LIFECYCLE_SCOPE_FIELD,
                reason: INVALID_LIFECYCLE_SCOPE_REASON,
            })
        },
        lifecycle_trust_scope_from_str,
    )
}

fn lifecycle_trust_scope_from_str(
    scope: &str,
) -> Result<animus_runtime::LifecycleHookTrustScope, crate::CodecError> {
    match scope {
        LIFECYCLE_SCOPE_MANAGED => Ok(animus_runtime::LifecycleHookTrustScope::Managed),
        LIFECYCLE_SCOPE_USER => Ok(animus_runtime::LifecycleHookTrustScope::User),
        LIFECYCLE_SCOPE_PROJECT => Ok(animus_runtime::LifecycleHookTrustScope::Project),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_LIFECYCLE_SCOPE_FIELD,
            reason: UNKNOWN_LIFECYCLE_SCOPE_REASON,
        }),
    }
}

fn lifecycle_outcome_from_str(
    outcome: &str,
) -> Result<animus_runtime::LifecycleHookOutcomeClass, crate::CodecError> {
    match outcome {
        LIFECYCLE_OUTCOME_APPLIED => Ok(animus_runtime::LifecycleHookOutcomeClass::Applied),
        LIFECYCLE_OUTCOME_SKIPPED => Ok(animus_runtime::LifecycleHookOutcomeClass::Skipped),
        LIFECYCLE_OUTCOME_UNTRUSTED => Ok(animus_runtime::LifecycleHookOutcomeClass::Untrusted),
        LIFECYCLE_OUTCOME_FAILED => Ok(animus_runtime::LifecycleHookOutcomeClass::Failed),
        LIFECYCLE_OUTCOME_DENIED => Ok(animus_runtime::LifecycleHookOutcomeClass::Denied),
        LIFECYCLE_OUTCOME_MALFORMED => Ok(animus_runtime::LifecycleHookOutcomeClass::Malformed),
        LIFECYCLE_OUTCOME_UNSUPPORTED => Ok(animus_runtime::LifecycleHookOutcomeClass::Unsupported),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_LIFECYCLE_OUTCOME_FIELD,
            reason: UNKNOWN_LIFECYCLE_OUTCOME_REASON,
        }),
    }
}

fn lifecycle_failure_from_str(
    failure: &str,
) -> Result<animus_runtime::LifecycleHookFailureClass, crate::CodecError> {
    match failure {
        LIFECYCLE_FAILURE_REQUIRED_HOOK_UNAVAILABLE => {
            Ok(animus_runtime::LifecycleHookFailureClass::RequiredHookUnavailable)
        }
        LIFECYCLE_FAILURE_HOOK_EXECUTION_FAILED => {
            Ok(animus_runtime::LifecycleHookFailureClass::HookExecutionFailed)
        }
        LIFECYCLE_FAILURE_HOOK_DENIED_BY_TRUST => {
            Ok(animus_runtime::LifecycleHookFailureClass::HookDeniedByTrust)
        }
        LIFECYCLE_FAILURE_MALFORMED_OUTPUT => {
            Ok(animus_runtime::LifecycleHookFailureClass::MalformedOutput)
        }
        LIFECYCLE_FAILURE_UNSUPPORTED_EFFECT => {
            Ok(animus_runtime::LifecycleHookFailureClass::UnsupportedEffect)
        }
        LIFECYCLE_FAILURE_POLICY_BYPASS => {
            Ok(animus_runtime::LifecycleHookFailureClass::PolicyBypass)
        }
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_LIFECYCLE_FAILURE_FIELD,
            reason: UNKNOWN_LIFECYCLE_FAILURE_REASON,
        }),
    }
}

fn lifecycle_redaction_from_str(
    redaction: &str,
) -> Result<animus_runtime::LifecycleHookRedaction, crate::CodecError> {
    match redaction {
        LIFECYCLE_REDACTION_NO_SENSITIVE_BODY => {
            Ok(animus_runtime::LifecycleHookRedaction::NoSensitiveBody)
        }
        LIFECYCLE_REDACTION_BODY_REDACTED => {
            Ok(animus_runtime::LifecycleHookRedaction::BodyRedacted)
        }
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_LIFECYCLE_REDACTION_FIELD,
            reason: UNKNOWN_LIFECYCLE_REDACTION_REASON,
        }),
    }
}

fn exec_decision_to_string(decision: animus_adapters::ExecPolicyDecision) -> String {
    String::from(decision.as_str())
}

fn exec_decision_from_str(
    decision: &str,
) -> Result<animus_adapters::ExecPolicyDecision, crate::CodecError> {
    match decision {
        EXEC_DECISION_ALLOW => Ok(animus_adapters::ExecPolicyDecision::Allow),
        EXEC_DECISION_PROMPT => Ok(animus_adapters::ExecPolicyDecision::Prompt),
        EXEC_DECISION_FORBID => Ok(animus_adapters::ExecPolicyDecision::Forbid),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_DECISION_FIELD,
            reason: UNKNOWN_EXEC_DECISION_REASON,
        }),
    }
}

fn stream_kind_to_string(kind: animus_adapters::ModelStreamFrameKind) -> String {
    String::from(kind.as_str())
}

fn stream_kind_from_str(
    kind: &str,
) -> Result<animus_adapters::ModelStreamFrameKind, crate::CodecError> {
    match kind {
        STREAM_CONTENT_DELTA => Ok(animus_adapters::ModelStreamFrameKind::ContentDelta),
        STREAM_THOUGHT_DELTA => Ok(animus_adapters::ModelStreamFrameKind::ThoughtDelta),
        STREAM_TOOL_CALL_DELTA => Ok(animus_adapters::ModelStreamFrameKind::ToolCallDelta),
        STREAM_USAGE => Ok(animus_adapters::ModelStreamFrameKind::Usage),
        STREAM_FINISH => Ok(animus_adapters::ModelStreamFrameKind::Finish),
        STREAM_CANCELLED => Ok(animus_adapters::ModelStreamFrameKind::Cancelled),
        STREAM_FAILURE => Ok(animus_adapters::ModelStreamFrameKind::Failure),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_STREAM_KIND_FIELD,
            reason: UNKNOWN_STREAM_KIND_REASON,
        }),
    }
}

fn stream_visibility_to_string(visibility: animus_adapters::ModelStreamVisibility) -> String {
    String::from(visibility.as_str())
}

fn stream_visibility_from_str(
    visibility: &str,
) -> Result<animus_adapters::ModelStreamVisibility, crate::CodecError> {
    match visibility {
        STREAM_ASSISTANT_VISIBLE => Ok(animus_adapters::ModelStreamVisibility::AssistantVisible),
        STREAM_POLICY_GATED => Ok(animus_adapters::ModelStreamVisibility::PolicyGated),
        STREAM_REDACTED => Ok(animus_adapters::ModelStreamVisibility::Redacted),
        _other => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_STREAM_VISIBILITY_FIELD,
            reason: UNKNOWN_STREAM_VISIBILITY_REASON,
        }),
    }
}

fn stream_usage(
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
) -> Result<Option<animus_adapters::ModelStreamUsage>, crate::CodecError> {
    match (input_tokens, output_tokens) {
        (Some(input_tokens), Some(output_tokens)) => Ok(Some(
            animus_adapters::ModelStreamUsage::new(input_tokens, output_tokens),
        )),
        (None, None) => Ok(None),
        (_partial_input, _partial_output) => Err(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_STREAM_FIELD,
            reason: INVALID_STREAM_USAGE_REASON,
        }),
    }
}

fn model_stream_error(error: animus_adapters::ModelStreamError) -> crate::CodecError {
    match error {
        animus_adapters::ModelStreamError::InvalidField { error } => error.into(),
        _other => crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_STREAM_FIELD,
            reason: INVALID_STREAM_FRAME_REASON,
        },
    }
}

struct ProviderRetryProgressParts {
    run_id: String,
    attempt_count: u32,
    max_attempt_count: u32,
    delay_ms: Option<u64>,
    reason: String,
    status: String,
    exact_plan_hash: String,
}

fn decode_provider_retry_progress(
    parts: ProviderRetryProgressParts,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    let reason = animus_runtime::provider_retry::ProviderRetryFailureClass::parse(&parts.reason)
        .ok_or(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_PROVIDER_RETRY_FIELD,
            reason: UNKNOWN_PROVIDER_RETRY_REASON,
        })?;
    let status = animus_runtime::provider_retry::ProviderRetryProgressStatus::parse(&parts.status)
        .ok_or(crate::CodecError::InvalidRuntimePayload {
            field: RUNTIME_PROVIDER_RETRY_FIELD,
            reason: UNKNOWN_PROVIDER_RETRY_STATUS,
        })?;
    let progress = animus_runtime::provider_retry::ProviderRetryProgressEvent {
        run_id: parts.run_id,
        attempt_count: parts.attempt_count,
        max_attempt_count: parts.max_attempt_count,
        delay_ms: parts.delay_ms,
        reason,
        status,
        exact_plan_hash: parts.exact_plan_hash,
    };
    let progress = animus_runtime::provider_retry::validate_provider_retry_progress_event(progress)
        .map_err(provider_retry_error)?;
    Ok(animus_runtime::RuntimeEvent::ProviderRetryProgress { progress })
}

fn provider_retry_error(
    _error: animus_runtime::provider_retry::ProviderRetryError,
) -> crate::CodecError {
    crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_PROVIDER_RETRY_FIELD,
        reason: INVALID_PROVIDER_RETRY_EVENT,
    }
}

struct SandboxEscalationParts<'a> {
    decision: &'a str,
    rule_ids: Vec<String>,
    command_fingerprints: Vec<String>,
    active_sandbox: &'a str,
    requested_capability: &'a str,
    escalation_reason: &'a str,
    redaction_marker: &'a str,
}

fn decode_sandbox_escalation(
    parts: SandboxEscalationParts<'_>,
) -> Result<animus_runtime::RuntimeEvent, crate::CodecError> {
    let sandbox = animus_adapters::ExecPolicySandboxContext::new(
        &animus_adapters::ExecPolicySandboxContextDraft {
            active_sandbox: Some(parts.active_sandbox),
            requested_capability: Some(parts.requested_capability),
            escalation_reason: Some(parts.escalation_reason),
        },
    )
    .map_err(|_error| crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_SANDBOX_FIELD,
        reason: INVALID_SANDBOX_CONTEXT_REASON,
    })?;
    validate_command_fingerprints(parts.command_fingerprints.as_slice())?;
    validate_redaction_marker(parts.redaction_marker)?;
    Ok(animus_runtime::RuntimeEvent::SandboxEscalation {
        decision: exec_decision_from_str(parts.decision)?,
        rule_ids: parts.rule_ids,
        command_fingerprints: parts.command_fingerprints,
        active_sandbox: String::from(sandbox.active_sandbox()),
        requested_capability: String::from(sandbox.requested_capability()),
        escalation_reason: String::from(sandbox.escalation_reason()),
        redaction_marker: String::from(parts.redaction_marker),
    })
}

fn validate_command_fingerprints(fingerprints: &[String]) -> Result<(), crate::CodecError> {
    for fingerprint in fingerprints {
        animus_adapters::SafeHash::new(fingerprint)?;
    }
    Ok(())
}

fn validate_redaction_marker(redaction_marker: &str) -> Result<(), crate::CodecError> {
    if redaction_marker == COMMAND_HASH_ONLY_REDACTION {
        return Ok(());
    }
    Err(crate::CodecError::InvalidRuntimePayload {
        field: RUNTIME_REDACTION_FIELD,
        reason: UNKNOWN_REDACTION_REASON,
    })
}

fn terminal_to_string(status: animus_runtime::RuntimeTerminalKind) -> String {
    String::from(status.as_str())
}

fn terminal_from_str(
    status: &str,
) -> Result<animus_runtime::RuntimeTerminalKind, crate::CodecError> {
    match status {
        TERMINAL_FINAL_RESPONSE => Ok(animus_runtime::RuntimeTerminalKind::FinalResponse),
        TERMINAL_COMPLETION_CANDIDATE => {
            Ok(animus_runtime::RuntimeTerminalKind::CompletionCandidate)
        }
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
    const TOOL_ARGUMENT_NAME: &str = "query";
    const TOOL_SCHEMA: &str = "schema.search.v1";
    const TOOL_CALL_ID: &str = "tool-call-1";
    const TOOL_VISIBLE_BUDGET: u32 = 64;
    const TOOL_MUTATION_TARGET: &str = "resource:search";
    const TOOL_CAPABILITY: &str = "tool.search";
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
    const EXEC_RULE_ID: &str = "allow-cargo";
    const EXEC_SANDBOX: &str = "workspace-readonly";
    const EXEC_CAPABILITY: &str = "exec:outside-sandbox";
    const EXEC_REASON: &str = "sandbox-escalation";
    const EXEC_COMMAND_HASH: &str =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const BAD_COMMAND_HASH: &str = "not-a-blake3-hash";
    const COMPLETION_REQUEST: &str = "COMPLETE";
    const INVALID_COMPLETION_REQUEST: &str = "COMPLETE later";
    const LIFECYCLE_HOOK_ID: &str = "audit_hook";
    const LIFECYCLE_EVENT_ID: &str = "event-1";
    const LIFECYCLE_EVENT_KIND: &str = "pre_tool_use";
    const LIFECYCLE_RESOURCE_CLASS: &str = "tool";
    const LIFECYCLE_SAFE_LABEL: &str = "search";
    const LIFECYCLE_SUMMARY: &str = "tool request ready";
    const LIFECYCLE_BODY: &str = "redacted tool body";
    const LIFECYCLE_STATUS: &str = "pending";
    const LIFECYCLE_DEFINITION_BODY: &str = "review hook";
    const STREAM_CALL_ID: &str = "call-1";
    const STREAM_BODY: &str = "hello";
    const STREAM_ORDER: u64 = 1;
    const RETRY_RUN_ID: &str = "run.retry";
    const RETRY_ATTEMPT_COUNT: u32 = 2;
    const RETRY_MAX_ATTEMPT_COUNT: u32 = 4;
    const RETRY_DELAY_MS: u64 = 250;
    const RETRY_EXACT_PLAN_HASH: &str =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const MAX_TURNS: usize = 2;

    fn steel_policy_denied_event() -> animus_runtime::RuntimeEvent {
        animus_runtime::RuntimeEvent::SteelPolicyDenied {
            profile: String::from(STEEL_PROFILE),
            script_hash: String::from(STEEL_SCRIPT_HASH),
            receipt_hash: String::from(STEEL_RECEIPT_HASH),
            reason: String::from(STEEL_REASON),
        }
    }

    fn sandbox_escalation_event() -> animus_runtime::RuntimeEvent {
        animus_runtime::RuntimeEvent::SandboxEscalation {
            decision: animus_adapters::ExecPolicyDecision::Allow,
            rule_ids: vec![String::from(EXEC_RULE_ID)],
            command_fingerprints: vec![String::from(EXEC_COMMAND_HASH)],
            active_sandbox: String::from(EXEC_SANDBOX),
            requested_capability: String::from(EXEC_CAPABILITY),
            escalation_reason: String::from(EXEC_REASON),
            redaction_marker: String::from(COMMAND_HASH_ONLY_REDACTION),
        }
    }

    fn lifecycle_hook_outcome_event() -> animus_runtime::RuntimeEvent {
        let definition = animus_runtime::LifecycleHookDefinition::validate(
            &animus_runtime::LifecycleHookDefinitionDraft {
                hook_id: LIFECYCLE_HOOK_ID,
                event_kind: LIFECYCLE_EVENT_KIND,
                source: animus_runtime::LifecycleHookDefinitionSource::Project,
                capabilities: &[animus_runtime::LifecycleHookCapability::Observe],
                definition_body: LIFECYCLE_DEFINITION_BODY,
            },
        )
        .expect("lifecycle hook definition should validate");
        let input = animus_runtime::LifecycleHookInput::validate(
            &animus_runtime::LifecycleHookInputDraft {
                event_id: LIFECYCLE_EVENT_ID,
                event_kind: LIFECYCLE_EVENT_KIND,
                resource_class: LIFECYCLE_RESOURCE_CLASS,
                safe_label: LIFECYCLE_SAFE_LABEL,
                summary: LIFECYCLE_SUMMARY,
                sensitive_body: Some(LIFECYCLE_BODY),
                status: Some(LIFECYCLE_STATUS),
            },
        )
        .expect("lifecycle hook input should validate");
        let outcome =
            animus_runtime::LifecycleHookOutcome::new(animus_runtime::LifecycleHookOutcomeDraft {
                definition: &definition,
                input: &input,
                trust_status: animus_runtime::LifecycleHookTrustStatus::Trusted {
                    scope: animus_runtime::LifecycleHookTrustScope::Project,
                },
                outcome_class: animus_runtime::LifecycleHookOutcomeClass::Applied,
                failure_class: None,
            });
        animus_runtime::RuntimeEvent::LifecycleHookOutcome { outcome }
    }

    fn provider_retry_progress_event() -> animus_runtime::RuntimeEvent {
        animus_runtime::RuntimeEvent::ProviderRetryProgress {
            progress: animus_runtime::provider_retry::ProviderRetryProgressEvent {
                run_id: String::from(RETRY_RUN_ID),
                attempt_count: RETRY_ATTEMPT_COUNT,
                max_attempt_count: RETRY_MAX_ATTEMPT_COUNT,
                delay_ms: Some(RETRY_DELAY_MS),
                reason: animus_runtime::provider_retry::ProviderRetryFailureClass::RateLimited,
                status: animus_runtime::provider_retry::ProviderRetryProgressStatus::Scheduled,
                exact_plan_hash: String::from(RETRY_EXACT_PLAN_HASH),
            },
        }
    }

    fn tool_request() -> animus_adapters::ToolRequest {
        let fields = [animus_adapters::tool::ArgumentFieldDraft {
            name: Some(TOOL_ARGUMENT_NAME),
            value: Some(TOOL_INPUT),
        }];
        let arguments = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TOOL_SCHEMA,
                allowed_field_names: &[TOOL_ARGUMENT_NAME],
                compatibility_rules: &[],
                fields: &fields,
            },
        )
        .expect("tool arguments should validate");
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(TOOL_NAME),
            arguments: Some(&arguments),
            correlation_id: Some(TOOL_CALL_ID),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_VISIBLE_BUDGET,
            },
            mutation_targets: &[TOOL_MUTATION_TARGET],
            required_capability_labels: &[TOOL_CAPABILITY],
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
        })
        .expect("tool request should validate")
    }

    fn tool_response() -> animus_adapters::ToolResponse {
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: TOOL_OUTPUT,
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_VISIBLE_BUDGET,
            },
        )
        .expect("tool output should validate");
        animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        )
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
        let request = tool_request();
        let message = Envelope::from_tool_request(&request);
        let decoded = message
            .try_into_tool_request()
            .expect("boundary message should convert");

        assert_eq!(decoded.name(), TOOL_NAME);
        assert_eq!(decoded.schema_identity(), TOOL_SCHEMA);
        assert_eq!(
            decoded
                .arguments()
                .first()
                .expect("decoded argument is present")
                .value(),
            TOOL_INPUT
        );
    }

    #[test]
    fn tool_response_round_trips_through_contract() {
        let response = tool_response();
        let message = Envelope::from_tool_response(&response);
        let decoded = message
            .try_into_tool_response()
            .expect("boundary message should convert");

        assert_eq!(decoded.output(), TOOL_OUTPUT);
        assert_eq!(
            decoded.redaction_class(),
            animus_adapters::tool::OutputRedactionClass::Safe
        );
    }

    #[test]
    fn runtime_event_round_trips_through_contract() {
        let event = animus_runtime::RuntimeEvent::ToolRequested {
            request: tool_request(),
        };
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("runtime event should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn provider_retry_progress_round_trips_without_transcript_material() {
        let event = provider_retry_progress_event();
        let envelope = Envelope::from_runtime_event(&event);
        let rendered = format!("{envelope:?}");
        let decoded = envelope
            .try_into_runtime_event()
            .expect("retry progress should convert");

        assert_eq!(decoded, event);
        assert!(!rendered.contains(MODEL_PROMPT));
        assert!(!rendered.contains(MODEL_MESSAGE));
    }

    #[test]
    fn provider_retry_progress_rejects_unknown_and_unsafe_metadata() {
        let unknown_reason = Envelope::RuntimeProviderRetryProgress {
            run_id: String::from(RETRY_RUN_ID),
            attempt_count: RETRY_ATTEMPT_COUNT,
            max_attempt_count: RETRY_MAX_ATTEMPT_COUNT,
            delay_ms: Some(RETRY_DELAY_MS),
            reason: String::from("provider-native-secret"),
            status: String::from("scheduled"),
            exact_plan_hash: String::from(RETRY_EXACT_PLAN_HASH),
        };
        let unknown_status = Envelope::RuntimeProviderRetryProgress {
            run_id: String::from(RETRY_RUN_ID),
            attempt_count: RETRY_ATTEMPT_COUNT,
            max_attempt_count: RETRY_MAX_ATTEMPT_COUNT,
            delay_ms: Some(RETRY_DELAY_MS),
            reason: String::from("rate_limited"),
            status: String::from("provider-native-status"),
            exact_plan_hash: String::from(RETRY_EXACT_PLAN_HASH),
        };
        let unsafe_identity = Envelope::RuntimeProviderRetryProgress {
            run_id: String::from("raw private run identity"),
            attempt_count: RETRY_ATTEMPT_COUNT,
            max_attempt_count: RETRY_MAX_ATTEMPT_COUNT,
            delay_ms: Some(RETRY_DELAY_MS),
            reason: String::from("rate_limited"),
            status: String::from("scheduled"),
            exact_plan_hash: String::from(RETRY_EXACT_PLAN_HASH),
        };

        assert!(matches!(
            unknown_reason.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_PROVIDER_RETRY_FIELD,
                reason: UNKNOWN_PROVIDER_RETRY_REASON,
            })
        ));
        assert!(matches!(
            unknown_status.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_PROVIDER_RETRY_FIELD,
                reason: UNKNOWN_PROVIDER_RETRY_STATUS,
            })
        ));
        assert!(matches!(
            unsafe_identity.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_PROVIDER_RETRY_FIELD,
                reason: INVALID_PROVIDER_RETRY_EVENT,
            })
        ));
    }

    #[test]
    fn model_stream_events_round_trip_through_contract() {
        let frame =
            animus_adapters::ModelStreamFrame::from_draft(animus_adapters::ModelStreamFrameDraft {
                call_id: STREAM_CALL_ID,
                order: STREAM_ORDER,
                kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
                body: Some(STREAM_BODY),
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
            })
            .expect("valid stream frame should parse");
        let events = [
            animus_runtime::RuntimeEvent::ModelStreamStarted {
                call_id: animus_adapters::ModelStreamCallId::new(STREAM_CALL_ID)
                    .expect("valid call id should parse"),
            },
            animus_runtime::RuntimeEvent::ModelStreamFrame { frame },
            animus_runtime::RuntimeEvent::ModelStreamEnded {
                call_id: animus_adapters::ModelStreamCallId::new(STREAM_CALL_ID)
                    .expect("valid call id should parse"),
                status: animus_adapters::ModelStreamFrameKind::Finish,
            },
            animus_runtime::RuntimeEvent::ModelStreamFailed {
                call_id: animus_adapters::ModelStreamCallId::new(STREAM_CALL_ID)
                    .expect("valid call id should parse"),
                reason: String::from("safe-failure"),
            },
        ];

        for event in events {
            let message = Envelope::from_runtime_event(&event);
            let decoded = message
                .try_into_runtime_event()
                .expect("stream runtime event should convert");
            assert_eq!(decoded, event);
        }
    }

    #[test]
    fn model_stream_rejects_unknown_kind_or_partial_usage() {
        let bad_kind = Envelope::RuntimeModelStreamEnded {
            call_id: String::from(STREAM_CALL_ID),
            status: String::from("provider-specific-terminal"),
        };
        let partial_usage = Envelope::RuntimeModelStreamFrame {
            call_id: String::from(STREAM_CALL_ID),
            order: STREAM_ORDER,
            frame_kind: String::from(STREAM_USAGE),
            body: None,
            tool_call_id: None,
            usage_input_tokens: Some(1),
            usage_output_tokens: None,
            visibility: String::from(STREAM_REDACTED),
        };

        assert!(matches!(
            bad_kind.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_STREAM_KIND_FIELD,
                reason: UNKNOWN_STREAM_KIND_REASON,
            })
        ));
        assert!(matches!(
            partial_usage.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_STREAM_FIELD,
                reason: INVALID_STREAM_USAGE_REASON,
            })
        ));
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
    fn sandbox_escalation_event_round_trips_with_hash_only_evidence() {
        let event = sandbox_escalation_event();
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("sandbox escalation event should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn sandbox_escalation_rejects_bad_hash_or_redaction_marker() {
        let bad_hash = Envelope::RuntimeSandboxEscalation {
            decision: String::from(EXEC_DECISION_ALLOW),
            rule_ids: vec![String::from(EXEC_RULE_ID)],
            command_fingerprints: vec![String::from(BAD_COMMAND_HASH)],
            active_sandbox: String::from(EXEC_SANDBOX),
            requested_capability: String::from(EXEC_CAPABILITY),
            escalation_reason: String::from(EXEC_REASON),
            redaction_marker: String::from(COMMAND_HASH_ONLY_REDACTION),
        };
        let bad_redaction = Envelope::RuntimeSandboxEscalation {
            decision: String::from(EXEC_DECISION_ALLOW),
            rule_ids: vec![String::from(EXEC_RULE_ID)],
            command_fingerprints: vec![String::from(EXEC_COMMAND_HASH)],
            active_sandbox: String::from(EXEC_SANDBOX),
            requested_capability: String::from(EXEC_CAPABILITY),
            escalation_reason: String::from(EXEC_REASON),
            redaction_marker: String::from("raw-command"),
        };

        assert!(matches!(
            bad_hash.try_into_runtime_event(),
            Err(crate::CodecError::AdapterContract { .. })
        ));
        assert!(matches!(
            bad_redaction.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_REDACTION_FIELD,
                reason: UNKNOWN_REDACTION_REASON,
            })
        ));
    }

    #[test]
    fn lifecycle_hook_outcome_round_trips_through_contract() {
        let event = lifecycle_hook_outcome_event();
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("lifecycle hook outcome should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn lifecycle_hook_outcome_rejects_missing_trust_scope() {
        let mut message = Envelope::from_runtime_event(&lifecycle_hook_outcome_event());
        match &mut message {
            Envelope::RuntimeLifecycleHookOutcome { trust_scope, .. } => {
                *trust_scope = None;
            }
            _other => unreachable!("lifecycle event should encode as lifecycle outcome"),
        }

        assert!(matches!(
            message.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_LIFECYCLE_SCOPE_FIELD,
                reason: INVALID_LIFECYCLE_SCOPE_REASON,
            })
        ));
    }

    #[test]
    fn completion_candidate_request_round_trips_with_hash_only_evidence() {
        let event = animus_runtime::RuntimeEvent::CompletionCandidateRequested {
            request_hash: String::from(STEEL_SCRIPT_HASH),
        };
        let message = Envelope::from_runtime_event(&event);
        let decoded = message
            .try_into_runtime_event()
            .expect("completion candidate request should convert");

        assert_eq!(decoded, event);
    }

    #[test]
    fn completion_candidate_request_rejects_non_hash_evidence() {
        let message = Envelope::RuntimeCompletionCandidateRequested {
            request_hash: String::from(BAD_COMMAND_HASH),
        };

        assert!(matches!(
            message.try_into_runtime_event(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_COMPLETION_CANDIDATE_FIELD,
                reason: INVALID_COMPLETION_CANDIDATE_HASH_REASON,
            })
        ));
    }

    #[test]
    fn completion_candidate_report_round_trips_through_contract() {
        let candidate = completion_candidate_from_request(COMPLETION_REQUEST)
            .expect("exact completion candidate should validate");
        let report = animus_runtime::RunReport::new(
            vec![animus_runtime::RuntimeEvent::CompletionCandidateRequested {
                request_hash: String::from(STEEL_SCRIPT_HASH),
            }],
            animus_runtime::RuntimeStatus::CompletionCandidate { candidate },
        );
        let message = Envelope::from_runtime_report(&report);
        let decoded = message
            .try_into_runtime_report()
            .expect("completion candidate report should convert");

        assert_eq!(decoded, report);

        let shutdown = Envelope::RuntimeShutdown {
            status: String::from(TERMINAL_COMPLETION_CANDIDATE),
        }
        .try_into_runtime_event()
        .expect("completion candidate terminal status should convert");
        assert_eq!(
            shutdown,
            animus_runtime::RuntimeEvent::Shutdown {
                status: animus_runtime::RuntimeTerminalKind::CompletionCandidate,
            }
        );
    }

    #[test]
    fn completion_candidate_report_rejects_incidental_substring() {
        let status = RuntimeReportStatus::CompletionCandidate {
            request: String::from(INVALID_COMPLETION_REQUEST),
        };

        assert!(matches!(
            status.into_runtime_status(),
            Err(crate::CodecError::InvalidRuntimePayload {
                field: RUNTIME_COMPLETION_CANDIDATE_FIELD,
                reason: INVALID_COMPLETION_CANDIDATE_REQUEST_REASON,
            })
        ));
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
