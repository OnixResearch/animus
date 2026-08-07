//! Optional bounded ACP stdio hosting for admitted Animus coding sessions.
//!
//! The crate pins one reviewed `agent-client-protocol` cohort for schema parity,
//! but owns its framing, bounds, correlation reducer, facade bridge, event
//! projection, permission intersection, and metadata-only receipts. It opens no
//! socket, performs no ambient discovery, and is absent from default workspace
//! members and default `animus` features.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

// r[impl agent_harness_runtime.acp_stdio_host.boundary]
mod admission;
mod codec;
mod core;
mod events;
mod facade;
mod identities;
mod model;
mod permission;
mod receipt;
mod transport;

// r[verify agent_harness_runtime.acp_nominal_identities.types.same_role]
// r[verify agent_harness_runtime.acp_nominal_identities.compile_time]
/// Opt-in compile-time fixtures for nominal ACP roles.
///
/// Same-role values compile:
///
/// ```
/// use animus_acp_stdio_host::nominal_compile_fixture::{acp_session, needs_acp_session};
/// needs_acp_session(acp_session());
/// ```
///
/// ACP and Animus session roles do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{animus_session, needs_acp_session};
/// needs_acp_session(animus_session());
/// ```
///
/// Session and connection identities do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{connection, needs_session_identity};
/// needs_session_identity(connection());
/// ```
///
/// Correlation and run identities do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{needs_correlation, run};
/// needs_correlation(run());
/// ```
///
/// Run and entry identities do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{entry, needs_run};
/// needs_run(entry());
/// ```
///
/// Workspace and resource identities do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{needs_workspace, resource};
/// needs_workspace(resource());
/// ```
///
/// Session and authority-profile identities do not interchange:
///
/// ```compile_fail
/// use animus_acp_stdio_host::nominal_compile_fixture::{authority, needs_session_identity};
/// needs_session_identity(authority());
/// ```
#[cfg(feature = "nominal-compile-fixtures")]
#[doc(hidden)]
#[allow(
    missing_docs,
    reason = "fixture-only role constructors are documented by compile examples"
)]
pub mod nominal_compile_fixture {
    pub use crate::identities::{
        AcpConnectionIdentity, AcpCorrelationId, AcpEntryId, AcpRunId, AcpSessionId,
        AnimusSessionId, AuthorityProfileIdentity, ResourceIdentity, SessionIdentity,
        WorkspaceIdentity,
    };

    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[must_use]
    pub fn acp_session() -> AcpSessionId {
        AcpSessionId::fixture("session.fixture")
    }
    #[must_use]
    pub fn animus_session() -> AnimusSessionId {
        AnimusSessionId::fixture(HASH)
    }
    #[must_use]
    pub fn connection() -> AcpConnectionIdentity {
        AcpConnectionIdentity::fixture(HASH)
    }
    #[must_use]
    pub fn session_identity() -> SessionIdentity {
        SessionIdentity::fixture(HASH)
    }
    #[must_use]
    pub fn correlation() -> AcpCorrelationId {
        AcpCorrelationId::fixture("rpc.fixture")
    }
    #[must_use]
    pub fn run() -> AcpRunId {
        AcpRunId::fixture("run.fixture")
    }
    #[must_use]
    pub fn entry() -> AcpEntryId {
        AcpEntryId::fixture("entry.fixture")
    }
    #[must_use]
    pub fn workspace() -> WorkspaceIdentity {
        WorkspaceIdentity::fixture(HASH)
    }
    #[must_use]
    pub fn resource() -> ResourceIdentity {
        ResourceIdentity::fixture(HASH)
    }
    #[must_use]
    pub fn authority() -> AuthorityProfileIdentity {
        AuthorityProfileIdentity::fixture(HASH)
    }

    pub fn needs_acp_session(_value: AcpSessionId) {}
    pub fn needs_session_identity(_value: SessionIdentity) {}
    pub fn needs_correlation(_value: AcpCorrelationId) {}
    pub fn needs_run(_value: AcpRunId) {}
    pub fn needs_workspace(_value: WorkspaceIdentity) {}
}

pub use codec::{decode_acp_frame, encode_acp_message};
pub use core::{new_acp_connection_state, reduce_acp_outcome, reduce_acp_request};
pub use events::project_runtime_events;
pub use facade::{
    AcpCodingSessionFacade, AcpCodingSessionFactory, AcpFacadeError, AcpFacadePrompt,
    AcpPromptResolver, AcpSessionHost, AcpSessionHostServices, AcpSessionReceiptContext,
    AcpToolAdmissionService,
};
pub use model::{
    ACP_PROTOCOL_COHORT, ACP_PROTOCOL_LICENSE, ACP_PROTOCOL_VERSION, AcpCleanupStatus,
    AcpClientPermissionDecision, AcpClientPermissionRequest, AcpConnectionPhase,
    AcpConnectionProfile, AcpConnectionState, AcpCorrelation, AcpDecodedRequest, AcpFacadeCommand,
    AcpFacadeObservation, AcpFailureClass, AcpFeature, AcpHostMessage, AcpHostOutcome,
    AcpOperationKind, AcpPermissionAdmission, AcpPermissionInput, AcpPermissionOutcome,
    AcpPermissionPlan, AcpPermissionStatus, AcpPromptPart, AcpProtocolAction, AcpProtocolError,
    AcpProtocolResponse, AcpProtocolTransition, AcpReceipt, AcpReceiptDraft, AcpSensitiveDraft,
    AcpSessionBinding, AcpStopClass, AcpTransportProfile, AcpTransportStatus, AcpUpdate,
    AcpUpdateKind,
};
pub use permission::{apply_permission_decision, plan_acp_permission};
pub use receipt::project_acp_receipt;
pub use transport::{
    AcpDiagnosticSink, AcpPeer, AcpStdioService, AcpTransportError, AcpTransportIo,
    AcpTransportReport, AcpTransportSignal, NoopTransportSignal, StderrDiagnosticSink,
    run_acp_stdio, run_acp_transport,
};
