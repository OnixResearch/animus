const DECISION_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.decision.v1";

/// Opens one explicit ownership handoff for a validated scope.
#[must_use]
pub fn open_change_handoff(scope: super::data::ChangeScope) -> super::data::ChangeHandoffState {
    super::data::ChangeHandoffState {
        scope,
        decision: None,
        status: super::data::ChangeHandoffStatus::PendingDecision,
    }
}

// r[impl animus.change_scope_handoff.handoff]
/// Records exactly one immutable developer ownership decision.
///
/// # Errors
///
/// Returns a typed issue for a crossed scope, repeated decision, or malformed subset.
pub fn record_change_handoff_decision(
    state: &super::data::ChangeHandoffState,
    draft: super::data::ChangeHandoffDecisionDraft<'_>,
) -> Result<super::data::ChangeHandoffState, super::data::ChangeHandoffError> {
    if state.decision.is_some() {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::DecisionAlreadyRecorded,
        ));
    }
    if draft.scope_identity != state.scope.scope_identity {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ScopeIdentityMismatch,
        ));
    }
    let approved = validate_approved_subset(draft, state.scope.hunks.len())?;
    debug_assert_eq!(
        state.status,
        super::data::ChangeHandoffStatus::PendingDecision
    );
    let decision = super::data::RecordedChangeHandoffDecision {
        decision_identity: decision_identity(&state.scope.scope_identity, draft)?,
        scope_identity: String::from(draft.scope_identity),
        kind: draft.kind,
        approved_hunks: approved,
    };
    debug_assert!(super::digest::is_blake3_identity(
        decision.decision_identity.as_str()
    ));
    Ok(super::data::ChangeHandoffState {
        scope: state.scope.clone(),
        decision: Some(decision),
        status: super::data::ChangeHandoffStatus::DecisionRecorded,
    })
}

fn validate_approved_subset(
    draft: super::data::ChangeHandoffDecisionDraft<'_>,
    hunk_count: usize,
) -> Result<Vec<u32>, super::data::ChangeHandoffError> {
    let hunk_count = u32::try_from(hunk_count).map_err(|_overflow| {
        super::digest::error(super::data::ChangeHandoffIssue::InvalidApprovedSubset)
    })?;
    debug_assert!(hunk_count > 0);
    match draft.kind {
        super::data::ChangeHandoffDecisionKind::Rejected
        | super::data::ChangeHandoffDecisionKind::RefineRequested => {
            if draft.approved_hunks.is_empty() {
                Ok(Vec::new())
            } else {
                Err(super::digest::error(
                    super::data::ChangeHandoffIssue::InvalidApprovedSubset,
                ))
            }
        }
        super::data::ChangeHandoffDecisionKind::Approved => {
            if draft.approved_hunks.is_empty() {
                return Err(super::digest::error(
                    super::data::ChangeHandoffIssue::InvalidApprovedSubset,
                ));
            }
            let mut prior: Option<u32> = None;
            for index in draft.approved_hunks {
                if *index >= hunk_count || prior.is_some_and(|seen| *index <= seen) {
                    return Err(super::digest::error(
                        super::data::ChangeHandoffIssue::InvalidApprovedSubset,
                    ));
                }
                prior = Some(*index);
            }
            debug_assert!(super::digest::is_within_bound(
                draft.approved_hunks.len(),
                hunk_count
            ));
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
    state: &super::data::ChangeHandoffState,
) -> Result<super::data::ChangeHandoffCompletion, super::data::ChangeHandoffCompletionBlock> {
    let Some(decision) = state.decision.as_ref() else {
        return Err(super::data::ChangeHandoffCompletionBlock::DecisionMissing);
    };
    if decision.kind != super::data::ChangeHandoffDecisionKind::Approved {
        return Err(super::data::ChangeHandoffCompletionBlock::DecisionNotAcceptance);
    }
    Ok(super::data::ChangeHandoffCompletion {
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
    state: &super::data::ChangeHandoffState,
) -> Result<super::data::ChangeContinuationPlan, super::data::ChangeHandoffError> {
    let Some(decision) = state.decision.as_ref() else {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::DecisionMissing,
        ));
    };
    debug_assert_eq!(decision.scope_identity, state.scope.scope_identity);
    debug_assert!(decision.approved_hunks.len() <= state.scope.hunks.len());
    let rejected = rejected_hunks(state, decision)?;
    let fresh_scope = if rejected.is_empty() {
        None
    } else {
        Some(fresh_scope_from_rejected(state, rejected.as_slice())?)
    };
    Ok(super::data::ChangeContinuationPlan {
        scope_identity: state.scope.scope_identity.clone(),
        decision_identity: decision.decision_identity.clone(),
        approved_hunks: decision.approved_hunks.clone(),
        fresh_scope,
    })
}

fn rejected_hunks<'scope>(
    state: &'scope super::data::ChangeHandoffState,
    decision: &super::data::RecordedChangeHandoffDecision,
) -> Result<Vec<super::data::ChangeScopeHunkDraft<'scope>>, super::data::ChangeHandoffError> {
    let mut rejected = Vec::with_capacity(state.scope.hunks.len());
    for (index, hunk) in state.scope.hunks.iter().enumerate() {
        let index = u32::try_from(index).map_err(|_overflow| {
            super::digest::error(super::data::ChangeHandoffIssue::InvalidApprovedSubset)
        })?;
        if !decision.approved_hunks.contains(&index) {
            rejected.push(super::data::ChangeScopeHunkDraft {
                file_path: hunk.file_path.as_str(),
                hunk_identity: hunk.hunk_identity.as_str(),
                old_start: hunk.old_start,
                old_lines: hunk.old_lines,
                new_start: hunk.new_start,
                new_lines: hunk.new_lines,
            });
        }
    }
    debug_assert!(rejected.len() <= state.scope.hunks.len());
    debug_assert_eq!(
        rejected.len().checked_add(decision.approved_hunks.len()),
        Some(state.scope.hunks.len())
    );
    Ok(rejected)
}

fn fresh_scope_from_rejected(
    state: &super::data::ChangeHandoffState,
    hunks: &[super::data::ChangeScopeHunkDraft<'_>],
) -> Result<super::data::ChangeScope, super::data::ChangeHandoffError> {
    debug_assert!(!hunks.is_empty());
    debug_assert!(hunks.len() <= state.scope.hunks.len());
    let declared: Vec<&str> = state
        .scope
        .files
        .iter()
        .map(String::as_str)
        .filter(|file| hunks.iter().any(|hunk| hunk.file_path == *file))
        .collect();
    let obligations: Vec<&str> = state.scope.obligations.iter().map(String::as_str).collect();
    super::binding::validate_change_scope(super::data::ChangeScopeDraft {
        driving_task: state.scope.driving_task.as_str(),
        files: declared.as_slice(),
        hunks,
        obligations: obligations.as_slice(),
    })
}

// r[impl animus.change_scope_handoff.artifacts]
/// Validates bounded review-artifact candidates before persistence.
///
/// # Errors
///
/// Returns a typed issue for raw payloads, malformed identities, or exceeded bounds.
pub fn validate_change_handoff_artifacts(
    drafts: &[super::data::ChangeHandoffArtifactDraft<'_>],
) -> Result<Vec<super::data::ChangeHandoffArtifact>, super::data::ChangeHandoffError> {
    if !super::digest::is_within_bound(drafts.len(), super::data::scope_artifact_bound()) {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ArtifactBoundExceeded,
        ));
    }
    let mut artifacts = Vec::with_capacity(drafts.len());
    for draft in drafts {
        if !draft.kind.is_bounded() {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::RawArtifactKind,
            ));
        }
        super::digest::validate_identity(draft.artifact_identity)?;
        if draft.declared_byte_count > super::data::artifact_declared_byte_bound() {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::ArtifactBoundExceeded,
            ));
        }
        artifacts.push(super::data::ChangeHandoffArtifact {
            kind: draft.kind,
            artifact_identity: String::from(draft.artifact_identity),
            declared_byte_count: draft.declared_byte_count,
        });
    }
    debug_assert_eq!(artifacts.len(), drafts.len());
    debug_assert!(artifacts.iter().all(|artifact| artifact.kind.is_bounded()));
    Ok(artifacts)
}

// r[impl animus.change_scope_handoff.nonclaims]
/// Rejects proposed evidence labels that carry forbidden ownership claims.
///
/// # Errors
///
/// Returns a typed issue for correctness, VCS, policy, production, or release claims.
pub fn validate_change_handoff_claims(
    claims: &[&str],
) -> Result<(), super::data::ChangeHandoffError> {
    for claim in claims {
        if super::data::FORBIDDEN_HANDOFF_CLAIM_LABELS.contains(claim) {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::NonClaimViolation,
            ));
        }
    }
    Ok(())
}

fn decision_identity(
    scope_identity: &str,
    draft: super::data::ChangeHandoffDecisionDraft<'_>,
) -> Result<String, super::data::ChangeHandoffError> {
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
    super::digest::hash_fields(DECISION_IDENTITY_DOMAIN, fields.as_slice())
}
