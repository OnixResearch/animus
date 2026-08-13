use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use animus_runtime::child_session_service::{
    ChildSessionAuthorityCeiling, ChildSessionLimits, ChildSessionServiceKind,
    ConcreteChildSessionAdmissionDraft, admit_concrete_child_session,
};
use animus_runtime::coding_session_host::{
    CODING_SESSION_EMPTY_HEAD_HASH, CodingSessionCommitRequest, CodingSessionConfig,
    CodingSessionQueueProfile, CodingSessionRecoveryRequest, CodingSessionRecoverySnapshot,
    CodingSessionRecoveryStatus, CodingSessionServiceIssue, CodingSessionServiceSet,
    CodingSessionState, new_coding_session_state,
};
use animus_runtime::session_authority_profile::{
    SessionAuthorityComponentClass, SessionAuthorityComponentDraft, SessionAuthorityProfileDraft,
    build_session_authority_profile,
};
use animus_runtime::subagents::{
    FreshContextSubagentDraft, SubagentChildRunDraft, SubagentChildRunKind,
    SubagentContextAdmissionPolicy, SubagentContextItemDraft, SubagentContextItemKind,
    SubagentContextPacketDraft, SubagentContextRedaction, SubagentHandleBinding,
    SubagentHandoffMode, SubagentWorkspaceAccessMode, TypedSubagentCapability,
    admit_subagent_child_run, validate_subagent_child_run,
};

use crate::coding_session_host::{
    AdmittedCodingSessionAuthority, AllowCodingSessionPolicy, AvailableCodingSessionTimer,
    CodingSessionCommitObservation, CodingSessionHostServices, DenyUnusedCodingSessionTool,
    FakeCodingSessionProvider, FakeCodingSessionSummarizer, NoopCodingSessionEventSink,
    ObservedCodingSessionCancellation,
};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const SERVICE_ID: &str = "child-service.focused";
const PARENT_ID: &str = "parent.root";
const CHILD_ID: &str = "child.focused";
const PROFILE_ID: &str = "focused-child";
const WORKSPACE: &str = "workspace.readonly";
const MODEL: &str = "model.focused";
const TOOL: &str = "search";
const OBJECTIVE: &str = "Review the selected module and return a bounded result.";
const RESULT: &str = "The selected module keeps authority explicit.";
const PRIVATE_TRANSCRIPT: &str = "RAW_TRANSCRIPT: private child reasoning";
const RUN_ID: &str = "run.child";
const ENTRY_ID: &str = "entry.child-user";
const SESSION_ID: &str = "session.child";
const BRANCH_ID: &str = "branch.child";
const QUEUE_POLICY_HASH: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const MAX_QUEUE_ITEMS: u32 = 8;
const TOKEN_LIMIT: u64 = 8_000;
const WALL_LIMIT_MS: u64 = 60_000;
const RESULT_LIMIT: u32 = 4_096;
const TURN_LIMIT: u32 = 8;
const TOOL_CALL_LIMIT: u32 = 16;
const DEPTH_LIMIT: u32 = 2;
const FAN_OUT_LIMIT: u32 = 4;
const EXPIRED_NOW_MS: u64 = WALL_LIMIT_MS + 1;

#[derive(Clone, Debug, Default)]
struct FactoryStats {
    creates: u32,
    restores: u32,
    starts: u32,
    cancels: u32,
    last_objective: Option<String>,
}

#[derive(Clone)]
struct FakeFactory {
    services: ChildSessionServiceSet,
    identity: String,
    stats: Rc<RefCell<FactoryStats>>,
    status: ChildSessionControllerStatus,
    result: ChildSessionControllerResult,
    head: ChildSessionHead,
}

impl ChildSessionFactory for FakeFactory {
    fn services(&self) -> ChildSessionServiceSet {
        self.services.clone()
    }

    fn service_identity(&self) -> &str {
        self.identity.as_str()
    }

    fn create(
        &mut self,
        _admission: &ConcreteChildSessionAdmission,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure> {
        {
            let mut stats = self.stats.borrow_mut();
            stats.creates = stats.creates.saturating_add(1);
        }
        Ok(Box::new(FakeController {
            stats: Rc::clone(&self.stats),
            status: self.status,
            result: self.result.clone(),
            head: self.head.clone(),
        }))
    }

    fn restore(
        &mut self,
        _admission: &ConcreteChildSessionAdmission,
        record: &ChildSessionLifecycleRecord,
    ) -> Result<Box<dyn ChildCodingSessionController>, ChildSessionHostFailure> {
        {
            let mut stats = self.stats.borrow_mut();
            stats.restores = stats.restores.saturating_add(1);
        }
        Ok(Box::new(FakeController {
            stats: Rc::clone(&self.stats),
            status: self.status,
            result: self.result.clone(),
            head: record.session_head.clone(),
        }))
    }
}

struct FakeController {
    stats: Rc<RefCell<FactoryStats>>,
    status: ChildSessionControllerStatus,
    result: ChildSessionControllerResult,
    head: ChildSessionHead,
}

impl ChildCodingSessionController for FakeController {
    fn start(
        &mut self,
        admission: &ConcreteChildSessionAdmission,
    ) -> Result<(), ChildSessionHostFailure> {
        let mut stats = self.stats.borrow_mut();
        stats.starts = stats.starts.saturating_add(1);
        stats.last_objective = Some(admission.objective().to_owned());
        Ok(())
    }

    fn status(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure> {
        Ok(self.status)
    }

    fn cancel(&mut self) -> Result<ChildSessionControllerStatus, ChildSessionHostFailure> {
        let mut stats = self.stats.borrow_mut();
        stats.cancels = stats.cancels.saturating_add(1);
        self.status = ChildSessionControllerStatus::Cancelled;
        Ok(self.status)
    }

    fn collect(&mut self) -> Result<ChildSessionControllerResult, ChildSessionHostFailure> {
        if matches!(
            self.status,
            ChildSessionControllerStatus::Running | ChildSessionControllerStatus::Cancelling
        ) {
            return Err(ChildSessionHostFailure::new(
                ChildSessionFailureClass::Session,
                "fake controller is not terminal",
            ));
        }
        Ok(self.result.clone())
    }

    fn session_head(&self) -> ChildSessionHead {
        self.head.clone()
    }
}

#[derive(Clone, Default)]
struct SharedStore {
    records: Rc<RefCell<Vec<ChildSessionLifecycleRecord>>>,
}

impl ChildSessionLifecycleStore for SharedStore {
    fn persist(
        &mut self,
        _service_id: &str,
        record: &ChildSessionLifecycleRecord,
    ) -> Result<(), ChildSessionHostFailure> {
        let mut records = self.records.borrow_mut();
        if let Some(existing) = records.iter_mut().find(|candidate| {
            candidate.parent_id == record.parent_id && candidate.child_id == record.child_id
        }) {
            *existing = record.clone();
        } else {
            records.push(record.clone());
        }
        Ok(())
    }

    fn load(
        &mut self,
        _service_id: &str,
    ) -> Result<Vec<ChildSessionLifecycleRecord>, ChildSessionHostFailure> {
        Ok(self.records.borrow().clone())
    }
}

#[derive(Clone)]
struct SharedAuthority {
    observation: Rc<RefCell<ChildSessionAuthorityObservation>>,
}

impl ChildSessionAuthorityService for SharedAuthority {
    fn observe(
        &mut self,
        _parent_id: &str,
    ) -> Result<ChildSessionAuthorityObservation, ChildSessionHostFailure> {
        Ok(self.observation.borrow().clone())
    }
}

#[derive(Clone)]
struct SharedTimer {
    now_ms: Rc<RefCell<u64>>,
}

impl ChildSessionTimer for SharedTimer {
    fn now_ms(&mut self) -> Result<u64, ChildSessionHostFailure> {
        Ok(*self.now_ms.borrow())
    }
}

#[derive(Clone, Default)]
struct SharedEvents {
    events: Rc<RefCell<Vec<ChildSessionEvent>>>,
}

impl ChildSessionEventSink for SharedEvents {
    fn observe(&mut self, event: &ChildSessionEvent) -> Result<(), ChildSessionHostFailure> {
        self.events.borrow_mut().push(event.clone());
        Ok(())
    }
}

#[test]
fn lifecycle_uses_one_explicit_factory_and_projects_bounded_rlm_result() {
    // r[verify agent_harness_runtime.child_session_service.lifecycle.enqueue]
    // r[verify agent_harness_runtime.child_session_service.results]
    // r[verify agent_harness_runtime.child_session_service.verification]
    let admission = admission();
    let harness = Harness::new(
        ChildSessionControllerStatus::Completed,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Completed,
            Some(String::from(RESULT)),
            None,
            Vec::new(),
        ),
    );
    let mut service = harness.service();
    service.stage(admission.clone()).expect("stage");
    let handle = SubagentService::enqueue(&mut service, admission.subagent()).expect("enqueue");
    assert_eq!(handle.parent_id(), Some(PARENT_ID));
    assert_eq!(handle.child_id(), Some(CHILD_ID));
    assert_eq!(handle.generation(), Some(1));
    assert_eq!(service.registry_len(), 1);
    assert_eq!(harness.stats.borrow().creates, 1);
    assert_eq!(harness.stats.borrow().starts, 1);
    assert_eq!(
        harness.stats.borrow().last_objective.as_deref(),
        Some(OBJECTIVE)
    );
    assert_eq!(
        SubagentService::status(&mut service, &handle).expect("status"),
        SubagentRunStatus::AwaitingParent
    );
    let result = service.collect_result(&handle).expect("collect result");
    assert_eq!(result.class, ChildSessionResultClass::Completed);
    assert_eq!(result.body(), Some(RESULT));
    assert_eq!(result.lineage.parent_id, PARENT_ID);
    assert_eq!(result.lineage.child_id, CHILD_ID);
    assert!(!format!("{result:?}").contains(RESULT));
    let report = SubagentService::collect(&mut service, &handle).expect("cached report");
    assert_eq!(report.status, SubagentRunStatus::Completed);
    let request = animus_adapter_rlm_kernel::ChildSpawnRequest::new(
        PROFILE_ID,
        "selected context",
        1,
        1,
        TOKEN_LIMIT,
        WALL_LIMIT_MS,
        RESULT_LIMIT,
        false,
        false,
    )
    .expect("RLM request");
    let projection =
        project_rlm_child_session_result(&result, &request, PROFILE_ID).expect("RLM projection");
    assert_eq!(projection.class, ChildSessionResultClass::Completed);
    assert_eq!(
        projection
            .result
            .as_ref()
            .map(animus_adapter_rlm_kernel::ChildResult::value),
        Some(RESULT)
    );
    let records = harness.store.records.borrow();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].state, ChildSessionLifecycleState::Completed);
    assert!(!format!("{:?}", records[0]).contains(RESULT));
    assert!(!format!("{:?}", harness.events.events.borrow()).contains(RESULT));
}

#[test]
fn stale_foreign_authority_and_duplicate_access_fail_before_child_mutation() {
    // r[verify agent_harness_runtime.child_session_service.lifecycle.stale]
    // r[verify agent_harness_runtime.child_session_service.verification]
    let admission = admission();
    let harness = Harness::new(
        ChildSessionControllerStatus::Running,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Cancelled,
            None,
            Some(ChildSessionFailureClass::Cancelled),
            Vec::new(),
        ),
    );
    let mut service = harness.service();
    service.stage(admission.clone()).expect("stage");
    let handle = SubagentService::enqueue(&mut service, admission.subagent()).expect("enqueue");
    let legacy = SubagentHandle::new(handle.handle_id.as_str()).expect("legacy handle");
    assert_eq!(
        SubagentService::status(&mut service, &legacy)
            .expect_err("legacy handle must fail")
            .kind,
        SubagentServiceFailureKind::StaleHandle
    );
    let foreign = SubagentHandle::generation_bound(
        handle.handle_id.as_str(),
        SubagentHandleBinding {
            parent_id: "parent.foreign",
            child_id: CHILD_ID,
            generation: handle.generation().expect("generation"),
            authority_identity: HASH_A,
        },
    )
    .expect("foreign handle shape");
    assert_eq!(
        SubagentService::cancel(&mut service, &foreign)
            .expect_err("foreign parent must fail")
            .kind,
        SubagentServiceFailureKind::StaleHandle
    );
    assert_eq!(harness.stats.borrow().cancels, 0);
    let duplicate = service
        .stage(admission.clone())
        .expect_err("duplicate child must fail");
    assert_eq!(duplicate.class, ChildSessionFailureClass::DuplicateChild);
    harness
        .authority
        .observation
        .borrow_mut()
        .authority_identity = String::from(HASH_B);
    assert_eq!(
        SubagentService::status(&mut service, &handle)
            .expect_err("authority drift must fail")
            .kind,
        SubagentServiceFailureKind::StaleHandle
    );
    assert_eq!(harness.stats.borrow().cancels, 0);
}

#[test]
fn missing_services_bounds_and_private_transcript_markers_fail_closed() {
    // r[verify agent_harness_runtime.child_session_service.factory.missing]
    // r[verify agent_harness_runtime.child_session_service.results.oversized]
    let admission = admission();
    let mut missing = Harness::new(
        ChildSessionControllerStatus::Completed,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Completed,
            Some(String::from(RESULT)),
            None,
            Vec::new(),
        ),
    );
    missing.factory.services = ChildSessionServiceSet::new([
        ChildSessionServiceKind::Tool,
        ChildSessionServiceKind::Persistence,
    ]);
    let mut missing_service = missing.service();
    let error = missing_service
        .stage(admission.clone())
        .expect_err("missing provider must fail");
    assert_eq!(error.class, ChildSessionFailureClass::MissingService);
    assert_eq!(missing.stats.borrow().creates, 0);

    let oversized_text = "x".repeat(usize::try_from(RESULT_LIMIT + 1).expect("limit fits"));
    let oversized = Harness::new(
        ChildSessionControllerStatus::Completed,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Completed,
            Some(oversized_text.clone()),
            None,
            Vec::new(),
        ),
    );
    let mut oversized_service = oversized.service();
    oversized_service
        .stage(admission.clone())
        .expect("stage oversized child");
    let oversized_handle = SubagentService::enqueue(&mut oversized_service, admission.subagent())
        .expect("enqueue oversized child");
    let bounded = oversized_service
        .collect_result(&oversized_handle)
        .expect("typed bound result");
    assert_eq!(bounded.class, ChildSessionResultClass::BoundExceeded);
    assert_eq!(bounded.body(), None);
    assert!(!format!("{bounded:?}").contains(oversized_text.as_str()));

    let leak = Harness::new(
        ChildSessionControllerStatus::Completed,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Completed,
            Some(String::from(PRIVATE_TRANSCRIPT)),
            None,
            Vec::new(),
        ),
    );
    let mut leak_service = leak.service();
    leak_service
        .stage(admission.clone())
        .expect("stage leak child");
    let leak_handle = SubagentService::enqueue(&mut leak_service, admission.subagent())
        .expect("enqueue leak child");
    let rejected = leak_service
        .collect_result(&leak_handle)
        .expect("typed leak rejection");
    assert_eq!(rejected.class, ChildSessionResultClass::Failed);
    assert_eq!(rejected.body(), None);
    assert!(!format!("{rejected:?}").contains(PRIVATE_TRANSCRIPT));
}

#[test]
fn completion_wins_cancellation_race_and_elapsed_limit_cancels_running_child() {
    // r[verify agent_harness_runtime.child_session_service.results]
    // r[verify agent_harness_runtime.child_session_service.verification]
    let admission = admission();
    let completion = Harness::new(
        ChildSessionControllerStatus::Completed,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Completed,
            Some(String::from(RESULT)),
            None,
            Vec::new(),
        ),
    );
    let mut complete_service = completion.service();
    complete_service.stage(admission.clone()).expect("stage");
    let complete_handle =
        SubagentService::enqueue(&mut complete_service, admission.subagent()).expect("enqueue");
    assert_eq!(
        SubagentService::cancel(&mut complete_service, &complete_handle).expect("cancel race"),
        SubagentRunStatus::AwaitingParent
    );
    assert_eq!(completion.stats.borrow().cancels, 0);

    let timeout = Harness::new(
        ChildSessionControllerStatus::Running,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Cancelled,
            None,
            Some(ChildSessionFailureClass::Cancelled),
            Vec::new(),
        ),
    );
    let mut timeout_service = timeout.service();
    timeout_service
        .stage(admission.clone())
        .expect("stage timeout");
    let timeout_handle =
        SubagentService::enqueue(&mut timeout_service, admission.subagent()).expect("enqueue");
    *timeout.timer.now_ms.borrow_mut() = EXPIRED_NOW_MS;
    assert_eq!(
        SubagentService::status(&mut timeout_service, &timeout_handle).expect("timeout status"),
        SubagentRunStatus::TimedOut
    );
    assert_eq!(timeout.stats.borrow().cancels, 1);
}

#[test]
fn restart_reopens_only_matching_non_terminal_records_without_restarting_prompt() {
    // r[verify agent_harness_runtime.child_session_service.persistence.restore]
    // r[verify agent_harness_runtime.child_session_service.verification]
    let admission = admission();
    let first = Harness::new(
        ChildSessionControllerStatus::Running,
        ChildSessionControllerResult::new(
            ChildSessionResultClass::Cancelled,
            None,
            Some(ChildSessionFailureClass::Cancelled),
            Vec::new(),
        ),
    );
    let mut first_service = first.service();
    first_service.stage(admission.clone()).expect("stage");
    SubagentService::enqueue(&mut first_service, admission.subagent()).expect("enqueue");
    assert_eq!(first.stats.borrow().starts, 1);
    drop(first_service);

    let restored_stats = Rc::new(RefCell::new(FactoryStats::default()));
    let restored_factory = FakeFactory {
        services: ChildSessionServiceSet::all_available(),
        identity: String::from(HASH_C),
        stats: Rc::clone(&restored_stats),
        status: ChildSessionControllerStatus::Running,
        result: ChildSessionControllerResult::new(
            ChildSessionResultClass::Cancelled,
            None,
            Some(ChildSessionFailureClass::Cancelled),
            Vec::new(),
        ),
        head: safe_head(),
    };
    let mut restored = ConcreteChildSessionService::new(
        SERVICE_ID,
        ConcreteChildSessionServices {
            factory: restored_factory,
            store: first.store.clone(),
            authority: first.authority.clone(),
            timer: first.timer.clone(),
            events: first.events.clone(),
        },
    )
    .expect("restored service");
    let summary = restored
        .restore(std::slice::from_ref(&admission))
        .expect("restore non-terminal child");
    assert_eq!(summary.handles.len(), 1);
    assert_eq!(summary.skipped_terminal, 0);
    assert_eq!(restored_stats.borrow().restores, 1);
    assert_eq!(restored_stats.borrow().creates, 0);
    assert_eq!(restored_stats.borrow().starts, 0);

    let drift_factory = FakeFactory {
        services: ChildSessionServiceSet::all_available(),
        identity: String::from(HASH_B),
        stats: Rc::new(RefCell::new(FactoryStats::default())),
        status: ChildSessionControllerStatus::Running,
        result: ChildSessionControllerResult::new(
            ChildSessionResultClass::Failed,
            None,
            Some(ChildSessionFailureClass::Session),
            Vec::new(),
        ),
        head: safe_head(),
    };
    let mut drifted = ConcreteChildSessionService::new(
        SERVICE_ID,
        ConcreteChildSessionServices {
            factory: drift_factory,
            store: first.store,
            authority: first.authority,
            timer: first.timer,
            events: first.events,
        },
    )
    .expect("drift service");
    assert_eq!(
        drifted
            .restore(std::slice::from_ref(&admission))
            .expect_err("service drift must fail")
            .class,
        ChildSessionFailureClass::RestoreDrift
    );
}

#[test]
fn normal_controller_runs_the_existing_coding_session_host() {
    // r[verify agent_harness_runtime.child_session_service.lifecycle.enqueue]
    // r[verify agent_harness_runtime.child_session_service.verification.closeout]
    let admission = admission();
    let host = normal_host().expect("normal coding-session host");
    let mut controller = NormalCodingSessionController::new(host, RUN_ID, ENTRY_ID);
    controller.start(&admission).expect("normal child start");
    assert_eq!(
        controller.status().expect("normal status"),
        ChildSessionControllerStatus::Completed
    );
    let result = controller.collect().expect("normal collect");
    assert_eq!(result.class, ChildSessionResultClass::Completed);
    assert_eq!(result.body(), Some(RESULT));
    assert!(!format!("{result:?}").contains(RESULT));
}

struct Harness {
    factory: FakeFactory,
    store: SharedStore,
    authority: SharedAuthority,
    timer: SharedTimer,
    events: SharedEvents,
    stats: Rc<RefCell<FactoryStats>>,
}

impl Harness {
    fn new(status: ChildSessionControllerStatus, result: ChildSessionControllerResult) -> Self {
        let factory_stats = Rc::new(RefCell::new(FactoryStats::default()));
        Self {
            factory: FakeFactory {
                services: ChildSessionServiceSet::all_available(),
                identity: String::from(HASH_C),
                stats: Rc::clone(&factory_stats),
                status,
                result,
                head: safe_head(),
            },
            store: SharedStore::default(),
            authority: SharedAuthority {
                observation: Rc::new(RefCell::new(ChildSessionAuthorityObservation {
                    parent_id: String::from(PARENT_ID),
                    authority_identity: String::from(HASH_A),
                    admitted: true,
                })),
            },
            timer: SharedTimer {
                now_ms: Rc::new(RefCell::new(0)),
            },
            events: SharedEvents::default(),
            stats: factory_stats,
        }
    }

    fn service(
        &self,
    ) -> ConcreteChildSessionService<
        FakeFactory,
        SharedStore,
        SharedAuthority,
        SharedTimer,
        SharedEvents,
    > {
        ConcreteChildSessionService::new(
            SERVICE_ID,
            ConcreteChildSessionServices {
                factory: self.factory.clone(),
                store: self.store.clone(),
                authority: self.authority.clone(),
                timer: self.timer.clone(),
                events: self.events.clone(),
            },
        )
        .expect("test service")
    }
}

fn admission() -> ConcreteChildSessionAdmission {
    let capability = TypedSubagentCapability::WorkspaceAccess {
        stable_label: String::from("workspace-read"),
        scope_label: String::from(WORKSPACE),
        mode: SubagentWorkspaceAccessMode::Read,
    };
    let child = validate_subagent_child_run(&SubagentChildRunDraft {
        child_id: String::from(CHILD_ID),
        parent_id: String::from(PARENT_ID),
        kind: SubagentChildRunKind::FreshContextInvestigation,
        context_packet: Some(SubagentContextPacketDraft {
            packet_id: String::from("packet.focused"),
            child_kind: SubagentChildRunKind::FreshContextInvestigation,
            admission: SubagentContextAdmissionPolicy::default(),
            items: vec![SubagentContextItemDraft {
                kind: SubagentContextItemKind::Summary,
                label: String::from("module-summary"),
                safe_body: String::from("Selected module facts only."),
                token_estimate: 32,
                redaction: SubagentContextRedaction::SafeSummary,
            }],
        }),
        capabilities: vec![capability.clone()],
        recipe: None,
        fresh_context: Some(FreshContextSubagentDraft {
            objective: String::from(OBJECTIVE),
            scheduler_label: String::from("scheduler.explicit"),
            read_only: true,
        }),
        worker_team: None,
        execution_requested: false,
    })
    .expect("subagent child");
    let limits = ChildSessionLimits {
        depth: DEPTH_LIMIT,
        fan_out: FAN_OUT_LIMIT,
        wall_time_ms: WALL_LIMIT_MS,
        tokens: TOKEN_LIMIT,
        turns: TURN_LIMIT,
        tool_calls: TOOL_CALL_LIMIT,
        result_chars: RESULT_LIMIT,
    };
    let ceiling = ChildSessionAuthorityCeiling {
        capabilities: vec![capability.clone()],
        workspace_scopes: vec![String::from(WORKSPACE)],
        models: vec![String::from(MODEL)],
        tools: vec![String::from(TOOL)],
        limits,
    };
    admit_concrete_child_session(&ConcreteChildSessionAdmissionDraft {
        subagent: admit_subagent_child_run(&child),
        context_packet: child.context_packet,
        objective: String::from(OBJECTIVE),
        profile_id: String::from(PROFILE_ID),
        parent_authority_identity: String::from(HASH_A),
        workspace_scope: String::from(WORKSPACE),
        model: String::from(MODEL),
        tools: vec![String::from(TOOL)],
        requested_capabilities: vec![capability.clone()],
        parent: ceiling.clone(),
        profile: ceiling,
        semantic_role_capabilities: vec![capability.clone()],
        workspace_capabilities: vec![capability.clone()],
        host_capabilities: vec![capability],
        requested_limits: limits,
        handoff_mode: SubagentHandoffMode::StatusPlusSummary,
        kernel_profile: None,
        skill_ids: Vec::new(),
    })
    .expect("concrete admission")
}

fn safe_head() -> ChildSessionHead {
    ChildSessionHead {
        committed_head_hash: String::from(HASH_B),
        active_leaf_id: Some(String::from("entry.child-assistant")),
        durable_cursor: 2,
    }
}

#[derive(Clone, Debug)]
struct MemoryPersistence {
    messages: Vec<animus_runtime::coding_session_host::CodingSessionMessage>,
    head_hash: String,
}

impl Default for MemoryPersistence {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            head_hash: String::from(CODING_SESSION_EMPTY_HEAD_HASH),
        }
    }
}

impl CodingSessionPersistenceService for MemoryPersistence {
    fn commit(
        &mut self,
        request: &CodingSessionCommitRequest,
    ) -> Result<CodingSessionCommitObservation, CodingSessionServiceIssue> {
        self.messages.extend(request.messages().iter().cloned());
        let durable_cursor = u64::try_from(self.messages.len())
            .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        self.head_hash = format!("{durable_cursor:064x}");
        Ok(CodingSessionCommitObservation {
            new_head_hash: self.head_hash.clone(),
            active_leaf_id: String::from(request.proposed_active_leaf_id()),
            durable_cursor,
            synchronized: true,
        })
    }

    fn recover(
        &mut self,
        _request: &CodingSessionRecoveryRequest,
    ) -> Result<CodingSessionRecoverySnapshot, CodingSessionServiceIssue> {
        Ok(CodingSessionRecoverySnapshot {
            status: if self.messages.is_empty() {
                CodingSessionRecoveryStatus::Empty
            } else {
                CodingSessionRecoveryStatus::Clean
            },
            committed_head_hash: self.head_hash.clone(),
            active_leaf_id: self
                .messages
                .last()
                .map(|message| message.entry_id().to_owned()),
            durable_cursor: u64::try_from(self.messages.len())
                .map_err(|_error| CodingSessionServiceIssue::Malformed)?,
            messages: self.messages.clone(),
        })
    }
}

type TestNormalCodingSessionHost = CodingSessionHost<
    FakeCodingSessionProvider,
    DenyUnusedCodingSessionTool,
    MemoryPersistence,
    AllowCodingSessionPolicy,
    AdmittedCodingSessionAuthority,
    ObservedCodingSessionCancellation,
    AvailableCodingSessionTimer,
    FakeCodingSessionSummarizer,
    NoopCodingSessionEventSink,
>;

fn normal_host() -> Result<TestNormalCodingSessionHost, CodingSessionHostError> {
    CodingSessionHost::new(
        normal_state(),
        CodingSessionHostServices {
            provider: FakeCodingSessionProvider {
                response: String::from(RESULT),
                dispatch_count: 0,
            },
            tool: DenyUnusedCodingSessionTool,
            persistence: MemoryPersistence::default(),
            policy: AllowCodingSessionPolicy,
            authority: AdmittedCodingSessionAuthority::default(),
            cancellation: ObservedCodingSessionCancellation,
            timer: AvailableCodingSessionTimer,
            summarizer: FakeCodingSessionSummarizer {
                summary: String::from("bounded summary"),
            },
            events: NoopCodingSessionEventSink,
        },
    )
}

fn normal_state() -> CodingSessionState {
    let components = SessionAuthorityComponentClass::ALL
        .iter()
        .map(|class| SessionAuthorityComponentDraft {
            class: *class,
            identity: HASH_A,
        })
        .collect::<Vec<_>>();
    let authority_profile = build_session_authority_profile(SessionAuthorityProfileDraft {
        components: components.as_slice(),
    })
    .expect("authority profile");
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: MAX_QUEUE_ITEMS,
            allowed_sources: &["coding-session-host"],
            policy_label: "child-session-test-queue",
            policy_hash: QUEUE_POLICY_HASH,
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .expect("queue policy");
    new_coding_session_state(
        CodingSessionConfig {
            session_id: String::from(SESSION_ID),
            branch_id: String::from(BRANCH_ID),
            queue_profile: CodingSessionQueueProfile::Sequential,
            services: CodingSessionServiceSet::all_available(),
            authority_profile,
            authority_authorization_identity: String::from(HASH_B),
            resume_required: false,
        },
        queue_policy,
    )
    .expect("coding-session state")
}
