//! Transport-neutral ACP host data.

#![allow(missing_docs)]

use crate::identities::{
    AcpConnectionIdentity, AcpCorrelationId, AcpEntryId, AcpRunId, AcpSessionId, AnimusSessionId,
    AuthorityProfileIdentity, SessionIdentity, SessionReference, WorkspaceIdentity,
};

pub const ACP_PROTOCOL_COHORT: &str = "agent-client-protocol/0.10.4";
pub const ACP_PROTOCOL_LICENSE: &str = "Apache-2.0";
pub const ACP_PROTOCOL_VERSION: &str = "1";
pub const ACP_RECEIPT_SCHEMA: &str = "animus.acp-stdio-host-receipt.v1";
pub const ACP_RECEIPT_HASH_DOMAIN: &str = "animus.acp-stdio-host-receipt.v1";
pub const ACP_CONNECTION_HASH_DOMAIN: &str = "animus.acp-stdio-connection.v1";
pub const ACP_CORRELATION_HASH_DOMAIN: &str = "animus.acp-stdio-correlation.v1";
pub const ACP_SESSION_HASH_DOMAIN: &str = "animus.acp-stdio-session.v1";
pub const ACP_WORKSPACE_HASH_DOMAIN: &str = "animus.acp-stdio-workspace.v1";
pub const ACP_REDACTION_MARKER: &str =
    "raw-frames-prompts-tools-provider-paths-credentials-and-reasoning-omitted";
pub const ACP_MAX_FRAME_BYTES: usize = 65_536;
pub const ACP_MAX_PROMPT_BYTES: usize = 8_192;
pub const ACP_MAX_SAFE_LABEL_BYTES: usize = 128;
pub const ACP_MAX_UPDATES_PER_OPERATION: usize = 256;
pub const ACP_MAX_UPDATE_BYTES_PER_OPERATION: usize = 262_144;
pub const ACP_MAX_CORRELATIONS: usize = 128;
pub const ACP_MAX_FRAMES_PER_CONNECTION: u32 = 4_096;
pub const ACP_MAX_PERMISSION_REQUESTS: u32 = 128;
pub const ACP_COUNTER_INCREMENT: u32 = 1;
const ACP_FEATURE_COUNT: usize = 6;
pub const ACP_HASH_HEX_BYTES: usize = 64;
pub const ACP_NON_CLAIMS: &[&str] = &[
    "fixture-client evidence does not prove arbitrary ACP client or editor compatibility",
    "stdio evidence does not prove WebSocket HTTP TLS socket or remote transport support",
    "client approval does not prove authorization sandbox enforcement capability or executor readiness",
    "session continuity does not prove replay equivalence provider quality tool correctness or release eligibility",
    "the adapter does not provide MCP serving plugin hosting terminal multiplexing or production readiness",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpFeature {
    TextPrompt,
    ResourceLinkPrompt,
    LoadSession,
    SessionUpdates,
    ClientPermission,
    Cancellation,
}

impl AcpFeature {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextPrompt => "text_prompt",
            Self::ResourceLinkPrompt => "resource_link_prompt",
            Self::LoadSession => "load_session",
            Self::SessionUpdates => "session_updates",
            Self::ClientPermission => "client_permission",
            Self::Cancellation => "cancellation",
        }
    }

    pub const ALL: [Self; ACP_FEATURE_COUNT] = [
        Self::TextPrompt,
        Self::ResourceLinkPrompt,
        Self::LoadSession,
        Self::SessionUpdates,
        Self::ClientPermission,
        Self::Cancellation,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpTransportProfile {
    pub frame_bytes: usize,
    pub frames_per_connection: u32,
    pub updates_per_operation: usize,
    pub update_bytes_per_operation: usize,
    pub permission_requests_per_connection: u32,
}

impl AcpTransportProfile {
    #[must_use]
    pub const fn reviewed() -> Self {
        Self {
            frame_bytes: ACP_MAX_FRAME_BYTES,
            frames_per_connection: ACP_MAX_FRAMES_PER_CONNECTION,
            updates_per_operation: ACP_MAX_UPDATES_PER_OPERATION,
            update_bytes_per_operation: ACP_MAX_UPDATE_BYTES_PER_OPERATION,
            permission_requests_per_connection: ACP_MAX_PERMISSION_REQUESTS,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConnectionProfile {
    pub protocol_version: String,
    pub cohort: String,
    pub features: Vec<AcpFeature>,
    pub transport: AcpTransportProfile,
}

impl AcpConnectionProfile {
    #[must_use]
    pub fn reviewed() -> Self {
        Self {
            protocol_version: String::from(ACP_PROTOCOL_VERSION),
            cohort: String::from(ACP_PROTOCOL_COHORT),
            features: AcpFeature::ALL.to_vec(),
            transport: AcpTransportProfile::reviewed(),
        }
    }

    #[must_use]
    pub fn supports(&self, feature: AcpFeature) -> bool {
        self.features.contains(&feature)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpConnectionPhase {
    AwaitingInitialize,
    Ready,
    SessionBound,
    Terminal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpCorrelation {
    pub identity: AcpCorrelationId,
    pub operation: AcpOperationKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpSessionBinding {
    pub acp_session_id: AcpSessionId,
    pub animus_session_id: AnimusSessionId,
    pub session_identity: SessionIdentity,
    pub authority_profile_identity: AuthorityProfileIdentity,
    pub active_run_id: Option<AcpRunId>,
}

// r[impl agent_harness_runtime.acp_nominal_identities.state]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpConnectionState {
    pub connection_identity: AcpConnectionIdentity,
    pub profile: AcpConnectionProfile,
    pub phase: AcpConnectionPhase,
    pub session: Option<AcpSessionBinding>,
    pub pending: Vec<AcpCorrelation>,
    pub completed: Vec<AcpCorrelationId>,
    pub frame_count: u32,
    pub update_count: u32,
    pub update_bytes: u64,
    pub permission_count: u32,
    pub terminal_failure: Option<AcpFailureClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpOperationKind {
    Initialize,
    NewSession,
    LoadSession,
    Prompt,
    Continue,
    Cancel,
    Permission,
    Shutdown,
}

impl AcpOperationKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::NewSession => "new_session",
            Self::LoadSession => "load_session",
            Self::Prompt => "prompt",
            Self::Continue => "continue",
            Self::Cancel => "cancel",
            Self::Permission => "permission",
            Self::Shutdown => "shutdown",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpPromptPart {
    Text(String),
    ResourceLink {
        safe_name: String,
        resource_identity: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpDecodedRequest {
    Initialize {
        correlation_id: String,
        protocol_version: String,
        required_features: Vec<AcpFeature>,
    },
    NewSession {
        correlation_id: String,
        workspace_identity: String,
        mcp_server_count: u32,
    },
    LoadSession {
        correlation_id: String,
        session_reference: String,
        workspace_identity: String,
        mcp_server_count: u32,
    },
    Prompt {
        correlation_id: String,
        session_id: String,
        parts: Vec<AcpPromptPart>,
    },
    Cancel {
        session_id: String,
    },
    Unsupported {
        correlation_id: Option<String>,
        method: String,
    },
}

impl AcpDecodedRequest {
    #[must_use]
    pub const fn operation(&self) -> AcpOperationKind {
        match self {
            Self::Initialize { .. } => AcpOperationKind::Initialize,
            Self::NewSession { .. } => AcpOperationKind::NewSession,
            Self::LoadSession { .. } => AcpOperationKind::LoadSession,
            Self::Prompt { .. } => AcpOperationKind::Prompt,
            Self::Cancel { .. } => AcpOperationKind::Cancel,
            Self::Unsupported { .. } => AcpOperationKind::Shutdown,
        }
    }

    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        match self {
            Self::Initialize { correlation_id, .. }
            | Self::NewSession { correlation_id, .. }
            | Self::LoadSession { correlation_id, .. }
            | Self::Prompt { correlation_id, .. } => Some(correlation_id.as_str()),
            Self::Unsupported { correlation_id, .. } => correlation_id.as_deref(),
            Self::Cancel { .. } => None,
        }
    }
}

// r[impl agent_harness_runtime.acp_nominal_identities.facade]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpFacadeCommand {
    CreateSession {
        correlation_id: AcpCorrelationId,
        workspace_identity: WorkspaceIdentity,
    },
    LoadSession {
        correlation_id: AcpCorrelationId,
        session_reference: SessionReference,
        workspace_identity: WorkspaceIdentity,
    },
    Prompt {
        correlation_id: AcpCorrelationId,
        session_id: AcpSessionId,
        run_id: AcpRunId,
        entry_id: AcpEntryId,
        parts: Vec<AcpPromptPart>,
    },
    Cancel {
        session_id: AcpSessionId,
        run_id: AcpRunId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpFacadeObservation {
    pub correlation_id: Option<AcpCorrelationId>,
    pub session: AcpSessionBinding,
    pub updates: Vec<AcpUpdate>,
    pub stop: Option<AcpStopClass>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpHostOutcome {
    SessionCreated(AcpFacadeObservation),
    SessionLoaded(AcpFacadeObservation),
    PromptCompleted(AcpFacadeObservation),
    CancellationObserved(AcpFacadeObservation),
    Failed {
        correlation_id: Option<AcpCorrelationId>,
        failure: AcpFailureClass,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpStopClass {
    EndTurn,
    Cancelled,
    Refused,
}

impl AcpStopClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EndTurn => "end_turn",
            Self::Cancelled => "cancelled",
            Self::Refused => "refused",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpFailureClass {
    InvalidProfile,
    MalformedFrame,
    OversizedFrame,
    UnsupportedProtocol,
    UnsupportedFeature,
    UnsupportedMethod,
    NotInitialized,
    SessionAlreadyBound,
    SessionNotBound,
    SessionMismatch,
    DuplicateCorrelation,
    UnknownCorrelation,
    StaleCorrelation,
    OutOfOrderOutcome,
    StaleRun,
    InvalidPrompt,
    PromptTooLarge,
    FacadeRejected,
    StoreRejected,
    AuthorityProfileMismatch,
    PermissionDenied,
    PermissionUnavailable,
    PermissionWidening,
    SandboxMismatch,
    ExecutorUnavailable,
    ProjectionRejected,
    Backpressure,
    InputEof,
    ConfirmationEof,
    BrokenPipe,
    ReadFailure,
    WriteFailure,
    FlushFailure,
    DeadlineExceeded,
    CancellationAmbiguous,
    FrameLimitExceeded,
    CleanupFailed,
    SensitiveEvidence,
}

impl AcpFailureClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidProfile => "invalid_profile",
            Self::MalformedFrame => "malformed_frame",
            Self::OversizedFrame => "oversized_frame",
            Self::UnsupportedProtocol => "unsupported_protocol",
            Self::UnsupportedFeature => "unsupported_feature",
            Self::UnsupportedMethod => "unsupported_method",
            Self::NotInitialized => "not_initialized",
            Self::SessionAlreadyBound => "session_already_bound",
            Self::SessionNotBound => "session_not_bound",
            Self::SessionMismatch => "session_mismatch",
            Self::DuplicateCorrelation => "duplicate_correlation",
            Self::UnknownCorrelation => "unknown_correlation",
            Self::StaleCorrelation => "stale_correlation",
            Self::OutOfOrderOutcome => "out_of_order_outcome",
            Self::StaleRun => "stale_run",
            Self::InvalidPrompt => "invalid_prompt",
            Self::PromptTooLarge => "prompt_too_large",
            Self::FacadeRejected => "facade_rejected",
            Self::StoreRejected => "store_rejected",
            Self::AuthorityProfileMismatch => "authority_profile_mismatch",
            Self::PermissionDenied => "permission_denied",
            Self::PermissionUnavailable => "permission_unavailable",
            Self::PermissionWidening => "permission_widening",
            Self::SandboxMismatch => "sandbox_mismatch",
            Self::ExecutorUnavailable => "executor_unavailable",
            Self::ProjectionRejected => "projection_rejected",
            Self::Backpressure => "backpressure",
            Self::InputEof => "input_eof",
            Self::ConfirmationEof => "confirmation_eof",
            Self::BrokenPipe => "broken_pipe",
            Self::ReadFailure => "read_failure",
            Self::WriteFailure => "write_failure",
            Self::FlushFailure => "flush_failure",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::CancellationAmbiguous => "cancellation_ambiguous",
            Self::FrameLimitExceeded => "frame_limit_exceeded",
            Self::CleanupFailed => "cleanup_failed",
            Self::SensitiveEvidence => "sensitive_evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpProtocolError {
    pub failure: AcpFailureClass,
    pub field: &'static str,
}

impl core::fmt::Display for AcpProtocolError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "ACP protocol rejected {}", self.field)
    }
}

impl std::error::Error for AcpProtocolError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpProtocolResponse {
    Initialized {
        protocol_version: String,
        features: Vec<AcpFeature>,
    },
    SessionBound {
        session_id: String,
        loaded: bool,
    },
    PromptCompleted {
        stop: AcpStopClass,
    },
    Failure {
        failure: AcpFailureClass,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpProtocolAction {
    Respond {
        correlation_id: String,
        response: AcpProtocolResponse,
    },
    Update(AcpUpdate),
    Terminate {
        failure: AcpFailureClass,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpProtocolTransition {
    pub state: AcpConnectionState,
    pub facade_command: Option<AcpFacadeCommand>,
    pub actions: Vec<AcpProtocolAction>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpUpdateKind {
    AgentMessage,
    ToolStarted,
    ToolCompleted,
    ToolDenied,
    SessionStatus,
    QueueStatus,
    ConfirmationStatus,
    DurableBoundary,
    AuthorityStatus,
    Cancellation,
    Terminal,
    Redacted,
    Diagnostic,
}

impl AcpUpdateKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgentMessage => "agent_message",
            Self::ToolStarted => "tool_started",
            Self::ToolCompleted => "tool_completed",
            Self::ToolDenied => "tool_denied",
            Self::SessionStatus => "session_status",
            Self::QueueStatus => "queue_status",
            Self::ConfirmationStatus => "confirmation_status",
            Self::DurableBoundary => "durable_boundary",
            Self::AuthorityStatus => "authority_status",
            Self::Cancellation => "cancellation",
            Self::Terminal => "terminal",
            Self::Redacted => "redacted",
            Self::Diagnostic => "diagnostic",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpUpdate {
    pub sequence: u32,
    pub kind: AcpUpdateKind,
    pub session_id: AcpSessionId,
    pub safe_text: Option<String>,
    pub correlation_identity: Option<AcpCorrelationId>,
    pub status: String,
}

impl AcpUpdate {
    #[must_use]
    pub fn estimated_bytes(&self) -> usize {
        self.session_id
            .len()
            .saturating_add(self.safe_text.as_ref().map_or(0, String::len))
            .saturating_add(
                self.correlation_identity
                    .as_ref()
                    .map_or(0, |identity| identity.as_str().len()),
            )
            .saturating_add(self.status.len())
            .saturating_add(self.kind.as_str().len())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPermissionAdmission {
    Admitted,
    Denied,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpPermissionInput {
    pub request_id: String,
    pub session_id: AcpSessionId,
    pub tool_name: String,
    pub tool_correlation_id: AcpCorrelationId,
    pub policy: AcpPermissionAdmission,
    pub permission_mode: AcpPermissionAdmission,
    pub authorization: AcpPermissionAdmission,
    pub capability: AcpPermissionAdmission,
    pub sandbox: AcpPermissionAdmission,
    pub profile: AcpPermissionAdmission,
    pub executor: AcpPermissionAdmission,
    pub confirmation_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPermissionStatus {
    PreApproved,
    ClientDecisionRequired,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpPermissionPlan {
    pub request_id: String,
    pub session_id: AcpSessionId,
    pub tool_name: String,
    pub tool_correlation_id: AcpCorrelationId,
    pub status: AcpPermissionStatus,
    pub failure: Option<AcpFailureClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpClientPermissionDecision {
    AllowOnce,
    AllowAlways,
    DenyOnce,
    DenyAlways,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpClientPermissionRequest {
    pub request_id: String,
    pub session_id: AcpSessionId,
    pub tool_name: String,
    pub tool_correlation_identity: AcpCorrelationId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpPermissionOutcome {
    pub request_id: String,
    pub client_decision: Option<AcpClientPermissionDecision>,
    pub dispatch_allowed: bool,
    pub failure: Option<AcpFailureClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpTransportStatus {
    CleanEof,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpCleanupStatus {
    Completed,
    Failed,
    NotRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpReceiptDraft {
    pub connection_identity: AcpConnectionIdentity,
    pub session_identity: Option<SessionIdentity>,
    pub correlation_identity: Option<AcpCorrelationId>,
    pub authority_profile_identity: Option<AuthorityProfileIdentity>,
    pub operation: AcpOperationKind,
    pub status: AcpTransportStatus,
    pub failure: Option<AcpFailureClass>,
    pub cleanup: AcpCleanupStatus,
    pub frame_count: u32,
    pub update_count: u32,
    pub update_bytes: u64,
    pub tool_count: u32,
    pub permission_count: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AcpSensitiveDraft<'a> {
    pub raw_frame: Option<&'a str>,
    pub prompt: Option<&'a str>,
    pub tool_arguments: Option<&'a str>,
    pub tool_output: Option<&'a str>,
    pub provider_payload: Option<&'a str>,
    pub credential: Option<&'a str>,
    pub environment_value: Option<&'a str>,
    pub physical_path: Option<&'a str>,
    pub hidden_reasoning: Option<&'a str>,
    pub raw_diagnostic: Option<&'a str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpReceipt {
    pub schema: &'static str,
    pub cohort: &'static str,
    pub license: &'static str,
    pub protocol_version: &'static str,
    pub feature_labels: Vec<String>,
    pub operation: String,
    pub status: String,
    pub failure: Option<String>,
    pub connection_identity: String,
    pub session_identity: Option<String>,
    pub correlation_identity: Option<String>,
    pub authority_profile_identity: Option<String>,
    pub frame_count: u32,
    pub update_count: u32,
    pub update_bytes: u64,
    pub tool_count: u32,
    pub permission_count: u32,
    pub cleanup_status: String,
    pub redaction_marker: &'static str,
    pub non_claims: Vec<String>,
    pub receipt_identity: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AcpHostMessage {
    Response {
        correlation_id: String,
        payload: serde_json::Value,
    },
    ErrorResponse {
        correlation_id: String,
        failure: AcpFailureClass,
    },
    Notification {
        method: String,
        payload: serde_json::Value,
    },
    Request {
        correlation_id: String,
        method: String,
        payload: serde_json::Value,
    },
}
