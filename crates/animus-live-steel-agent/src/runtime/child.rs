//! Explicit concrete child-session service composition.

use std::collections::BTreeMap;

use animus_runtime::child_session_service::{
    ChildSessionFailureClass, ChildSessionLifecycleRecord, ChildSessionServiceSet,
    ConcreteChildSessionAdmission, ConcreteChildSessionAdmissionDraft,
};
use animus_runtime::subagents::{
    FreshContextSubagentDraft, SubagentChildRunDraft, SubagentChildRunKind,
    SubagentContextAdmissionPolicy, SubagentContextItemDraft, SubagentContextItemKind,
    SubagentContextPacketDraft, SubagentContextRedaction, SubagentHandoffMode,
    SubagentWorkspaceAccessMode, TypedSubagentCapability,
};
use animus_std::child_session_host::{
    ChildCodingSessionController, ChildSessionAuthorityObservation, ChildSessionAuthorityService,
    ChildSessionEvent, ChildSessionEventSink, ChildSessionFactory, ChildSessionHostFailure,
    ChildSessionLifecycleStore, ChildSessionTimer, ConcreteChildSessionService,
    ConcreteChildSessionServices, NormalCodingSessionController,
};

const CHILD_PARENT_ID: &str = "parent.live-steel-agent";
const CHILD_WORKSPACE_SCOPE: &str = "workspace.explicit-none";
const CHILD_TOOL_LABEL: &str = "steel_repl";
const CHILD_CONTEXT_LABEL: &str = "selected-context";
const CHILD_PACKET_PREFIX: &str = "packet.live-steel-agent.";
const CHILD_ID_PREFIX: &str = "child.live-steel-agent.";
const CHILD_SCHEDULER_LABEL: &str = "scheduler.live-steel-agent";
const CHILD_CONTEXT_TOKEN_ESTIMATE: u32 = 1;
const CHILD_BRANCH_ID: &str = "branch.live-steel-agent-child";
const CHILD_QUEUE_POLICY_LABEL: &str = "live-steel-agent-child-queue";
const CHILD_RUN_PREFIX: &str = "run.live-steel-agent-child.";
const CHILD_ENTRY_PREFIX: &str = "entry.live-steel-agent-child.user.";
const MAX_CHILD_QUEUE_ITEMS: u32 = 1;
const CHILD_SUMMARY: &str = "bounded child summary";

/// Explicit child-session factory with no provider or credential discovery.
#[derive(Clone)]
pub struct ExplicitChildFactory {
    service_identity: String,
    response: String,
    deny: bool,
}

impl ExplicitChildFactory {
    /// Builds one admitted deterministic child factory.
    #[must_use]
    pub fn admitted(service_identity: impl Into<String>, response: impl Into<String>) -> Self {
        Self {
            service_identity: service_identity.into(),
            response: response.into(),
            deny: false,
        }
    }

    /// Builds one explicit denied child factory for negative checks.
    #[must_use]
    pub fn denied(service_identity: impl Into<String>) -> Self {
        Self {
            service_identity: service_identity.into(),
            response: String::new(),
            deny: true,
        }
    }
}

impl ChildSessionFactory for ExplicitChildFactory {
    fn services(&self) -> ChildSessionServiceSet {
        ChildSessionServiceSet::all_available()
    }

    fn service_identity(&self) -> &str {
        self.service_identity.as_str()
    }

    fn create(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure> {
        if self.deny {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Denied,
                "explicit child factory denied creation",
            ));
        }
        build_normal_child_controller(admission, self.response.clone(), false)
    }

    fn restore(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
        _record: &ChildSessionLifecycleRecord,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure> {
        if self.deny {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Denied,
                "explicit child factory denied restore",
            ));
        }
        build_normal_child_controller(admission, self.response.clone(), true)
    }
}

fn build_normal_child_controller(
    admission: &ConcreteChildSessionAdmission,
    response: String,
    restore: bool,
) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure> {
    use animus_runtime::coding_session_host::{
        CodingSessionConfig, CodingSessionQueueProfile, CodingSessionServiceSet,
        new_coding_session_state,
    };
    use animus_runtime::session_authority_profile::{
        SessionAuthorityComponentClass, SessionAuthorityComponentDraft,
        SessionAuthorityProfileDraft, build_session_authority_profile,
    };
    use animus_std::coding_session_host as coding_host;

    let components = SessionAuthorityComponentClass::ALL
        .iter()
        .map(|class| SessionAuthorityComponentDraft {
            class: *class,
            identity: admission.parent_authority_identity(),
        })
        .collect::<Vec<_>>();
    let authority_profile = build_session_authority_profile(SessionAuthorityProfileDraft {
        components: components.as_slice(),
    })
    .map_err(|_error| child_session_failure("child authority profile is malformed"))?;
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: MAX_CHILD_QUEUE_ITEMS,
            allowed_sources: &["coding-session-host"],
            policy_label: CHILD_QUEUE_POLICY_LABEL,
            policy_hash: admission.admission_identity(),
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .map_err(|_error| child_session_failure("child queue policy is malformed"))?;
    let state = new_coding_session_state(
        CodingSessionConfig {
            session_id: admission.subagent().child_id.clone(),
            branch_id: String::from(CHILD_BRANCH_ID),
            queue_profile: CodingSessionQueueProfile::Sequential,
            services: CodingSessionServiceSet::all_available(),
            authority_profile,
            authority_authorization_identity: admission.parent_authority_identity().to_owned(),
            resume_required: restore,
        },
        queue_policy,
    )
    .map_err(|_error| child_session_failure("child coding-session state is malformed"))?;
    let host = coding_host::CodingSessionHost::new(
        state,
        coding_host::CodingSessionHostServices {
            provider: coding_host::FakeCodingSessionProvider {
                response,
                dispatch_count: 0,
            },
            tool: coding_host::DenyUnusedCodingSessionTool,
            persistence: coding_host::MemoryCodingSessionPersistence::default(),
            policy: coding_host::AllowCodingSessionPolicy,
            authority: coding_host::AdmittedCodingSessionAuthority::default(),
            cancellation: coding_host::ObservedCodingSessionCancellation,
            timer: coding_host::AvailableCodingSessionTimer,
            summarizer: coding_host::FakeCodingSessionSummarizer {
                summary: String::from(CHILD_SUMMARY),
            },
            events: coding_host::NoopCodingSessionEventSink,
        },
    )
    .map_err(|_error| child_session_failure("child coding-session host is unavailable"))?;
    let run_id = format!("{CHILD_RUN_PREFIX}{}", admission.admission_identity());
    let entry_id = format!("{CHILD_ENTRY_PREFIX}{}", admission.admission_identity());
    let controller = if restore {
        NormalCodingSessionController::restore(host, run_id, entry_id)
            .map_err(|_error| child_session_failure("child coding-session restore failed"))?
    } else {
        NormalCodingSessionController::new(host, run_id, entry_id)
    };
    Ok(Box::new(controller))
}

fn child_session_failure(message: &'static str) -> ChildSessionHostFailure {
    ChildSessionHostFailure::new(ChildSessionFailureClass::Session, message)
}

/// Metadata-only in-memory lifecycle records owned by one live run.
#[derive(Clone, Default)]
pub struct MetadataChildStore {
    records: BTreeMap<String, ChildSessionLifecycleRecord>,
}

impl ChildSessionLifecycleStore for MetadataChildStore {
    fn persist(
        &mut self,
        _service_id: &str,
        record: &ChildSessionLifecycleRecord,
    ) -> Result<(), ChildSessionHostFailure> {
        self.records.insert(record.child_id.clone(), record.clone());
        Ok(())
    }

    fn load(
        &mut self,
        _service_id: &str,
    ) -> Result<Vec<ChildSessionLifecycleRecord>, ChildSessionHostFailure> {
        Ok(self.records.values().cloned().collect())
    }
}

/// Fresh parent-authority observation for every child operation.
#[derive(Clone)]
pub struct ExplicitChildAuthority {
    parent_id: String,
    authority_identity: String,
    admitted: bool,
}

impl ExplicitChildAuthority {
    /// Builds current admitted authority.
    #[must_use]
    pub fn admitted(authority_identity: impl Into<String>) -> Self {
        Self {
            parent_id: String::from(CHILD_PARENT_ID),
            authority_identity: authority_identity.into(),
            admitted: true,
        }
    }

    /// Builds current denied authority.
    #[must_use]
    pub fn denied(authority_identity: impl Into<String>) -> Self {
        Self {
            parent_id: String::from(CHILD_PARENT_ID),
            authority_identity: authority_identity.into(),
            admitted: false,
        }
    }
}

impl ChildSessionAuthorityService for ExplicitChildAuthority {
    fn observe(
        &mut self,
        parent_id: &str,
    ) -> Result<ChildSessionAuthorityObservation, ChildSessionHostFailure> {
        if parent_id != self.parent_id {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Denied,
                "child parent identity drifted",
            ));
        }
        Ok(ChildSessionAuthorityObservation {
            parent_id: self.parent_id.clone(),
            authority_identity: self.authority_identity.clone(),
            admitted: self.admitted,
        })
    }
}

/// Explicit monotonic child timer.
#[derive(Clone, Copy, Default)]
pub struct FixedChildTimer {
    now_ms: u64,
}

impl ChildSessionTimer for FixedChildTimer {
    fn now_ms(&mut self) -> Result<u64, ChildSessionHostFailure> {
        Ok(self.now_ms)
    }
}

/// Metadata-only child event sink.
#[derive(Clone, Default)]
pub struct MetadataChildEvents {
    events: Vec<ChildSessionEvent>,
}

impl MetadataChildEvents {
    /// Returns the safe event count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether no event was observed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl ChildSessionEventSink for MetadataChildEvents {
    fn observe(&mut self, event: &ChildSessionEvent) -> Result<(), ChildSessionHostFailure> {
        self.events.push(event.clone());
        Ok(())
    }
}

/// Concrete child service type selected by the live application.
pub type LiveConcreteChildService = ConcreteChildSessionService<
    ExplicitChildFactory,
    MetadataChildStore,
    ExplicitChildAuthority,
    FixedChildTimer,
    MetadataChildEvents,
>;

/// Builds the concrete child service selected at the visible root.
///
/// # Errors
///
/// Returns a safe host failure when the service identity is malformed.
pub fn build_child_service(
    config: &crate::ValidatedLiveSteelAgentConfig,
    response: impl Into<String>,
) -> Result<LiveConcreteChildService, ChildSessionHostFailure> {
    ConcreteChildSessionService::new(
        config.child_service_label(),
        ConcreteChildSessionServices {
            factory: ExplicitChildFactory::admitted(
                config.authority_component_identity(),
                response,
            ),
            store: MetadataChildStore::default(),
            authority: ExplicitChildAuthority::admitted(config.authority_authorization_identity()),
            timer: FixedChildTimer::default(),
            events: MetadataChildEvents::default(),
        },
    )
}

/// Builds one exact child admission from the current request and config.
///
/// # Errors
///
/// Returns [`DispatchFailure`] for profile, limit, or authority drift.
#[allow(
    clippy::too_many_lines,
    reason = "keeps one explicit closed child admission projection"
)]
pub fn admit_child_request(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
) -> Result<ConcreteChildSessionAdmission, animus_std_rlm_host::DispatchFailure> {
    use animus_runtime::subagents::{admit_subagent_child_run, validate_subagent_child_run};
    use animus_std_rlm_host::{DispatchFailure, DispatchFailureClass, RouteClass};

    if !config
        .admitted_child_profiles()
        .iter()
        .any(|profile| profile == request.profile())
    {
        return Err(DispatchFailure::new(
            RouteClass::Subagent,
            DispatchFailureClass::Denied,
            "child_profile",
        ));
    }
    let request_identity = blake3::hash(request.context_slice().as_bytes())
        .to_hex()
        .to_string();
    let child_id = format!("{CHILD_ID_PREFIX}{request_identity}");
    let packet_id = format!("{CHILD_PACKET_PREFIX}{request_identity}");
    let capability = TypedSubagentCapability::WorkspaceAccess {
        stable_label: String::from("workspace-read"),
        scope_label: String::from(CHILD_WORKSPACE_SCOPE),
        mode: SubagentWorkspaceAccessMode::Read,
    };
    let child = validate_subagent_child_run(&SubagentChildRunDraft {
        child_id,
        parent_id: String::from(CHILD_PARENT_ID),
        kind: SubagentChildRunKind::FreshContextInvestigation,
        context_packet: Some(SubagentContextPacketDraft {
            packet_id,
            child_kind: SubagentChildRunKind::FreshContextInvestigation,
            admission: SubagentContextAdmissionPolicy::default(),
            items: vec![SubagentContextItemDraft {
                kind: SubagentContextItemKind::Summary,
                label: String::from(CHILD_CONTEXT_LABEL),
                safe_body: request.context_slice().to_owned(),
                token_estimate: CHILD_CONTEXT_TOKEN_ESTIMATE,
                redaction: SubagentContextRedaction::SafeSummary,
            }],
        }),
        capabilities: vec![capability.clone()],
        recipe: None,
        fresh_context: Some(FreshContextSubagentDraft {
            objective: request.context_slice().to_owned(),
            scheduler_label: String::from(CHILD_SCHEDULER_LABEL),
            read_only: true,
        }),
        worker_team: None,
        execution_requested: false,
    })
    .map_err(|_error| {
        DispatchFailure::new(
            RouteClass::Subagent,
            DispatchFailureClass::Malformed,
            "child_request",
        )
    })?;
    let configured_limits = config.child_session_limits();
    let requested_limits = animus_runtime::child_session_service::ChildSessionLimits {
        depth: request.depth(),
        fan_out: request.fan_out(),
        wall_time_ms: request.wall_time_ms(),
        tokens: request.token_budget(),
        turns: configured_limits.turns,
        tool_calls: configured_limits.tool_calls,
        result_chars: request.result_char_budget(),
    };
    let ceiling = animus_runtime::child_session_service::ChildSessionAuthorityCeiling {
        capabilities: vec![capability.clone()],
        workspace_scopes: vec![String::from(CHILD_WORKSPACE_SCOPE)],
        models: vec![String::from(config.model_id())],
        tools: vec![String::from(CHILD_TOOL_LABEL)],
        limits: configured_limits,
    };
    animus_runtime::child_session_service::admit_concrete_child_session(
        &ConcreteChildSessionAdmissionDraft {
            subagent: admit_subagent_child_run(&child),
            context_packet: child.context_packet,
            objective: request.context_slice().to_owned(),
            profile_id: request.profile().to_owned(),
            parent_authority_identity: config.authority_authorization_identity().to_owned(),
            workspace_scope: String::from(CHILD_WORKSPACE_SCOPE),
            model: String::from(config.model_id()),
            tools: vec![String::from(CHILD_TOOL_LABEL)],
            requested_capabilities: vec![capability.clone()],
            parent: ceiling.clone(),
            profile: ceiling,
            semantic_role_capabilities: vec![capability.clone()],
            workspace_capabilities: vec![capability.clone()],
            host_capabilities: vec![capability],
            requested_limits,
            handoff_mode: SubagentHandoffMode::StatusPlusSummary,
            kernel_profile: None,
            skill_ids: Vec::new(),
        },
    )
    .map_err(|_error| {
        DispatchFailure::new(
            RouteClass::Subagent,
            DispatchFailureClass::Denied,
            "child_admission",
        )
    })
}
