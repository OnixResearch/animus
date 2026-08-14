//! Component-scope host and daemon mapping tests.

use animus_local_daemon::{
    ClientDeliveryScopeLease, ComponentScopeCancellationPort, ComponentScopeDeliveryPort,
    ComponentScopeHost, ComponentScopeHostBounds, ComponentScopePorts,
    ComponentScopeQuiescencePort, ComponentScopeReceiptPort, ComponentScopeReleasePort,
    RestoreStatus, ScopeBoundPortRequest, ScopeMemberPortRequest, ScopePortStatus,
    ScopeReceiptPortRequest, WorkerLifecycle, WorkerRegistryRecord,
    plan_client_delivery_scope_lease, plan_worker_root_scope,
};
use animus_session_core::component_scope::{
    ComponentRegistrationDraft, ComponentScopeDraft, ComponentScopeState, ScopeGeneration,
    ScopeMemberKind, ScopeTerminalOutcome, ScopeVisibility, admit_component_scope,
};
use animus_session_core::composition_activation_graph::{
    ActivationAuthorityClass, ActivationComponent, ActivationGraphDraft, ActivationPort,
    ActivationPosture, PortMultiplicity, ValidatedActivationGraph,
};
use std::cell::RefCell;
use std::rc::Rc;

const GENERATION_VALUE: u64 = 3;
const HOST_EFFECT_BOUND: u16 = 16;
const PROCESS_ID: u32 = 42;
const PROVIDER: &str = "component.provider";
const CONSUMER: &str = "component.consumer";
const SCOPE_ID: &str = "scope.worker";
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const ACTIVATION_NON_CLAIMS: &[&str] = &[
    "not-implementation-correctness",
    "not-runtime-liveness",
    "not-effect-success",
    "not-authority-grant",
    "not-production-readiness",
    "not-release-eligibility",
];
const SCOPE_NON_CLAIMS: &[&str] = &[
    "not-effect-authority",
    "not-task-cancellation",
    "not-process-death",
    "not-resource-reclamation",
    "not-sandboxing",
    "not-production-readiness",
    "not-release-eligibility",
    "not-external-liveness",
];

type Log = Rc<RefCell<Vec<String>>>;

struct DeliveryFake(Log);
struct CancellationFake(Log);
struct QuiescenceFake {
    log: Log,
    unresolved_member: Option<&'static str>,
}
struct ReleaseFake(Log);
struct ReceiptFake(Log);

impl ComponentScopeDeliveryPort for DeliveryFake {
    fn close_delivery(&mut self, request: ScopeBoundPortRequest<'_>) -> ScopePortStatus {
        self.0
            .borrow_mut()
            .push(format!("close:{}", request.scope_id));
        ScopePortStatus::Complete
    }
}

impl ComponentScopeCancellationPort for CancellationFake {
    fn request_cancellation(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus {
        self.0
            .borrow_mut()
            .push(format!("cancel:{}", request.member_id));
        ScopePortStatus::Complete
    }
}

impl ComponentScopeQuiescencePort for QuiescenceFake {
    fn observe_quiescence(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus {
        self.log
            .borrow_mut()
            .push(format!("quiesce:{}", request.member_id));
        if self.unresolved_member == Some(request.member_id) {
            ScopePortStatus::Unresolved
        } else {
            ScopePortStatus::Complete
        }
    }
}

impl ComponentScopeReleasePort for ReleaseFake {
    fn release_member(&mut self, request: ScopeMemberPortRequest<'_>) -> ScopePortStatus {
        self.0
            .borrow_mut()
            .push(format!("release:{}", request.member_id));
        ScopePortStatus::Complete
    }
}

impl ComponentScopeReceiptPort for ReceiptFake {
    fn commit_receipt(&mut self, request: ScopeReceiptPortRequest<'_>) -> ScopePortStatus {
        self.0
            .borrow_mut()
            .push(format!("commit:{:?}", request.outcome));
        ScopePortStatus::Complete
    }
}

#[test]
fn fake_shell_executes_only_reducer_effects_in_reverse_dependency_order() {
    // r[verify agent_harness_runtime.component_scope.teardown]
    // r[verify agent_harness_runtime.component_scope.verification]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentScopeHost::new(
        ComponentScopePorts {
            delivery: DeliveryFake(Rc::clone(&log)),
            cancellation: CancellationFake(Rc::clone(&log)),
            quiescence: QuiescenceFake {
                log: Rc::clone(&log),
                unresolved_member: None,
            },
            release: ReleaseFake(Rc::clone(&log)),
            receipt: ReceiptFake(Rc::clone(&log)),
        },
        ComponentScopeHostBounds {
            max_effects: HOST_EFFECT_BOUND,
        },
    )
    .expect("fake host");
    let outcome = host.close_scope(&scope()).expect("clean close");

    assert_eq!(outcome.scope().state(), ComponentScopeState::Closed);
    assert_eq!(outcome.receipt().outcome, ScopeTerminalOutcome::Closed);
    assert_eq!(
        log.borrow().as_slice(),
        [
            "close:scope.worker",
            "cancel:component.consumer",
            "cancel:component.provider",
            "quiesce:component.consumer",
            "quiesce:component.provider",
            "release:component.consumer",
            "release:component.provider",
            "commit:Closed",
        ]
    );
}

#[test]
fn fake_shell_preserves_quiescence_deadline_as_failed_cleanup() {
    // r[verify agent_harness_runtime.component_scope.evidence]
    // r[verify agent_harness_runtime.component_scope.verification]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentScopeHost::new(
        ComponentScopePorts {
            delivery: DeliveryFake(Rc::clone(&log)),
            cancellation: CancellationFake(Rc::clone(&log)),
            quiescence: QuiescenceFake {
                log: Rc::clone(&log),
                unresolved_member: Some(CONSUMER),
            },
            release: ReleaseFake(Rc::clone(&log)),
            receipt: ReceiptFake(Rc::clone(&log)),
        },
        ComponentScopeHostBounds {
            max_effects: HOST_EFFECT_BOUND,
        },
    )
    .expect("fake host");
    let outcome = host.close_scope(&scope()).expect("failed cleanup receipt");

    assert_eq!(outcome.scope().state(), ComponentScopeState::FailedCleanup);
    assert_eq!(
        outcome.receipt().outcome,
        ScopeTerminalOutcome::FailedCleanup
    );
    assert!(!outcome.receipt().unresolved_cleanup.is_empty());
    assert!(
        log.borrow()
            .iter()
            .all(|entry| !entry.starts_with("release:"))
    );
}

#[test]
fn worker_scope_and_client_delivery_lease_share_generation_without_authority_confusion() {
    // r[verify agent_harness_runtime.component_scope.registration]
    // r[verify agent_harness_runtime.component_scope.verification]
    let record = worker_record();
    let worker = plan_worker_root_scope(&record).expect("worker scope plan");
    let client: ClientDeliveryScopeLease =
        plan_client_delivery_scope_lease(&record, "client-a").expect("client lease");

    assert_eq!(worker.generation, client.worker_generation);
    assert_eq!(worker.scope_id, client.worker_scope_id);
    assert!(!client.owns_worker_scope());
}

fn scope() -> animus_session_core::component_scope::ComponentScope {
    admit_component_scope(
        &graph(),
        ComponentScopeDraft {
            scope_id: String::from(SCOPE_ID),
            generation: generation(),
            parent: None,
            registrations: vec![
                ComponentRegistrationDraft {
                    member_id: String::from(PROVIDER),
                    component_id: String::from(PROVIDER),
                    member_kind: ScopeMemberKind::Component,
                    visibility: ScopeVisibility::LocalOnly,
                    dependency_member_ids: Vec::new(),
                },
                ComponentRegistrationDraft {
                    member_id: String::from(CONSUMER),
                    component_id: String::from(CONSUMER),
                    member_kind: ScopeMemberKind::Component,
                    visibility: ScopeVisibility::LocalOnly,
                    dependency_member_ids: vec![String::from(PROVIDER)],
                },
            ],
            non_claims: SCOPE_NON_CLAIMS.iter().copied().map(String::from).collect(),
        },
    )
    .expect("scope")
}

fn graph() -> ValidatedActivationGraph {
    ValidatedActivationGraph::validate(ActivationGraphDraft {
        components: vec![component(PROVIDER), component(CONSUMER)],
        ports: vec![ActivationPort {
            port_id: String::from("port.provider"),
            owner_component_id: String::from(CONSUMER),
            provider_component_ids: vec![String::from(PROVIDER)],
            consumer_component_ids: vec![String::from(CONSUMER)],
            scope_class: String::from("scope"),
            multiplicity: PortMultiplicity::ExactlyOne,
            lifecycle_owner_component_id: String::from(CONSUMER),
            posture: ActivationPosture::Required,
            feature_gate: String::from("always"),
            authority: ActivationAuthorityClass::InjectedOnly,
            focused_check: String::from("component-scope"),
        }],
        events: Vec::new(),
        non_claims: ACTIVATION_NON_CLAIMS
            .iter()
            .copied()
            .map(String::from)
            .collect(),
    })
    .expect("graph")
}

fn component(component_id: &str) -> ActivationComponent {
    ActivationComponent {
        component_id: String::from(component_id),
        feature_gate: String::from("always"),
        authority: ActivationAuthorityClass::InjectedOnly,
        posture: ActivationPosture::Required,
    }
}

fn generation() -> ScopeGeneration {
    ScopeGeneration::new(GENERATION_VALUE).expect("generation")
}

fn worker_record() -> WorkerRegistryRecord {
    WorkerRegistryRecord {
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: GENERATION_VALUE,
        process_id: PROCESS_ID,
        process_identity: String::from(HASH),
        endpoint_identity: String::from(HASH),
        session_head_identity: String::from(HASH),
        lifecycle: WorkerLifecycle::Ready,
        restore_status: RestoreStatus::Restored,
        attachments: Vec::new(),
        event_sequence: 0,
        receipt_identity: String::from(HASH),
    }
}
