//! Pure rejected-attempt admission, projection, and receipt logic.

use super::data::{
    RawDiffDisposition, ReceiptIdentity, RejectedAttemptContextPlan, RejectedAttemptIssue,
    RejectedAttemptPolicy, RejectedAttemptReceipt, RejectedAttemptRequest, ReviewDisposition,
};

const REQUIRED_MATCH_COUNT: u32 = 1;
const FRESH_CONTEXT_PROFILE: &str = "fresh-context-task";
const RALPH_PROFILE: &str = "ralph-loop";
const OMITTED_RAW_DIFF: &str = "raw_diff";
const OMITTED_TRANSCRIPT: &str = "attempt_transcript";
const RECEIPT_HASH_DOMAIN: &str = "animus.rejected-attempt-receipt.v1";
const HASH_SEPARATOR: &str = "|";
const CLAIM_REVIEW_BOUND: &str = "review-evidence-bound";
const CLAIM_CURRENT_ATTEMPT: &str = "current-attempt-identity";
const CLAIM_BOUNDED_SUMMARY: &str = "bounded-redacted-summary";
const UNSAFE_MARKERS: [&str; 4] = ["API_KEY", "/home/", "provider payload", "tool_output:"];

/// Plans safe rejected-attempt context from already-loaded facts.
///
/// # Errors
///
/// Returns all detected admission, freshness, classification, and bound issues.
pub fn plan_rejected_attempt_context(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
) -> Result<RejectedAttemptContextPlan, Vec<RejectedAttemptIssue>> {
    let mut issues = Vec::new();
    validate_identities(request, policy, &mut issues);
    validate_review(request, policy, &mut issues);
    validate_vcs(request, policy, &mut issues);
    validate_summary(request, policy, &mut issues);

    if !issues.is_empty() {
        return Err(issues);
    }

    let raw_diff_disposition = classify_raw_diff(request, policy);
    let mut omissions = vec![OMITTED_TRANSCRIPT.to_string()];
    if raw_diff_disposition != RawDiffDisposition::Admitted {
        omissions.push(OMITTED_RAW_DIFF.to_string());
    }

    Ok(RejectedAttemptContextPlan {
        task_id: request.task_id.clone(),
        run_id: request.run_id.clone(),
        attempt_id: request.attempt_id.clone(),
        handoff_id: request.review.handoff_id.clone(),
        revision_id: request.revision_id.clone(),
        summary_id: request.context.summary_id.clone(),
        redacted_summary: request.context.redacted_summary.clone(),
        raw_diff_disposition,
        context_profile: FRESH_CONTEXT_PROFILE.to_string(),
        loop_profile: RALPH_PROFILE.to_string(),
        omissions,
    })
}

/// Projects a metadata-only advisory receipt.
#[must_use]
pub fn project_rejected_attempt_receipt(
    plan: &RejectedAttemptContextPlan,
    policy: &RejectedAttemptPolicy,
) -> RejectedAttemptReceipt {
    let disposition = raw_diff_disposition_label(plan.raw_diff_disposition);
    let mut hasher = blake3::Hasher::new();
    for part in [
        RECEIPT_HASH_DOMAIN,
        plan.task_id.as_str(),
        plan.run_id.as_str(),
        plan.attempt_id.as_str(),
        plan.revision_id.as_str(),
        plan.handoff_id.as_str(),
        plan.summary_id.as_str(),
        disposition,
        policy.policy_id.as_str(),
    ] {
        hasher.update(part.as_bytes());
        hasher.update(HASH_SEPARATOR.as_bytes());
    }

    RejectedAttemptReceipt {
        receipt_id: ReceiptIdentity::new(hasher.finalize().to_hex().to_string()),
        task_id: plan.task_id.clone(),
        run_id: plan.run_id.clone(),
        attempt_id: plan.attempt_id.clone(),
        revision_id: plan.revision_id.clone(),
        handoff_id: plan.handoff_id.clone(),
        summary_id: plan.summary_id.clone(),
        raw_diff_disposition: plan.raw_diff_disposition,
        claims: vec![
            CLAIM_REVIEW_BOUND.to_string(),
            CLAIM_CURRENT_ATTEMPT.to_string(),
            CLAIM_BOUNDED_SUMMARY.to_string(),
        ],
        non_claims: policy.required_non_claims.clone(),
    }
}

/// Validates the bounded receipt claim surface.
///
/// # Errors
///
/// Returns an overclaim or missing-non-claim issue.
pub fn validate_rejected_attempt_receipt(
    receipt: &RejectedAttemptReceipt,
    policy: &RejectedAttemptPolicy,
) -> Result<(), Vec<RejectedAttemptIssue>> {
    let mut issues = Vec::new();
    let allowed_claims = [
        CLAIM_REVIEW_BOUND,
        CLAIM_CURRENT_ATTEMPT,
        CLAIM_BOUNDED_SUMMARY,
    ];
    if receipt
        .claims
        .iter()
        .any(|claim| !allowed_claims.contains(&claim.as_str()))
    {
        issues.push(RejectedAttemptIssue::ReceiptOverclaim);
    }
    if policy
        .required_non_claims
        .iter()
        .any(|required| !receipt.non_claims.contains(required))
    {
        issues.push(RejectedAttemptIssue::MissingNonClaim);
    }
    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

fn validate_identities(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
    issues: &mut Vec<RejectedAttemptIssue>,
) {
    let identities = [
        request.task_id.as_str(),
        request.run_id.as_str(),
        request.attempt_id.as_str(),
        request.revision_id.as_str(),
        request.workspace_snapshot_id.as_str(),
        request.content_id.as_str(),
        request.review.handoff_id.as_str(),
        request.context.summary_id.as_str(),
        policy.policy_id.as_str(),
    ];
    if identities.iter().any(|value| value.trim().is_empty()) {
        issues.push(RejectedAttemptIssue::MissingIdentity);
    }
}

fn validate_review(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
    issues: &mut Vec<RejectedAttemptIssue>,
) {
    let review = &request.review;
    let review_denied = !review.validated
        || review.disposition != ReviewDisposition::Rejected
        || !policy.accepted_review_profiles.contains(&review.profile);
    if review_denied {
        issues.push(RejectedAttemptIssue::ReviewEvidenceDenied);
    }
    let wrong_binding = review.task_id != request.task_id
        || review.run_id != request.run_id
        || review.attempt_id != request.attempt_id
        || review.revision_id != request.revision_id;
    if wrong_binding {
        issues.push(RejectedAttemptIssue::WrongTaskOrRun);
    }
    if review.matching_attempts != REQUIRED_MATCH_COUNT {
        issues.push(RejectedAttemptIssue::AmbiguousAttempt);
    }
}

fn validate_vcs(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
    issues: &mut Vec<RejectedAttemptIssue>,
) {
    if !policy
        .accepted_vcs_providers
        .contains(&request.vcs.provider)
    {
        issues.push(RejectedAttemptIssue::VcsProviderDenied);
    }
    let stale = !request.vcs.current
        || request.vcs.attempt_id != request.attempt_id
        || request.vcs.revision_id != request.revision_id
        || request.vcs.workspace_snapshot_id != request.workspace_snapshot_id
        || request.vcs.content_id != request.content_id;
    if stale {
        issues.push(RejectedAttemptIssue::StaleAttempt);
    }
}

fn validate_summary(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
    issues: &mut Vec<RejectedAttemptIssue>,
) {
    let summary = request.context.redacted_summary.as_str();
    if !request.context.summary_classification_safe
        || summary.trim().is_empty()
        || UNSAFE_MARKERS.iter().any(|marker| summary.contains(marker))
    {
        issues.push(RejectedAttemptIssue::UnsafeSummary);
    }
    let summary_bound = u64::from(policy.max_summary_bytes);
    let summary_len = u64::try_from(summary.len()).unwrap_or(u64::MAX);
    if summary_len > summary_bound {
        issues.push(RejectedAttemptIssue::OversizedSummary);
    }
}

fn classify_raw_diff(
    request: &RejectedAttemptRequest,
    policy: &RejectedAttemptPolicy,
) -> RawDiffDisposition {
    let Some(raw_diff) = request.context.raw_diff.as_ref() else {
        return RawDiffDisposition::NotSupplied;
    };
    if !policy.allow_raw_diff || !request.context.raw_diff_capability_admitted {
        return RawDiffDisposition::DeniedByPolicy;
    }
    if !request.context.raw_diff_classification_safe {
        return RawDiffDisposition::DeniedByClassification;
    }
    let diff_bound = u64::from(policy.max_raw_diff_bytes);
    let diff_len = u64::try_from(raw_diff.len()).unwrap_or(u64::MAX);
    if diff_len > diff_bound {
        return RawDiffDisposition::DeniedByBudget;
    }
    RawDiffDisposition::Admitted
}

fn raw_diff_disposition_label(disposition: RawDiffDisposition) -> &'static str {
    match disposition {
        RawDiffDisposition::DeniedByPolicy => "denied_by_policy",
        RawDiffDisposition::DeniedByClassification => "denied_by_classification",
        RawDiffDisposition::DeniedByBudget => "denied_by_budget",
        RawDiffDisposition::Admitted => "admitted",
        RawDiffDisposition::NotSupplied => "not_supplied",
    }
}
