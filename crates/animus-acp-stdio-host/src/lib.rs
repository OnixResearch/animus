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
mod codec;
mod core;
mod events;
mod facade;
mod model;
mod permission;
mod receipt;
mod transport;

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
