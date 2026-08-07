use super::*;

const SUMMARY_BOUND: u32 = 128;
const DIFF_BOUND: u32 = 256;
const SINGLE_MATCH: u32 = 1;
const AMBIGUOUS_MATCHES: u32 = 2;
const NON_CLAIM_LIFECYCLE: &str = "does-not-grant-lifecycle-authority";
const NON_CLAIM_RETRY: &str = "does-not-authorize-retry";
const NON_CLAIM_CORRECTNESS: &str = "does-not-prove-later-correction";

#[test]
fn valid_cairn_handoff_projects_bounded_summary_and_metadata_receipt() {
    // r[agent_harness_runtime.rejected_attempt.boundary]
    // r[agent_harness_runtime.rejected_attempt.planning]
    // r[agent_harness_runtime.rejected_attempt.evidence]
    let plan = plan_rejected_attempt_context(&request(), &policy()).expect("valid handoff");
    assert_eq!(plan.redacted_summary, "check failed: stale projection");
    assert_eq!(
        plan.raw_diff_disposition,
        RawDiffDisposition::DeniedByPolicy
    );
    assert_eq!(plan.context_profile, "fresh-context-task");
    assert_eq!(plan.loop_profile, "ralph-loop");

    let receipt = project_rejected_attempt_receipt(&plan, &policy());
    assert!(validate_rejected_attempt_receipt(&receipt, &policy()).is_ok());
    assert!(!receipt.receipt_id.as_str().is_empty());
    assert_eq!(receipt.non_claims.len(), policy().required_non_claims.len());
}

#[test]
fn current_jujutsu_attempt_is_vcs_neutral_and_deterministic() {
    // r[agent_harness_runtime.rejected_attempt.model]
    let first = plan_rejected_attempt_context(&request(), &policy()).expect("first plan");
    let second = plan_rejected_attempt_context(&request(), &policy()).expect("second plan");
    assert_eq!(first, second);
    assert_eq!(request().vcs.provider, VcsProvider::Jujutsu);
    assert_eq!(
        project_rejected_attempt_receipt(&first, &policy()).receipt_id,
        project_rejected_attempt_receipt(&second, &policy()).receipt_id
    );
}

#[test]
fn missing_review_wrong_task_and_ambiguous_attempt_fail_closed() {
    // r[agent_harness_runtime.rejected_attempt.boundary]
    // r[agent_harness_runtime.rejected_attempt.verification]
    let mut input = request();
    input.review.validated = false;
    input.review.task_id = TaskIdentity::new("task.other");
    input.review.matching_attempts = AMBIGUOUS_MATCHES;
    let issues = plan_rejected_attempt_context(&input, &policy()).expect_err("must deny");
    assert!(issues.contains(&RejectedAttemptIssue::ReviewEvidenceDenied));
    assert!(issues.contains(&RejectedAttemptIssue::WrongTaskOrRun));
    assert!(issues.contains(&RejectedAttemptIssue::AmbiguousAttempt));
}

#[test]
fn stale_revision_unsafe_summary_and_oversized_summary_fail_closed() {
    // r[agent_harness_runtime.rejected_attempt.policy]
    // r[agent_harness_runtime.rejected_attempt.verification]
    let mut input = request();
    input.vcs.current = false;
    input.vcs.content_id = ContentIdentity::new("content.changed");
    input.context.redacted_summary = format!("API_KEY {}", "x".repeat(SUMMARY_BOUND as usize));
    input.context.summary_classification_safe = false;
    let issues = plan_rejected_attempt_context(&input, &policy()).expect_err("must deny");
    assert!(issues.contains(&RejectedAttemptIssue::StaleAttempt));
    assert!(issues.contains(&RejectedAttemptIssue::UnsafeSummary));
    assert!(issues.contains(&RejectedAttemptIssue::OversizedSummary));
}

#[test]
fn raw_diff_requires_policy_capability_classification_and_budget() {
    // r[agent_harness_runtime.rejected_attempt.policy]
    let mut input = request();
    let mut admitted_policy = policy();
    admitted_policy.allow_raw_diff = true;

    input.context.raw_diff_capability_admitted = true;
    input.context.raw_diff_classification_safe = false;
    let classified =
        plan_rejected_attempt_context(&input, &admitted_policy).expect("summary valid");
    assert_eq!(
        classified.raw_diff_disposition,
        RawDiffDisposition::DeniedByClassification
    );

    input.context.raw_diff_classification_safe = true;
    input.context.raw_diff = Some(vec![b'x'; (DIFF_BOUND + SINGLE_MATCH) as usize]);
    let oversized = plan_rejected_attempt_context(&input, &admitted_policy).expect("summary valid");
    assert_eq!(
        oversized.raw_diff_disposition,
        RawDiffDisposition::DeniedByBudget
    );
}

#[test]
fn receipt_rejects_completion_overclaims_and_weakened_non_claims() {
    // r[agent_harness_runtime.rejected_attempt.evidence]
    // r[agent_harness_runtime.rejected_attempt.verification]
    let plan = plan_rejected_attempt_context(&request(), &policy()).expect("valid handoff");
    let mut receipt = project_rejected_attempt_receipt(&plan, &policy());
    receipt.claims.push("task-complete".to_string());
    receipt.non_claims.clear();
    let issues = validate_rejected_attempt_receipt(&receipt, &policy()).expect_err("must deny");
    assert!(issues.contains(&RejectedAttemptIssue::ReceiptOverclaim));
    assert!(issues.contains(&RejectedAttemptIssue::MissingNonClaim));
}

fn policy() -> RejectedAttemptPolicy {
    RejectedAttemptPolicy {
        policy_id: "rejected-attempt.default.v1".to_string(),
        accepted_review_profiles: vec![ReviewEvidenceProfile::CairnRevisionHandoff],
        accepted_vcs_providers: vec![VcsProvider::Jujutsu, VcsProvider::Git],
        max_summary_bytes: SUMMARY_BOUND,
        max_raw_diff_bytes: DIFF_BOUND,
        allow_raw_diff: false,
        required_non_claims: vec![
            NON_CLAIM_LIFECYCLE.to_string(),
            NON_CLAIM_RETRY.to_string(),
            NON_CLAIM_CORRECTNESS.to_string(),
        ],
    }
}

fn request() -> RejectedAttemptRequest {
    let task_id = TaskIdentity::new("task.one");
    let run_id = RunIdentity::new("run.one");
    let attempt_id = AttemptIdentity::new("attempt.one");
    let revision_id = RevisionIdentity::new("revision.one");
    let workspace_snapshot_id = WorkspaceSnapshotIdentity::new("workspace.one");
    let content_id = ContentIdentity::new("content.one");
    RejectedAttemptRequest {
        task_id: task_id.clone(),
        run_id: run_id.clone(),
        attempt_id: attempt_id.clone(),
        revision_id: revision_id.clone(),
        workspace_snapshot_id: workspace_snapshot_id.clone(),
        content_id: content_id.clone(),
        review: ReviewHandoffFacts {
            handoff_id: ReviewHandoffIdentity::new("handoff.one"),
            profile: ReviewEvidenceProfile::CairnRevisionHandoff,
            task_id,
            run_id,
            attempt_id: attempt_id.clone(),
            revision_id: revision_id.clone(),
            disposition: ReviewDisposition::Rejected,
            matching_attempts: SINGLE_MATCH,
            validated: true,
        },
        vcs: VcsAttemptFacts {
            provider: VcsProvider::Jujutsu,
            attempt_id,
            revision_id,
            workspace_snapshot_id,
            content_id,
            current: true,
        },
        context: RejectionContextFacts {
            summary_id: SummaryIdentity::new("summary.one"),
            redacted_summary: "check failed: stale projection".to_string(),
            summary_classification_safe: true,
            raw_diff: Some(b"redacted metadata diff".to_vec()),
            raw_diff_classification_safe: true,
            raw_diff_capability_admitted: false,
        },
    }
}
