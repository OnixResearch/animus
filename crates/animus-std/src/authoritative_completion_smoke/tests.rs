use super::*;

const GENERATED_ROLES: &str = include_str!("../../../../config/generated/agent-role-profiles.json");
const GENERATED_MODELS: &str =
    include_str!("../../../../config/generated/model-runtime-profiles.json");
const GENERATED_REGISTRY: &str =
    include_str!("../../../../config/generated/agent-check-registry.json");
const SUBJECT: &str = "private authoritative completion subject";
const EXPECTED_CHECK_COUNT: usize = 2;
const EXPECTED_PERSIST_CALLS: usize = 2;

fn loaded() -> (
    animus_runtime::agent_roles::AgentRoleCatalog,
    animus_runtime::ModelRuntimeProfileSet,
    animus_runtime::authoritative_completion::AgentCheckRegistry,
) {
    let roles = super::super::parse_agent_role_profiles_json(GENERATED_ROLES)
        .expect("generated role catalog");
    let models = super::super::parse_model_runtime_profiles_json(GENERATED_MODELS)
        .expect("generated model profiles");
    let registry = super::super::parse_agent_check_registry_json(GENERATED_REGISTRY)
        .expect("generated check registry");
    (roles, models, registry)
}

#[test]
fn exact_role_selection_and_completion_facts_produce_safe_verified_receipt() {
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.completion.pass]
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.receipt.safe]
    let (roles, models, registry) = loaded();
    let report = run_completion_workflow(LoadedCompletionInput {
        roles: &roles,
        model_profiles: &models,
        registry: &registry,
        subject: SUBJECT,
        head_state: HeadState::Clean,
        fixture: CompletionFixture::Admitted,
    })
    .expect("exact fixture should verify");
    let output = render_report(&report);

    assert!(report.authoritative_completion);
    assert_eq!(report.judge_decision, "select");
    assert_eq!(report.required_check_count, EXPECTED_CHECK_COUNT);
    assert_eq!(report.passed_check_count, EXPECTED_CHECK_COUNT);
    assert_eq!(report.service_calls.artifact_observer, 1);
    assert_eq!(report.service_calls.subject_hasher, 1);
    assert_eq!(report.service_calls.check_executor, EXPECTED_CHECK_COUNT);
    assert_eq!(
        report.service_calls.artifact_persister,
        EXPECTED_PERSIST_CALLS
    );
    assert_eq!(report.service_calls.cairn_validator, 1);
    assert!(output.contains("authoritative_completion=true"));
    assert!(output.contains("surface=check-oracle-soundness"));
    assert!(!output.contains(SUBJECT));
    assert!(!output.contains("completion-request"));
    assert!(!output.contains("nix run"));
    assert!(!output.contains("judge rationale"));
}

#[test]
fn veto_self_attestation_and_dirty_state_stop_before_completion() {
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.role_boundary.veto]
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.role_boundary.self_attestation]
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.host_services.dirty]
    let (roles, models, registry) = loaded();
    for (fixture, expected_issue) in [
        (CompletionFixture::JudgeVeto, "issue=judge-veto"),
        (
            CompletionFixture::RoleSelfAttestation,
            "issue=role-self-attestation",
        ),
    ] {
        let failure = run_completion_workflow(LoadedCompletionInput {
            roles: &roles,
            model_profiles: &models,
            registry: &registry,
            subject: SUBJECT,
            head_state: HeadState::Clean,
            fixture,
        })
        .expect_err("pre-completion fixture should fail");
        assert!(failure.contains(expected_issue), "{failure}");
        assert!(!failure.contains(SUBJECT));
    }
    let dirty = run_completion_workflow(LoadedCompletionInput {
        roles: &roles,
        model_profiles: &models,
        registry: &registry,
        subject: SUBJECT,
        head_state: HeadState::Dirty,
        fixture: CompletionFixture::Admitted,
    })
    .expect_err("dirty head should fail");
    assert!(dirty.contains("issue=head-dirty"));
}

#[test]
fn missing_result_stale_subject_and_missing_cairn_block_completion() {
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.failure.missing_result]
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.failure.stale_subject]
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.failure.missing_cairn]
    let (roles, models, registry) = loaded();
    for (fixture, expected_issue) in [
        (CompletionFixture::MissingResult, "issue=missing-result"),
        (CompletionFixture::StaleSubject, "issue=stale-subject"),
        (CompletionFixture::MissingCairn, "issue=missing-cairn"),
    ] {
        let failure = run_completion_workflow(LoadedCompletionInput {
            roles: &roles,
            model_profiles: &models,
            registry: &registry,
            subject: SUBJECT,
            head_state: HeadState::Clean,
            fixture,
        })
        .expect_err("completion evidence fixture should fail");
        assert!(failure.contains(expected_issue), "{failure}");
        assert!(!failure.contains(SUBJECT));
    }
}

#[test]
fn missing_unknown_and_conflicting_arguments_fail_closed() {
    // r[verify agent_harness_runtime.authoritative_completion_host_smoke.loading.failure]
    let missing = parse_args(&[]).expect_err("missing arguments");
    let unknown = parse_args(&[String::from("--unknown")]).expect_err("unknown argument");
    let conflict = parse_args(&[
        String::from(ARG_JUDGE_VETO),
        String::from(ARG_MISSING_RESULT),
    ])
    .expect_err("conflicting fixtures");

    assert!(missing.contains(ERROR_MISSING_ARGUMENT));
    assert!(unknown.contains(ERROR_UNKNOWN_ARGUMENT));
    assert!(conflict.contains(ERROR_CONFLICTING_FIXTURE));
}
