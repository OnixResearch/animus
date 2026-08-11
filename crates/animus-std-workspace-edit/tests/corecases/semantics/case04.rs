fn assert_snapshot_budgets() -> Result<(), String> {
    let base = base_snapshot(BaseCase {
        source: b"alpha\nbeta\ngamma\n",
        normalization: "lf",
        resource: RESOURCE_ID,
    })?;
    let oversized_source_bytes =
        vec![b'a'; animus_std_workspace_edit::MAX_SOURCE_BYTES.saturating_add(1)];
    let full_span = animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    };
    let mut draft = animus_std_workspace_edit::SnapshotDraft {
        base: &base,
        root_identity: ROOT_ID,
        resource_identity: RESOURCE_ID,
        revision_identity: REVISION_ID,
        metadata_policy_identity: POLICY_ID,
        file_metadata_identity: METADATA_ID,
        writer_generation: WRITER_GENERATION,
        source_bytes: &oversized_source_bytes,
        line_ending_mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Absent,
        replacement_ending: animus_std_workspace_edit::ReplacementEnding::Lf,
        mixed_endings_admitted: false,
        visible_spans: &[full_span],
        visible_boundaries: &[],
        end_boundary_visible: true,
    };
    assert_eq!(
        animus_std_workspace_edit::admit_snapshot(&draft),
        Err(animus_std_workspace_edit::HostIssue::SourceTooLarge)
    );
    let visibility_overflow_fixture =
        vec![full_span; animus_std_workspace_edit::MAX_VISIBILITY_ITEMS.saturating_add(1)];
    draft.source_bytes = b"alpha\nbeta\ngamma\n";
    draft.final_newline = animus_std_workspace_edit::FinalNewlineState::Present;
    draft.visible_spans = &visibility_overflow_fixture;
    assert_eq!(
        animus_std_workspace_edit::admit_snapshot(&draft),
        Err(animus_std_workspace_edit::HostIssue::InvalidVisibility)
    );
    Ok(())
}

fn assert_operation_budgets() -> Result<(), String> {
    let admitted = admitted_for(SourceCase {
        source: b"alpha\nbeta\ngamma\n",
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: false,
    })?;
    let oversized_replacement_bytes =
        "x".repeat(animus_std_workspace_edit::MAX_REPLACEMENT_BYTES.saturating_add(1));
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &admitted,
            &[animus_std_workspace_edit::EditOperationDraft::Replace {
                start_tag: TAG_SECOND,
                end_tag: TAG_SECOND,
                replacement: &oversized_replacement_bytes,
            }],
        ),
        Err(animus_std_workspace_edit::HostIssue::InvalidReplacement)
    );
    let too_many_operations = vec![
        animus_std_workspace_edit::EditOperationDraft::InsertEnd { text: "x" };
        animus_std_workspace_edit::MAX_EDIT_OPERATIONS.saturating_add(1)
    ];
    assert_eq!(
        animus_std_workspace_edit::plan_edit(&admitted, &too_many_operations),
        Err(animus_std_workspace_edit::HostIssue::InvalidOperationCount)
    );
    Ok(())
}

#[test]
fn every_collection_and_byte_budget_fails_at_its_boundary() -> Result<(), String> {
    assert_snapshot_budgets()?;
    assert_operation_budgets()?;
    Ok(())
}
