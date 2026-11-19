use super::{
    CodecError, CurrentRecord, CurrentReply, FixtureOwner, Invocation, InvocationReport, OwnerCall,
    Selection, UnavailableOwner, invoke, parse_profile, parse_snapshot, receipt_json,
};
use crate::mesh::{
    MeshHostError, MeshTransport, TransportOutcome, TransportRequest, TransportStatus,
};
use animus_adapters::serving::{
    CallerBinding, CallerDraft, Counter, DispatchOutcome, EvidenceLabel, Freshness, GateBlock,
    GateRejection, NotDispatched, OwnerRejection, Readiness, ResponseFacts, ResponseRejection,
    RouteCandidate, RouteCandidateDraft, ScreenRequest, UncertainCause, Usage, UsageOrigin,
};

const PROFILE_JSON: &[u8] =
    include_bytes!("../../../../config/generated/serving-capacity-profile.json");
const SNAPSHOT_JSON: &[u8] =
    include_bytes!("../../../../fixtures/serving-admission/snapshot-cpu-worker-remote.json");
const CONFIG_A: &str = "blake3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const ROUTE_REF: &str = "blake3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const SERVICE: &str = "svc-serving-b";
const MODEL: &str = "model-a";
const LABEL: &str = "model:serving-b";
const BACKEND: &str = "backend:remote";
const NOW_TICK: u64 = 105;
const GRANT_WINDOW: (u64, u64) = (105, 130);
const PROMPT: &[u8] = b"secret prompt text that must never reach a receipt";
const REPORTED: Usage = Usage {
    input_tokens: Counter::Known(1_812),
    output_tokens: Counter::Known(240),
    origin: UsageOrigin::ProviderReported,
};

struct ScriptedTransport {
    calls: u32,
    status: TransportStatus,
    model: Option<&'static str>,
    facts: ResponseFacts,
    failure: Option<MeshHostError>,
}

impl ScriptedTransport {
    const fn answering(facts: ResponseFacts) -> Self {
        Self {
            calls: 0,
            status: TransportStatus::Success,
            model: Some(MODEL),
            facts,
            failure: None,
        }
    }
}

impl MeshTransport for ScriptedTransport {
    fn send(&mut self, request: &TransportRequest<'_>) -> Result<TransportOutcome, MeshHostError> {
        self.calls += 1;
        assert_eq!(request.endpoint_identity, "endpoint:serving-b");
        if let Some(failure) = self.failure {
            return Err(failure);
        }
        Ok(TransportOutcome {
            status: self.status,
            body: b"{\"output\":\"fixture\"}".to_vec(),
            response_model: self.model.map(str::to_owned),
            facts: self.facts,
        })
    }
}

struct World {
    profile: animus_adapters::serving::Profile,
    snapshot: animus_adapters::serving::AvailabilitySnapshot,
    candidates: Vec<RouteCandidate>,
    signals: animus_router::Signals,
    recipes: Vec<animus_router::Recipe>,
    budget: animus_router::Budget,
    caller: CallerBinding,
}

fn candidate_draft() -> RouteCandidateDraft<'static> {
    RouteCandidateDraft {
        route_name: "primary",
        route_ref: Some(ROUTE_REF),
        label: LABEL,
        backend: BACKEND,
        role: "primary",
        service_id: SERVICE,
        model_id: MODEL,
        configuration_ref: CONFIG_A,
        requirements: animus_adapters::mesh::Requirements {
            tools: true,
            streaming: true,
            structured_output: false,
            developer_role: false,
            input_tokens: 2_000,
            output_tokens: 500,
        },
        rollout: None,
    }
}

fn world() -> World {
    let signals = animus_router::Signals::project(animus_router::SignalDraft {
        task_shape: "simple",
        risk_band: "low",
        contract_pressure: "low",
        context_pressure: "low",
        latency_band: "low",
        cost_band: "low",
        privacy_band: "remote_allowed",
        fallback_eligibility: "ineligible",
    })
    .unwrap();
    let recipe = animus_router::Recipe::new(animus_router::RecipeDraft {
        id: "recipe:single",
        family: "single",
        roles: &["primary"],
        output_contract: "text",
        max_candidates: 2,
        max_parallelism: 1,
        min_success_quorum: 1,
        timeout_class: "standard",
        max_synthesis_attempts: 0,
        max_repair_attempts: Some(0),
        fallback_policy: "deny",
    })
    .unwrap();
    World {
        profile: parse_profile(PROFILE_JSON).unwrap(),
        snapshot: parse_snapshot(SNAPSHOT_JSON).unwrap(),
        candidates: vec![RouteCandidate::new(&candidate_draft()).unwrap()],
        signals,
        recipes: vec![recipe],
        budget: animus_router::Budget::new(2, &[BACKEND]).unwrap(),
        caller: caller("session-1"),
    }
}

fn caller(session: &str) -> CallerBinding {
    CallerBinding::new(CallerDraft {
        agent: "agent-cpu-worker",
        session,
        attempt: "attempt-1",
    })
    .unwrap()
}

fn current_record() -> CurrentRecord {
    CurrentRecord {
        service_id: SERVICE.to_owned(),
        endpoint_identity: "endpoint:serving-b".to_owned(),
        deployment_id: "deploy-serving-b".to_owned(),
        deployment_generation: 3,
        model_id: MODEL.to_owned(),
        configuration_ref: CONFIG_A.to_owned(),
        readiness: Readiness::Ready,
        admission_epoch: 1,
        now_tick: Some(NOW_TICK),
        policy_generation: None,
    }
}

fn owner(free_slots: u32) -> FixtureOwner {
    let mut owner = FixtureOwner::new(current_record(), free_slots, GRANT_WINDOW);
    owner.replica = Some("replica-1".to_owned());
    owner
}

fn run<O: super::OwnerPort>(
    world: &World,
    owner: &mut O,
    transport: &mut ScriptedTransport,
    call: (&CallerBinding, &str, &[u8]),
) -> InvocationReport {
    invoke(
        owner,
        transport,
        &Invocation {
            evidence: EvidenceLabel::Fixture,
            screen: ScreenRequest {
                profile: &world.profile,
                snapshot: &world.snapshot,
                freshness: Freshness {
                    now_tick: Some(NOW_TICK),
                    newest_accepted: None,
                },
                privacy: animus_router::PrivacyBand::RemoteAllowed,
                candidates: &world.candidates,
            },
            selection: Selection {
                signals: &world.signals,
                recipes: &world.recipes,
                budget: &world.budget,
                rotation: None,
            },
            caller: call.0,
            request_id: call.1,
            existing: animus_adapters::mesh::DispatchAdmission {
                exact_model: animus_adapters::mesh::ExactModelDecision::Admitted,
                profile_admitted: true,
                privacy_admitted: true,
                budget_admitted: true,
                authority_admitted: true,
                remote_admitted: true,
                deadline_available: true,
                cancellation_available: true,
                transport_available: true,
            },
            provider_label: "mesh-private",
            projected_body: call.2,
        },
    )
    .unwrap()
}

fn reported() -> ResponseFacts {
    ResponseFacts {
        usage: REPORTED,
        policy_generation: None,
    }
}

#[test]
fn cpu_worker_reaches_a_remote_endpoint_through_owner_admission() {
    let world = world();
    let mut owner = owner(1);
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut owner,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(report.receipt.outcome(), DispatchOutcome::Completed);
    assert_eq!(transport.calls, 1);
    assert_eq!(
        owner.calls(),
        [
            OwnerCall::Admit("request-1".to_owned()),
            OwnerCall::Recheck("request-1".to_owned())
        ]
    );
    let route = report.receipt.selected().unwrap();
    assert_eq!(route.route().route_ref(), Some(ROUTE_REF));
    assert_eq!(route.deployment().deployment_generation(), 3);
    assert_eq!(route.resource_consumer_ref(), Some("serving-b"));
    assert_eq!(report.receipt.usage(), REPORTED);
    let provider_receipt = report.provider_receipt.unwrap();
    assert_eq!(provider_receipt.physical_route, "unknown");
    assert_eq!(
        report.receipt.provider_receipt_identity(),
        Some(provider_receipt.identity.as_str())
    );
    assert_eq!(report.screening.exclusions().len(), 0);
}

#[test]
fn standalone_owner_fails_closed_before_provider_contact() {
    let world = world();
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut UnavailableOwner,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::Blocked(GateBlock::AdmissionUnavailable))
    );
    assert_eq!(transport.calls, 0);
    assert_eq!(report.receipt.usage().origin, UsageOrigin::Unknown);
}

#[test]
fn generation_replaced_between_admission_and_dispatch_never_dispatches() {
    let world = world();
    let mut replaced = owner(1);
    replaced.after_admission = Some(CurrentRecord {
        deployment_generation: 4,
        ..current_record()
    });
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut replaced,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::Rejected(GateRejection::GenerationReplaced))
    );
    let mut granted_for_new = owner(1);
    granted_for_new.current.deployment_generation = 4;
    let again = run(
        &world,
        &mut granted_for_new,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        again.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::Rejected(GateRejection::GenerationReplaced))
    );
    assert_eq!(transport.calls, 0);
}

#[test]
fn sessions_sharing_the_last_slot_get_one_grant_and_separate_receipts() {
    let world = world();
    let mut shared = owner(1);
    let mut transport = ScriptedTransport::answering(reported());
    let first_caller = caller("session-1");
    let second_caller = caller("session-2");
    let first = run(
        &world,
        &mut shared,
        &mut transport,
        (&first_caller, "request-1", PROMPT),
    );
    let second = run(
        &world,
        &mut shared,
        &mut transport,
        (&second_caller, "request-2", PROMPT),
    );
    assert_eq!(first.receipt.outcome(), DispatchOutcome::Completed);
    assert_eq!(
        second.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::Rejected(GateRejection::OwnerRejected(
            OwnerRejection::Saturated
        )))
    );
    assert_eq!(transport.calls, 1);
    assert_ne!(first.receipt.identity(), second.receipt.identity());
    assert_eq!(
        first.receipt.selected().unwrap().deployment(),
        second.receipt.selected().unwrap().deployment()
    );
}

#[test]
fn same_model_replica_change_is_recorded_with_stable_agent_identity() {
    let world = world();
    let mut moved = owner(1);
    moved.replica = Some("replica-2".to_owned());
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut moved,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(report.receipt.outcome(), DispatchOutcome::Completed);
    assert!(report.receipt.is_replica_changed());
    assert_eq!(report.receipt.replica().label(), "replica-2");
    assert_eq!(report.receipt.caller().agent_id(), "agent-cpu-worker");
}

#[test]
fn substitution_and_uncertain_results_are_never_completed_or_retried() {
    let world = world();
    let mut substituted = ScriptedTransport::answering(reported());
    substituted.model = Some("model-b");
    let report = run(
        &world,
        &mut owner(1),
        &mut substituted,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::ResponseRejected(ResponseRejection::ModelSubstituted)
    );
    assert_eq!(report.receipt.usage().origin, UsageOrigin::Unknown);
    let mut timed_out = ScriptedTransport::answering(reported());
    timed_out.status = TransportStatus::TimedOut;
    let mut timing_owner = owner(1);
    let report = run(
        &world,
        &mut timing_owner,
        &mut timed_out,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::Uncertain(UncertainCause::TimedOut)
    );
    assert_eq!(timed_out.calls, 1);
    assert_eq!(timing_owner.calls().len(), 2);
    let mut missing = ScriptedTransport::answering(reported());
    missing.failure = Some(MeshHostError::MissingTransport);
    let report = run(
        &world,
        &mut owner(1),
        &mut missing,
        (&world.caller, "request-1", PROMPT),
    );
    assert!(!report.receipt.outcome().is_dispatched());
}

#[test]
fn prompts_never_reach_receipts() {
    let world = world();
    let mut transport = ScriptedTransport::answering(reported());
    let first = run(
        &world,
        &mut owner(1),
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    let second = run(
        &world,
        &mut owner(1),
        &mut transport,
        (&world.caller, "request-1", b"another prompt"),
    );
    assert_eq!(first.receipt.identity(), second.receipt.identity());
    let text = receipt_json(&first.receipt).to_string();
    assert!(!text.contains("secret prompt"));
    assert!(!text.contains("fixture\\\""));
    assert!(text.contains("\"schema\":\"animus.serving-invocation-receipt.v1\""));
    assert!(text.contains(ROUTE_REF));
}

#[test]
fn ineligible_or_unselectable_routes_never_reach_the_owner() {
    let mut world = world();
    world.candidates = vec![
        RouteCandidate::new(&RouteCandidateDraft {
            model_id: "model-b",
            ..candidate_draft()
        })
        .unwrap(),
    ];
    let mut never = owner(1);
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut never,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::NoEligibleRoute)
    );
    let mut world = self::world();
    world.budget = animus_router::Budget::new(2, &["backend:other"]).unwrap();
    let report = run(
        &world,
        &mut never,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::SelectionDenied)
    );
    assert!(never.calls().is_empty());
    assert_eq!(transport.calls, 0);
}

#[test]
fn rotation_operates_only_over_eligible_routes() {
    let mut world = world();
    let hosted = RouteCandidate::new(&RouteCandidateDraft {
        route_name: "hosted",
        route_ref: None,
        label: "model:hosted",
        service_id: "svc-hosted",
        model_id: "model-h",
        configuration_ref: "blake3:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        ..candidate_draft()
    })
    .unwrap();
    world.candidates.push(hosted);
    let catalog_hash = "0000000000000000000000000000000000000000000000000000000000000000";
    let rotation =
        animus_router::RouterRotationSnapshot::new(animus_router::RouterRotationSnapshotDraft {
            policy: "role_candidates",
            stale_cursor_policy: "reset",
            catalog_hash,
            cursors: &[animus_router::RouterRotationCursorDraft {
                role: "primary",
                position: 1,
                catalog_hash: Some(catalog_hash),
            }],
        })
        .unwrap();
    let screening = animus_adapters::serving::screen_routes(&ScreenRequest {
        profile: &world.profile,
        snapshot: &world.snapshot,
        freshness: Freshness {
            now_tick: Some(NOW_TICK),
            newest_accepted: None,
        },
        privacy: animus_router::PrivacyBand::RemoteAllowed,
        candidates: &world.candidates,
    })
    .unwrap();
    assert_eq!(screening.exclusions().len(), 1);
    assert_eq!(screening.exclusions()[0].route_name(), "hosted");
    let catalog = screening.eligible_catalog();
    let plan =
        animus_router::select_recipe_with_rotation(animus_router::RotatingSelectionRequest {
            signals: &world.signals,
            recipes: &world.recipes,
            catalog: &catalog,
            budget: &world.budget,
            rotation: &rotation,
        })
        .unwrap();
    assert_eq!(plan.selected_models(), [LABEL]);
}

#[test]
fn snapshot_json_is_bounded_and_strict() {
    let snapshot = parse_snapshot(SNAPSHOT_JSON).unwrap();
    assert_eq!(snapshot.services().len(), 2);
    let mut value: serde_json::Value = serde_json::from_slice(SNAPSHOT_JSON).unwrap();
    value["schema"] = "animus.serving-availability-snapshot.v2".into();
    assert_eq!(
        parse_snapshot(value.to_string().as_bytes()).unwrap_err(),
        CodecError::WrongSchema
    );
    let mut value: serde_json::Value = serde_json::from_slice(SNAPSHOT_JSON).unwrap();
    value["services"][0]["prompt"] = "leak".into();
    assert_eq!(
        parse_snapshot(value.to_string().as_bytes()).unwrap_err(),
        CodecError::Malformed
    );
    let mut value: serde_json::Value = serde_json::from_slice(SNAPSHOT_JSON).unwrap();
    value["services"][0]["capacity"][0]["unit"] = "bytes".into();
    assert_eq!(
        parse_snapshot(value.to_string().as_bytes()).unwrap_err(),
        CodecError::Contract(animus_adapters::serving::ContractIssue::UnitMismatch)
    );
    let mut value: serde_json::Value = serde_json::from_slice(SNAPSHOT_JSON).unwrap();
    value["services"][0]["readiness"] = "warming".into();
    assert_eq!(
        parse_snapshot(value.to_string().as_bytes()).unwrap_err(),
        CodecError::Malformed
    );
    let oversized = vec![b' '; super::MAX_SNAPSHOT_BYTES + 1];
    assert_eq!(
        parse_snapshot(&oversized).unwrap_err(),
        CodecError::TooLarge
    );
}

#[test]
fn generated_profile_parses_and_drift_fails() {
    let profile = parse_profile(PROFILE_JSON).unwrap();
    assert_eq!(profile.profile_id(), "capacity-aware-observed-v1");
    for (needle, replacement) in [
        ("\"required\"", "\"optional\""),
        ("\"metadata_only\"", "\"raw\""),
        ("\"max_services\": 64", "\"max_services\": 65"),
        ("\"not-cost-or-pricing\"", "\"cost-is-known\""),
    ] {
        let drift = String::from_utf8_lossy(PROFILE_JSON).replace(needle, replacement);
        assert_eq!(
            parse_profile(drift.as_bytes()).unwrap_err(),
            CodecError::ParityMismatch
        );
    }
    let unknown_rule = String::from_utf8_lossy(PROFILE_JSON)
        .replace("\"known_required\"", "\"free_memory_sufficient\"");
    assert_eq!(
        parse_profile(unknown_rule.as_bytes()).unwrap_err(),
        CodecError::Malformed
    );
}

#[test]
fn recheck_unavailability_blocks_after_a_grant() {
    let world = world();
    let mut owner = owner(1);
    owner.recheck_override = Some(CurrentReply::PortIssue);
    let mut transport = ScriptedTransport::answering(reported());
    let report = run(
        &world,
        &mut owner,
        &mut transport,
        (&world.caller, "request-1", PROMPT),
    );
    assert_eq!(
        report.receipt.outcome(),
        DispatchOutcome::NotDispatched(NotDispatched::Blocked(GateBlock::RecheckPortIssue))
    );
    assert_eq!(transport.calls, 0);
}
