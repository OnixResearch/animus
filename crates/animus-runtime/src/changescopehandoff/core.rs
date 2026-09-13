use super::binding::validate_change_scope;
use super::data::{
    ChangeContinuationPlan, ChangeHandoffArtifact, ChangeHandoffArtifactDraft,
    ChangeHandoffCompletion, ChangeHandoffCompletionBlock, ChangeHandoffDecisionDraft,
    ChangeHandoffDecisionKind, ChangeHandoffError, ChangeHandoffIssue, ChangeHandoffState,
    ChangeHandoffStatus, ChangeScope, ChangeScopeDraft, ChangeScopeHunkDraft,
    FORBIDDEN_HANDOFF_CLAIM_LABELS, RecordedChangeHandoffDecision, artifact_declared_byte_bound,
    scope_artifact_bound,
};

const DECISION_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.decision.v1";

/// Opens one explicit ownership handoff for a validated scope.
#[must_use]
pub fn open_change_handoff(scope: ChangeScope) -> ChangeHandoffState {
    ChangeHandoffState {
        scope,
        decision: None,
        status: ChangeHandoffStatus::PendingDecision,
    }
}

// r[impl animus.change_scope_handoff.handoff]
/// Records exactly one immutable developer ownership decision.
///
/// # Errors
///
/// Returns a typed issue when the decision references a different scope,
/// duplicates a recorded decision, or carries a malformed approved subset.
pub fn record_change_handoff_decision(
    state: &ChangeHandoffState,
    draft: ChangeHandoffDecisionDraft<'_>,
) -> Result<ChangeHandoffState, ChangeHandoffError> {
    if state.decision.is_some() {
        return Err(error(ChangeHandoffIssue::DecisionAlreadyRecorded));
    }
    if draft.scope_identity != state.scope.scope_identity {
        return Err(error(ChangeHandoffIssue::ScopeIdentityMismatch));
    }
    let approved = validate_approved_subset(draft, state.scope.hunks.len())?;
    let decision = RecordedChangeHandoffDecision {
        decision_identity: decision_identity(&state.scope.scope_identity, draft),
        scope_identity: String::from(draft.scope_identity),
        kind: draft.kind,
        approved_hunks: approved,
    };
    debug_assert!(is_blake3_identity(decision.decision_identity.as_str()));
    Ok(ChangeHandoffState {
        scope: state.scope.clone(),
        decision: Some(decision),
        status: ChangeHandoffStatus::DecisionRecorded,
    })
}

fn validate_approved_subset(
    draft: ChangeHandoffDecisionDraft<'_>,
    hunk_count: usize,
) -> Result<Vec<usize>, ChangeHandoffError> {
    match draft.kind {
        ChangeHandoffDecisionKind::Rejected | ChangeHandoffDecisionKind::RefineRequested => {
            if draft.approved_hunks.is_empty() {
                Ok(Vec::new())
            } else {
                Err(error(ChangeHandoffIssue::InvalidApprovedSubset))
            }
        }
        ChangeHandoffDecisionKind::Approved => {
            if draft.approved_hunks.is_empty() {
                return Err(error(ChangeHandoffIssue::InvalidApprovedSubset));
            }
            let mut prior: Option<usize> = None;
            for index in draft.approved_hunks {
                if *index >= hunk_count {
                    return Err(error(ChangeHandoffIssue::InvalidApprovedSubset));
                }
                if prior.is_some_and(|seen| *index <= seen) {
                    return Err(error(ChangeHandoffIssue::InvalidApprovedSubset));
                }
                prior = Some(*index);
            }
            Ok(draft.approved_hunks.to_vec())
        }
    }
}

// r[impl animus.change_scope_handoff.handoff]
/// Evaluates completion and cites the recorded acceptance when present.
///
/// # Errors
///
/// Returns the explicit blocker class when no acceptance is recorded.
pub fn evaluate_change_handoff_completion(
    state: &ChangeHandoffState,
) -> Result<ChangeHandoffCompletion, ChangeHandoffCompletionBlock> {
    let Some(decision) = state.decision.as_ref() else {
        return Err(ChangeHandoffCompletionBlock::DecisionMissing);
    };
    if decision.kind != ChangeHandoffDecisionKind::Approved {
        return Err(ChangeHandoffCompletionBlock::DecisionNotAcceptance);
    }
    Ok(ChangeHandoffCompletion {
        scope_identity: state.scope.scope_identity.clone(),
        decision_identity: decision.decision_identity.clone(),
        approved_hunks: decision.approved_hunks.clone(),
    })
}

// r[impl animus.change_scope_handoff.continuation]
/// Plans subset-bounded continuation from the recorded decision.
///
/// # Errors
///
/// Returns a typed issue when no decision is recorded yet.
pub fn plan_change_continuation(
    state: &ChangeHandoffState,
) -> Result<ChangeContinuationPlan, ChangeHandoffError> {
    let Some(decision) = state.decision.as_ref() else {
        return Err(error(ChangeHandoffIssue::DecisionMissing));
    };
    let rejected_indexes = rejected_hunk_indexes(state, decision);
    let fresh_scope = if rejected_indexes.is_empty() {
        None
    } else {
        Some(fresh_scope_from_rejected(
            state,
            rejected_indexes.as_slice(),
        )?)
    };
    Ok(ChangeContinuationPlan {
        scope_identity: state.scope.scope_identity.clone(),
        decision_identity: decision.decision_identity.clone(),
        approved_hunks: decision.approved_hunks.clone(),
        fresh_scope,
    })
}

fn rejected_hunk_indexes(
    state: &ChangeHandoffState,
    decision: &RecordedChangeHandoffDecision,
) -> Vec<usize> {
    (0..state.scope.hunks.len())
        .filter(|index| !decision.approved_hunks.contains(index))
        .collect()
}

fn fresh_scope_from_rejected(
    state: &ChangeHandoffState,
    rejected: &[usize],
) -> Result<ChangeScope, ChangeHandoffError> {
    let hunks: Vec<ChangeScopeHunkDraft<'_>> = rejected
        .iter()
        .filter_map(|index| state.scope.hunks.get(*index))
        .map(|hunk| ChangeScopeHunkDraft {
            file_path: hunk.file_path.as_str(),
            hunk_identity: hunk.hunk_identity.as_str(),
            old_start: hunk.old_start,
            old_lines: hunk.old_lines,
            new_start: hunk.new_start,
            new_lines: hunk.new_lines,
        })
        .collect();
    let declared: Vec<&str> = state
        .scope
        .files
        .iter()
        .map(String::as_str)
        .filter(|file| hunks.iter().any(|hunk| hunk.file_path == *file))
        .collect();
    let obligations: Vec<&str> = state.scope.obligations.iter().map(String::as_str).collect();
    validate_change_scope(ChangeScopeDraft {
        driving_task: state.scope.driving_task.as_str(),
        files: declared.as_slice(),
        hunks: hunks.as_slice(),
        obligations: obligations.as_slice(),
    })
}

// r[impl animus.change_scope_handoff.artifacts]
/// Validates bounded review-artifact candidates before persistence.
///
/// # Errors
///
/// Returns a typed issue when a candidate declares raw transcript, tool-output,
/// or prompt payload, or exceeds the named count or byte bounds.
pub fn validate_change_handoff_artifacts(
    drafts: &[ChangeHandoffArtifactDraft<'_>],
) -> Result<Vec<ChangeHandoffArtifact>, ChangeHandoffError> {
    if drafts.len() > scope_artifact_bound() {
        return Err(error(ChangeHandoffIssue::ArtifactBoundExceeded));
    }
    let mut artifacts = Vec::with_capacity(drafts.len());
    for draft in drafts {
        if !draft.kind.is_bounded() {
            return Err(error(ChangeHandoffIssue::RawArtifactKind));
        }
        validate_identity(draft.artifact_identity)?;
        if draft.declared_byte_count > artifact_declared_byte_bound() {
            return Err(error(ChangeHandoffIssue::ArtifactBoundExceeded));
        }
        artifacts.push(ChangeHandoffArtifact {
            kind: draft.kind,
            artifact_identity: String::from(draft.artifact_identity),
            declared_byte_count: draft.declared_byte_count,
        });
    }
    Ok(artifacts)
}

// r[impl animus.change_scope_handoff.nonclaims]
/// Rejects proposed evidence labels that carry forbidden ownership claims.
///
/// # Errors
///
/// Returns a typed issue when any label claims change correctness, VCS
/// completion, policy enforcement, tool correctness, production readiness, or
/// release eligibility.
pub fn validate_change_handoff_claims(claims: &[&str]) -> Result<(), ChangeHandoffError> {
    for claim in claims {
        if FORBIDDEN_HANDOFF_CLAIM_LABELS.contains(claim) {
            return Err(error(ChangeHandoffIssue::NonClaimViolation));
        }
    }
    Ok(())
}

pub(super) fn validate_identity(value: &str) -> Result<(), ChangeHandoffError> {
    if is_blake3_identity(value) {
        Ok(())
    } else {
        Err(error(ChangeHandoffIssue::InvalidIdentity))
    }
}

pub(super) const fn error(issue: ChangeHandoffIssue) -> ChangeHandoffError {
    ChangeHandoffError::new(issue)
}

pub(super) fn is_blake3_identity(value: &str) -> bool {
    const BLAKE3_HEX_CHARS: usize = 64;
    value.len() == BLAKE3_HEX_CHARS
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn decision_identity(scope_identity: &str, draft: ChangeHandoffDecisionDraft<'_>) -> String {
    let index_labels: Vec<String> = draft
        .approved_hunks
        .iter()
        .map(ToString::to_string)
        .collect();
    let mut fields = vec![
        super::data::CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY.as_bytes(),
        scope_identity.as_bytes(),
        draft.kind.as_str().as_bytes(),
    ];
    for label in &index_labels {
        fields.push(label.as_bytes());
    }
    hash_fields(DECISION_IDENTITY_DOMAIN, fields.as_slice())
}

pub(super) fn hash_fields(domain: &str, fields: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_field(&mut hasher, domain.as_bytes());
    for field in fields {
        hash_field(&mut hasher, field);
    }
    hasher.finalize().to_hex().to_string()
}

fn hash_field(hasher: &mut blake3::Hasher, field: &[u8]) {
    let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
    hasher.update(&length.to_le_bytes());
    hasher.update(field);
}
