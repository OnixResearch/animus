//! Pure coding-session lifecycle composition over explicit effect requests.
//!
//! The reducer owns deterministic run, queue, durable-boundary, recovery,
//! cancellation, compaction, and terminal transitions. It never calls a model,
//! tool, store, timer, policy service, cancellation handle, summarizer, terminal,
//! filesystem, network, process, clock, or credential source.

mod core;
mod data;
#[path = "receipt.rs"]
mod evidence;

pub use core::{
    coding_session_effect_step_bound, decode_coding_session_message, encode_coding_session_message,
    new_coding_session_state, reduce_coding_session_command, reduce_coding_session_frontend_event,
    reduce_coding_session_outcome,
};
pub use data::{
    CODING_SESSION_EFFECT_HASH_DOMAIN, CODING_SESSION_EMPTY_HEAD_HASH,
    CODING_SESSION_MESSAGE_HASH_DOMAIN, CODING_SESSION_RECEIPT_HASH_DOMAIN,
    CODING_SESSION_RECEIPT_SCHEMA, CodingSessionAfterCommit, CodingSessionCancellationRequest,
    CodingSessionCommand, CodingSessionCommitRequest, CodingSessionConfig,
    CodingSessionEffectOutcome, CodingSessionEffectRequest, CodingSessionEffectService,
    CodingSessionError, CodingSessionFrontendState, CodingSessionInput, CodingSessionInputPosture,
    CodingSessionIssue, CodingSessionMessage, CodingSessionMessageKind, CodingSessionProviderReply,
    CodingSessionProviderRequest, CodingSessionQueueBehavior, CodingSessionQueueProfile,
    CodingSessionReceipt, CodingSessionRecoveryPosture, CodingSessionRecoveryRequest,
    CodingSessionRecoverySnapshot, CodingSessionRecoveryStatus, CodingSessionRun,
    CodingSessionRunStatus, CodingSessionSensitiveDraft, CodingSessionServiceIssue,
    CodingSessionServicePosture, CodingSessionServiceSet, CodingSessionState,
    CodingSessionSummarizerRequest, CodingSessionTerminalKind, CodingSessionToolRequest,
    CodingSessionTransition,
};
pub use evidence::project_coding_session_receipt;

#[cfg(test)]
mod tests;
