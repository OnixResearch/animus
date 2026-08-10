use super::*;

const GENERATED_ROLES: &str = include_str!("../../../../config/generated/agent-role-profiles.json");
const GENERATED_MODELS: &str =
    include_str!("../../../../config/generated/model-runtime-profiles.json");
const SUBJECT: &str = "private immutable multi-role subject";
const IMPLEMENTER: &str = "implementer";
const REVIEWER: &str = "reviewer";
const VERIFIER: &str = "verifier";
const JUDGE: &str = "judge";

#[test]
fn exact_four_role_workflow_emits_safe_non_completion_evidence() {
    // r[verify agent_harness_runtime.multi_role_host_smoke.activation.pass]
    // r[verify agent_harness_runtime.multi_role_host_smoke.outputs.pass]
    // r[verify agent_harness_runtime.multi_role_host_smoke.receipt.safe]
    let roles = super::super::parse_agent_role_profiles_json(GENERATED_ROLES)
        .expect("generated role catalog");
    let models = super::super::parse_model_runtime_profiles_json(GENERATED_MODELS)
        .expect("generated model profiles");
    let subject_hash = domain_hash(SUBJECT_HASH_DOMAIN, SUBJECT);
    let report = run_workflow(
        &roles,
        &models,
        subject_hash.as_str(),
        SmokeFixture::Admitted,
    )
    .expect("four-role workflow");
    let output = render_report(&report);
    let role_ids = report
        .roles
        .iter()
        .map(|role| role.role_id.as_str())
        .collect::<Vec<_>>();
    let effects = report
        .roles
        .iter()
        .map(|role| role.output_effect)
        .collect::<Vec<_>>();

    assert_eq!(role_ids, [IMPLEMENTER, REVIEWER, VERIFIER, JUDGE]);
    assert_eq!(effects, ["candidate", "finding", "check_evidence", "veto"]);
    assert!(report.boundaries.reviewer_mutation_rejected);
    assert!(report.boundaries.completion_claim_rejected);
    assert!(report.boundaries.completion_candidate_rejected);
    assert!(!report.authoritative_completion);
    assert!(output.contains("role_count=4"));
    assert!(output.contains("authoritative_completion=false"));
    assert!(output.contains("surface=provider-quality"));
    assert!(!output.contains(SUBJECT));
    assert!(!output.contains("judge rationale"));
}

#[test]
fn collapsed_reviewer_identity_stops_before_later_outputs() {
    // r[verify agent_harness_runtime.multi_role_host_smoke.separation.failure]
    let roles = super::super::parse_agent_role_profiles_json(GENERATED_ROLES)
        .expect("generated role catalog");
    let models = super::super::parse_model_runtime_profiles_json(GENERATED_MODELS)
        .expect("generated model profiles");
    let subject_hash = domain_hash(SUBJECT_HASH_DOMAIN, SUBJECT);
    let failure = run_workflow(
        &roles,
        &models,
        subject_hash.as_str(),
        SmokeFixture::CollapseReviewerActor,
    )
    .expect_err("collapsed reviewer separation");

    assert!(failure.contains("issue=separation"));
    assert!(!failure.contains(SUBJECT));
}

#[test]
fn denied_veto_reviewer_mutation_and_completion_claim_fail_closed() {
    // r[verify agent_harness_runtime.multi_role_host_smoke.outputs.failure]
    // r[verify agent_harness_runtime.multi_role_host_smoke.completion_boundary.mutation]
    // r[verify agent_harness_runtime.multi_role_host_smoke.completion_boundary.completion]
    let roles = super::super::parse_agent_role_profiles_json(GENERATED_ROLES)
        .expect("generated role catalog");
    let models = super::super::parse_model_runtime_profiles_json(GENERATED_MODELS)
        .expect("generated model profiles");
    let subject_hash = domain_hash(SUBJECT_HASH_DOMAIN, SUBJECT);
    let denied_veto = run_workflow(
        &roles,
        &models,
        subject_hash.as_str(),
        SmokeFixture::DenyJudgeVeto,
    )
    .expect_err("denied judge veto");
    let reviewer_mutation = run_workflow(
        &roles,
        &models,
        subject_hash.as_str(),
        SmokeFixture::RequireReviewerMutation,
    )
    .expect_err("reviewer mutation");
    let completion_claim = run_workflow(
        &roles,
        &models,
        subject_hash.as_str(),
        SmokeFixture::ClaimCompletion,
    )
    .expect_err("completion claim");

    assert!(denied_veto.contains("issue=output"));
    assert!(reviewer_mutation.contains("issue=reviewer-mutation"));
    assert!(completion_claim.contains("issue=output"));
}

#[test]
fn missing_and_unknown_arguments_fail_before_policy_loading() {
    // r[verify agent_harness_runtime.multi_role_host_smoke.contract.failure]
    let missing = parse_args(&[]).expect_err("missing arguments");
    let unknown = parse_args(&[String::from("--unknown")]).expect_err("unknown argument");
    let missing_value =
        parse_args(&[String::from(ARG_ROLE_POLICY)]).expect_err("missing argument value");

    assert!(missing.contains(ERROR_MISSING_ARGUMENT));
    assert!(unknown.contains(ERROR_UNKNOWN_ARGUMENT));
    assert!(missing_value.contains(ERROR_MISSING_ARGUMENT_VALUE));
}
