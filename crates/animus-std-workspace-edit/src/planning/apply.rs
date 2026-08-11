fn collect_operations(
    snapshot: &crate::model::AdmittedSnapshot,
    drafts: &[crate::model::EditOperationDraft<'_>],
) -> Result<Vec<crate::model::mutation::AppliedOperation>, crate::model::HostIssue> {
    if drafts.is_empty() || drafts.len() > crate::model::MAX_EDIT_OPERATIONS {
        return Err(crate::model::HostIssue::InvalidOperationCount);
    }
    let mut operations = Vec::with_capacity(drafts.len());
    for draft in drafts {
        let operation = match draft {
            crate::model::EditOperationDraft::Replace {
                start_tag,
                end_tag,
                replacement,
            } => crate::planning::operation::span(crate::planning::operation::SpanInput {
                snapshot,
                start_tag,
                end_tag,
                replacement_text: Some(replacement),
            })?,
            crate::model::EditOperationDraft::Delete { start_tag, end_tag } => {
                crate::planning::operation::span(crate::planning::operation::SpanInput {
                    snapshot,
                    start_tag,
                    end_tag,
                    replacement_text: None,
                })?
            }
            crate::model::EditOperationDraft::Insert {
                line_tag,
                side,
                text,
            } => crate::planning::operation::insertion(snapshot, line_tag, *side, text)?,
            crate::model::EditOperationDraft::InsertEnd { text } => {
                crate::planning::operation::end_insertion(snapshot, text)?
            }
        };
        operations.push(operation);
    }
    operations.sort_by_key(|operation| (operation.start, operation.end));
    for pair in operations.windows(crate::planning::common::ADJACENT_OPERATION_WINDOW) {
        let previous = &pair[0];
        let current = &pair[1];
        let is_range_overlap = current.start < previous.end;
        let is_boundary_conflict =
            current.start == previous.end && (current.is_insertion || previous.is_insertion);
        if is_range_overlap || is_boundary_conflict {
            return Err(crate::model::HostIssue::OverlappingOperation);
        }
    }
    debug_assert_eq!(operations.len(), drafts.len());
    Ok(operations)
}

fn render_operations(
    source: &[u8],
    operations: &[crate::model::mutation::AppliedOperation],
) -> Vec<u8> {
    let mut output = source.to_vec();
    for operation in operations.iter().rev() {
        output.splice(
            operation.start..operation.end,
            operation.replacement.iter().copied(),
        );
    }
    output
}

fn validate_after_shape(
    snapshot: &crate::model::AdmittedSnapshot,
    after_bytes: &[u8],
) -> Result<(), crate::model::HostIssue> {
    if after_bytes.len() > crate::model::MAX_SOURCE_BYTES {
        return Err(crate::model::HostIssue::SourceTooLarge);
    }
    let (_, shape) = crate::planning::syntax::parse_lines(after_bytes)?;
    if shape.final_newline != snapshot.final_newline {
        return Err(crate::model::HostIssue::FinalNewlineMismatch);
    }
    Ok(())
}

fn identity(
    snapshot: &crate::model::AdmittedSnapshot,
    operations: &[crate::model::mutation::AppliedOperation],
    after: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    crate::planning::common::frame_part(&mut hasher, crate::planning::common::FRAME_DOMAIN_PLAN);
    crate::planning::common::frame_part(&mut hasher, snapshot.identity.as_bytes());
    crate::planning::common::frame_part(&mut hasher, after.as_bytes());
    for operation in operations {
        crate::planning::common::frame_part(&mut hasher, operation.kind.as_bytes());
        crate::planning::common::frame_part(&mut hasher, &operation.first_line.to_le_bytes());
        crate::planning::common::frame_part(&mut hasher, &operation.last_line.to_le_bytes());
        crate::planning::common::frame_part(&mut hasher, operation.replacement_identity.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

/// Admit visible operations and compute exact after bytes without I/O.
///
/// # Errors
///
/// Returns a bounded issue for hidden, malformed, overlapping, or over-budget
/// operations. No caller can recover after bytes from a failed plan.
// r[impl agent_harness_runtime.hashed_edit_host.visibility]
// r[impl agent_harness_runtime.hashed_edit_host.snapshot]
pub fn plan_edit(
    snapshot: &crate::model::AdmittedSnapshot,
    drafts: &[crate::model::EditOperationDraft<'_>],
) -> Result<crate::model::EditPlan, crate::model::HostIssue> {
    let operations = collect_operations(snapshot, drafts)?;
    let after_bytes = render_operations(&snapshot.source_bytes, &operations);
    validate_after_shape(snapshot, &after_bytes)?;
    let after_identity = crate::planning::common::hash_bytes(&after_bytes);
    let plan_identity = identity(snapshot, &operations, &after_identity);
    let operation_count = u32::try_from(operations.len())
        .map_err(|_| crate::model::HostIssue::InvalidOperationCount)?;
    debug_assert_eq!(
        crate::planning::common::hash_bytes(&after_bytes),
        after_identity
    );
    Ok(crate::model::EditPlan {
        snapshot: snapshot.clone(),
        after_bytes,
        after_identity,
        plan_identity,
        operations,
        operation_count,
    })
}
