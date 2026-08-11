fn frame_part(hasher: &mut blake3::Hasher, value: &[u8]) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn expected_released_generation(
    admitted_generation: u64,
    state: crate::model::PublicationState,
) -> Option<u64> {
    match state {
        crate::model::PublicationState::FailedBeforePublication => Some(admitted_generation),
        crate::model::PublicationState::Published
        | crate::model::PublicationState::DurabilityPending
        | crate::model::PublicationState::DurabilityComplete
        | crate::model::PublicationState::UncertainAfterPublication => {
            admitted_generation.checked_add(1)
        }
        crate::model::PublicationState::Prepared | crate::model::PublicationState::Staged => None,
    }
}

fn identity(receipt: &crate::evidence::model::EditReceipt) -> String {
    let mut hasher = blake3::Hasher::new();
    frame_part(&mut hasher, crate::evidence::model::RECEIPT_DOMAIN);
    frame_part(&mut hasher, receipt.root_identity.as_bytes());
    frame_part(&mut hasher, receipt.resource_identity.as_bytes());
    frame_part(&mut hasher, receipt.revision_identity.as_bytes());
    frame_part(&mut hasher, receipt.before_identity.as_bytes());
    frame_part(&mut hasher, receipt.after_identity.as_bytes());
    frame_part(&mut hasher, receipt.snapshot_identity.as_bytes());
    frame_part(&mut hasher, receipt.plan_identity.as_bytes());
    frame_part(&mut hasher, receipt.visibility_identity.as_bytes());
    frame_part(&mut hasher, &receipt.writer_generation.to_le_bytes());
    frame_part(
        &mut hasher,
        &receipt.released_writer_generation.to_le_bytes(),
    );
    frame_part(&mut hasher, receipt.publication_state.as_bytes());
    frame_part(&mut hasher, receipt.durability_state.as_bytes());
    frame_part(&mut hasher, &receipt.operation_count.to_le_bytes());
    frame_part(&mut hasher, &receipt.observation_count.to_le_bytes());
    frame_part(&mut hasher, receipt.redaction.as_bytes());
    for non_claim in receipt.non_claims {
        frame_part(&mut hasher, non_claim.as_bytes());
    }
    let value = hasher.finalize().to_hex().to_string();
    debug_assert!(!value.is_empty());
    value
}

/// Project one deterministic metadata-only edit receipt.
///
/// # Errors
///
/// Rejects private fields, overclaims, and invalid writer generations without
/// echoing the rejected value.
// r[impl agent_harness_runtime.hashed_edit_host.evidence]
pub fn project_receipt(
    draft: crate::evidence::model::ReceiptDraft<'_>,
) -> Result<crate::evidence::model::EditReceipt, crate::evidence::model::ReceiptIssue> {
    if draft.leak_facts.has_private_material() {
        return Err(crate::evidence::model::ReceiptIssue::PrivateMaterial);
    }
    if draft.leak_facts.has_overclaim() {
        return Err(crate::evidence::model::ReceiptIssue::UnsupportedClaim);
    }
    let admitted_generation = draft.plan.snapshot().writer_generation();
    let expected_generation =
        expected_released_generation(admitted_generation, draft.publication.state())
            .ok_or(crate::evidence::model::ReceiptIssue::InvalidWriterGeneration)?;
    if draft.released_writer_generation != expected_generation {
        return Err(crate::evidence::model::ReceiptIssue::InvalidWriterGeneration);
    }
    let mut receipt = crate::evidence::model::EditReceipt {
        schema: crate::evidence::model::RECEIPT_SCHEMA,
        root_identity: String::from(draft.plan.snapshot().root_identity()),
        resource_identity: String::from(draft.plan.snapshot().resource_identity()),
        revision_identity: String::from(draft.plan.snapshot().revision_identity()),
        before_identity: String::from(draft.plan.snapshot().content_identity()),
        after_identity: String::from(draft.plan.after_identity()),
        snapshot_identity: String::from(draft.plan.snapshot().identity()),
        plan_identity: String::from(draft.plan.plan_identity()),
        visibility_identity: String::from(draft.plan.snapshot().visibility_identity()),
        writer_generation: admitted_generation,
        released_writer_generation: draft.released_writer_generation,
        publication_state: draft.publication.state().as_str(),
        durability_state: draft.publication.durability().as_str(),
        operation_count: draft.plan.operation_count(),
        observation_count: draft.publication.observation_count(),
        redaction: crate::evidence::model::REDACTION,
        non_claims: crate::evidence::model::NON_CLAIMS,
        identity: String::new(),
    };
    receipt.identity = identity(&receipt);
    debug_assert!(!receipt.identity.is_empty());
    Ok(receipt)
}
