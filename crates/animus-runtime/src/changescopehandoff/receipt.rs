use super::core::{error, hash_fields, is_blake3_identity};
use super::data::{
    CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY, CHANGE_SCOPE_HANDOFF_NON_CLAIMS,
    CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA, CHANGE_SCOPE_HANDOFF_REDACTION_MARKER,
    ChangeHandoffArtifact, ChangeHandoffDecisionKind, ChangeHandoffError, ChangeHandoffIssue,
    ChangeHandoffReceipt, ChangeHandoffState, ChangeHandoffStatus, scope_label_char_bound,
};

const RECEIPT_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.receipt.v1";
const NO_APPROVED_HUNKS: &[usize] = &[];

// r[impl animus.change_scope_handoff.handoff]
// r[impl animus.change_scope_handoff.artifacts]
/// Projects one terminal metadata-only change-scope handoff receipt.
///
/// # Errors
///
/// Returns a typed issue when artifact identities are malformed, the status and
/// decision fields disagree, or safe receipt fields are inconsistent.
pub fn project_change_handoff_receipt(
    state: &ChangeHandoffState,
    artifacts: &[ChangeHandoffArtifact],
) -> Result<ChangeHandoffReceipt, ChangeHandoffError> {
    for artifact in artifacts {
        if !artifact.kind.is_bounded() || !is_blake3_identity(artifact.artifact_identity()) {
            return Err(error(ChangeHandoffIssue::RawArtifactKind));
        }
    }
    let decision = state.decision.as_ref();
    let status = if decision.is_some() {
        ChangeHandoffStatus::DecisionRecorded
    } else {
        ChangeHandoffStatus::PendingDecision
    };
    let completion = super::core::evaluate_change_handoff_completion(state).ok();
    if completion.is_some()
        != matches!(
            decision.map(super::data::RecordedChangeHandoffDecision::kind),
            Some(ChangeHandoffDecisionKind::Approved)
        )
    {
        return Err(error(ChangeHandoffIssue::ReceiptFieldMismatch));
    }
    let hunk_count = u32::try_from(state.scope.hunks.len())
        .map_err(|_overflow| error(ChangeHandoffIssue::ReceiptFieldMismatch))?;
    let approved_hunk_count = u32::try_from(
        decision
            .map_or(
                NO_APPROVED_HUNKS,
                super::data::RecordedChangeHandoffDecision::approved_hunks,
            )
            .len(),
    )
    .map_err(|_overflow| error(ChangeHandoffIssue::ReceiptFieldMismatch))?;
    if !safe_label(state.scope.driving_task.as_str()) {
        return Err(error(ChangeHandoffIssue::ReceiptFieldMismatch));
    }
    let mut receipt = ChangeHandoffReceipt {
        schema: CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA,
        contract_identity: CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY,
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
        redaction_marker: CHANGE_SCOPE_HANDOFF_REDACTION_MARKER,
        non_claims: CHANGE_SCOPE_HANDOFF_NON_CLAIMS.to_vec(),
        receipt_identity: String::new(),
    };
    receipt.receipt_identity = receipt_identity(&receipt);
    validate_change_handoff_receipt(&receipt)?;
    Ok(receipt)
}

/// Validates one metadata-only change-scope handoff receipt.
///
/// # Errors
///
/// Returns a typed issue for malformed identities, changed non-claims, unsafe
/// labels, impossible status combinations, or receipt identity drift.
pub fn validate_change_handoff_receipt(
    receipt: &ChangeHandoffReceipt,
) -> Result<(), ChangeHandoffError> {
    let scalar_fields_valid = receipt.schema == CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA
        && receipt.contract_identity == CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY
        && safe_label(receipt.driving_task.as_str())
        && receipt.redaction_marker == CHANGE_SCOPE_HANDOFF_REDACTION_MARKER
        && receipt.non_claims == CHANGE_SCOPE_HANDOFF_NON_CLAIMS;
    if !scalar_fields_valid || !is_blake3_identity(receipt.scope_identity.as_str()) {
        return Err(error(ChangeHandoffIssue::ReceiptFieldMismatch));
    }
    for identity in receipt
        .decision_identity
        .iter()
        .chain(receipt.completion_decision_identity.iter())
        .chain(receipt.artifact_identities.iter())
        .chain(core::iter::once(&receipt.receipt_identity))
        .map(String::as_str)
    {
        if !is_blake3_identity(identity) {
            return Err(error(ChangeHandoffIssue::ReceiptFieldMismatch));
        }
    }
    let status_fields_valid = match receipt.status {
        ChangeHandoffStatus::PendingDecision => {
            receipt.decision_identity.is_none()
                && receipt.decision_kind.is_none()
                && receipt.completion_decision_identity.is_none()
                && receipt.approved_hunk_count == 0
        }
        ChangeHandoffStatus::DecisionRecorded => receipt.decision_identity.is_some(),
    };
    if !status_fields_valid || receipt.receipt_identity != receipt_identity(receipt) {
        return Err(error(ChangeHandoffIssue::ReceiptFieldMismatch));
    }
    Ok(())
}

fn receipt_identity(receipt: &ChangeHandoffReceipt) -> String {
    let status = match receipt.status {
        ChangeHandoffStatus::PendingDecision => "pending_decision",
        ChangeHandoffStatus::DecisionRecorded => "decision_recorded",
    };
    let approved_hunk_count = receipt.approved_hunk_count.to_string();
    let hunk_count = receipt.hunk_count.to_string();
    let mut fields = vec![
        receipt.schema.as_bytes(),
        receipt.contract_identity.as_bytes(),
        receipt.scope_identity.as_bytes(),
        receipt.driving_task.as_bytes(),
        status.as_bytes(),
        receipt
            .decision_identity
            .as_deref()
            .unwrap_or("none")
            .as_bytes(),
        receipt
            .decision_kind
            .map_or("none", ChangeHandoffDecisionKind::as_str)
            .as_bytes(),
        approved_hunk_count.as_bytes(),
        hunk_count.as_bytes(),
    ];
    for identity in &receipt.artifact_identities {
        fields.push(identity.as_bytes());
    }
    fields.push(
        receipt
            .completion_decision_identity
            .as_deref()
            .unwrap_or("none")
            .as_bytes(),
    );
    fields.push(receipt.redaction_marker.as_bytes());
    for claim in &receipt.non_claims {
        fields.push(claim.as_bytes());
    }
    hash_fields(RECEIPT_IDENTITY_DOMAIN, fields.as_slice())
}

fn safe_label(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().count() <= scope_label_char_bound()
}
