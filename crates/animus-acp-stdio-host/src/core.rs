//! Pure ACP connection and session reducer.

#![allow(missing_docs)]

use crate::admission::{AcpAdmittedPromptPart, AcpAdmittedRequest, admit_request};
use crate::identities::{
    AcpConnectionIdentity, AcpCorrelationId, AcpEntryId, AcpRunId, AcpSessionId, SessionReference,
    WorkspaceIdentity,
};
use crate::model::{
    ACP_CONNECTION_HASH_DOMAIN, ACP_CORRELATION_HASH_DOMAIN, ACP_MAX_CORRELATIONS,
    ACP_MAX_PROMPT_BYTES, ACP_MAX_SAFE_LABEL_BYTES, ACP_PROTOCOL_COHORT, ACP_PROTOCOL_VERSION,
    AcpConnectionPhase, AcpConnectionProfile, AcpConnectionState, AcpCorrelation,
    AcpDecodedRequest, AcpFacadeCommand, AcpFailureClass, AcpFeature, AcpHostOutcome,
    AcpOperationKind, AcpProtocolAction, AcpProtocolError, AcpProtocolResponse,
    AcpProtocolTransition, AcpSessionBinding,
};

const FIELD_PROFILE: &str = "profile";
const FIELD_CONNECTION: &str = "connection";
const FIELD_PHASE: &str = "phase";
const FIELD_CORRELATION: &str = "correlation";
const FIELD_SESSION: &str = "session";
const FIELD_PROMPT: &str = "prompt";
const FIELD_OUTCOME: &str = "outcome";
const FIELD_BOUND: &str = "bound";
const GENERATED_RUN_PREFIX: &str = "run.acp.";
const GENERATED_ENTRY_PREFIX: &str = "entry.acp.";
const GENERATED_ID_HEX_CHARS: usize = 24;

#[derive(Clone, Copy)]
struct LoadSessionRequest<'a> {
    correlation_id: &'a str,
    session_reference: &'a str,
    workspace_identity: &'a str,
    mcp_server_count: u32,
}

/// # Errors
/// Returns a typed protocol error when the connection label or reviewed profile is invalid.
pub fn new_acp_connection_state(
    connection_label: &str,
    profile: AcpConnectionProfile,
) -> Result<AcpConnectionState, AcpProtocolError> {
    validate_safe_label(connection_label, FIELD_CONNECTION)?;
    validate_profile(&profile)?;
    let connection_identity = AcpConnectionIdentity::from_domain_hash(domain_hash(
        ACP_CONNECTION_HASH_DOMAIN,
        &[connection_label],
    ));
    let state = AcpConnectionState {
        connection_identity,
        profile,
        phase: AcpConnectionPhase::AwaitingInitialize,
        session: None,
        pending: Vec::new(),
        completed: Vec::new(),
        frame_count: 0,
        update_count: 0,
        update_bytes: 0,
        permission_count: 0,
        terminal_failure: None,
    };
    debug_assert!(state.pending.is_empty());
    debug_assert!(state.session.is_none());
    Ok(state)
}

/// # Errors
/// Returns a typed protocol error when the request is malformed, out of order, or widening.
// r[impl agent_harness_runtime.acp_stdio_host.protocol_core]
// r[impl agent_harness_runtime.acp_stdio_host.session_bridge]
// r[impl agent_harness_runtime.acp_nominal_identities.state.binding]
pub fn reduce_acp_request(
    state: &AcpConnectionState,
    request: &AcpDecodedRequest,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    if state.phase == AcpConnectionPhase::Terminal {
        return Err(error(AcpFailureClass::OutOfOrderOutcome, FIELD_PHASE));
    }
    let admitted = admit_request(request)?;
    match &admitted {
        AcpAdmittedRequest::Initialize {
            correlation,
            protocol_version,
            required_features,
        } => reduce_initialize(
            state,
            correlation.as_str(),
            protocol_version,
            required_features,
        ),
        AcpAdmittedRequest::NewSession {
            correlation,
            workspace,
            mcp_server_count,
        } => reduce_new_session(
            state,
            correlation.as_str(),
            workspace.as_str(),
            *mcp_server_count,
        ),
        AcpAdmittedRequest::LoadSession {
            correlation,
            session_reference,
            workspace,
            mcp_server_count,
        } => reduce_load_session(
            state,
            LoadSessionRequest {
                correlation_id: correlation.as_str(),
                session_reference: session_reference.as_str(),
                workspace_identity: workspace.as_str(),
                mcp_server_count: *mcp_server_count,
            },
        ),
        AcpAdmittedRequest::Prompt {
            correlation,
            session,
            parts,
        } => reduce_prompt(state, correlation.as_str(), session.as_str(), parts),
        AcpAdmittedRequest::Cancel { session } => reduce_cancel(state, session.as_str()),
        AcpAdmittedRequest::Unsupported {
            correlation,
            method,
        } => reduce_unsupported(
            state,
            correlation.as_ref().map(AcpCorrelationId::as_str),
            method,
        ),
    }
}

/// # Errors
/// Returns a typed protocol error when facade evidence does not match pending protocol state.
pub fn reduce_acp_outcome(
    state: &AcpConnectionState,
    outcome: &AcpHostOutcome,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    match outcome {
        AcpHostOutcome::SessionCreated(observation) => {
            complete_session_binding(state, observation, false, AcpOperationKind::NewSession)
        }
        AcpHostOutcome::SessionLoaded(observation) => {
            complete_session_binding(state, observation, true, AcpOperationKind::LoadSession)
        }
        AcpHostOutcome::PromptCompleted(observation) => complete_prompt(state, observation),
        AcpHostOutcome::CancellationObserved(observation) => {
            complete_cancellation(state, observation)
        }
        AcpHostOutcome::Failed {
            correlation_id,
            failure,
        } => complete_failure(
            state,
            correlation_id.as_ref().map(AcpCorrelationId::as_str),
            *failure,
        ),
    }
}

fn reduce_initialize(
    state: &AcpConnectionState,
    correlation_id: &str,
    protocol_version: &str,
    required_features: &[AcpFeature],
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    if state.phase != AcpConnectionPhase::AwaitingInitialize {
        return Err(error(AcpFailureClass::OutOfOrderOutcome, FIELD_PHASE));
    }
    validate_new_correlation(state, correlation_id)?;
    if protocol_version != state.profile.protocol_version
        || protocol_version != ACP_PROTOCOL_VERSION
    {
        return Err(error(AcpFailureClass::UnsupportedProtocol, FIELD_PROFILE));
    }
    for feature in required_features {
        if !state.profile.supports(*feature) {
            return Err(error(AcpFailureClass::UnsupportedFeature, FIELD_PROFILE));
        }
    }
    let mut next = state.clone();
    next.phase = AcpConnectionPhase::Ready;
    complete_correlation(&mut next, correlation_id)?;
    let response = AcpProtocolResponse::Initialized {
        protocol_version: String::from(protocol_version),
        features: state.profile.features.clone(),
    };
    Ok(transition(
        next,
        None,
        vec![response_action(correlation_id, response)],
    ))
}

fn reduce_new_session(
    state: &AcpConnectionState,
    correlation_id: &str,
    workspace_identity: &str,
    mcp_server_count: u32,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    validate_new_correlation(state, correlation_id)?;
    require_ready_unbound(state)?;
    require_no_mcp_servers(mcp_server_count)?;
    validate_hash(workspace_identity, FIELD_SESSION)?;
    let mut next = state.clone();
    begin_correlation(&mut next, correlation_id, AcpOperationKind::NewSession)?;
    let command = AcpFacadeCommand::CreateSession {
        correlation_id: parse_correlation(correlation_id)?,
        workspace_identity: WorkspaceIdentity::parse(workspace_identity)
            .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_SESSION))?,
    };
    Ok(transition(next, Some(command), Vec::new()))
}

fn reduce_load_session(
    state: &AcpConnectionState,
    request: LoadSessionRequest<'_>,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    validate_new_correlation(state, request.correlation_id)?;
    require_ready_unbound(state)?;
    if !state.profile.supports(AcpFeature::LoadSession) {
        return Err(error(AcpFailureClass::UnsupportedFeature, FIELD_PROFILE));
    }
    require_no_mcp_servers(request.mcp_server_count)?;
    validate_safe_label(request.session_reference, FIELD_SESSION)?;
    validate_hash(request.workspace_identity, FIELD_SESSION)?;
    let mut next = state.clone();
    begin_correlation(
        &mut next,
        request.correlation_id,
        AcpOperationKind::LoadSession,
    )?;
    let command = AcpFacadeCommand::LoadSession {
        correlation_id: parse_correlation(request.correlation_id)?,
        session_reference: SessionReference::parse(request.session_reference)
            .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_SESSION))?,
        workspace_identity: WorkspaceIdentity::parse(request.workspace_identity)
            .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_SESSION))?,
    };
    Ok(transition(next, Some(command), Vec::new()))
}

fn reduce_prompt(
    state: &AcpConnectionState,
    correlation_id: &str,
    session_id: &str,
    parts: &[AcpAdmittedPromptPart],
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    let session = require_bound_session(state, session_id)?;
    if session.active_run_id.is_some() {
        return Err(error(AcpFailureClass::StaleRun, FIELD_SESSION));
    }
    let projected_parts = project_prompt_parts(parts);
    validate_prompt(projected_parts.as_slice())?;
    let mut next = state.clone();
    begin_correlation(&mut next, correlation_id, AcpOperationKind::Prompt)?;
    let correlation_hash = domain_hash(ACP_CORRELATION_HASH_DOMAIN, &[correlation_id]);
    let suffix = correlation_hash
        .get(..GENERATED_ID_HEX_CHARS)
        .ok_or_else(|| error(AcpFailureClass::InvalidPrompt, FIELD_CORRELATION))?;
    let run_id = AcpRunId::parse(format!("{GENERATED_RUN_PREFIX}{suffix}"))
        .map_err(|_| error(AcpFailureClass::InvalidPrompt, FIELD_SESSION))?;
    let entry_id = AcpEntryId::parse(format!("{GENERATED_ENTRY_PREFIX}{suffix}"))
        .map_err(|_| error(AcpFailureClass::InvalidPrompt, FIELD_SESSION))?;
    let Some(next_session) = next.session.as_mut() else {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    };
    next_session.active_run_id = Some(run_id.clone());
    let command = AcpFacadeCommand::Prompt {
        correlation_id: parse_correlation(correlation_id)?,
        session_id: AcpSessionId::parse(session_id)
            .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_SESSION))?,
        run_id,
        entry_id,
        parts: projected_parts,
    };
    Ok(transition(next, Some(command), Vec::new()))
}

fn project_prompt_parts(parts: &[AcpAdmittedPromptPart]) -> Vec<crate::model::AcpPromptPart> {
    parts
        .iter()
        .map(|part| match part {
            AcpAdmittedPromptPart::Text(text) => crate::model::AcpPromptPart::Text(text.clone()),
            AcpAdmittedPromptPart::ResourceLink {
                safe_name,
                resource_identity,
            } => crate::model::AcpPromptPart::ResourceLink {
                safe_name: safe_name.clone(),
                resource_identity: resource_identity.as_str().to_string(),
            },
        })
        .collect()
}

fn reduce_cancel(
    state: &AcpConnectionState,
    session_id: &str,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    let session = require_bound_session(state, session_id)?;
    let Some(run_id) = session.active_run_id.as_ref() else {
        return Err(error(AcpFailureClass::CancellationAmbiguous, FIELD_SESSION));
    };
    let command = AcpFacadeCommand::Cancel {
        session_id: AcpSessionId::parse(session_id)
            .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_SESSION))?,
        run_id: run_id.clone(),
    };
    Ok(transition(state.clone(), Some(command), Vec::new()))
}

fn reduce_unsupported(
    state: &AcpConnectionState,
    correlation_id: Option<&str>,
    method: &str,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    validate_safe_label(method, FIELD_PROFILE)?;
    let mut next = state.clone();
    next.phase = AcpConnectionPhase::Terminal;
    next.terminal_failure = Some(AcpFailureClass::UnsupportedMethod);
    let mut actions = Vec::new();
    if let Some(id) = correlation_id {
        validate_new_correlation(state, id)?;
        complete_correlation(&mut next, id)?;
        actions.push(response_action(
            id,
            AcpProtocolResponse::Failure {
                failure: AcpFailureClass::UnsupportedMethod,
            },
        ));
    }
    actions.push(AcpProtocolAction::Terminate {
        failure: AcpFailureClass::UnsupportedMethod,
    });
    Ok(transition(next, None, actions))
}

fn complete_session_binding(
    state: &AcpConnectionState,
    observation: &crate::model::AcpFacadeObservation,
    loaded: bool,
    operation: AcpOperationKind,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    let correlation_id = required_outcome_correlation(observation)?;
    require_pending(state, correlation_id, operation)?;
    validate_session_binding(&observation.session)?;
    let mut next = state.clone();
    remove_pending(&mut next, correlation_id)?;
    complete_correlation(&mut next, correlation_id)?;
    next.phase = AcpConnectionPhase::SessionBound;
    next.session = Some(observation.session.clone());
    let response = AcpProtocolResponse::SessionBound {
        session_id: observation.session.acp_session_id.as_str().to_string(),
        loaded,
    };
    let actions = bounded_update_actions(
        &mut next,
        observation.updates.as_slice(),
        Some(response_action(correlation_id, response)),
    )?;
    Ok(transition(next, None, actions))
}

fn complete_prompt(
    state: &AcpConnectionState,
    observation: &crate::model::AcpFacadeObservation,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    let correlation_id = required_outcome_correlation(observation)?;
    require_pending(state, correlation_id, AcpOperationKind::Prompt)?;
    validate_matching_session(state, &observation.session)?;
    let stop = observation
        .stop
        .ok_or_else(|| error(AcpFailureClass::OutOfOrderOutcome, FIELD_OUTCOME))?;
    let mut next = state.clone();
    remove_pending(&mut next, correlation_id)?;
    complete_correlation(&mut next, correlation_id)?;
    let Some(session) = next.session.as_mut() else {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    };
    session.active_run_id = None;
    let response = AcpProtocolResponse::PromptCompleted { stop };
    let actions = bounded_update_actions(
        &mut next,
        observation.updates.as_slice(),
        Some(response_action(correlation_id, response)),
    )?;
    Ok(transition(next, None, actions))
}

fn complete_cancellation(
    state: &AcpConnectionState,
    observation: &crate::model::AcpFacadeObservation,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    validate_matching_session(state, &observation.session)?;
    if observation.stop != Some(crate::model::AcpStopClass::Cancelled) {
        return Err(error(AcpFailureClass::CancellationAmbiguous, FIELD_OUTCOME));
    }
    let mut next = state.clone();
    let Some(session) = next.session.as_mut() else {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    };
    session.active_run_id = None;
    let actions = bounded_update_actions(&mut next, observation.updates.as_slice(), None)?;
    Ok(transition(next, None, actions))
}

fn complete_failure(
    state: &AcpConnectionState,
    correlation_id: Option<&str>,
    failure: AcpFailureClass,
) -> Result<AcpProtocolTransition, AcpProtocolError> {
    let mut next = state.clone();
    let mut actions = Vec::new();
    if let Some(id) = correlation_id {
        require_any_pending(state, id)?;
        remove_pending(&mut next, id)?;
        complete_correlation(&mut next, id)?;
        actions.push(response_action(
            id,
            AcpProtocolResponse::Failure { failure },
        ));
    } else if !state.pending.is_empty() {
        return Err(error(AcpFailureClass::UnknownCorrelation, FIELD_OUTCOME));
    }
    next.phase = AcpConnectionPhase::Terminal;
    next.terminal_failure = Some(failure);
    if let Some(session) = next.session.as_mut() {
        session.active_run_id = None;
    }
    actions.push(AcpProtocolAction::Terminate { failure });
    Ok(transition(next, None, actions))
}

fn bounded_update_actions(
    state: &mut AcpConnectionState,
    updates: &[crate::model::AcpUpdate],
    terminal_response: Option<AcpProtocolAction>,
) -> Result<Vec<AcpProtocolAction>, AcpProtocolError> {
    if updates.len() > state.profile.transport.updates_per_operation {
        return Err(error(AcpFailureClass::Backpressure, FIELD_BOUND));
    }
    let mut bytes = 0_usize;
    let mut actions = Vec::with_capacity(
        updates
            .len()
            .saturating_add(usize::from(terminal_response.is_some())),
    );
    for update in updates {
        bytes = bytes.saturating_add(update.estimated_bytes());
        if bytes > state.profile.transport.update_bytes_per_operation {
            return Err(error(AcpFailureClass::Backpressure, FIELD_BOUND));
        }
        actions.push(AcpProtocolAction::Update(update.clone()));
    }
    let update_count = u32::try_from(updates.len())
        .map_err(|_error| error(AcpFailureClass::Backpressure, FIELD_BOUND))?;
    let update_bytes =
        u64::try_from(bytes).map_err(|_error| error(AcpFailureClass::Backpressure, FIELD_BOUND))?;
    state.update_count = state.update_count.saturating_add(update_count);
    state.update_bytes = state.update_bytes.saturating_add(update_bytes);
    if let Some(response) = terminal_response {
        actions.push(response);
    }
    Ok(actions)
}

fn validate_profile(profile: &AcpConnectionProfile) -> Result<(), AcpProtocolError> {
    if profile.protocol_version != ACP_PROTOCOL_VERSION || profile.cohort != ACP_PROTOCOL_COHORT {
        return Err(error(AcpFailureClass::InvalidProfile, FIELD_PROFILE));
    }
    if profile.features.is_empty() || profile.features.len() > AcpFeature::ALL.len() {
        return Err(error(AcpFailureClass::InvalidProfile, FIELD_PROFILE));
    }
    for (index, feature) in profile.features.iter().enumerate() {
        if profile.features[..index].contains(feature) {
            return Err(error(AcpFailureClass::InvalidProfile, FIELD_PROFILE));
        }
    }
    let transport = profile.transport;
    if transport.frame_bytes == 0
        || transport.frames_per_connection == 0
        || transport.updates_per_operation == 0
        || transport.update_bytes_per_operation == 0
        || transport.permission_requests_per_connection == 0
    {
        return Err(error(AcpFailureClass::InvalidProfile, FIELD_PROFILE));
    }
    Ok(())
}

fn validate_prompt(parts: &[crate::model::AcpPromptPart]) -> Result<(), AcpProtocolError> {
    if parts.is_empty() {
        return Err(error(AcpFailureClass::InvalidPrompt, FIELD_PROMPT));
    }
    let mut bytes = 0_usize;
    for part in parts {
        match part {
            crate::model::AcpPromptPart::Text(text) => {
                if text.trim().is_empty() || text.contains('\0') {
                    return Err(error(AcpFailureClass::InvalidPrompt, FIELD_PROMPT));
                }
                bytes = bytes.saturating_add(text.len());
            }
            crate::model::AcpPromptPart::ResourceLink {
                safe_name,
                resource_identity,
            } => {
                validate_safe_label(safe_name, FIELD_PROMPT)?;
                validate_hash(resource_identity, FIELD_PROMPT)?;
                bytes = bytes
                    .saturating_add(safe_name.len())
                    .saturating_add(resource_identity.len());
            }
        }
    }
    if bytes > ACP_MAX_PROMPT_BYTES {
        return Err(error(AcpFailureClass::PromptTooLarge, FIELD_PROMPT));
    }
    Ok(())
}

fn validate_session_binding(binding: &AcpSessionBinding) -> Result<(), AcpProtocolError> {
    validate_safe_label(binding.acp_session_id.as_str(), FIELD_SESSION)?;
    validate_safe_label(binding.animus_session_id.as_str(), FIELD_SESSION)?;
    validate_hash(binding.session_identity.as_str(), FIELD_SESSION)?;
    validate_hash(binding.authority_profile_identity.as_str(), FIELD_SESSION)?;
    if binding.active_run_id.is_some() {
        return Err(error(AcpFailureClass::OutOfOrderOutcome, FIELD_SESSION));
    }
    Ok(())
}

fn validate_matching_session(
    state: &AcpConnectionState,
    observed: &AcpSessionBinding,
) -> Result<(), AcpProtocolError> {
    validate_session_binding_observation(observed)?;
    let Some(bound) = state.session.as_ref() else {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    };
    if bound.acp_session_id != observed.acp_session_id
        || bound.animus_session_id != observed.animus_session_id
        || bound.session_identity != observed.session_identity
    {
        return Err(error(AcpFailureClass::SessionMismatch, FIELD_SESSION));
    }
    if bound.authority_profile_identity != observed.authority_profile_identity {
        return Err(error(
            AcpFailureClass::AuthorityProfileMismatch,
            FIELD_SESSION,
        ));
    }
    Ok(())
}

fn validate_session_binding_observation(
    binding: &AcpSessionBinding,
) -> Result<(), AcpProtocolError> {
    validate_safe_label(binding.acp_session_id.as_str(), FIELD_SESSION)?;
    validate_safe_label(binding.animus_session_id.as_str(), FIELD_SESSION)?;
    validate_hash(binding.session_identity.as_str(), FIELD_SESSION)?;
    validate_hash(binding.authority_profile_identity.as_str(), FIELD_SESSION)
}

fn require_ready_unbound(state: &AcpConnectionState) -> Result<(), AcpProtocolError> {
    if state.phase == AcpConnectionPhase::AwaitingInitialize {
        return Err(error(AcpFailureClass::NotInitialized, FIELD_PHASE));
    }
    if state.phase != AcpConnectionPhase::Ready {
        return Err(error(AcpFailureClass::SessionAlreadyBound, FIELD_SESSION));
    }
    if state.session.is_some() {
        return Err(error(AcpFailureClass::SessionAlreadyBound, FIELD_SESSION));
    }
    if !state.pending.is_empty() {
        return Err(error(AcpFailureClass::OutOfOrderOutcome, FIELD_OUTCOME));
    }
    Ok(())
}

fn require_bound_session<'a>(
    state: &'a AcpConnectionState,
    session_id: &str,
) -> Result<&'a AcpSessionBinding, AcpProtocolError> {
    if state.phase != AcpConnectionPhase::SessionBound {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    }
    let Some(session) = state.session.as_ref() else {
        return Err(error(AcpFailureClass::SessionNotBound, FIELD_SESSION));
    };
    if session.acp_session_id.as_str() != session_id {
        return Err(error(AcpFailureClass::SessionMismatch, FIELD_SESSION));
    }
    Ok(session)
}

fn begin_correlation(
    state: &mut AcpConnectionState,
    correlation_id: &str,
    operation: AcpOperationKind,
) -> Result<(), AcpProtocolError> {
    validate_new_correlation(state, correlation_id)?;
    if state.pending.len() >= ACP_MAX_CORRELATIONS {
        return Err(error(AcpFailureClass::Backpressure, FIELD_BOUND));
    }
    state.pending.push(AcpCorrelation {
        identity: parse_correlation(correlation_id)?,
        operation,
    });
    Ok(())
}

fn validate_new_correlation(
    state: &AcpConnectionState,
    correlation_id: &str,
) -> Result<(), AcpProtocolError> {
    validate_safe_label(correlation_id, FIELD_CORRELATION)?;
    if state
        .pending
        .iter()
        .any(|pending| pending.identity.as_str() == correlation_id)
    {
        return Err(error(
            AcpFailureClass::DuplicateCorrelation,
            FIELD_CORRELATION,
        ));
    }
    if state
        .completed
        .iter()
        .any(|completed| completed.as_str() == correlation_id)
    {
        return Err(error(AcpFailureClass::StaleCorrelation, FIELD_CORRELATION));
    }
    Ok(())
}

fn require_pending(
    state: &AcpConnectionState,
    correlation_id: &str,
    operation: AcpOperationKind,
) -> Result<(), AcpProtocolError> {
    let Some(pending) = state
        .pending
        .iter()
        .find(|pending| pending.identity.as_str() == correlation_id)
    else {
        return Err(error(
            AcpFailureClass::UnknownCorrelation,
            FIELD_CORRELATION,
        ));
    };
    if pending.operation != operation {
        return Err(error(AcpFailureClass::OutOfOrderOutcome, FIELD_OUTCOME));
    }
    Ok(())
}

fn require_any_pending(
    state: &AcpConnectionState,
    correlation_id: &str,
) -> Result<(), AcpProtocolError> {
    if state
        .pending
        .iter()
        .any(|pending| pending.identity.as_str() == correlation_id)
    {
        Ok(())
    } else {
        Err(error(
            AcpFailureClass::UnknownCorrelation,
            FIELD_CORRELATION,
        ))
    }
}

fn remove_pending(
    state: &mut AcpConnectionState,
    correlation_id: &str,
) -> Result<(), AcpProtocolError> {
    let Some(index) = state
        .pending
        .iter()
        .position(|pending| pending.identity.as_str() == correlation_id)
    else {
        return Err(error(
            AcpFailureClass::UnknownCorrelation,
            FIELD_CORRELATION,
        ));
    };
    state.pending.remove(index);
    Ok(())
}

fn complete_correlation(
    state: &mut AcpConnectionState,
    correlation_id: &str,
) -> Result<(), AcpProtocolError> {
    if state.completed.len() >= ACP_MAX_CORRELATIONS {
        return Err(error(AcpFailureClass::Backpressure, FIELD_BOUND));
    }
    state.completed.push(parse_correlation(correlation_id)?);
    Ok(())
}

fn required_outcome_correlation(
    observation: &crate::model::AcpFacadeObservation,
) -> Result<&str, AcpProtocolError> {
    observation
        .correlation_id
        .as_ref()
        .map(AcpCorrelationId::as_str)
        .ok_or_else(|| error(AcpFailureClass::UnknownCorrelation, FIELD_OUTCOME))
}

fn require_no_mcp_servers(count: u32) -> Result<(), AcpProtocolError> {
    if count == 0 {
        Ok(())
    } else {
        Err(error(AcpFailureClass::UnsupportedFeature, FIELD_PROFILE))
    }
}

fn parse_correlation(value: &str) -> Result<AcpCorrelationId, AcpProtocolError> {
    AcpCorrelationId::parse(value)
        .map_err(|_| error(AcpFailureClass::MalformedFrame, FIELD_CORRELATION))
}

fn validate_safe_label(value: &str, field: &'static str) -> Result<(), AcpProtocolError> {
    if value.is_empty()
        || value.len() > ACP_MAX_SAFE_LABEL_BYTES
        || value.contains('\0')
        || value.chars().any(char::is_whitespace)
    {
        return Err(error(AcpFailureClass::MalformedFrame, field));
    }
    Ok(())
}

fn validate_hash(value: &str, field: &'static str) -> Result<(), AcpProtocolError> {
    if value.len() != crate::model::ACP_HASH_HEX_BYTES
        || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(error(AcpFailureClass::MalformedFrame, field));
    }
    Ok(())
}

pub(crate) fn domain_hash(domain: &str, fields: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for field in fields {
        hasher.update(&[0]);
        hasher.update(field.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn response_action(correlation_id: &str, response: AcpProtocolResponse) -> AcpProtocolAction {
    AcpProtocolAction::Respond {
        correlation_id: String::from(correlation_id),
        response,
    }
}

fn transition(
    state: AcpConnectionState,
    facade_command: Option<AcpFacadeCommand>,
    actions: Vec<AcpProtocolAction>,
) -> AcpProtocolTransition {
    debug_assert!(state.pending.len() <= ACP_MAX_CORRELATIONS);
    debug_assert!(state.completed.len() <= ACP_MAX_CORRELATIONS);
    AcpProtocolTransition {
        state,
        facade_command,
        actions,
    }
}

const fn error(failure: AcpFailureClass, field: &'static str) -> AcpProtocolError {
    AcpProtocolError { failure, field }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identities::{
        AcpCorrelationId, AcpSessionId, AnimusSessionId, AuthorityProfileIdentity, SessionIdentity,
    };
    use crate::model::{AcpFacadeObservation, AcpPromptPart, AcpStopClass};

    const CONNECTION: &str = "connection.fixture";
    const INITIALIZE_CORRELATION: &str = "rpc.1";
    const NEW_CORRELATION: &str = "rpc.2";
    const PROMPT_CORRELATION: &str = "rpc.3";
    const SESSION_ID: &str = "session.acp";
    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const COMPLETED_TRACE_CORRELATION_COUNT: usize = 3;

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.protocol_core]
    // r[verify agent_harness_runtime.acp_stdio_host.session_bridge]
    fn deterministic_initialize_session_and_prompt_trace() {
        let first = run_trace().expect("first trace");
        let second = run_trace().expect("second trace");

        assert_eq!(first, second);
        assert_eq!(first.phase, AcpConnectionPhase::SessionBound);
        assert!(first.pending.is_empty());
        assert_eq!(first.completed.len(), COMPLETED_TRACE_CORRELATION_COUNT);
        assert_eq!(
            first
                .session
                .as_ref()
                .and_then(|session| session.active_run_id.as_ref()),
            None
        );
    }

    #[test]
    // r[verify agent_harness_runtime.acp_nominal_identities.state.binding]
    // r[verify agent_harness_runtime.acp_nominal_identities.failure_matrix]
    fn duplicate_and_out_of_order_correlations_fail_closed() {
        let initialized = initialized_state().expect("initialized state");
        let request = AcpDecodedRequest::NewSession {
            correlation_id: String::from(NEW_CORRELATION),
            workspace_identity: String::from(HASH),
            mcp_server_count: 0,
        };
        let pending = reduce_acp_request(&initialized, &request).expect("pending session");
        let duplicate = reduce_acp_request(&pending.state, &request).expect_err("duplicate denied");
        assert_eq!(duplicate.failure, AcpFailureClass::DuplicateCorrelation);

        let wrong = AcpHostOutcome::PromptCompleted(AcpFacadeObservation {
            correlation_id: Some(correlation(NEW_CORRELATION)),
            session: session_binding(),
            updates: Vec::new(),
            stop: Some(AcpStopClass::EndTurn),
        });
        let error = reduce_acp_outcome(&pending.state, &wrong).expect_err("wrong outcome denied");
        assert_eq!(error.failure, AcpFailureClass::OutOfOrderOutcome);
    }

    #[test]
    fn uninitialized_and_mcp_widening_requests_are_denied() {
        let state = new_acp_connection_state(CONNECTION, AcpConnectionProfile::reviewed())
            .expect("connection state");
        let prompt = AcpDecodedRequest::Prompt {
            correlation_id: String::from(PROMPT_CORRELATION),
            session_id: String::from(SESSION_ID),
            parts: vec![AcpPromptPart::Text(String::from("bounded"))],
        };
        let error = reduce_acp_request(&state, &prompt).expect_err("uninitialized denied");
        assert_eq!(error.failure, AcpFailureClass::SessionNotBound);

        let initialized = initialized_state().expect("initialized state");
        let mcp = AcpDecodedRequest::NewSession {
            correlation_id: String::from(NEW_CORRELATION),
            workspace_identity: String::from(HASH),
            mcp_server_count: crate::model::ACP_COUNTER_INCREMENT,
        };
        let error = reduce_acp_request(&initialized, &mcp).expect_err("MCP denied");
        assert_eq!(error.failure, AcpFailureClass::UnsupportedFeature);

        let unsupported = reduce_acp_request(
            &initialized,
            &AcpDecodedRequest::Unsupported {
                correlation_id: Some(String::from("rpc.unsupported")),
                method: String::from("method.unsupported"),
            },
        )
        .expect("typed unsupported-method response");
        assert_eq!(unsupported.state.phase, AcpConnectionPhase::Terminal);
        assert_eq!(
            unsupported.state.terminal_failure,
            Some(AcpFailureClass::UnsupportedMethod)
        );

        let mut limited_profile = AcpConnectionProfile::reviewed();
        limited_profile
            .features
            .retain(|feature| *feature != AcpFeature::Cancellation);
        let limited = new_acp_connection_state("connection.limited", limited_profile)
            .expect("limited connection");
        let feature_error = reduce_acp_request(
            &limited,
            &AcpDecodedRequest::Initialize {
                correlation_id: String::from("rpc.feature"),
                protocol_version: String::from(ACP_PROTOCOL_VERSION),
                required_features: vec![AcpFeature::Cancellation],
            },
        )
        .expect_err("unknown required feature denied");
        assert_eq!(feature_error.failure, AcpFailureClass::UnsupportedFeature);
    }

    fn run_trace() -> Result<AcpConnectionState, AcpProtocolError> {
        let initialized = initialized_state()?;
        let pending_session = reduce_acp_request(
            &initialized,
            &AcpDecodedRequest::NewSession {
                correlation_id: String::from(NEW_CORRELATION),
                workspace_identity: String::from(HASH),
                mcp_server_count: 0,
            },
        )?;
        let bound = reduce_acp_outcome(
            &pending_session.state,
            &AcpHostOutcome::SessionCreated(AcpFacadeObservation {
                correlation_id: Some(correlation(NEW_CORRELATION)),
                session: session_binding(),
                updates: Vec::new(),
                stop: None,
            }),
        )?;
        let prompt = reduce_acp_request(
            &bound.state,
            &AcpDecodedRequest::Prompt {
                correlation_id: String::from(PROMPT_CORRELATION),
                session_id: String::from(SESSION_ID),
                parts: vec![AcpPromptPart::Text(String::from("bounded prompt"))],
            },
        )?;
        let completed = reduce_acp_outcome(
            &prompt.state,
            &AcpHostOutcome::PromptCompleted(AcpFacadeObservation {
                correlation_id: Some(correlation(PROMPT_CORRELATION)),
                session: session_binding(),
                updates: Vec::new(),
                stop: Some(AcpStopClass::EndTurn),
            }),
        )?;
        Ok(completed.state)
    }

    fn initialized_state() -> Result<AcpConnectionState, AcpProtocolError> {
        let state = new_acp_connection_state(CONNECTION, AcpConnectionProfile::reviewed())?;
        let transition = reduce_acp_request(
            &state,
            &AcpDecodedRequest::Initialize {
                correlation_id: String::from(INITIALIZE_CORRELATION),
                protocol_version: String::from(ACP_PROTOCOL_VERSION),
                required_features: vec![AcpFeature::TextPrompt],
            },
        )?;
        Ok(transition.state)
    }

    fn correlation(value: &str) -> AcpCorrelationId {
        AcpCorrelationId::parse(value).expect("correlation identity")
    }

    fn session_binding() -> AcpSessionBinding {
        AcpSessionBinding {
            acp_session_id: AcpSessionId::parse(SESSION_ID).expect("ACP session"),
            animus_session_id: AnimusSessionId::parse(HASH).expect("Animus session"),
            session_identity: SessionIdentity::parse(HASH).expect("session identity"),
            authority_profile_identity: AuthorityProfileIdentity::parse(HASH)
                .expect("authority identity"),
            active_run_id: None,
        }
    }
}
