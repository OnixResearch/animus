const RECEIPT_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.receipt.v1";
const ABSENT_DECISION_LABEL: &str = "none";

// r[impl animus.change_scope_handoff.handoff]
// r[impl animus.change_scope_handoff.artifacts]
/// Projects one terminal metadata-only change-scope handoff receipt.
///
/// # Errors
///
/// Returns a typed issue for malformed artifacts, conflicting decisions, or unsafe receipt fields.
pub fn project_change_handoff_receipt(
    state: &super::data::ChangeHandoffState,
    artifacts: &[super::data::ChangeHandoffArtifact],
) -> Result<super::data::ChangeHandoffReceipt, super::data::ChangeHandoffError> {
    for artifact in artifacts {
        if !artifact.kind.is_bounded()
            || !super::digest::is_blake3_identity(artifact.artifact_identity())
        {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::RawArtifactKind,
            ));
        }
    }
    let decision = state.decision.as_ref();
    let status = if decision.is_some() {
        super::data::ChangeHandoffStatus::DecisionRecorded
    } else {
        super::data::ChangeHandoffStatus::PendingDecision
    };
    let completion = decision_completion(state)?;
    let hunk_count = checked_count(state.scope.hunks.len())?;
    let approved_hunk_count =
        checked_count(decision.map_or(0, |value| value.approved_hunks.len()))?;
    if !is_safe_label(state.scope.driving_task.as_str()) {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ReceiptFieldMismatch,
        ));
    }
    debug_assert!(approved_hunk_count <= hunk_count);
    debug_assert_eq!(status, state.status);
    let mut receipt = super::data::ChangeHandoffReceipt {
        schema: super::data::CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA,
        contract_identity: super::data::CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY,
        scope_identity: state.scope.scope_identity.clone(),
        driving_task: state.scope.driving_task.clone(),
        status,
        decision_identity: decision.map(|recorded| recorded.decision_identity.clone()),
        decision_kind: decision.map(super::data::RecordedChangeHandoffDecision::kind),
        approved_hunk_count,
        hunk_count,
        artifact_identities: artifacts
            .iter()
            .map(|artifact| artifact.artifact_identity.clone())
            .collect(),
        completion_decision_identity: completion.map(|completed| completed.decision_identity),
        redaction_marker: super::data::CHANGE_SCOPE_HANDOFF_REDACTION_MARKER,
        non_claims: super::data::CHANGE_SCOPE_HANDOFF_NON_CLAIMS.to_vec(),
        receipt_identity: String::new(),
    };
    receipt.receipt_identity = receipt_identity(&receipt)?;
    validate_change_handoff_receipt(&receipt)?;
    Ok(receipt)
}

fn checked_count(count: usize) -> Result<u32, super::data::ChangeHandoffError> {
    u32::try_from(count).map_err(|_overflow| {
        super::digest::error(super::data::ChangeHandoffIssue::ReceiptFieldMismatch)
    })
}

fn decision_completion(
    state: &super::data::ChangeHandoffState,
) -> Result<Option<super::data::ChangeHandoffCompletion>, super::data::ChangeHandoffError> {
    let completion = super::core::evaluate_change_handoff_completion(state).ok();
    let is_accepted = matches!(
        state
            .decision
            .as_ref()
            .map(super::data::RecordedChangeHandoffDecision::kind),
        Some(super::data::ChangeHandoffDecisionKind::Approved)
    );
    if completion.is_some() != is_accepted {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ReceiptFieldMismatch,
        ));
    }
    Ok(completion)
}

/// Validates one metadata-only change-scope handoff receipt.
///
/// # Errors
///
/// Returns a typed issue for malformed identities, changed non-claims, unsafe labels,
/// impossible status combinations, or receipt identity drift.
pub fn validate_change_handoff_receipt(
    receipt: &super::data::ChangeHandoffReceipt,
) -> Result<(), super::data::ChangeHandoffError> {
    let has_valid_scalar_fields = receipt.schema
        == super::data::CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA
        && receipt.contract_identity == super::data::CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY
        && is_safe_label(receipt.driving_task.as_str())
        && receipt.redaction_marker == super::data::CHANGE_SCOPE_HANDOFF_REDACTION_MARKER
        && receipt.non_claims == super::data::CHANGE_SCOPE_HANDOFF_NON_CLAIMS;
    if !has_valid_scalar_fields
        || !super::digest::is_blake3_identity(receipt.scope_identity.as_str())
    {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ReceiptFieldMismatch,
        ));
    }
    for identity in receipt
        .decision_identity
        .iter()
        .chain(receipt.completion_decision_identity.iter())
        .chain(receipt.artifact_identities.iter())
        .chain(::core::iter::once(&receipt.receipt_identity))
        .map(String::as_str)
    {
        if !super::digest::is_blake3_identity(identity) {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::ReceiptFieldMismatch,
            ));
        }
    }
    let has_valid_status_fields = match receipt.status {
        super::data::ChangeHandoffStatus::PendingDecision => {
            receipt.decision_identity.is_none()
                && receipt.decision_kind.is_none()
                && receipt.completion_decision_identity.is_none()
                && receipt.approved_hunk_count == 0
        }
        super::data::ChangeHandoffStatus::DecisionRecorded => receipt.decision_identity.is_some(),
    };
    if !has_valid_status_fields || receipt.receipt_identity != receipt_identity(receipt)? {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::ReceiptFieldMismatch,
        ));
    }
    debug_assert_eq!(
        receipt.schema,
        super::data::CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA
    );
    debug_assert!(super::digest::is_blake3_identity(
        receipt.receipt_identity()
    ));
    Ok(())
}

fn receipt_identity(
    receipt: &super::data::ChangeHandoffReceipt,
) -> Result<String, super::data::ChangeHandoffError> {
    debug_assert_eq!(
        receipt.non_claims,
        super::data::CHANGE_SCOPE_HANDOFF_NON_CLAIMS
    );
    debug_assert!(super::digest::is_blake3_identity(receipt.scope_identity()));
    let decision_identity = match receipt.decision_identity.as_deref() {
        Some(identity) => identity,
        None => ABSENT_DECISION_LABEL,
    };
    let decision_kind = match receipt.decision_kind {
        Some(kind) => kind.as_str(),
        None => ABSENT_DECISION_LABEL,
    };
    let completion_identity = match receipt.completion_decision_identity.as_deref() {
        Some(identity) => identity,
        None => ABSENT_DECISION_LABEL,
    };
    let approved_hunk_count = receipt.approved_hunk_count.to_string();
    let hunk_count = receipt.hunk_count.to_string();
    let mut fields = vec![
        receipt.schema.as_bytes(),
        receipt.contract_identity.as_bytes(),
        receipt.scope_identity.as_bytes(),
        receipt.driving_task.as_bytes(),
        receipt.status.as_str().as_bytes(),
        decision_identity.as_bytes(),
        decision_kind.as_bytes(),
        approved_hunk_count.as_bytes(),
        hunk_count.as_bytes(),
    ];
    for identity in &receipt.artifact_identities {
        fields.push(identity.as_bytes());
    }
    fields.push(completion_identity.as_bytes());
    fields.push(receipt.redaction_marker.as_bytes());
    for claim in &receipt.non_claims {
        fields.push(claim.as_bytes());
    }
    super::digest::hash_fields(RECEIPT_IDENTITY_DOMAIN, fields.as_slice())
}

fn is_safe_label(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && super::digest::is_within_bound(
            trimmed.chars().count(),
            super::data::scope_label_char_bound(),
        )
}
