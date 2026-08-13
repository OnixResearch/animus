//! Non-default concrete child-session host over normal coding-session controllers.
//!
//! The host owns one explicit registry. A caller injects the factory, lifecycle
//! store, current-authority probe, timer, and event sink. No process-global
//! registry, provider discovery, credential discovery, or ambient fallback exists.

use std::collections::{BTreeMap, BTreeSet};

use animus_runtime::child_session_service::{
    ChildSessionFailureClass, ChildSessionHead, ChildSessionLifecycleRecord,
    ChildSessionLifecycleState, ChildSessionResultClass, ChildSessionServiceSet,
    ConcreteChildSessionAdmission, ConcreteChildSessionResult, ConcreteChildSessionResultDraft,
    child_session_hash_parts, validate_child_session_lifecycle_record,
};
use animus_runtime::coding_session_host::{
    CodingSessionInput, CodingSessionInputPosture, CodingSessionMessageKind,
};
use animus_runtime::subagents::{
    SubagentCoordinatorEvent, SubagentDiagnostic, SubagentDiagnosticKind, SubagentHandle,
    SubagentHandleBinding, SubagentLifecycleEvent, SubagentLineageRecordDraft, SubagentReport,
    SubagentRunStatus, SubagentService, SubagentServiceFailure, SubagentServiceFailureKind,
    transition_subagent_state, validate_subagent_lineage_record,
};

use crate::coding_session_host::{
    CodingSessionAuthorityService, CodingSessionCancellationService, CodingSessionEventSink,
    CodingSessionHost, CodingSessionHostError, CodingSessionHostIssue,
    CodingSessionPersistenceService, CodingSessionPolicyService, CodingSessionProviderService,
    CodingSessionSummarizerService, CodingSessionTimerService, CodingSessionToolService,
};

const HANDLE_HASH_DOMAIN: &str = "animus.concrete-child-session.handle.v1";
const OUTPUT_HASH_DOMAIN: &str = "animus.concrete-child-session.output.v1";
const MAX_SAFE_DIAGNOSTIC_CHARS: usize = 256;
const MAX_EVIDENCE_REFS: usize = 16;
const LINEAGE_EVIDENCE_REF_COUNT: usize = 1;
const REDACTED_PRIVATE: &str = "<private>";
const LINEAGE_DELEGATION_SOURCE: &str = "concrete-child-session-service";
const LINEAGE_SAFE_SUMMARY: &str = "bounded-child-session-result";
const PRIVATE_RESULT_MARKERS: &[&str] = &[
    "RAW_TRANSCRIPT:",
    "RAW_CHILD_LOG:",
    "PROVIDER_PAYLOAD:",
    "BEGIN PRIVATE KEY",
    "API_KEY=",
    "AWS_SECRET_ACCESS_KEY",
    "/home/",
];

/// Safe failure returned by an explicit factory, controller, or host service.
#[derive(Clone, Eq, PartialEq)]
pub struct ChildSessionHostFailure {
    /// Stable failure class.
    pub class: ChildSessionFailureClass,
    /// Safe bounded diagnostic without child content.
    diagnostic: String,
}

impl ChildSessionHostFailure {
    /// Builds a safe host failure.
    #[must_use]
    pub fn new(class: ChildSessionFailureClass, diagnostic: impl AsRef<str>) -> Self {
        let diagnostic = diagnostic.as_ref().trim();
        let bounded = diagnostic
            .chars()
            .take(MAX_SAFE_DIAGNOSTIC_CHARS)
            .collect::<String>();
        Self {
            class,
            diagnostic: if bounded.is_empty() {
                String::from("child-session host operation failed")
            } else {
                bounded
            },
        }
    }

    /// Returns the safe bounded diagnostic.
    #[must_use]
    pub fn diagnostic(&self) -> &str {
        self.diagnostic.as_str()
    }
}

impl core::fmt::Debug for ChildSessionHostFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ChildSessionHostFailure")
            .field("class", &self.class)
            .field("diagnostic", &self.diagnostic)
            .finish()
    }
}

impl core::fmt::Display for ChildSessionHostFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "child-session host failed: {:?}", self.class)
    }
}

impl std::error::Error for ChildSessionHostFailure {}

/// Current safe controller status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildSessionControllerStatus {
    /// Normal coding-session work remains active.
    Running,
    /// The normal coding session completed and can be collected.
    Completed,
    /// Cancellation is in progress.
    Cancelling,
    /// The normal coding session failed.
    Failed,
    /// The normal coding session timed out.
    TimedOut,
    /// Cancellation completed.
    Cancelled,
}

/// Private terminal controller outcome before bounded parent projection.
#[derive(Clone, Eq, PartialEq)]
pub struct ChildSessionControllerResult {
    /// Terminal class observed by the controller.
    pub class: ChildSessionResultClass,
    /// Optional private result or partial body.
    body: Option<String>,
    /// Optional safe failure class.
    pub failure_class: Option<ChildSessionFailureClass>,
    /// Safe evidence references.
    pub evidence_refs: Vec<String>,
}

impl ChildSessionControllerResult {
    /// Builds a controller result. The concrete service applies the parent bound.
    #[must_use]
    pub fn new(
        class: ChildSessionResultClass,
        body: Option<String>,
        failure_class: Option<ChildSessionFailureClass>,
        evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            class,
            body,
            failure_class,
            evidence_refs,
        }
    }

    /// Returns the private candidate body to the bounding service.
    #[must_use]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

impl core::fmt::Debug for ChildSessionControllerResult {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ChildSessionControllerResult")
            .field("class", &self.class)
            .field("body", &self.body.as_ref().map(|_body| REDACTED_PRIVATE))
            .field("failure_class", &self.failure_class)
            .field("evidence_refs", &self.evidence_refs)
            .finish()
    }
}

/// Owned controller for one normal child coding-session instance.
pub trait ChildCodingSessionController {
    /// Starts the child through the normal coding-session prompt path.
    ///
    /// # Errors
    ///
    /// Returns a safe failure when normal session creation cannot progress.
    fn start(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure>;

    /// Returns current safe status.
    ///
    /// # Errors
    ///
    /// Returns a safe failure when status cannot be observed.
    fn status(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure>;

    /// Requests cancellation through the normal coding-session cancellation path.
    ///
    /// # Errors
    ///
    /// Returns a safe failure when cancellation cannot be observed.
    fn cancel(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure>;

    /// Collects one terminal candidate outcome.
    ///
    /// # Errors
    ///
    /// Returns a safe failure when the child is not terminal or is malformed.
    fn collect(&mut self) -> Result<ChildSessionControllerResult, ChildSessionHostFailure>;

    /// Returns metadata-only durable coding-session head facts.
    fn session_head(&self) -> ChildSessionHead;
}

/// Explicit factory for normal child coding-session controllers.
pub trait ChildSessionFactory {
    /// Returns the exact explicit service inventory.
    fn services(&self) -> ChildSessionServiceSet;

    /// Returns a BLAKE3 identity for the current explicit service composition.
    fn service_identity(&self) -> &str;

    /// Creates one normal child coding-session controller.
    ///
    /// # Errors
    ///
    /// Returns a typed safe failure without ambient discovery.
    fn create(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure>;

    /// Reopens one normal child through its coding-session recovery path.
    ///
    /// # Errors
    ///
    /// Returns a typed safe failure without creating replacement services.
    fn restore(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
        record: &ChildSessionLifecycleRecord,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure>;
}

/// Explicit metadata-only lifecycle store.
pub trait ChildSessionLifecycleStore {
    /// Persists one safe lifecycle record.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when persistence does not complete.
    fn persist(
        &mut self,
        service_id: &str,
        record: &ChildSessionLifecycleRecord,
    ) -> Result<(), ChildSessionHostFailure>;

    /// Loads safe lifecycle records for one explicit service instance.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when recovery facts cannot be loaded.
    fn load(
        &mut self,
        service_id: &str,
    ) -> Result<Vec<ChildSessionLifecycleRecord>, ChildSessionHostFailure>;
}

/// Explicit current parent-authority observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildSessionAuthorityObservation {
    /// Parent identity that the observation covers.
    pub parent_id: String,
    /// Current parent authority identity.
    pub authority_identity: String,
    /// Whether current policy admits child lifecycle access.
    pub admitted: bool,
}

/// Injected current-authority probe.
pub trait ChildSessionAuthorityService {
    /// Observes current authority for one exact parent.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when current authority is unavailable.
    fn observe(
        &mut self,
        parent_id: &str,
    ) -> Result<ChildSessionAuthorityObservation, ChildSessionHostFailure>;
}

/// Injected monotonic timer.
pub trait ChildSessionTimer {
    /// Returns current elapsed-time source value in milliseconds.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when time cannot be observed.
    fn now_ms(&mut self) -> Result<u64, ChildSessionHostFailure>;
}

/// Safe metadata-only child-session lifecycle event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildSessionEvent {
    /// Parent identity.
    pub parent_id: String,
    /// Child identity.
    pub child_id: String,
    /// Registry generation.
    pub generation: u64,
    /// Safe lifecycle state.
    pub state: ChildSessionLifecycleState,
    /// Concrete admission identity.
    pub admission_identity: String,
    /// Optional result identity.
    pub result_identity: Option<String>,
    /// Optional safe failure class.
    pub failure_class: Option<ChildSessionFailureClass>,
}

/// Injected metadata-only event sink.
pub trait ChildSessionEventSink {
    /// Observes one safe child-session event.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when event publication does not complete.
    fn observe(&mut self, event: &ChildSessionEvent) -> Result<(), ChildSessionHostFailure>;
}

/// Normal coding-session controller adapter.
pub struct NormalCodingSessionController<
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
    host: CodingSessionHost<
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
    run_id: String,
    entry_id: String,
    started: bool,
}

impl<Provider, Tool, Persistence, Policy, Authority, Cancellation, Timer, Summarizer, Events>
    NormalCodingSessionController<
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
    /// Wraps one newly constructed normal coding-session host.
    #[must_use]
    pub fn new(
        host: CodingSessionHost<
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
        run_id: impl Into<String>,
        entry_id: impl Into<String>,
    ) -> Self {
        Self {
            host,
            run_id: run_id.into(),
            entry_id: entry_id.into(),
            started: false,
        }
    }

    /// Reopens one normal coding-session host through its accepted recovery path.
    ///
    /// # Errors
    ///
    /// Returns a typed failure when coding-session recovery blocks.
    pub fn restore(
        mut host: CodingSessionHost<
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
        run_id: impl Into<String>,
        entry_id: impl Into<String>,
    ) -> Result<Self, ChildSessionHostFailure> {
        host.resume()
            .map_err(|error| map_coding_session_error(&error))?;
        Ok(Self {
            host,
            run_id: run_id.into(),
            entry_id: entry_id.into(),
            started: true,
        })
    }
}

impl<Provider, Tool, Persistence, Policy, Authority, Cancellation, Timer, Summarizer, Events>
    ChildCodingSessionController
    for NormalCodingSessionController<
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
    fn start(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure> {
        if self.started {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Session,
                "normal child coding session was already started",
            ));
        }
        let input = CodingSessionInput {
            body: admission.objective().to_owned(),
            posture: CodingSessionInputPosture::ExpandedAndAdmitted,
        };
        match self
            .host
            .prompt(self.run_id.as_str(), self.entry_id.as_str(), input)
        {
            Ok(()) => {
                self.started = true;
                Ok(())
            }
            Err(error) if error.issue() == CodingSessionHostIssue::SessionBlocked => {
                self.started = true;
                Ok(())
            }
            Err(error) => Err(map_coding_session_error(&error)),
        }
    }

    fn status(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure> {
        Ok(normal_status(self.host.state_snapshot()))
    }

    fn cancel(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure> {
        match self.host.cancel(self.run_id.as_str()) {
            Ok(()) => self.status(),
            Err(error) if error.issue() == CodingSessionHostIssue::SessionBlocked => self.status(),
            Err(error) => Err(map_coding_session_error(&error)),
        }
    }

    fn collect(&mut self) -> Result<ChildSessionControllerResult, ChildSessionHostFailure> {
        let state = self.host.state_snapshot();
        let body = self
            .host
            .message_snapshot()
            .iter()
            .rev()
            .find(|message| message.kind() == CodingSessionMessageKind::Assistant)
            .map(|message| message.body().to_owned());
        match state.terminal() {
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Completed) => {
                let Some(body) = body else {
                    return Err(ChildSessionHostFailure::new(
                        ChildSessionFailureClass::Malformed,
                        "completed child coding session has no assistant result",
                    ));
                };
                Ok(ChildSessionControllerResult::new(
                    ChildSessionResultClass::Completed,
                    Some(body),
                    None,
                    Vec::new(),
                ))
            }
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Cancelled) => {
                Ok(ChildSessionControllerResult::new(
                    if body.is_some() {
                        ChildSessionResultClass::Partial
                    } else {
                        ChildSessionResultClass::Cancelled
                    },
                    body,
                    Some(ChildSessionFailureClass::Cancelled),
                    Vec::new(),
                ))
            }
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Blocked) => {
                Ok(ChildSessionControllerResult::new(
                    if body.is_some() {
                        ChildSessionResultClass::Partial
                    } else {
                        ChildSessionResultClass::Failed
                    },
                    body,
                    Some(ChildSessionFailureClass::Session),
                    Vec::new(),
                ))
            }
            None => Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Session,
                "child coding session is not terminal",
            )),
        }
    }

    fn session_head(&self) -> ChildSessionHead {
        ChildSessionHead {
            committed_head_hash: self.host.state_snapshot().durable_head_hash().to_owned(),
            active_leaf_id: self
                .host
                .state_snapshot()
                .active_leaf_id()
                .map(str::to_owned),
            durable_cursor: self.host.state_snapshot().durable_cursor(),
        }
    }
}

/// Recovery summary for one service instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildSessionRestoreSummary {
    /// Reopened generation-fenced handles.
    pub handles: Vec<SubagentHandle>,
    /// Terminal durable records that required no controller.
    pub skipped_terminal: u32,
}

struct RegistryEntry {
    admission: ConcreteChildSessionAdmission,
    handle: SubagentHandle,
    controller: Box<dyn ChildCodingSessionController>,
    started_at_ms: u64,
    state: ChildSessionLifecycleState,
    failure_class: Option<ChildSessionFailureClass>,
    result: Option<ConcreteChildSessionResult>,
    report: Option<SubagentReport>,
}

/// Explicit services owned by one concrete child-session service instance.
pub struct ConcreteChildSessionServices<Factory, Store, Authority, Timer, Events> {
    /// Explicit normal coding-session factory.
    pub factory: Factory,
    /// Explicit metadata-only lifecycle store.
    pub store: Store,
    /// Explicit current-authority observation service.
    pub authority: Authority,
    /// Explicit monotonic timer.
    pub timer: Timer,
    /// Explicit metadata-only event sink.
    pub events: Events,
}

/// Concrete per-instance child-session service.
pub struct ConcreteChildSessionService<Factory, Store, Authority, Timer, Events> {
    service_id: String,
    factory: Factory,
    store: Store,
    authority: Authority,
    timer: Timer,
    events: Events,
    staged: BTreeMap<String, ConcreteChildSessionAdmission>,
    registry: BTreeMap<String, RegistryEntry>,
    next_generation: u64,
}

impl<Factory, Store, Authority, Timer, Events>
    ConcreteChildSessionService<Factory, Store, Authority, Timer, Events>
where
    Factory: ChildSessionFactory,
    Store: ChildSessionLifecycleStore,
    Authority: ChildSessionAuthorityService,
    Timer: ChildSessionTimer,
    Events: ChildSessionEventSink,
{
    /// Builds one explicit non-global child-session service.
    ///
    /// # Errors
    ///
    /// Returns a typed failure for an empty service identity or malformed factory identity.
    pub fn new(
        service_id: impl AsRef<str>,
        services: ConcreteChildSessionServices<Factory, Store, Authority, Timer, Events>,
    ) -> Result<Self, ChildSessionHostFailure> {
        let service_id = service_id.as_ref().trim();
        if service_id.is_empty() || service_id.contains('\0') {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Malformed,
                "child-session service identity is malformed",
            ));
        }
        validate_blake3(
            services.factory.service_identity(),
            "factory service identity",
        )?;
        Ok(Self {
            service_id: service_id.to_owned(),
            factory: services.factory,
            store: services.store,
            authority: services.authority,
            timer: services.timer,
            events: services.events,
            staged: BTreeMap::new(),
            registry: BTreeMap::new(),
            next_generation: 1,
        })
    }

    /// Stages one fully admitted request for the existing `SubagentService` port.
    ///
    /// # Errors
    ///
    /// Returns a typed failure for duplicate parent-child identity or service gaps.
    pub fn stage(
        &mut self,
        admission: ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure> {
        if let Some(missing) = self.factory.services().first_missing_for(&admission) {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::MissingService,
                format!("required child-session service is absent: {missing:?}"),
            ));
        }
        let key = admission_key(
            admission.subagent().parent_id.as_str(),
            admission.subagent().child_id.as_str(),
        );
        if self.staged.contains_key(key.as_str())
            || self.registry.values().any(|entry| {
                entry.admission.subagent().parent_id == admission.subagent().parent_id
                    && entry.admission.subagent().child_id == admission.subagent().child_id
            })
        {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::DuplicateChild,
                "duplicate child identity is already staged or active",
            ));
        }
        self.staged.insert(key, admission);
        Ok(())
    }

    /// Returns the number of service-local registered controllers.
    #[must_use]
    pub fn registry_len(&self) -> usize {
        self.registry.len()
    }

    /// Collects a typed bounded parent result.
    ///
    /// # Errors
    ///
    /// Returns a typed failure for stale access, non-terminal work, malformed
    /// output, persistence failure, or event failure.
    pub fn collect_result(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<ConcreteChildSessionResult, ChildSessionHostFailure> {
        self.collect_internal(handle)
            .map(|(result, _report)| result)
    }

    /// Restores admitted non-terminal children through the factory recovery path.
    ///
    /// # Errors
    ///
    /// Returns a typed failure for missing admission, service drift, duplicate
    /// records, stale authority, head drift, or store failure.
    // r[impl agent_harness_runtime.child_session_service.persistence.restore]
    pub fn restore(
        &mut self,
        admissions: &[ConcreteChildSessionAdmission],
    ) -> Result<ChildSessionRestoreSummary, ChildSessionHostFailure> {
        let records = self.store.load(self.service_id.as_str())?;
        let mut seen_children = BTreeSet::new();
        let mut handles = Vec::new();
        let mut skipped_terminal = 0_u32;
        for raw_record in records {
            let record = validate_child_session_lifecycle_record(raw_record).map_err(|_error| {
                ChildSessionHostFailure::new(
                    ChildSessionFailureClass::Malformed,
                    "durable child lifecycle record is malformed",
                )
            })?;
            let child_key = admission_key(record.parent_id.as_str(), record.child_id.as_str());
            if !seen_children.insert(child_key) {
                return Err(ChildSessionHostFailure::new(
                    ChildSessionFailureClass::DuplicateChild,
                    "durable child lifecycle records contain a duplicate child",
                ));
            }
            if record.state.is_terminal() {
                skipped_terminal = skipped_terminal.checked_add(1).ok_or_else(|| {
                    ChildSessionHostFailure::new(
                        ChildSessionFailureClass::BoundExceeded,
                        "terminal restore count overflowed",
                    )
                })?;
                continue;
            }
            let admission = admissions
                .iter()
                .find(|candidate| {
                    candidate.subagent().parent_id == record.parent_id
                        && candidate.subagent().child_id == record.child_id
                        && candidate.admission_identity() == record.admission_identity
                })
                .cloned()
                .ok_or_else(|| {
                    ChildSessionHostFailure::new(
                        ChildSessionFailureClass::RestoreDrift,
                        "non-terminal child has no matching current admission",
                    )
                })?;
            self.validate_factory_services(&admission)?;
            if self.factory.service_identity() != record.factory_service_identity {
                return Err(ChildSessionHostFailure::new(
                    ChildSessionFailureClass::RestoreDrift,
                    "factory service composition differs from the durable record",
                ));
            }
            self.validate_current_authority(&admission)?;
            let controller = self.factory.restore(&admission, &record)?;
            if controller.session_head() != record.session_head {
                return Err(ChildSessionHostFailure::new(
                    ChildSessionFailureClass::RestoreDrift,
                    "restored coding-session head differs from the durable child record",
                ));
            }
            let handle = self.make_handle(&admission, record.generation)?;
            if self.registry.contains_key(handle.handle_id.as_str()) {
                return Err(ChildSessionHostFailure::new(
                    ChildSessionFailureClass::DuplicateChild,
                    "restored handle duplicates an active registry entry",
                ));
            }
            self.next_generation =
                self.next_generation
                    .max(record.generation.checked_add(1).ok_or_else(|| {
                        ChildSessionHostFailure::new(
                            ChildSessionFailureClass::BoundExceeded,
                            "child generation overflowed during restore",
                        )
                    })?);
            let entry = RegistryEntry {
                admission,
                handle: handle.clone(),
                controller,
                started_at_ms: record.started_at_ms,
                state: record.state,
                failure_class: record.failure_class,
                result: None,
                report: None,
            };
            self.registry.insert(handle.handle_id.clone(), entry);
            self.persist_and_emit(handle.handle_id.as_str())?;
            handles.push(handle);
        }
        Ok(ChildSessionRestoreSummary {
            handles,
            skipped_terminal,
        })
    }

    /// Returns the owned explicit services after host use.
    #[must_use]
    pub fn into_services(self) -> (Factory, Store, Authority, Timer, Events) {
        (
            self.factory,
            self.store,
            self.authority,
            self.timer,
            self.events,
        )
    }

    fn enqueue_internal(
        &mut self,
        subagent: &animus_runtime::subagents::SubagentAdmission,
    ) -> Result<SubagentHandle, ChildSessionHostFailure> {
        let key = admission_key(subagent.parent_id.as_str(), subagent.child_id.as_str());
        let admission = self.staged.get(key.as_str()).cloned().ok_or_else(|| {
            ChildSessionHostFailure::new(
                ChildSessionFailureClass::Denied,
                "subagent admission has no explicitly staged child-session request",
            )
        })?;
        if admission.subagent() != subagent {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Denied,
                "staged child-session request differs from the subagent admission",
            ));
        }
        self.validate_factory_services(&admission)?;
        self.validate_current_authority(&admission)?;
        if self.registry.values().any(|entry| {
            entry.admission.subagent().parent_id == subagent.parent_id
                && entry.admission.subagent().child_id == subagent.child_id
        }) {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::DuplicateChild,
                "child identity is already registered",
            ));
        }
        let started_at_ms = self.timer.now_ms()?;
        let generation = self.next_generation;
        let next_generation = generation.checked_add(1).ok_or_else(|| {
            ChildSessionHostFailure::new(
                ChildSessionFailureClass::BoundExceeded,
                "child generation overflowed",
            )
        })?;
        let mut controller = self.factory.create(&admission)?;
        self.next_generation = next_generation;
        let handle = self.make_handle(&admission, generation)?;
        controller.start(&admission)?;
        let state = lifecycle_from_controller(controller.status()?);
        let entry = RegistryEntry {
            admission,
            handle: handle.clone(),
            controller,
            started_at_ms,
            state,
            failure_class: failure_for_lifecycle(state),
            result: None,
            report: None,
        };
        self.staged.remove(key.as_str());
        self.registry.insert(handle.handle_id.clone(), entry);
        if let Err(error) = self.persist_and_emit(handle.handle_id.as_str()) {
            if let Some(mut orphan) = self.registry.remove(handle.handle_id.as_str()) {
                let _cancellation_result = orphan.controller.cancel();
            }
            return Err(error);
        }
        Ok(handle)
    }

    fn status_internal(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<SubagentRunStatus, ChildSessionHostFailure> {
        self.validate_handle_and_authority(handle)?;
        let now_ms = self.timer.now_ms()?;
        let handle_id = handle.handle_id.clone();
        let state = {
            let entry = self.entry_mut(handle)?;
            let observed = entry.controller.status()?;
            let mut state = lifecycle_from_controller(observed);
            let elapsed = now_ms.saturating_sub(entry.started_at_ms);
            if matches!(state, ChildSessionLifecycleState::Running)
                && elapsed > entry.admission.limits().wall_time_ms
            {
                let raced = entry.controller.cancel()?;
                state = if raced == ChildSessionControllerStatus::Completed {
                    ChildSessionLifecycleState::AwaitingParent
                } else {
                    ChildSessionLifecycleState::TimedOut
                };
            }
            entry.state = state;
            entry.failure_class = failure_for_lifecycle(state);
            state
        };
        self.persist_and_emit(handle_id.as_str())?;
        Ok(subagent_status(state))
    }

    fn cancel_internal(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<SubagentRunStatus, ChildSessionHostFailure> {
        self.validate_handle_and_authority(handle)?;
        let handle_id = handle.handle_id.clone();
        let state = {
            let entry = self.entry_mut(handle)?;
            let before = entry.controller.status()?;
            let state = if before == ChildSessionControllerStatus::Completed {
                ChildSessionLifecycleState::AwaitingParent
            } else {
                lifecycle_from_controller(entry.controller.cancel()?)
            };
            entry.state = state;
            entry.failure_class = failure_for_lifecycle(state);
            state
        };
        self.persist_and_emit(handle_id.as_str())?;
        Ok(subagent_status(state))
    }

    fn collect_internal(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<(ConcreteChildSessionResult, SubagentReport), ChildSessionHostFailure> {
        self.validate_handle_and_authority(handle)?;
        if let Some(cached) = self.cached_collection(handle)? {
            return Ok(cached);
        }
        let handle_id = handle.handle_id.clone();
        let (result, report) = {
            let entry = self.entry_mut(handle)?;
            let candidate = entry.controller.collect()?;
            project_parent_result(entry, candidate)?
        };
        {
            let entry = self.entry_mut(handle)?;
            entry.state = lifecycle_from_result(result.class);
            entry.failure_class = failure_for_result(result.class);
            entry.result = Some(result.clone());
            entry.report = Some(report.clone());
        }
        self.persist_and_emit(handle_id.as_str())?;
        Ok((result, report))
    }

    fn cached_collection(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<Option<(ConcreteChildSessionResult, SubagentReport)>, ChildSessionHostFailure> {
        let entry = self.entry_mut(handle)?;
        Ok(entry.result.clone().zip(entry.report.clone()))
    }

    fn validate_factory_services(
        &self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure> {
        if let Some(missing) = self.factory.services().first_missing_for(admission) {
            Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::MissingService,
                format!("required child-session service is absent: {missing:?}"),
            ))
        } else {
            Ok(())
        }
    }

    fn validate_current_authority(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure> {
        let observation = self
            .authority
            .observe(admission.subagent().parent_id.as_str())?;
        if observation.parent_id != admission.subagent().parent_id
            || observation.authority_identity != admission.parent_authority_identity()
            || !observation.admitted
        {
            Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::AuthorityDrift,
                "current parent authority does not match child admission",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_handle_and_authority(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<(), ChildSessionHostFailure> {
        let (parent_id, expected_authority) = {
            let entry = self.entry(handle)?;
            validate_handle_binding(handle, entry)?;
            (
                entry.admission.subagent().parent_id.clone(),
                entry.admission.parent_authority_identity().to_owned(),
            )
        };
        let observation = self.authority.observe(parent_id.as_str())?;
        if observation.parent_id != parent_id
            || observation.authority_identity != expected_authority
            || !observation.admitted
        {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::AuthorityDrift,
                "current parent authority differs from the generation-bound handle",
            ));
        }
        Ok(())
    }

    fn make_handle(
        &self,
        admission: &ConcreteChildSessionAdmission,
        generation: u64,
    ) -> Result<SubagentHandle, ChildSessionHostFailure> {
        let generation_text = generation.to_string();
        let handle_id = child_session_hash_parts(
            HANDLE_HASH_DOMAIN,
            &[
                self.service_id.as_str(),
                admission.subagent().parent_id.as_str(),
                admission.subagent().child_id.as_str(),
                admission.admission_identity(),
                generation_text.as_str(),
            ],
        );
        SubagentHandle::generation_bound(
            handle_id,
            SubagentHandleBinding {
                parent_id: admission.subagent().parent_id.as_str(),
                child_id: admission.subagent().child_id.as_str(),
                generation,
                authority_identity: admission.parent_authority_identity(),
            },
        )
        .map_err(|_diagnostic| {
            ChildSessionHostFailure::new(
                ChildSessionFailureClass::Malformed,
                "generation-bound child handle is malformed",
            )
        })
    }

    fn entry(&self, handle: &SubagentHandle) -> Result<&RegistryEntry, ChildSessionHostFailure> {
        self.registry.get(handle.handle_id.as_str()).ok_or_else(|| {
            ChildSessionHostFailure::new(
                ChildSessionFailureClass::StaleHandle,
                "child handle is stale or unknown",
            )
        })
    }

    fn entry_mut(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<&mut RegistryEntry, ChildSessionHostFailure> {
        self.registry
            .get_mut(handle.handle_id.as_str())
            .ok_or_else(|| {
                ChildSessionHostFailure::new(
                    ChildSessionFailureClass::StaleHandle,
                    "child handle is stale or unknown",
                )
            })
    }

    fn persist_and_emit(&mut self, handle_id: &str) -> Result<(), ChildSessionHostFailure> {
        let (record, event) = {
            let entry = self.registry.get(handle_id).ok_or_else(|| {
                ChildSessionHostFailure::new(
                    ChildSessionFailureClass::StaleHandle,
                    "child registry entry disappeared before persistence",
                )
            })?;
            let record = lifecycle_record(self.factory.service_identity(), entry)?;
            let event = ChildSessionEvent {
                parent_id: record.parent_id.clone(),
                child_id: record.child_id.clone(),
                generation: record.generation,
                state: record.state,
                admission_identity: record.admission_identity.clone(),
                result_identity: record.result_identity.clone(),
                failure_class: record.failure_class,
            };
            (record, event)
        };
        self.store.persist(self.service_id.as_str(), &record)?;
        self.events.observe(&event)
    }
}

impl<Factory, Store, Authority, Timer, Events> SubagentService
    for ConcreteChildSessionService<Factory, Store, Authority, Timer, Events>
where
    Factory: ChildSessionFactory,
    Store: ChildSessionLifecycleStore,
    Authority: ChildSessionAuthorityService,
    Timer: ChildSessionTimer,
    Events: ChildSessionEventSink,
{
    fn enqueue(
        &mut self,
        admission: &animus_runtime::subagents::SubagentAdmission,
    ) -> Result<SubagentHandle, SubagentServiceFailure> {
        self.enqueue_internal(admission).map_err(service_failure)
    }

    fn status(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<SubagentRunStatus, SubagentServiceFailure> {
        self.status_internal(handle).map_err(service_failure)
    }

    fn cancel(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<SubagentRunStatus, SubagentServiceFailure> {
        self.cancel_internal(handle).map_err(service_failure)
    }

    fn collect(
        &mut self,
        handle: &SubagentHandle,
    ) -> Result<SubagentReport, SubagentServiceFailure> {
        self.collect_internal(handle)
            .map(|(_result, report)| report)
            .map_err(service_failure)
    }
}

/// Parent-visible RLM projection with explicit terminal class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RlmChildSessionProjection {
    /// Terminal child result class.
    pub class: ChildSessionResultClass,
    /// Bounded RLM child result for completed or partial result text.
    pub result: Option<animus_adapter_rlm_kernel::ChildResult>,
    /// Safe content identity for every terminal class.
    pub content_identity: String,
}

/// Projects one bounded concrete result into the existing RLM child contract.
///
/// # Errors
///
/// Returns a typed failure when lineage identities or the admitted body do not
/// satisfy the RLM request contract.
pub fn project_rlm_child_session_result(
    result: &ConcreteChildSessionResult,
    request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
    profile: &str,
) -> Result<RlmChildSessionProjection, ChildSessionHostFailure> {
    let projected = result
        .body()
        .map(|body| {
            let parent_hash = child_session_hash_parts(
                "animus.concrete-child-session.parent.v1",
                &[result.parent_id.as_str()],
            );
            let child_hash = child_session_hash_parts(
                "animus.concrete-child-session.child.v1",
                &[result.child_id.as_str()],
            );
            let lineage = animus_adapter_rlm_kernel::Lineage::new(
                parent_hash.as_str(),
                child_hash.as_str(),
                profile,
            )
            .map_err(|_error| {
                ChildSessionHostFailure::new(
                    ChildSessionFailureClass::Malformed,
                    "RLM child lineage projection failed",
                )
            })?;
            animus_adapter_rlm_kernel::ChildResult::new(request, lineage, body).map_err(|_error| {
                ChildSessionHostFailure::new(
                    ChildSessionFailureClass::BoundExceeded,
                    "RLM child result projection rejected the bounded body",
                )
            })
        })
        .transpose()?;
    Ok(RlmChildSessionProjection {
        class: result.class,
        result: projected,
        content_identity: result.content_identity.clone(),
    })
}

#[allow(clippy::too_many_lines)]
fn project_parent_result(
    entry: &RegistryEntry,
    candidate: ChildSessionControllerResult,
) -> Result<(ConcreteChildSessionResult, SubagentReport), ChildSessionHostFailure> {
    if candidate.evidence_refs.len() > MAX_EVIDENCE_REFS.saturating_sub(LINEAGE_EVIDENCE_REF_COUNT)
    {
        return Err(ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            "child result contains too many evidence references",
        ));
    }
    let mut class = candidate.class;
    let mut body = candidate.body().map(str::to_owned);
    if class == ChildSessionResultClass::Completed && body.is_none() {
        return Err(ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            "completed child result has no model-visible body",
        ));
    }
    if let Some(value) = body.as_deref() {
        let char_count = u32::try_from(value.chars().count()).unwrap_or(u32::MAX);
        if char_count > entry.admission.limits().result_chars {
            class = ChildSessionResultClass::BoundExceeded;
            body = None;
        } else if PRIVATE_RESULT_MARKERS
            .iter()
            .any(|marker| value.contains(marker))
        {
            class = ChildSessionResultClass::Failed;
            body = None;
        }
    }
    let output_hash = child_session_hash_parts(
        OUTPUT_HASH_DOMAIN,
        &[
            entry.admission.subagent().child_id.as_str(),
            result_class_label(class),
            candidate.body().unwrap_or(""),
        ],
    );
    let report_status = report_status(class);
    let lifecycle = SubagentLifecycleEvent {
        child_id: entry.admission.subagent().child_id.clone(),
        parent_id: entry.admission.subagent().parent_id.clone(),
        kind: entry.admission.subagent().kind,
        status: report_status,
        failure_class: candidate.failure_class.map(subagent_failure_kind),
        admission_identity: entry.admission.subagent().admission_identity.clone(),
        capability_identity: entry.admission.subagent().capability_identity.clone(),
        context_packet_identity: entry.admission.subagent().context_packet_identity.clone(),
        budget: entry.admission.subagent().budget,
        evidence_refs: candidate.evidence_refs.clone(),
    };
    let lineage = validate_subagent_lineage_record(&SubagentLineageRecordDraft {
        parent_id: lifecycle.parent_id.clone(),
        child_id: lifecycle.child_id.clone(),
        kind: lifecycle.kind,
        delegation_source: String::from(LINEAGE_DELEGATION_SOURCE),
        admission_identity: lifecycle.admission_identity.clone(),
        capability_identity: lifecycle.capability_identity.clone(),
        context_packet_identity: lifecycle.context_packet_identity.clone(),
        output_hash: output_hash.clone(),
        handoff_mode: entry.admission.handoff_mode(),
        terminal_status: report_status,
        failure_class: candidate
            .failure_class
            .map(|failure| format!("{failure:?}")),
        evidence_refs: candidate.evidence_refs.clone(),
        safe_summary: String::from(LINEAGE_SAFE_SUMMARY),
    })
    .map_err(|_diagnostic| {
        ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            "child lineage projection rejected safe lifecycle facts",
        )
    })?;
    let result = ConcreteChildSessionResult::new(
        &entry.admission,
        ConcreteChildSessionResultDraft {
            generation: entry.handle.generation().unwrap_or_default(),
            class,
            lineage: lineage.clone(),
            body,
        },
    )
    .map_err(|_error| {
        ChildSessionHostFailure::new(
            ChildSessionFailureClass::BoundExceeded,
            "bounded parent result projection failed",
        )
    })?;
    let mut evidence_refs = candidate.evidence_refs;
    evidence_refs.push(lineage.record_hash);
    let report = SubagentReport {
        child_id: entry.admission.subagent().child_id.clone(),
        status: report_status,
        output_hash: Some(output_hash),
        evidence_refs,
        mutation_intents: Vec::new(),
    };
    let state =
        animus_runtime::subagents::SubagentRunState::admitted(entry.admission.subagent().clone());
    let queued = transition_subagent_state(
        &state,
        SubagentCoordinatorEvent::Enqueued(entry.handle.clone()),
    )
    .map_err(|_diagnostic| {
        ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            "child report handle transition failed",
        )
    })?;
    transition_subagent_state(&queued, SubagentCoordinatorEvent::Collect(report.clone())).map_err(
        |_diagnostic| {
            ChildSessionHostFailure::new(
                ChildSessionFailureClass::Malformed,
                "child report failed existing subagent validation",
            )
        },
    )?;
    Ok((result, report))
}

fn lifecycle_record(
    factory_service_identity: &str,
    entry: &RegistryEntry,
) -> Result<ChildSessionLifecycleRecord, ChildSessionHostFailure> {
    validate_child_session_lifecycle_record(ChildSessionLifecycleRecord {
        parent_id: entry.admission.subagent().parent_id.clone(),
        child_id: entry.admission.subagent().child_id.clone(),
        generation: entry.handle.generation().unwrap_or_default(),
        admission_identity: entry.admission.admission_identity().to_owned(),
        subagent_admission_identity: entry.admission.subagent().admission_identity.clone(),
        capability_identity: entry.admission.subagent().capability_identity.clone(),
        context_packet_identity: entry.admission.subagent().context_packet_identity.clone(),
        parent_authority_identity: entry.admission.parent_authority_identity().to_owned(),
        profile_id: entry.admission.profile_id().to_owned(),
        factory_service_identity: factory_service_identity.to_owned(),
        started_at_ms: entry.started_at_ms,
        state: entry.state,
        session_head: entry.controller.session_head(),
        result_identity: entry
            .result
            .as_ref()
            .map(|result| result.content_identity.clone()),
        failure_class: entry.failure_class,
        record_identity: String::new(),
    })
    .map_err(|_error| {
        ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            "safe child lifecycle record failed validation",
        )
    })
}

fn validate_handle_binding(
    handle: &SubagentHandle,
    entry: &RegistryEntry,
) -> Result<(), ChildSessionHostFailure> {
    let expected_generation = entry.handle.generation();
    if handle.parent_id() != Some(entry.admission.subagent().parent_id.as_str())
        || handle.child_id() != Some(entry.admission.subagent().child_id.as_str())
        || handle.generation() != expected_generation
        || handle.authority_identity() != Some(entry.admission.parent_authority_identity())
    {
        return Err(ChildSessionHostFailure::new(
            if handle.parent_id() == Some(entry.admission.subagent().parent_id.as_str()) {
                ChildSessionFailureClass::StaleHandle
            } else {
                ChildSessionFailureClass::ForeignParent
            },
            "child handle binding does not match the registry entry",
        ));
    }
    Ok(())
}

fn lifecycle_from_controller(status: ChildSessionControllerStatus) -> ChildSessionLifecycleState {
    match status {
        ChildSessionControllerStatus::Running => ChildSessionLifecycleState::Running,
        ChildSessionControllerStatus::Completed => ChildSessionLifecycleState::AwaitingParent,
        ChildSessionControllerStatus::Cancelling => ChildSessionLifecycleState::Cancelling,
        ChildSessionControllerStatus::Failed => ChildSessionLifecycleState::Failed,
        ChildSessionControllerStatus::TimedOut => ChildSessionLifecycleState::TimedOut,
        ChildSessionControllerStatus::Cancelled => ChildSessionLifecycleState::Cancelled,
    }
}

fn lifecycle_from_result(class: ChildSessionResultClass) -> ChildSessionLifecycleState {
    match class {
        ChildSessionResultClass::Completed => ChildSessionLifecycleState::Completed,
        ChildSessionResultClass::Partial => ChildSessionLifecycleState::Partial,
        ChildSessionResultClass::Failed => ChildSessionLifecycleState::Failed,
        ChildSessionResultClass::TimedOut => ChildSessionLifecycleState::TimedOut,
        ChildSessionResultClass::Cancelled => ChildSessionLifecycleState::Cancelled,
        ChildSessionResultClass::BoundExceeded => ChildSessionLifecycleState::BoundExceeded,
    }
}

fn failure_for_lifecycle(state: ChildSessionLifecycleState) -> Option<ChildSessionFailureClass> {
    match state {
        ChildSessionLifecycleState::Failed | ChildSessionLifecycleState::Blocked => {
            Some(ChildSessionFailureClass::Session)
        }
        ChildSessionLifecycleState::TimedOut => Some(ChildSessionFailureClass::TimedOut),
        ChildSessionLifecycleState::Cancelled => Some(ChildSessionFailureClass::Cancelled),
        ChildSessionLifecycleState::BoundExceeded => Some(ChildSessionFailureClass::BoundExceeded),
        ChildSessionLifecycleState::Queued
        | ChildSessionLifecycleState::Running
        | ChildSessionLifecycleState::AwaitingParent
        | ChildSessionLifecycleState::Cancelling
        | ChildSessionLifecycleState::Completed
        | ChildSessionLifecycleState::Partial => None,
    }
}

fn failure_for_result(class: ChildSessionResultClass) -> Option<ChildSessionFailureClass> {
    match class {
        ChildSessionResultClass::Completed => None,
        ChildSessionResultClass::Partial | ChildSessionResultClass::Failed => {
            Some(ChildSessionFailureClass::Session)
        }
        ChildSessionResultClass::TimedOut => Some(ChildSessionFailureClass::TimedOut),
        ChildSessionResultClass::Cancelled => Some(ChildSessionFailureClass::Cancelled),
        ChildSessionResultClass::BoundExceeded => Some(ChildSessionFailureClass::BoundExceeded),
    }
}

fn subagent_status(state: ChildSessionLifecycleState) -> SubagentRunStatus {
    match state {
        ChildSessionLifecycleState::Queued => SubagentRunStatus::Queued,
        ChildSessionLifecycleState::Running => SubagentRunStatus::Running,
        ChildSessionLifecycleState::AwaitingParent => SubagentRunStatus::AwaitingParent,
        ChildSessionLifecycleState::Cancelling => SubagentRunStatus::Cancelling,
        ChildSessionLifecycleState::Completed => SubagentRunStatus::Collected,
        ChildSessionLifecycleState::Partial
        | ChildSessionLifecycleState::Failed
        | ChildSessionLifecycleState::BoundExceeded => SubagentRunStatus::Failed,
        ChildSessionLifecycleState::TimedOut => SubagentRunStatus::TimedOut,
        ChildSessionLifecycleState::Cancelled => SubagentRunStatus::Cancelled,
        ChildSessionLifecycleState::Blocked => SubagentRunStatus::Blocked,
    }
}

fn report_status(class: ChildSessionResultClass) -> SubagentRunStatus {
    match class {
        ChildSessionResultClass::Completed => SubagentRunStatus::Completed,
        ChildSessionResultClass::Partial
        | ChildSessionResultClass::Failed
        | ChildSessionResultClass::BoundExceeded => SubagentRunStatus::Failed,
        ChildSessionResultClass::TimedOut => SubagentRunStatus::TimedOut,
        ChildSessionResultClass::Cancelled => SubagentRunStatus::Cancelled,
    }
}

fn subagent_failure_kind(class: ChildSessionFailureClass) -> SubagentServiceFailureKind {
    match class {
        ChildSessionFailureClass::TimedOut => SubagentServiceFailureKind::TimedOut,
        ChildSessionFailureClass::Cancelled => SubagentServiceFailureKind::Cancelled,
        ChildSessionFailureClass::StaleHandle
        | ChildSessionFailureClass::ForeignParent
        | ChildSessionFailureClass::AuthorityDrift
        | ChildSessionFailureClass::RestoreDrift => SubagentServiceFailureKind::StaleHandle,
        ChildSessionFailureClass::Malformed | ChildSessionFailureClass::BoundExceeded => {
            SubagentServiceFailureKind::MalformedReport
        }
        ChildSessionFailureClass::Denied | ChildSessionFailureClass::DuplicateChild => {
            SubagentServiceFailureKind::Denied
        }
        ChildSessionFailureClass::MissingService
        | ChildSessionFailureClass::Factory
        | ChildSessionFailureClass::Session
        | ChildSessionFailureClass::Persistence
        | ChildSessionFailureClass::Timer
        | ChildSessionFailureClass::Cancellation => SubagentServiceFailureKind::Unavailable,
    }
}

fn service_failure(failure: ChildSessionHostFailure) -> SubagentServiceFailure {
    let kind = subagent_failure_kind(failure.class);
    let diagnostic_kind = match kind {
        SubagentServiceFailureKind::Unavailable => SubagentDiagnosticKind::ServiceUnavailable,
        SubagentServiceFailureKind::Denied => SubagentDiagnosticKind::ServiceDenied,
        SubagentServiceFailureKind::TimedOut => SubagentDiagnosticKind::TimedOut,
        SubagentServiceFailureKind::Cancelled => SubagentDiagnosticKind::Cancelled,
        SubagentServiceFailureKind::MalformedReport => SubagentDiagnosticKind::MalformedReport,
        SubagentServiceFailureKind::StaleHandle => SubagentDiagnosticKind::StaleHandle,
    };
    SubagentServiceFailure {
        kind,
        diagnostic: SubagentDiagnostic::new(
            diagnostic_kind,
            "child_session_service",
            failure.diagnostic,
        ),
    }
}

fn normal_status(
    state: &animus_runtime::coding_session_host::CodingSessionState,
) -> ChildSessionControllerStatus {
    match state.terminal() {
        Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Completed) => {
            ChildSessionControllerStatus::Completed
        }
        Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Cancelled) => {
            ChildSessionControllerStatus::Cancelled
        }
        Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Blocked) => {
            ChildSessionControllerStatus::Failed
        }
        None => match state
            .run()
            .map(animus_runtime::coding_session_host::CodingSessionRun::status)
        {
            Some(animus_runtime::coding_session_host::CodingSessionRunStatus::Cancelling) => {
                ChildSessionControllerStatus::Cancelling
            }
            Some(animus_runtime::coding_session_host::CodingSessionRunStatus::Active) | None => {
                ChildSessionControllerStatus::Running
            }
        },
    }
}

fn map_coding_session_error(error: &CodingSessionHostError) -> ChildSessionHostFailure {
    let class = match error.issue() {
        CodingSessionHostIssue::PolicyDenied => ChildSessionFailureClass::Denied,
        CodingSessionHostIssue::ReducerRejected | CodingSessionHostIssue::ReceiptRejected => {
            ChildSessionFailureClass::Malformed
        }
        CodingSessionHostIssue::ServiceFailed | CodingSessionHostIssue::DurableStoreRejected => {
            ChildSessionFailureClass::Session
        }
        CodingSessionHostIssue::EffectStepBoundExceeded => ChildSessionFailureClass::BoundExceeded,
        CodingSessionHostIssue::SessionBlocked => ChildSessionFailureClass::Session,
    };
    ChildSessionHostFailure::new(
        class,
        format!("coding-session host issue: {:?}", error.issue()),
    )
}

fn admission_key(parent_id: &str, child_id: &str) -> String {
    format!("{parent_id}\u{1f}{child_id}")
}

fn validate_blake3(value: &str, field: &str) -> Result<(), ChildSessionHostFailure> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(ChildSessionHostFailure::new(
            ChildSessionFailureClass::Malformed,
            format!("{field} is not a lowercase BLAKE3 identity"),
        ))
    }
}

const fn result_class_label(class: ChildSessionResultClass) -> &'static str {
    match class {
        ChildSessionResultClass::Completed => "completed",
        ChildSessionResultClass::Partial => "partial",
        ChildSessionResultClass::Failed => "failed",
        ChildSessionResultClass::TimedOut => "timed_out",
        ChildSessionResultClass::Cancelled => "cancelled",
        ChildSessionResultClass::BoundExceeded => "bound_exceeded",
    }
}

#[cfg(test)]
#[path = "childsession/tests.rs"]
mod tests;
