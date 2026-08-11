fn assert_crlf_shape_is_preserved() -> Result<(), String> {
    let source = b"alpha\r\nbeta\r\ngamma\r\n";
    let base = base_snapshot(BaseCase {
        source,
        normalization: "crlf",
        resource: RESOURCE_ID,
    })?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let snapshot =
        animus_std_workspace_edit::admit_snapshot(&animus_std_workspace_edit::SnapshotDraft {
            base: &base,
            root_identity: ROOT_ID,
            resource_identity: RESOURCE_ID,
            revision_identity: REVISION_ID,
            metadata_policy_identity: POLICY_ID,
            file_metadata_identity: METADATA_ID,
            writer_generation: WRITER_GENERATION,
            source_bytes: source,
            line_ending_mode: animus_std_workspace_edit::LineEndingMode::CrLf,
            final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
            replacement_ending: animus_std_workspace_edit::ReplacementEnding::CrLf,
            mixed_endings_admitted: false,
            visible_spans: &spans,
            visible_boundaries: &[],
            end_boundary_visible: true,
        })
        .map_err(|error| format!("crlf snapshot: {error:?}"))?;
    let plan = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Replace {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
            replacement: "BETA\nBETA-2",
        }],
    )
    .map_err(|error| format!("crlf replacement: {error:?}"))?;
    assert_eq!(snapshot.line_ending_mode(), animus_std_workspace_edit::LineEndingMode::CrLf);
    assert_eq!(plan.after_bytes(), b"alpha\r\nBETA\r\nBETA-2\r\ngamma\r\n");
    Ok(())
}

fn assert_mixed_shape_is_preserved() -> Result<(), String> {
    let snapshot = admitted_for(SourceCase {
        source: b"alpha\r\nbeta\ngamma\r\n",
        normalization: "mixed_normalized_lf",
        mode: animus_std_workspace_edit::LineEndingMode::Mixed,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: true,
    })?;
    let plan = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Replace {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
            replacement: "BETA",
        }],
    )
    .map_err(|error| format!("explicit mixed replacement: {error:?}"))?;
    assert_eq!(snapshot.line_ending_mode(), animus_std_workspace_edit::LineEndingMode::Mixed);
    assert_eq!(plan.after_bytes(), b"alpha\r\nBETA\ngamma\r\n");
    Ok(())
}

fn assert_missing_final_newline_is_preserved() -> Result<(), String> {
    let snapshot = admitted_for(SourceCase {
        source: b"alpha\nbeta\ngamma",
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Absent,
        mixed: false,
    })?;
    let plan = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::InsertEnd { text: "omega" }],
    )
    .map_err(|error| format!("end insertion without final newline: {error:?}"))?;
    assert_eq!(snapshot.final_newline(), animus_std_workspace_edit::FinalNewlineState::Absent);
    assert_eq!(plan.after_bytes(), b"alpha\nbeta\ngamma\nomega");
    Ok(())
}

#[test]
fn crlf_mixed_and_missing_final_newline_are_preserved_by_explicit_policy() -> Result<(), String> {
    assert_crlf_shape_is_preserved()?;
    assert_mixed_shape_is_preserved()?;
    assert_missing_final_newline_is_preserved()?;
    Ok(())
}
