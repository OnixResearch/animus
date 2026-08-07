//! ACP-to-coding-session imperative bridge over the shared facade owner.

#![allow(missing_docs)]

use animus_runtime::coding_session_host::{
    CodingSessionCommand, CodingSessionEffectRequest, CodingSessionInput,
    CodingSessionInputPosture, CodingSessionIssue, CodingSessionServiceIssue,
    CodingSessionTerminalKind, CodingSessionToolRequest,
};
use animus_std::coding_session_host::{
    CodingSessionAuthorityService, CodingSessionCancellationService, CodingSessionEffectGate,
    CodingSessionEventSink, CodingSessionHost, CodingSessionHostError,
    CodingSessionPersistenceService, CodingSessionPolicyService, CodingSessionProviderService,
    CodingSessionSummarizerService, CodingSessionTimerService, CodingSessionToolService,
};

use crate::codec::host_messages_for_actions;
use crate::core::{domain_hash, reduce_acp_outcome, reduce_acp_request};
use crate::events::project_runtime_events;
use crate::identities::{
    AcpCorrelationId, AcpRunId, AcpSessionId, AnimusSessionId, AuthorityProfileIdentity,
    SessionIdentity,
};
use crate::model::{
    ACP_SESSION_HASH_DOMAIN, AcpClientPermissionRequest, AcpConnectionState, AcpDecodedRequest,
    AcpFacadeCommand, AcpFacadeObservation, AcpFailureClass, AcpHostMessage, AcpHostOutcome,
    AcpOperationKind, AcpPermissionInput, AcpPermissionStatus, AcpPromptPart, AcpProtocolError,
    AcpReceipt, AcpReceiptDraft, AcpSensitiveDraft, AcpSessionBinding, AcpStopClass,
};
use crate::permission::{apply_permission_decision, plan_acp_permission};
use crate::receipt::project_acp_receipt;
use crate::transport::{AcpPeer, AcpStdioService, AcpTransportError, AcpTransportReport};

const FIELD_SESSION: &str = "session";
#[cfg(test)]
const FIELD_PROMPT: &str = "prompt";
const FIELD_PERMISSION: &str = "permission";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpFacadeError {
    failure: AcpFailureClass,
    field: &'static str,
}

impl AcpFacadeError {
    const fn new(failure: AcpFailureClass, field: &'static str) -> Self {
        Self { failure, field }
    }

    #[must_use]
    pub const fn failure(&self) -> AcpFailureClass {
        self.failure
    }
}

impl core::fmt::Display for AcpFacadeError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "ACP facade bridge rejected {}", self.field)
    }
}

impl std::error::Error for AcpFacadeError {}

pub struct AcpFacadePrompt {
    pub run_id: String,
    pub entry_id: String,
    pub body: String,
}

pub trait AcpCodingSessionFacade {
    fn session_id(&self) -> &str;
    fn active_run_id(&self) -> Option<&str>;
    fn authority_profile_identity(&self) -> &str;
    fn event_count(&self) -> usize;
    fn events_from(&self, start: usize) -> &[animus_runtime::RuntimeEvent];
    fn terminal(&self) -> Option<CodingSessionTerminalKind>;
    /// # Errors
    /// Returns an error when durable recovery or current authority admission fails.
    fn resume(&mut self) -> Result<(), AcpFacadeError>;
    /// # Errors
    /// Returns an error when the prompt cannot be durably reduced and driven.
    fn prompt(
        &mut self,
        prompt: AcpFacadePrompt,
        gate: &mut dyn CodingSessionEffectGate,
    ) -> Result<(), AcpFacadeError>;
    /// # Errors
    /// Returns an error when cancellation cannot be recorded by the shared facade.
    fn cancel(&mut self, run_id: &str) -> Result<(), AcpFacadeError>;
}

impl<Provider, Tool, Persistence, Policy, Authority, Cancellation, Timer, Summarizer, Events>
    AcpCodingSessionFacade
    for CodingSessionHost<
        Provider,
        Tool,
        Persistence,
        Policy,
        Authority,
        Cancellation,
        Timer,
        Summarizer,
        Events,
    >
where
    Provider: CodingSessionProviderService,
    Tool: CodingSessionToolService,
    Persistence: CodingSessionPersistenceService,
    Policy: CodingSessionPolicyService,
    Authority: CodingSessionAuthorityService,
    Cancellation: CodingSessionCancellationService,
    Timer: CodingSessionTimerService,
    Summarizer: CodingSessionSummarizerService,
    Events: CodingSessionEventSink,
{
    fn session_id(&self) -> &str {
        self.state_snapshot().session_id()
    }

    fn active_run_id(&self) -> Option<&str> {
        self.state_snapshot()
            .run()
            .map(animus_runtime::coding_session_host::CodingSessionRun::run_id)
    }

    fn authority_profile_identity(&self) -> &str {
        self.state_snapshot().authority_profile().profile_identity()
    }

    fn event_count(&self) -> usize {
        self.event_snapshot().len()
    }

    fn events_from(&self, start: usize) -> &[animus_runtime::RuntimeEvent] {
        self.event_snapshot().get(start..).unwrap_or_default()
    }

    fn terminal(&self) -> Option<CodingSessionTerminalKind> {
        self.state_snapshot().terminal()
    }

    fn resume(&mut self) -> Result<(), AcpFacadeError> {
        CodingSessionHost::resume(self).map_err(|error| map_host_error(&error))
    }

    fn prompt(
        &mut self,
        prompt: AcpFacadePrompt,
        gate: &mut dyn CodingSessionEffectGate,
    ) -> Result<(), AcpFacadeError> {
        self.begin_without_driving(&CodingSessionCommand::Prompt {
            run_id: prompt.run_id,
            entry_id: prompt.entry_id,
            input: CodingSessionInput {
                body: prompt.body,
                posture: CodingSessionInputPosture::ExpandedAndAdmitted,
            },
        })
        .map_err(|error| map_host_error(&error))?;
        self.drive_pending_with_gate(gate)
            .map_err(|error| map_host_error(&error))
    }

    fn cancel(&mut self, run_id: &str) -> Result<(), AcpFacadeError> {
        CodingSessionHost::cancel(self, run_id).map_err(|error| map_host_error(&error))
    }
}

pub trait AcpCodingSessionFactory {
    type Session: AcpCodingSessionFacade;

    /// # Errors
    /// Returns an error when fresh session creation or admission fails.
    fn create(&mut self, workspace_identity: &str) -> Result<Self::Session, AcpFacadeError>;

    /// # Errors
    /// Returns an error when durable lookup or admission fails.
    fn load(
        &mut self,
        session_reference: &str,
        workspace_identity: &str,
    ) -> Result<Self::Session, AcpFacadeError>;

    /// # Errors
    /// Returns an error when host-owned leases or resources cannot be released.
    fn cleanup(&mut self, session: Option<Self::Session>) -> Result<(), AcpFacadeError>;
}

pub trait AcpPromptResolver {
    /// # Errors
    /// Returns an error when bounded prompt parts cannot be safely admitted.
    fn resolve(&mut self, parts: &[AcpPromptPart]) -> Result<String, AcpFacadeError>;
}

pub trait AcpToolAdmissionService {
    /// # Errors
    /// Returns an error when current host-owned admission cannot be observed.
    fn observe(
        &mut self,
        session_id: &str,
        request: &CodingSessionToolRequest,
    ) -> Result<AcpPermissionInput, AcpFacadeError>;
}

pub struct AcpSessionHostServices<Factory, Resolver, Admission> {
    pub factory: Factory,
    pub prompt_resolver: Resolver,
    pub tool_admission: Admission,
}

pub struct AcpSessionReceiptContext {
    pub operation: AcpOperationKind,
    pub correlation_identity: Option<String>,
    pub tool_count: u32,
}

pub struct AcpSessionHost<Factory, Resolver, Admission>
where
    Factory: AcpCodingSessionFactory,
{
    state: AcpConnectionState,
    services: AcpSessionHostServices<Factory, Resolver, Admission>,
    session: Option<Factory::Session>,
    cleaned: bool,
}

struct PromptExecution {
    correlation_id: AcpCorrelationId,
    session_id: AcpSessionId,
    prompt: AcpFacadePrompt,
    parts: Vec<AcpPromptPart>,
}

impl<Factory, Resolver, Admission> AcpSessionHost<Factory, Resolver, Admission>
where
    Factory: AcpCodingSessionFactory,
    Resolver: AcpPromptResolver,
    Admission: AcpToolAdmissionService,
{
    #[must_use]
    pub const fn new(
        state: AcpConnectionState,
        services: AcpSessionHostServices<Factory, Resolver, Admission>,
    ) -> Self {
        Self {
            state,
            services,
            session: None,
            cleaned: false,
        }
    }

    #[must_use]
    pub const fn state(&self) -> &AcpConnectionState {
        &self.state
    }

    /// # Errors
    /// Returns a typed protocol error when receipt facts are unsafe or inconsistent.
    pub fn receipt(
        &self,
        context: AcpSessionReceiptContext,
        transport: AcpTransportReport,
        sensitive: AcpSensitiveDraft<'_>,
    ) -> Result<AcpReceipt, AcpProtocolError> {
        let session_identity = self
            .state
            .session
            .as_ref()
            .map(|session| session.session_identity.clone());
        let authority_profile_identity = self
            .state
            .session
            .as_ref()
            .map(|session| session.authority_profile_identity.clone());
        project_acp_receipt(
            &self.state.profile,
            &AcpReceiptDraft {
                connection_identity: self.state.connection_identity.clone(),
                session_identity,
                correlation_identity: context
                    .correlation_identity
                    .map(AcpCorrelationId::parse)
                    .transpose()
                    .map_err(|_| AcpProtocolError {
                        failure: AcpFailureClass::MalformedFrame,
                        field: FIELD_SESSION,
                    })?,
                authority_profile_identity,
                operation: context.operation,
                status: transport.status,
                failure: transport.failure,
                cleanup: transport.cleanup,
                frame_count: transport.frame_count,
                update_count: self.state.update_count,
                update_bytes: self.state.update_bytes,
                tool_count: context.tool_count,
                permission_count: transport.permission_count,
            },
            sensitive,
        )
    }

    fn execute_command(
        &mut self,
        command: AcpFacadeCommand,
        peer: &mut dyn AcpPeer,
    ) -> AcpHostOutcome {
        match command {
            AcpFacadeCommand::CreateSession {
                correlation_id,
                workspace_identity,
            } => self.create_session(correlation_id, workspace_identity.as_str()),
            AcpFacadeCommand::LoadSession {
                correlation_id,
                session_reference,
                workspace_identity,
            } => self.load_session(
                correlation_id,
                session_reference.as_str(),
                workspace_identity.as_str(),
            ),
            AcpFacadeCommand::Prompt {
                correlation_id,
                session_id,
                run_id,
                entry_id,
                parts,
            } => self.prompt_session(
                PromptExecution {
                    correlation_id,
                    session_id,
                    prompt: AcpFacadePrompt {
                        run_id: run_id.as_str().to_string(),
                        entry_id: entry_id.as_str().to_string(),
                        body: String::new(),
                    },
                    parts,
                },
                peer,
            ),
            AcpFacadeCommand::Cancel { session_id, run_id } => {
                self.cancel_session(session_id.as_str(), run_id.as_str())
            }
        }
    }

    fn create_session(
        &mut self,
        correlation_id: AcpCorrelationId,
        workspace_identity: &str,
    ) -> AcpHostOutcome {
        match self.services.factory.create(workspace_identity) {
            Ok(session) => {
                let binding = match binding_for(&session) {
                    Ok(binding) => binding,
                    Err(failure) => return failed(Some(correlation_id), failure),
                };
                self.session = Some(session);
                AcpHostOutcome::SessionCreated(AcpFacadeObservation {
                    correlation_id: Some(correlation_id),
                    session: binding,
                    updates: Vec::new(),
                    stop: None,
                })
            }
            Err(error) => failed(Some(correlation_id), error.failure()),
        }
    }

    // r[impl agent_harness_runtime.acp_stdio_host.resume]
    fn load_session(
        &mut self,
        correlation_id: AcpCorrelationId,
        session_reference: &str,
        workspace_identity: &str,
    ) -> AcpHostOutcome {
        let mut session = match self
            .services
            .factory
            .load(session_reference, workspace_identity)
        {
            Ok(session) => session,
            Err(error) => return failed(Some(correlation_id), error.failure()),
        };
        let event_start = session.event_count();
        if let Err(error) = session.resume() {
            return failed(Some(correlation_id.clone()), error.failure());
        }
        let binding = match binding_for(&session) {
            Ok(binding) => binding,
            Err(failure) => return failed(Some(correlation_id.clone()), failure),
        };
        let updates = match project_runtime_events(
            binding.acp_session_id.as_str(),
            session.events_from(event_start),
            self.state.profile.transport,
        ) {
            Ok(updates) => updates,
            Err(error) => return failed(Some(correlation_id.clone()), error.failure),
        };
        self.session = Some(session);
        AcpHostOutcome::SessionLoaded(AcpFacadeObservation {
            correlation_id: Some(correlation_id),
            session: binding,
            updates,
            stop: None,
        })
    }

    // r[impl agent_harness_runtime.acp_stdio_host.session_bridge]
    fn prompt_session(
        &mut self,
        execution: PromptExecution,
        peer: &mut dyn AcpPeer,
    ) -> AcpHostOutcome {
        let PromptExecution {
            correlation_id,
            session_id,
            mut prompt,
            parts,
        } = execution;
        let Some(bound_session) = self.session.as_ref() else {
            return failed(
                Some(correlation_id.clone()),
                AcpFailureClass::SessionNotBound,
            );
        };
        if let Err(failure) =
            validate_current_facade_binding(&self.state, session_id.as_str(), bound_session)
        {
            return failed(Some(correlation_id.clone()), failure);
        }
        prompt.body = match self.services.prompt_resolver.resolve(parts.as_slice()) {
            Ok(body) => body,
            Err(error) => return failed(Some(correlation_id.clone()), error.failure()),
        };
        let Some(session) = self.session.as_mut() else {
            return failed(
                Some(correlation_id.clone()),
                AcpFailureClass::SessionNotBound,
            );
        };
        let event_start = session.event_count();
        let mut gate = AcpPermissionEffectGate {
            session_id: session_id.as_str(),
            admission: &mut self.services.tool_admission,
            peer,
        };
        if let Err(error) = session.prompt(prompt, &mut gate) {
            return failed(Some(correlation_id.clone()), error.failure());
        }
        let binding = match binding_for(session) {
            Ok(binding) => binding,
            Err(failure) => return failed(Some(correlation_id.clone()), failure),
        };
        let updates = match project_runtime_events(
            session_id.as_str(),
            session.events_from(event_start),
            self.state.profile.transport,
        ) {
            Ok(updates) => updates,
            Err(error) => return failed(Some(correlation_id.clone()), error.failure),
        };
        let stop = stop_from_terminal(session.terminal());
        AcpHostOutcome::PromptCompleted(AcpFacadeObservation {
            correlation_id: Some(correlation_id),
            session: binding,
            updates,
            stop: Some(stop),
        })
    }

    fn cancel_session(&mut self, session_id: &str, run_id: &str) -> AcpHostOutcome {
        let Some(bound_session) = self.session.as_ref() else {
            return failed(None, AcpFailureClass::SessionNotBound);
        };
        if let Err(failure) =
            validate_current_facade_binding(&self.state, session_id, bound_session)
        {
            return failed(None, failure);
        }
        let Some(session) = self.session.as_mut() else {
            return failed(None, AcpFailureClass::SessionNotBound);
        };
        let event_start = session.event_count();
        if let Err(error) = session.cancel(run_id) {
            return failed(None, error.failure());
        }
        let binding = match binding_for(session) {
            Ok(binding) => binding,
            Err(failure) => return failed(None, failure),
        };
        let updates = match project_runtime_events(
            session_id,
            session.events_from(event_start),
            self.state.profile.transport,
        ) {
            Ok(updates) => updates,
            Err(error) => return failed(None, error.failure),
        };
        AcpHostOutcome::CancellationObserved(AcpFacadeObservation {
            correlation_id: None,
            session: binding,
            updates,
            stop: Some(AcpStopClass::Cancelled),
        })
    }
}

impl<Factory, Resolver, Admission> AcpStdioService for AcpSessionHost<Factory, Resolver, Admission>
where
    Factory: AcpCodingSessionFactory,
    Resolver: AcpPromptResolver,
    Admission: AcpToolAdmissionService,
{
    fn handle(
        &mut self,
        request: AcpDecodedRequest,
        peer: &mut dyn AcpPeer,
    ) -> Result<Vec<AcpHostMessage>, AcpFailureClass> {
        let request_transition =
            reduce_acp_request(&self.state, &request).map_err(|error| error.failure)?;
        self.state = request_transition.state;
        let mut actions = request_transition.actions;
        if let Some(command) = request_transition.facade_command {
            let outcome = self.execute_command(command, peer);
            let outcome_transition =
                reduce_acp_outcome(&self.state, &outcome).map_err(|error| error.failure)?;
            self.state = outcome_transition.state;
            actions.extend(outcome_transition.actions);
        }
        host_messages_for_actions(actions.as_slice()).map_err(|error| error.failure)
    }

    fn accepts_eof(&self) -> bool {
        self.state.pending.is_empty()
    }

    fn cleanup(&mut self) -> Result<(), AcpFailureClass> {
        if self.cleaned {
            return Ok(());
        }
        let session = self.session.take();
        self.services
            .factory
            .cleanup(session)
            .map_err(|error| error.failure())?;
        self.cleaned = true;
        Ok(())
    }
}

struct AcpPermissionEffectGate<'a, Admission> {
    session_id: &'a str,
    admission: &'a mut Admission,
    peer: &'a mut dyn AcpPeer,
}

impl<Admission: AcpToolAdmissionService> CodingSessionEffectGate
    for AcpPermissionEffectGate<'_, Admission>
{
    fn admit(
        &mut self,
        effect: &CodingSessionEffectRequest,
    ) -> Result<(), CodingSessionServiceIssue> {
        let CodingSessionEffectRequest::Tool(request) = effect else {
            return Ok(());
        };
        let input = self
            .admission
            .observe(self.session_id, request)
            .map_err(|error| service_issue(error.failure()))?;
        validate_permission_input(self.session_id, request, &input)
            .map_err(|error| service_issue(error.failure()))?;
        let plan = plan_acp_permission(&input);
        let decision = match plan.status {
            AcpPermissionStatus::PreApproved | AcpPermissionStatus::Denied => None,
            AcpPermissionStatus::ClientDecisionRequired => Some(
                self.peer
                    .request_permission(&AcpClientPermissionRequest {
                        request_id: plan.request_id.clone(),
                        session_id: plan.session_id.clone(),
                        tool_name: plan.tool_name.clone(),
                        tool_correlation_identity: AcpCorrelationId::from_admitted(domain_hash(
                            crate::model::ACP_CORRELATION_HASH_DOMAIN,
                            &[plan.tool_correlation_id.as_str()],
                        )),
                    })
                    .map_err(|error| map_transport_error(&error))?,
            ),
        };
        let outcome = apply_permission_decision(&plan, decision);
        if outcome.dispatch_allowed {
            Ok(())
        } else {
            Err(service_issue(
                outcome.failure.unwrap_or(AcpFailureClass::PermissionDenied),
            ))
        }
    }
}

fn validate_permission_input(
    session_id: &str,
    request: &CodingSessionToolRequest,
    input: &AcpPermissionInput,
) -> Result<(), AcpFacadeError> {
    if input.session_id.as_str() != session_id
        || input.tool_name != request.request().name()
        || input.tool_correlation_id.as_str() != request.request().correlation_id()
        || input.request_id.is_empty()
    {
        return Err(AcpFacadeError::new(
            AcpFailureClass::PermissionWidening,
            FIELD_PERMISSION,
        ));
    }
    Ok(())
}

fn validate_current_facade_binding<Session: AcpCodingSessionFacade>(
    state: &AcpConnectionState,
    session_id: &str,
    session: &Session,
) -> Result<(), AcpFailureClass> {
    let bound = state
        .session
        .as_ref()
        .ok_or(AcpFailureClass::SessionNotBound)?;
    if bound.acp_session_id.as_str() != session_id || session.session_id() != session_id {
        return Err(AcpFailureClass::SessionMismatch);
    }
    if bound.authority_profile_identity.as_str() != session.authority_profile_identity() {
        return Err(AcpFailureClass::AuthorityProfileMismatch);
    }
    Ok(())
}

fn binding_for<Session: AcpCodingSessionFacade>(
    session: &Session,
) -> Result<AcpSessionBinding, AcpFailureClass> {
    let acp_session_id =
        AcpSessionId::parse(session.session_id()).map_err(|_| AcpFailureClass::SessionMismatch)?;
    let animus_session_id = AnimusSessionId::from_domain_hash(domain_hash(
        ACP_SESSION_HASH_DOMAIN,
        &[session.session_id()],
    ));
    let session_identity = SessionIdentity::from_domain_hash(domain_hash(
        ACP_SESSION_HASH_DOMAIN,
        &[session.session_id()],
    ));
    let authority_profile_identity =
        AuthorityProfileIdentity::parse(session.authority_profile_identity())
            .map_err(|_| AcpFailureClass::AuthorityProfileMismatch)?;
    let active_run_id = session
        .active_run_id()
        .map(AcpRunId::parse)
        .transpose()
        .map_err(|_| AcpFailureClass::StaleRun)?;
    Ok(AcpSessionBinding {
        acp_session_id,
        animus_session_id,
        session_identity,
        authority_profile_identity,
        active_run_id,
    })
}

const fn stop_from_terminal(terminal: Option<CodingSessionTerminalKind>) -> AcpStopClass {
    match terminal {
        Some(CodingSessionTerminalKind::Completed) | None => AcpStopClass::EndTurn,
        Some(CodingSessionTerminalKind::Cancelled) => AcpStopClass::Cancelled,
        Some(CodingSessionTerminalKind::Blocked) => AcpStopClass::Refused,
    }
}

fn failed(correlation_id: Option<AcpCorrelationId>, failure: AcpFailureClass) -> AcpHostOutcome {
    AcpHostOutcome::Failed {
        correlation_id,
        failure,
    }
}

fn map_host_error(error: &CodingSessionHostError) -> AcpFacadeError {
    let failure = match error.reducer_issue() {
        Some(
            CodingSessionIssue::AuthorityProfileMismatch
            | CodingSessionIssue::AuthorityProfileUnbound
            | CodingSessionIssue::AuthorityReauthorizationRequired,
        ) => AcpFailureClass::AuthorityProfileMismatch,
        Some(CodingSessionIssue::RecoveryBlocked) => AcpFailureClass::StoreRejected,
        Some(CodingSessionIssue::AlreadyRunning | CodingSessionIssue::StaleRun) => {
            AcpFailureClass::StaleRun
        }
        Some(CodingSessionIssue::ToolFailure) => AcpFailureClass::PermissionDenied,
        _other => AcpFailureClass::FacadeRejected,
    };
    AcpFacadeError::new(failure, FIELD_SESSION)
}

const fn service_issue(failure: AcpFailureClass) -> CodingSessionServiceIssue {
    match failure {
        AcpFailureClass::DeadlineExceeded => CodingSessionServiceIssue::TimedOut,
        AcpFailureClass::CancellationAmbiguous => CodingSessionServiceIssue::Cancelled,
        AcpFailureClass::PermissionUnavailable | AcpFailureClass::ExecutorUnavailable => {
            CodingSessionServiceIssue::Unavailable
        }
        AcpFailureClass::MalformedFrame | AcpFailureClass::PermissionWidening => {
            CodingSessionServiceIssue::Malformed
        }
        _other => CodingSessionServiceIssue::Denied,
    }
}

fn map_transport_error(error: &AcpTransportError) -> CodingSessionServiceIssue {
    service_issue(error.failure())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AcpConnectionProfile, AcpPermissionAdmission, AcpTransportProfile};

    const SESSION_ID: &str = "session.fixture";
    const AUTHORITY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const DRIFTED_AUTHORITY_HASH: &str =
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const FIXTURE_FRAME_COUNT: u32 = 3;

    #[derive(Clone)]
    struct FakeSession {
        resumed: bool,
        authority_profile_identity: String,
        active_run: Option<String>,
        events: Vec<animus_runtime::RuntimeEvent>,
        terminal: Option<CodingSessionTerminalKind>,
    }

    impl AcpCodingSessionFacade for FakeSession {
        fn session_id(&self) -> &str {
            SESSION_ID
        }
        fn active_run_id(&self) -> Option<&str> {
            self.active_run.as_deref()
        }
        fn authority_profile_identity(&self) -> &str {
            self.authority_profile_identity.as_str()
        }
        fn event_count(&self) -> usize {
            self.events.len()
        }
        fn events_from(&self, start: usize) -> &[animus_runtime::RuntimeEvent] {
            self.events.get(start..).unwrap_or_default()
        }
        fn terminal(&self) -> Option<CodingSessionTerminalKind> {
            self.terminal
        }
        fn resume(&mut self) -> Result<(), AcpFacadeError> {
            self.resumed = true;
            Ok(())
        }
        fn prompt(
            &mut self,
            prompt: AcpFacadePrompt,
            _gate: &mut dyn CodingSessionEffectGate,
        ) -> Result<(), AcpFacadeError> {
            self.active_run = Some(prompt.run_id);
            self.events
                .push(animus_runtime::RuntimeEvent::FinalResponse {
                    response: animus_adapters::ModelResponse::new(prompt.body.as_str()).map_err(
                        |_error| AcpFacadeError::new(AcpFailureClass::InvalidPrompt, FIELD_PROMPT),
                    )?,
                });
            self.active_run = None;
            self.terminal = Some(CodingSessionTerminalKind::Completed);
            Ok(())
        }
        fn cancel(&mut self, _run_id: &str) -> Result<(), AcpFacadeError> {
            self.active_run = None;
            self.terminal = Some(CodingSessionTerminalKind::Cancelled);
            Ok(())
        }
    }

    struct Factory;
    impl AcpCodingSessionFactory for Factory {
        type Session = FakeSession;
        fn create(&mut self, _workspace_identity: &str) -> Result<Self::Session, AcpFacadeError> {
            Ok(fake_session())
        }
        fn load(
            &mut self,
            _session_reference: &str,
            _workspace_identity: &str,
        ) -> Result<Self::Session, AcpFacadeError> {
            Ok(fake_session())
        }
        fn cleanup(&mut self, _session: Option<Self::Session>) -> Result<(), AcpFacadeError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct Resolver {
        call_count: u32,
    }
    impl AcpPromptResolver for Resolver {
        fn resolve(&mut self, parts: &[AcpPromptPart]) -> Result<String, AcpFacadeError> {
            self.call_count = self
                .call_count
                .saturating_add(crate::model::ACP_COUNTER_INCREMENT);
            parts
                .iter()
                .find_map(|part| match part {
                    AcpPromptPart::Text(text) => Some(text.clone()),
                    AcpPromptPart::ResourceLink { .. } => None,
                })
                .ok_or_else(|| AcpFacadeError::new(AcpFailureClass::InvalidPrompt, FIELD_PROMPT))
        }
    }

    struct Admission;
    impl AcpToolAdmissionService for Admission {
        fn observe(
            &mut self,
            session_id: &str,
            request: &CodingSessionToolRequest,
        ) -> Result<AcpPermissionInput, AcpFacadeError> {
            Ok(AcpPermissionInput {
                request_id: String::from("permission.fixture"),
                session_id: AcpSessionId::from_admitted(String::from(session_id)),
                tool_name: String::from(request.request().name()),
                tool_correlation_id: AcpCorrelationId::from_admitted(String::from(
                    request.request().correlation_id(),
                )),
                policy: AcpPermissionAdmission::Admitted,
                permission_mode: AcpPermissionAdmission::Admitted,
                authorization: AcpPermissionAdmission::Admitted,
                capability: AcpPermissionAdmission::Admitted,
                sandbox: AcpPermissionAdmission::Admitted,
                profile: AcpPermissionAdmission::Admitted,
                executor: AcpPermissionAdmission::Admitted,
                confirmation_required: false,
            })
        }
    }

    struct Peer;
    impl AcpPeer for Peer {
        fn send(&mut self, _message: &AcpHostMessage) -> Result<(), AcpTransportError> {
            Ok(())
        }
        fn request_permission(
            &mut self,
            _request: &AcpClientPermissionRequest,
        ) -> Result<crate::model::AcpClientPermissionDecision, AcpTransportError> {
            Ok(crate::model::AcpClientPermissionDecision::AllowOnce)
        }
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.session_bridge]
    fn new_session_and_prompt_delegate_to_one_facade() {
        let state =
            crate::new_acp_connection_state("connection.fixture", AcpConnectionProfile::reviewed())
                .expect("connection");
        let services = AcpSessionHostServices {
            factory: Factory,
            prompt_resolver: Resolver::default(),
            tool_admission: Admission,
        };
        let mut host = AcpSessionHost::new(state, services);
        let mut peer = Peer;

        host.handle(
            AcpDecodedRequest::Initialize {
                correlation_id: String::from("rpc.1"),
                protocol_version: String::from(crate::model::ACP_PROTOCOL_VERSION),
                required_features: Vec::new(),
            },
            &mut peer,
        )
        .expect("initialize");
        host.handle(
            AcpDecodedRequest::NewSession {
                correlation_id: String::from("rpc.2"),
                workspace_identity: String::from(AUTHORITY_HASH),
                mcp_server_count: 0,
            },
            &mut peer,
        )
        .expect("new session");
        let messages = host
            .handle(
                AcpDecodedRequest::Prompt {
                    correlation_id: String::from("rpc.3"),
                    session_id: String::from(SESSION_ID),
                    parts: vec![AcpPromptPart::Text(String::from("visible"))],
                },
                &mut peer,
            )
            .expect("prompt");

        assert_eq!(
            host.state()
                .session
                .as_ref()
                .map(|session| session.acp_session_id.as_str()),
            Some(SESSION_ID)
        );
        assert_eq!(host.state().pending.len(), 0);
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, AcpHostMessage::Response { .. }))
        );
        let receipt = host
            .receipt(
                AcpSessionReceiptContext {
                    operation: AcpOperationKind::Prompt,
                    correlation_identity: Some(String::from(AUTHORITY_HASH)),
                    tool_count: 0,
                },
                AcpTransportReport {
                    status: crate::model::AcpTransportStatus::CleanEof,
                    failure: None,
                    cleanup: crate::model::AcpCleanupStatus::Completed,
                    frame_count: FIXTURE_FRAME_COUNT,
                    permission_count: 0,
                },
                AcpSensitiveDraft::default(),
            )
            .expect("safe integrated receipt");
        assert_eq!(
            receipt.authority_profile_identity.as_deref(),
            Some(AUTHORITY_HASH)
        );
        assert_eq!(
            host.state().profile.transport,
            AcpTransportProfile::reviewed()
        );
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.resume]
    fn load_resumes_before_binding_and_profile_drift_blocks_before_prompt() {
        let state = crate::new_acp_connection_state(
            "connection.resume-fixture",
            AcpConnectionProfile::reviewed(),
        )
        .expect("connection");
        let services = AcpSessionHostServices {
            factory: Factory,
            prompt_resolver: Resolver::default(),
            tool_admission: Admission,
        };
        let mut host = AcpSessionHost::new(state, services);
        let mut peer = Peer;
        host.handle(
            AcpDecodedRequest::Initialize {
                correlation_id: String::from("rpc.load-1"),
                protocol_version: String::from(crate::model::ACP_PROTOCOL_VERSION),
                required_features: Vec::new(),
            },
            &mut peer,
        )
        .expect("initialize");
        host.handle(
            AcpDecodedRequest::LoadSession {
                correlation_id: String::from("rpc.load-2"),
                session_reference: String::from(SESSION_ID),
                workspace_identity: String::from(AUTHORITY_HASH),
                mcp_server_count: 0,
            },
            &mut peer,
        )
        .expect("load");

        assert!(host.session.as_ref().is_some_and(|session| session.resumed));
        assert_eq!(
            host.state()
                .session
                .as_ref()
                .map(|binding| binding.authority_profile_identity.as_str()),
            Some(AUTHORITY_HASH)
        );
        let session = host.session.as_mut().expect("bound fixture session");
        session.authority_profile_identity = String::from(DRIFTED_AUTHORITY_HASH);
        let messages = host
            .handle(
                AcpDecodedRequest::Prompt {
                    correlation_id: String::from("rpc.load-3"),
                    session_id: String::from(SESSION_ID),
                    parts: vec![AcpPromptPart::Text(String::from("must-not-run"))],
                },
                &mut peer,
            )
            .expect("safe drift response");

        assert_eq!(
            host.state().terminal_failure,
            Some(AcpFailureClass::AuthorityProfileMismatch)
        );
        assert!(
            host.session
                .as_ref()
                .is_some_and(|session| session.events.is_empty() && session.terminal.is_none())
        );
        assert_eq!(host.services.prompt_resolver.call_count, 0);
        assert!(messages.iter().any(|message| matches!(
            message,
            AcpHostMessage::ErrorResponse {
                failure: AcpFailureClass::AuthorityProfileMismatch,
                ..
            }
        )));
    }

    fn fake_session() -> FakeSession {
        FakeSession {
            resumed: false,
            authority_profile_identity: String::from(AUTHORITY_HASH),
            active_run: None,
            events: Vec::new(),
            terminal: None,
        }
    }
}
