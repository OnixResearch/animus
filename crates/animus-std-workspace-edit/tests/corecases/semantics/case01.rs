#[test]
// r[verify agent_harness_runtime.hashed_edit_host.snapshot.valid]
// r[verify agent_harness_runtime.hashed_edit_host.visibility.allowed]
fn visible_replace_delete_and_insertions_preserve_exact_text_shape() -> Result<(), String> {
    let source = b"alpha\nbeta\ngamma\n";
    let snapshot = admitted_for(SourceCase {
        source,
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: false,
    })?;
    let replacement = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Replace {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
            replacement: "BETA\nBETA-2",
        }],
    )
    .map_err(|error| format!("visible replacement: {error:?}"))?;
    assert_eq!(replacement.after_bytes(), b"alpha\nBETA\nBETA-2\ngamma\n");

    let deletion = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Delete {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
        }],
    )
    .map_err(|error| format!("visible deletion: {error:?}"))?;
    assert_eq!(deletion.after_bytes(), b"alpha\ngamma\n");

    let before = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Insert {
            line_tag: TAG_SECOND,
            side: animus_std_workspace_edit::BoundarySide::Before,
            text: "before",
        }],
    )
    .map_err(|error| format!("before insertion: {error:?}"))?;
    assert_eq!(before.after_bytes(), b"alpha\nbefore\nbeta\ngamma\n");

    let after = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Insert {
            line_tag: TAG_SECOND,
            side: animus_std_workspace_edit::BoundarySide::After,
            text: "after",
        }],
    )
    .map_err(|error| format!("after insertion: {error:?}"))?;
    assert_eq!(after.after_bytes(), b"alpha\nbeta\nafter\ngamma\n");

    let end = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::InsertEnd { text: "omega" }],
    )
    .map_err(|error| format!("end insertion: {error:?}"))?;
    assert_eq!(end.after_bytes(), b"alpha\nbeta\ngamma\nomega\n");
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &snapshot,
            &[animus_std_workspace_edit::EditOperationDraft::InsertEnd { text: "omega" }]
        ),
        Ok(end)
    );
    Ok(())
}
