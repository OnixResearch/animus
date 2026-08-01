//! Non-default coding-session host over explicit injected services.

#![allow(clippy::too_many_lines, missing_docs)]

use animus_runtime::RuntimeEvent;
use animus_runtime::coding_session_host::{
    CodingSessionCommand, CodingSessionCommitRequest, CodingSessionConfig,
    CodingSessionEffectOutcome, CodingSessionEffectRequest, CodingSessionEffectService,
    CodingSessionError, CodingSessionInput, CodingSessionInputPosture, CodingSessionIssue,
    CodingSessionMessage, CodingSessionMessageKind, CodingSessionProviderReply,
    CodingSessionProviderRequest, CodingSessionQueueProfile, CodingSessionReceipt,
    CodingSessionRecoveryRequest, CodingSessionRecoverySnapshot, CodingSessionRecoveryStatus,
    CodingSessionSensitiveDraft, CodingSessionServiceIssue, CodingSessionServiceSet,
    CodingSessionState, CodingSessionSummarizerRequest, CodingSessionTerminalKind,
    CodingSessionToolRequest, CodingSessionTransition, coding_session_effect_step_bound,
    decode_coding_session_message, encode_coding_session_message, new_coding_session_state,
    project_coding_session_receipt, reduce_coding_session_command, reduce_coding_session_outcome,
};
use animus_runtime::session_authority_profile::{
    SessionAuthorityComponentClass, SessionAuthorityEnforcementDraft,
    SessionAuthorityEnforcementObservation, SessionRuntimeAuthorityProfile,
    build_session_authority_enforcement_observation, validate_session_authority_enforcement,
};

const GENERATED_DURABLE_POLICY: &str =
    include_str!("../../../config/generated/durable-session-store-policy.json");
const SMOKE_SESSION_ID: &str = "session.packaged-smoke";
const SMOKE_BRANCH_ID: &str = "branch.main";
const SMOKE_RUN_ID: &str = "run.packaged-smoke";
const SMOKE_USER_ENTRY_ID: &str = "entry.packaged-user";
const SMOKE_STORE_ID: &str = "store.packaged-smoke";
const SMOKE_LOCATION_CAPABILITY: &str = "capability.explicit-store-root";
const SMOKE_RECEIPT_POLICY: &str = "metadata-only";
const SMOKE_JOURNAL_NAME: &str = "coding-session.log";
const SMOKE_LOCK_NAME: &str = "coding-session.lock";
const SMOKE_LEASE_ID: &str = "lease.packaged-smoke";
const SMOKE_PROVIDER_RESPONSE: &str = "deterministic fake provider response";
const SMOKE_SUMMARY: &str = "deterministic fake compaction summary";
const SMOKE_RECEIPT_SCHEMA: &str = "animus.coding-session-packaged-smoke.v1";
const SMOKE_REDACTION_MARKER: &str = "raw-prompt-provider-tool-and-path-material-omitted";
const SMOKE_AUTHORITY_COMPONENT_IDENTITY: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SMOKE_AUTHORIZATION_IDENTITY: &str =
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const SMOKE_NON_CLAIMS: &[&str] = &[
    "live-provider",
    "concrete-product-tools",
    "tui-dogfood",
    "daemon-attach",
    "plugin-runtime",
    "oauth",
    "production-readiness",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodingSessionHostIssue {
    PolicyDenied,
    ReducerRejected,
    ServiceFailed,
    EffectStepBoundExceeded,
    SessionBlocked,
    DurableStoreRejected,
    ReceiptRejected,
}

#[derive(Debug)]
pub struct CodingSessionHostError {
    issue: CodingSessionHostIssue,
    reducer_issue: Option<CodingSessionIssue>,
}

impl CodingSessionHostError {
    const fn new(issue: CodingSessionHostIssue) -> Self {
        Self {
            issue,
            reducer_issue: None,
        }
    }

    fn from_reducer(error: &CodingSessionError) -> Self {
        Self {
            issue: CodingSessionHostIssue::ReducerRejected,
            reducer_issue: Some(error.issue()),
        }
    }

    const fn blocked(reducer_issue: Option<CodingSessionIssue>) -> Self {
        Self {
            issue: CodingSessionHostIssue::SessionBlocked,
            reducer_issue,
        }
    }

    #[must_use]
    pub const fn issue(&self) -> CodingSessionHostIssue {
        self.issue
    }

    #[must_use]
    pub const fn reducer_issue(&self) -> Option<CodingSessionIssue> {
        self.reducer_issue
    }
}

impl core::fmt::Display for CodingSessionHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "coding session host failed: {:?}", self.issue)
    }
}

impl std::error::Error for CodingSessionHostError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodingSessionCommitObservation {
    pub new_head_hash: String,
    pub active_leaf_id: String,
    pub durable_cursor: u64,
    pub synchronized: bool,
}

pub trait CodingSessionProviderService {
    /// # Errors
    /// Returns a typed issue when provider admission or dispatch fails.
    fn dispatch(
        &mut self,
        request: &CodingSessionProviderRequest,
    ) -> Result<CodingSessionProviderReply, CodingSessionServiceIssue>;
}

pub trait CodingSessionToolService {
    /// # Errors
    /// Returns a typed issue when tool admission or execution fails.
    fn dispatch(
        &mut self,
        request: &CodingSessionToolRequest,
    ) -> Result<animus_adapters::ToolResponse, CodingSessionServiceIssue>;
}

pub trait CodingSessionPersistenceService {
    /// # Errors
    /// Returns a typed issue when the durable commit is not confirmed.
    fn commit(
        &mut self,
        request: &CodingSessionCommitRequest,
    ) -> Result<CodingSessionCommitObservation, CodingSessionServiceIssue>;

    /// # Errors
    /// Returns a typed issue when durable recovery or validation fails.
    fn recover(
        &mut self,
        request: &CodingSessionRecoveryRequest,
    ) -> Result<CodingSessionRecoverySnapshot, CodingSessionServiceIssue>;
}

pub trait CodingSessionPolicyService {
    fn admit(&mut self, command: &CodingSessionCommand) -> bool;
}

pub trait CodingSessionAuthorityService {
    /// # Errors
    /// Returns a typed issue when fresh authority enforcement cannot be observed.
    fn observe_current_enforcement(
        &mut self,
        profile: &SessionRuntimeAuthorityProfile,
    ) -> Result<SessionAuthorityEnforcementObservation, CodingSessionServiceIssue>;
}

pub trait CodingSessionCancellationService {
    /// # Errors
    /// Returns a typed issue when cancellation observation fails.
    fn cancel(&mut self, run_id: &str) -> Result<bool, CodingSessionServiceIssue>;
}

pub trait CodingSessionTimerService {
    fn available(&self) -> bool;
}

pub trait CodingSessionSummarizerService {
    /// # Errors
    /// Returns a typed issue when summary generation or admission fails.
    fn summarize(
        &mut self,
        request: &CodingSessionSummarizerRequest,
    ) -> Result<String, CodingSessionServiceIssue>;
}

pub trait CodingSessionEventSink {
    fn observe(&mut self, event: &RuntimeEvent);
}

/// Explicit pre-effect gate used by protocol hosts that must obtain an
/// additional caller-owned decision before the existing service executes.
///
/// A successful gate decision never replaces the coding-session authority
/// probe or the injected service's own admission and enforcement checks.
pub trait CodingSessionEffectGate {
    /// Admits or rejects one pending effect before its existing service runs.
    ///
    /// # Errors
    ///
    /// Returns a typed service issue when the protocol-side decision is absent,
    /// stale, denied, malformed, cancelled, or unavailable.
    fn admit(
        &mut self,
        effect: &CodingSessionEffectRequest,
    ) -> Result<(), CodingSessionServiceIssue>;
}

/// Additive agent-control gate for coding-session provider, tool, and summarizer effects.
pub struct CodingSessionAgentControlGate<'a, Waiter>
where
    Waiter: animus_adapters::AgentControlWait + ?Sized,
{
    waiter: &'a mut Waiter,
}

impl<'a, Waiter> CodingSessionAgentControlGate<'a, Waiter>
where
    Waiter: animus_adapters::AgentControlWait + ?Sized,
{
    /// Builds a coding-session gate over one explicit control waiter.
    #[must_use]
    pub const fn new(waiter: &'a mut Waiter) -> Self {
        Self { waiter }
    }
}

impl<Waiter> CodingSessionEffectGate for CodingSessionAgentControlGate<'_, Waiter>
where
    Waiter: animus_adapters::AgentControlWait + ?Sized,
{
    fn admit(
        &mut self,
        effect: &CodingSessionEffectRequest,
    ) -> Result<(), CodingSessionServiceIssue> {
        // r[impl agent_harness_runtime.agent_control.runtime.gate]
        let Some(checkpoint) = coding_session_control_checkpoint(effect)? else {
            return Ok(());
        };
        match self.waiter.wait_until_runnable(&checkpoint) {
            Ok(animus_adapters::AgentControlPermit::Continue) => Ok(()),
            Ok(animus_adapters::AgentControlPermit::Cancel) => {
                Err(CodingSessionServiceIssue::Cancelled)
            }
            Err(animus_adapters::AgentControlWaitError::RunMismatch) => {
                Err(CodingSessionServiceIssue::Stale)
            }
            Err(
                animus_adapters::AgentControlWaitError::Closed
                | animus_adapters::AgentControlWaitError::SynchronizationFailed
                | animus_adapters::AgentControlWaitError::Unavailable,
            ) => Err(CodingSessionServiceIssue::Unavailable),
        }
    }
}

fn coding_session_control_checkpoint(
    effect: &CodingSessionEffectRequest,
) -> Result<Option<animus_adapters::AgentControlCheckpoint>, CodingSessionServiceIssue> {
    let (run_id, kind) = match effect {
        CodingSessionEffectRequest::Provider(request) => (
            request.run_id(),
            animus_adapters::AgentControlCheckpointKind::BeforeModel,
        ),
        CodingSessionEffectRequest::Tool(request) => (
            request.run_id(),
            animus_adapters::AgentControlCheckpointKind::BeforeTool,
        ),
        CodingSessionEffectRequest::Summarizer(request) => (
            request.run_id(),
            animus_adapters::AgentControlCheckpointKind::BeforeSummarizer,
        ),
        CodingSessionEffectRequest::Commit(_)
        | CodingSessionEffectRequest::Cancellation(_)
        | CodingSessionEffectRequest::Recovery(_) => return Ok(None),
    };
    let run_id = animus_adapters::AgentControlRunId::new(run_id)
        .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
    Ok(Some(animus_adapters::AgentControlCheckpoint::new(
        run_id, kind,
    )))
}

struct AdmitCodingSessionEffects;

impl CodingSessionEffectGate for AdmitCodingSessionEffects {
    fn admit(
        &mut self,
        _effect: &CodingSessionEffectRequest,
    ) -> Result<(), CodingSessionServiceIssue> {
        Ok(())
    }
}

pub struct CodingSessionHostServices<
    Provider,
    Tool,
    Persistence,
    Policy,
    Authority,
    Cancellation,
    Timer,
    Summarizer,
    Events,
> {
    pub provider: Provider,
    pub tool: Tool,
    pub persistence: Persistence,
    pub policy: Policy,
    pub authority: Authority,
    pub cancellation: Cancellation,
    pub timer: Timer,
    pub summarizer: Summarizer,
    pub events: Events,
}

// r[impl agent_harness_runtime.coding_session_host.facade]
pub struct CodingSessionHost<
    Provider,
    Tool,
    Persistence,
    Policy,
    Authority,
    Cancellation,
    Timer,
    Summarizer,
    Events,
> {
    state: CodingSessionState,
    services: CodingSessionHostServices<
        Provider,
        Tool,
        Persistence,
        Policy,
        Authority,
        Cancellation,
        Timer,
        Summarizer,
        Events,
    >,
    events: Vec<RuntimeEvent>,
}

impl<Provider, Tool, Persistence, Policy, Authority, Cancellation, Timer, Summarizer, Events>
    CodingSessionHost<
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
    /// # Errors
    /// Returns a host error when required injected services are unavailable.
    pub fn new(
        state: CodingSessionState,
        services: CodingSessionHostServices<
            Provider,
            Tool,
            Persistence,
            Policy,
            Authority,
            Cancellation,
            Timer,
            Summarizer,
            Events,
        >,
    ) -> Result<Self, CodingSessionHostError> {
        if !services.timer.available() {
            return Err(CodingSessionHostError::new(
                CodingSessionHostIssue::ServiceFailed,
            ));
        }
        Ok(Self {
            state,
            services,
            events: Vec::new(),
        })
    }

    /// # Errors
    /// Returns a host error when policy, reduction, authority, persistence, or effects fail.
    pub fn prompt(
        &mut self,
        run_id: &str,
        entry_id: &str,
        input: CodingSessionInput,
    ) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Prompt {
            run_id: String::from(run_id),
            entry_id: String::from(entry_id),
            input,
        })
    }

    /// # Errors
    /// Returns a host error when continuation cannot be safely reduced and driven.
    pub fn continue_run(&mut self, run_id: &str) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Continue {
            run_id: String::from(run_id),
        })
    }

    /// # Errors
    /// Returns a host error when steering cannot be safely queued and persisted.
    pub fn steer(
        &mut self,
        item_id: &str,
        entry_id: &str,
        input: CodingSessionInput,
    ) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Steer {
            item_id: String::from(item_id),
            entry_id: String::from(entry_id),
            input,
        })
    }

    /// # Errors
    /// Returns a host error when follow-up input cannot be safely queued and persisted.
    pub fn follow_up(
        &mut self,
        item_id: &str,
        entry_id: &str,
        input: CodingSessionInput,
    ) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::FollowUp {
            item_id: String::from(item_id),
            entry_id: String::from(entry_id),
            input,
        })
    }

    /// # Errors
    /// Returns a host error when cancellation cannot be reduced or observed.
    pub fn cancel(&mut self, run_id: &str) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Cancel {
            run_id: String::from(run_id),
        })
    }

    /// # Errors
    /// Returns a host error when recovery or fresh authority admission fails.
    pub fn resume(&mut self) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Resume)?;
        self.admit_current_authority().map_err(|_issue| {
            CodingSessionHostError::blocked(Some(
                CodingSessionIssue::AuthorityEnforcementUnavailable,
            ))
        })
    }

    /// # Errors
    /// Returns a host error when compaction cannot be summarized and committed safely.
    pub fn compact(&mut self, entry_id: &str) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::Compact {
            entry_id: String::from(entry_id),
        })
    }

    /// # Errors
    /// Returns a host error when a queued item cannot be edited and committed safely.
    pub fn edit_queued(
        &mut self,
        item_id: &str,
        entry_id: &str,
        input: CodingSessionInput,
    ) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::EditQueued {
            item_id: String::from(item_id),
            entry_id: String::from(entry_id),
            input,
        })
    }

    /// # Errors
    /// Returns a host error when a queued item cannot be removed and committed safely.
    pub fn remove_queued(&mut self, item_id: &str) -> Result<(), CodingSessionHostError> {
        self.submit(&CodingSessionCommand::RemoveQueued {
            item_id: String::from(item_id),
        })
    }

    /// # Errors
    /// Returns a host error when policy or pure command reduction rejects the command.
    pub fn begin_without_driving(
        &mut self,
        command: &CodingSessionCommand,
    ) -> Result<(), CodingSessionHostError> {
        if !self.services.policy.admit(command) {
            return Err(CodingSessionHostError::new(
                CodingSessionHostIssue::PolicyDenied,
            ));
        }
        let transition = reduce_coding_session_command(&self.state, command)
            .map_err(|error| CodingSessionHostError::from_reducer(&error))?;
        self.apply_transition(transition);
        Ok(())
    }

    /// # Errors
    /// Returns a host error when the pending effect cannot be reduced or executed safely.
    pub fn drive_one_pending(&mut self) -> Result<bool, CodingSessionHostError> {
        let mut gate = AdmitCodingSessionEffects;
        self.drive_one_pending_with_gate(&mut gate)
    }

    /// Drives one pending effect after an explicit protocol-side gate.
    ///
    /// The gate is additive: provider/tool/summarizer authority probes and the
    /// injected effect service still run after the gate admits the effect.
    ///
    /// # Errors
    /// Returns a host error when the gate, service, or outcome reduction fails.
    pub fn drive_one_pending_with_gate<Gate>(
        &mut self,
        gate: &mut Gate,
    ) -> Result<bool, CodingSessionHostError>
    where
        Gate: CodingSessionEffectGate + ?Sized,
    {
        let Some(effect) = self.state.pending_effect().cloned() else {
            self.finish_drive()?;
            return Ok(false);
        };
        let outcome = gate.admit(&effect).map_or_else(
            |issue| failed_outcome(&effect, effect_service(&effect), issue),
            |()| self.execute_effect(&effect),
        );
        let transition = reduce_coding_session_outcome(&self.state, &outcome)
            .map_err(|error| CodingSessionHostError::from_reducer(&error))?;
        self.apply_transition(transition);
        Ok(true)
    }

    /// # Errors
    /// Returns a host error when pending effects fail or exceed the step bound.
    pub fn drive_pending(&mut self) -> Result<(), CodingSessionHostError> {
        let mut gate = AdmitCodingSessionEffects;
        self.drive_pending_with_gate(&mut gate)
    }

    /// Drives all pending effects through an explicit additive gate.
    ///
    /// # Errors
    /// Returns a host error when the gate, service, reducer, or step bound rejects progress.
    pub fn drive_pending_with_gate<Gate>(
        &mut self,
        gate: &mut Gate,
    ) -> Result<(), CodingSessionHostError>
    where
        Gate: CodingSessionEffectGate + ?Sized,
    {
        for _step in 0..coding_session_effect_step_bound() {
            if !self.drive_one_pending_with_gate(gate)? {
                return Ok(());
            }
        }
        Err(CodingSessionHostError::new(
            CodingSessionHostIssue::EffectStepBoundExceeded,
        ))
    }

    #[must_use]
    pub const fn state_snapshot(&self) -> &CodingSessionState {
        &self.state
    }

    #[must_use]
    pub fn message_snapshot(&self) -> &[CodingSessionMessage] {
        self.state.messages()
    }

    #[must_use]
    pub fn event_snapshot(&self) -> &[RuntimeEvent] {
        self.events.as_slice()
    }

    /// # Errors
    /// Returns a host error when receipt projection detects sensitive or inconsistent facts.
    pub fn receipt(
        &self,
        sensitive: CodingSessionSensitiveDraft<'_>,
    ) -> Result<CodingSessionReceipt, CodingSessionHostError> {
        project_coding_session_receipt(&self.state, sensitive)
            .map_err(|_error| CodingSessionHostError::new(CodingSessionHostIssue::ReceiptRejected))
    }

    #[must_use]
    pub fn into_services(
        self,
    ) -> CodingSessionHostServices<
        Provider,
        Tool,
        Persistence,
        Policy,
        Authority,
        Cancellation,
        Timer,
        Summarizer,
        Events,
    > {
        self.services
    }

    fn submit(&mut self, command: &CodingSessionCommand) -> Result<(), CodingSessionHostError> {
        self.begin_without_driving(command)?;
        self.drive_pending()
    }

    fn execute_effect(
        &mut self,
        effect: &CodingSessionEffectRequest,
    ) -> CodingSessionEffectOutcome {
        match effect {
            CodingSessionEffectRequest::Commit(request) => {
                self.services.persistence.commit(request).map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Persistence, issue),
                    |observation| CodingSessionEffectOutcome::CommitSucceeded {
                        effect_id: String::from(request.effect_id()),
                        new_head_hash: observation.new_head_hash,
                        active_leaf_id: observation.active_leaf_id,
                        durable_cursor: observation.durable_cursor,
                        synchronized: observation.synchronized,
                    },
                )
            }
            CodingSessionEffectRequest::Provider(request) => {
                if let Err(issue) = self.admit_current_authority() {
                    return failed_outcome(effect, CodingSessionEffectService::Authority, issue);
                }
                self.services.provider.dispatch(request).map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Provider, issue),
                    |reply| CodingSessionEffectOutcome::ProviderSucceeded {
                        effect_id: String::from(request.effect_id()),
                        reply,
                    },
                )
            }
            CodingSessionEffectRequest::Tool(request) => {
                if let Err(issue) = self.admit_current_authority() {
                    return failed_outcome(effect, CodingSessionEffectService::Authority, issue);
                }
                self.services.tool.dispatch(request).map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Tool, issue),
                    |response| CodingSessionEffectOutcome::ToolSucceeded {
                        effect_id: String::from(request.effect_id()),
                        response,
                    },
                )
            }
            CodingSessionEffectRequest::Cancellation(request) => self
                .services
                .cancellation
                .cancel(request.run_id())
                .map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Cancellation, issue),
                    |observed| CodingSessionEffectOutcome::CancellationObserved {
                        effect_id: String::from(request.effect_id()),
                        run_id: String::from(request.run_id()),
                        observed,
                    },
                ),
            CodingSessionEffectRequest::Recovery(request) => {
                self.services.persistence.recover(request).map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Recovery, issue),
                    |snapshot| CodingSessionEffectOutcome::RecoverySucceeded {
                        effect_id: String::from(request.effect_id()),
                        snapshot,
                    },
                )
            }
            CodingSessionEffectRequest::Summarizer(request) => {
                if let Err(issue) = self.admit_current_authority() {
                    return failed_outcome(effect, CodingSessionEffectService::Authority, issue);
                }
                self.services.summarizer.summarize(request).map_or_else(
                    |issue| failed_outcome(effect, CodingSessionEffectService::Summarizer, issue),
                    |summary| CodingSessionEffectOutcome::SummarizerSucceeded {
                        effect_id: String::from(request.effect_id()),
                        entry_id: String::from(request.entry_id()),
                        summary,
                    },
                )
            }
        }
    }

    fn admit_current_authority(&mut self) -> Result<(), CodingSessionServiceIssue> {
        let profile = self.state.authority_profile().clone();
        let observation = self
            .services
            .authority
            .observe_current_enforcement(&profile)?;
        validate_session_authority_enforcement(&profile, &observation)
            .map_err(|_error| CodingSessionServiceIssue::Denied)?;
        let event = RuntimeEvent::SessionAuthorityProfileObserved {
            session_id: String::from(self.state.session_id()),
            branch_id: String::from(self.state.branch_id()),
            status: String::from("enforcement_admitted"),
            profile_identity: String::from(profile.profile_identity()),
            binding_identity: self.state.authority_binding_identity().map(String::from),
            authorization_identity: Some(String::from(
                self.state.authority_authorization_identity(),
            )),
            enforcement_observation_identity: Some(String::from(
                observation.observation_identity(),
            )),
            changed_components: Vec::new(),
        };
        self.services.events.observe(&event);
        self.events.push(event);
        debug_assert!(self.state.authority_binding_identity().is_some());
        Ok(())
    }

    fn apply_transition(&mut self, transition: CodingSessionTransition) {
        self.state = transition.state;
        for event in transition.events {
            self.services.events.observe(&event);
            self.events.push(event);
        }
    }

    fn finish_drive(&self) -> Result<(), CodingSessionHostError> {
        if self.state.terminal() == Some(CodingSessionTerminalKind::Blocked) {
            Err(CodingSessionHostError::blocked(self.state.last_issue()))
        } else {
            Ok(())
        }
    }
}

pub struct DurableCodingSessionPersistence<'store, Synchronizer>
where
    Synchronizer: super::durablefile::DurableSessionSynchronizer,
{
    store: super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
    policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'store animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
}

impl<'store, Synchronizer> DurableCodingSessionPersistence<'store, Synchronizer>
where
    Synchronizer: super::durablefile::DurableSessionSynchronizer,
{
    #[must_use]
    pub const fn new(
        store: super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
        policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
        descriptor: &'store animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    ) -> Self {
        Self {
            store,
            policy,
            descriptor,
        }
    }

    /// # Errors
    /// Returns a durable-store error when lease cleanup cannot be confirmed.
    pub fn close(self) -> Result<(), super::durablefile::DurableSessionHostError> {
        self.store.close()
    }
}

impl<Synchronizer> CodingSessionPersistenceService
    for DurableCodingSessionPersistence<'_, Synchronizer>
where
    Synchronizer: super::durablefile::DurableSessionSynchronizer,
{
    fn commit(
        &mut self,
        request: &CodingSessionCommitRequest,
    ) -> Result<CodingSessionCommitObservation, CodingSessionServiceIssue> {
        if !durable_binding_matches(self.descriptor, request.session_id(), request.branch_id()) {
            return Err(CodingSessionServiceIssue::Stale);
        }
        let payloads = request
            .messages()
            .iter()
            .map(encode_coding_session_message)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        let entries = request
            .messages()
            .iter()
            .zip(payloads.iter())
            .map(|(message, payload)| {
                animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
                    entry_id: message.entry_id(),
                    parent_id: message.parent_id(),
                    kind: durable_record_kind(message.kind()),
                    payload: payload.as_slice(),
                }
            })
            .collect::<Vec<_>>();
        let first_sequence =
            self.store
                .recovery()
                .committed_frames()
                .last()
                .map_or(Ok(1_u64), |frame| {
                    frame
                        .sequence()
                        .checked_add(1)
                        .ok_or(CodingSessionServiceIssue::Malformed)
                })?;
        let transaction = animus_runtime::durable_session_store::plan_durable_session_transaction(
            self.policy,
            &animus_runtime::durable_session_store::DurableSessionTransactionDraft {
                descriptor: self.descriptor,
                prior_head_hash: request.prior_head_hash(),
                prior_active_leaf_id: request.prior_active_leaf_id(),
                first_sequence,
                active_leaf_id: request.proposed_active_leaf_id(),
                entries: entries.as_slice(),
            },
        )
        .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        let outcome = self
            .store
            .commit(&transaction)
            .map_err(|_error| CodingSessionServiceIssue::Failed)?;
        let durable_cursor = u64::try_from(self.store.recovery().committed_frames().len())
            .map_err(|_overflow| CodingSessionServiceIssue::Malformed)?;
        Ok(CodingSessionCommitObservation {
            new_head_hash: outcome.new_head_hash,
            active_leaf_id: String::from(request.proposed_active_leaf_id()),
            durable_cursor,
            synchronized: outcome.synchronized,
        })
    }

    fn recover(
        &mut self,
        request: &CodingSessionRecoveryRequest,
    ) -> Result<CodingSessionRecoverySnapshot, CodingSessionServiceIssue> {
        if !durable_binding_matches(self.descriptor, request.session_id(), request.branch_id()) {
            return Err(CodingSessionServiceIssue::Stale);
        }
        let recovery = self.store.recovery();
        let messages = recovery
            .committed_frames()
            .iter()
            .map(|frame| {
                let message = decode_coding_session_message(
                    frame.entry_id(),
                    frame.parent_id(),
                    frame.private_payload(),
                )
                .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
                if !record_kind_matches(frame.kind(), message.kind()) {
                    return Err(CodingSessionServiceIssue::Malformed);
                }
                Ok(message)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let status = match recovery.status() {
            animus_runtime::durable_session_store::DurableSessionRecoveryStatus::Empty => {
                CodingSessionRecoveryStatus::Empty
            }
            animus_runtime::durable_session_store::DurableSessionRecoveryStatus::Clean => {
                CodingSessionRecoveryStatus::Clean
            }
            animus_runtime::durable_session_store::DurableSessionRecoveryStatus::IncompleteTailFrame
            | animus_runtime::durable_session_store::DurableSessionRecoveryStatus::IncompleteTailTransaction => {
                CodingSessionRecoveryStatus::IncompleteTail
            }
        };
        let durable_cursor = u64::try_from(messages.len())
            .map_err(|_overflow| CodingSessionServiceIssue::Malformed)?;
        Ok(CodingSessionRecoverySnapshot {
            status,
            committed_head_hash: String::from(recovery.committed_head_hash()),
            active_leaf_id: recovery.active_leaf_id().map(String::from),
            durable_cursor,
            messages,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AllowCodingSessionPolicy;

impl CodingSessionPolicyService for AllowCodingSessionPolicy {
    fn admit(&mut self, _command: &CodingSessionCommand) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AdmittedCodingSessionAuthority {
    pub observation_count: u32,
}

impl CodingSessionAuthorityService for AdmittedCodingSessionAuthority {
    fn observe_current_enforcement(
        &mut self,
        profile: &SessionRuntimeAuthorityProfile,
    ) -> Result<SessionAuthorityEnforcementObservation, CodingSessionServiceIssue> {
        self.observation_count = self
            .observation_count
            .checked_add(1)
            .ok_or(CodingSessionServiceIssue::Failed)?;
        let permission_identity = profile
            .component(SessionAuthorityComponentClass::Permission)
            .ok_or(CodingSessionServiceIssue::Unavailable)?
            .identity();
        let sandbox_identity = profile
            .component(SessionAuthorityComponentClass::SandboxIntent)
            .ok_or(CodingSessionServiceIssue::Unavailable)?
            .identity();
        let adapter_identity = profile
            .component(SessionAuthorityComponentClass::AdapterCohort)
            .ok_or(CodingSessionServiceIssue::Unavailable)?
            .identity();
        let tool_identity = profile
            .component(SessionAuthorityComponentClass::ToolCatalog)
            .ok_or(CodingSessionServiceIssue::Unavailable)?
            .identity();
        let model_identity = profile
            .component(SessionAuthorityComponentClass::ModelRuntime)
            .ok_or(CodingSessionServiceIssue::Unavailable)?
            .identity();
        build_session_authority_enforcement_observation(SessionAuthorityEnforcementDraft {
            profile_identity: profile.profile_identity(),
            authorization_identity: SMOKE_AUTHORIZATION_IDENTITY,
            adapter_cohort_identity: adapter_identity,
            capability_observation_identity: permission_identity,
            tool_readiness_identity: tool_identity,
            provider_readiness_identity: model_identity,
            sandbox_observation_identity: sandbox_identity,
            admitted: true,
        })
        .map_err(|_error| CodingSessionServiceIssue::Malformed)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoopCodingSessionEventSink;

impl CodingSessionEventSink for NoopCodingSessionEventSink {
    fn observe(&mut self, _event: &RuntimeEvent) {}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AvailableCodingSessionTimer;

impl CodingSessionTimerService for AvailableCodingSessionTimer {
    fn available(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ObservedCodingSessionCancellation;

impl CodingSessionCancellationService for ObservedCodingSessionCancellation {
    fn cancel(&mut self, _run_id: &str) -> Result<bool, CodingSessionServiceIssue> {
        Ok(true)
    }
}

#[derive(Clone, Debug)]
pub struct FakeCodingSessionProvider {
    pub response: String,
    pub dispatch_count: u32,
}

impl CodingSessionProviderService for FakeCodingSessionProvider {
    fn dispatch(
        &mut self,
        _request: &CodingSessionProviderRequest,
    ) -> Result<CodingSessionProviderReply, CodingSessionServiceIssue> {
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        let response = animus_adapters::ModelResponse::new(self.response.as_str())
            .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        Ok(CodingSessionProviderReply::Final(response))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DenyUnusedCodingSessionTool;

impl CodingSessionToolService for DenyUnusedCodingSessionTool {
    fn dispatch(
        &mut self,
        _request: &CodingSessionToolRequest,
    ) -> Result<animus_adapters::ToolResponse, CodingSessionServiceIssue> {
        Err(CodingSessionServiceIssue::Denied)
    }
}

#[derive(Clone, Debug)]
pub struct FakeCodingSessionSummarizer {
    pub summary: String,
}

impl CodingSessionSummarizerService for FakeCodingSessionSummarizer {
    fn summarize(
        &mut self,
        _request: &CodingSessionSummarizerRequest,
    ) -> Result<String, CodingSessionServiceIssue> {
        Ok(self.summary.clone())
    }
}

// r[impl agent_harness_runtime.coding_session_host.conformance]
/// # Errors
/// Returns a host error when the packaged durable smoke path cannot complete safely.
pub fn run_packaged_durable_coding_session_smoke(
    store_root: &std::path::Path,
    prompt: &str,
) -> Result<String, CodingSessionHostError> {
    let policy =
        super::durablefile::parse_durable_session_store_policy_json(GENERATED_DURABLE_POLICY)
            .map_err(|_error| {
                CodingSessionHostError::new(CodingSessionHostIssue::DurableStoreRejected)
            })?;
    let descriptor = animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(SMOKE_STORE_ID),
        location_capability: String::from(SMOKE_LOCATION_CAPABILITY),
        session_id: String::from(SMOKE_SESSION_ID),
        branch_id: String::from(SMOKE_BRANCH_ID),
        policy_hash: String::from(policy.policy_hash()),
        durability:
            animus_runtime::durable_session_store::DurableSessionDurability::SyncAllEachCommit,
        locking:
            animus_runtime::durable_session_store::DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data:
            animus_runtime::durable_session_store::DurableSessionPrivacyPosture::ReplayPrivate,
        encryption:
            animus_runtime::durable_session_store::DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from(SMOKE_RECEIPT_POLICY),
    };
    let root = cap_std::fs::Dir::open_ambient_dir(store_root, cap_std::ambient_authority())
        .map_err(|_error| {
            CodingSessionHostError::new(CodingSessionHostIssue::DurableStoreRejected)
        })?;
    let store = super::durablefile::open_durable_session_store(
        &super::durablefile::DurableSessionStoreOpenRequest {
            policy: &policy,
            descriptor: &descriptor,
            location: super::durablefile::DurableSessionStoreLocation {
                root: &root,
                journal_name: SMOKE_JOURNAL_NAME,
                lock_name: SMOKE_LOCK_NAME,
            },
            lease_id: SMOKE_LEASE_ID,
        },
        super::durablefile::FileSynchronizer,
    )
    .map_err(|_error| CodingSessionHostError::new(CodingSessionHostIssue::DurableStoreRejected))?;
    let state = new_coding_session_state(
        CodingSessionConfig {
            session_id: String::from(SMOKE_SESSION_ID),
            branch_id: String::from(SMOKE_BRANCH_ID),
            queue_profile: CodingSessionQueueProfile::Sequential,
            services: CodingSessionServiceSet::all_available(),
            authority_profile: smoke_authority_profile()?,
            authority_authorization_identity: String::from(SMOKE_AUTHORIZATION_IDENTITY),
            resume_required: false,
        },
        smoke_queue_policy()?,
    )
    .map_err(|error| CodingSessionHostError::from_reducer(&error))?;
    let persistence = DurableCodingSessionPersistence::new(store, &policy, &descriptor);
    let services = CodingSessionHostServices {
        provider: FakeCodingSessionProvider {
            response: String::from(SMOKE_PROVIDER_RESPONSE),
            dispatch_count: 0,
        },
        tool: DenyUnusedCodingSessionTool,
        persistence,
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: NoopCodingSessionEventSink,
    };
    let mut host = CodingSessionHost::new(state, services)?;
    host.prompt(
        SMOKE_RUN_ID,
        SMOKE_USER_ENTRY_ID,
        CodingSessionInput {
            body: String::from(prompt),
            posture: CodingSessionInputPosture::ExpandedAndAdmitted,
        },
    )?;
    let receipt = host.receipt(CodingSessionSensitiveDraft::default())?;
    let rendered = render_smoke_receipt(&receipt);
    let services = host.into_services();
    services.persistence.close().map_err(|_error| {
        CodingSessionHostError::new(CodingSessionHostIssue::DurableStoreRejected)
    })?;
    Ok(rendered)
}

fn smoke_queue_policy() -> Result<animus_runtime::MessageQueuePolicy, CodingSessionHostError> {
    const MAX_QUEUE_ITEMS: u32 = 8;
    const POLICY_HASH: &str = "e97d1a2c69d597e28fdd17eab77e086b7e7687a5364e2b43675820e59c961d8d";
    animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
        max_items: MAX_QUEUE_ITEMS,
        allowed_sources: &["coding-session-host"],
        policy_label: "packaged-coding-session-queue",
        policy_hash: POLICY_HASH,
        allow_queue_mutations: true,
        allow_interrupts: true,
    })
    .map_err(|_error| CodingSessionHostError::new(CodingSessionHostIssue::ReducerRejected))
}

fn render_smoke_receipt(receipt: &CodingSessionReceipt) -> String {
    let mut lines = vec![format!(
        "CODING_SESSION_SMOKE schema={SMOKE_RECEIPT_SCHEMA} status={} committed_messages={} commits={} provider_dispatches={} tool_dispatches={} durable_cursor={} head_hash={} receipt_hash={} redaction={SMOKE_REDACTION_MARKER}",
        receipt.terminal_status.as_str(),
        receipt.committed_message_count,
        receipt.commit_count,
        receipt.provider_dispatch_count,
        receipt.tool_dispatch_count,
        receipt.durable_cursor,
        receipt.committed_head_hash,
        receipt.receipt_hash,
    )];
    lines.extend(
        SMOKE_NON_CLAIMS
            .iter()
            .map(|surface| format!("NON_CLAIM surface={surface}")),
    );
    lines.push(String::new());
    lines.join("\n")
}

fn smoke_authority_profile() -> Result<
    animus_runtime::session_authority_profile::SessionRuntimeAuthorityProfile,
    CodingSessionHostError,
> {
    let components = animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL
        .iter()
        .map(
            |class| animus_runtime::session_authority_profile::SessionAuthorityComponentDraft {
                class: *class,
                identity: SMOKE_AUTHORITY_COMPONENT_IDENTITY,
            },
        )
        .collect::<Vec<_>>();
    animus_runtime::session_authority_profile::build_session_authority_profile(
        animus_runtime::session_authority_profile::SessionAuthorityProfileDraft {
            components: components.as_slice(),
        },
    )
    .map_err(|_error| CodingSessionHostError::new(CodingSessionHostIssue::ReducerRejected))
}

const fn effect_service(effect: &CodingSessionEffectRequest) -> CodingSessionEffectService {
    match effect {
        CodingSessionEffectRequest::Commit(_) => CodingSessionEffectService::Persistence,
        CodingSessionEffectRequest::Provider(_) => CodingSessionEffectService::Provider,
        CodingSessionEffectRequest::Tool(_) => CodingSessionEffectService::Tool,
        CodingSessionEffectRequest::Cancellation(_) => CodingSessionEffectService::Cancellation,
        CodingSessionEffectRequest::Recovery(_) => CodingSessionEffectService::Recovery,
        CodingSessionEffectRequest::Summarizer(_) => CodingSessionEffectService::Summarizer,
    }
}

fn failed_outcome(
    effect: &CodingSessionEffectRequest,
    service: CodingSessionEffectService,
    issue: CodingSessionServiceIssue,
) -> CodingSessionEffectOutcome {
    CodingSessionEffectOutcome::Failed {
        effect_id: String::from(effect.effect_id()),
        service,
        issue,
    }
}

fn durable_binding_matches(
    descriptor: &animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    session_id: &str,
    branch_id: &str,
) -> bool {
    descriptor.session_id == session_id && descriptor.branch_id == branch_id
}

const fn durable_record_kind(
    kind: CodingSessionMessageKind,
) -> animus_runtime::durable_session_store::DurableSessionRecordKind {
    match kind {
        CodingSessionMessageKind::User => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::User
        }
        CodingSessionMessageKind::Assistant | CodingSessionMessageKind::AssistantToolCall => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::Assistant
        }
        CodingSessionMessageKind::ToolResult => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::ToolResult
        }
        CodingSessionMessageKind::Summary => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::Summary
        }
        CodingSessionMessageKind::Repair => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::Repair
        }
        CodingSessionMessageKind::BranchMove => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::BranchMove
        }
        CodingSessionMessageKind::AuthorityProfileBinding => {
            animus_runtime::durable_session_store::DurableSessionRecordKind::AuthorityProfileBinding
        }
    }
}

const fn record_kind_matches(
    durable: animus_runtime::durable_session_store::DurableSessionRecordKind,
    message: CodingSessionMessageKind,
) -> bool {
    matches!(
        (durable, message),
        (
            animus_runtime::durable_session_store::DurableSessionRecordKind::User,
            CodingSessionMessageKind::User
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::Assistant,
            CodingSessionMessageKind::Assistant | CodingSessionMessageKind::AssistantToolCall
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::ToolResult,
            CodingSessionMessageKind::ToolResult
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::Summary,
            CodingSessionMessageKind::Summary
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::Repair,
            CodingSessionMessageKind::Repair
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::BranchMove,
            CodingSessionMessageKind::BranchMove
        ) | (
            animus_runtime::durable_session_store::DurableSessionRecordKind::AuthorityProfileBinding,
            CodingSessionMessageKind::AuthorityProfileBinding
        )
    )
}

#[cfg(test)]
#[path = "codingsession/tests.rs"]
mod tests;
