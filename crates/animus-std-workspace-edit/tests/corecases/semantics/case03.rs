fn partially_visible_snapshot(
    source: &[u8],
) -> Result<animus_std_workspace_edit::AdmittedSnapshot, String> {
    assert!(!source.is_empty());
    let base = base_snapshot(BaseCase {
        source,
        normalization: "lf",
        resource: RESOURCE_ID,
    })?;
    let hidden = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_FIRST,
    }];
    assert_eq!(hidden.len(), 1);
    animus_std_workspace_edit::admit_snapshot(&animus_std_workspace_edit::SnapshotDraft {
        base: &base,
        root_identity: ROOT_ID,
        resource_identity: RESOURCE_ID,
        revision_identity: REVISION_ID,
        metadata_policy_identity: POLICY_ID,
        file_metadata_identity: METADATA_ID,
        writer_generation: WRITER_GENERATION,
        source_bytes: source,
        line_ending_mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        replacement_ending: animus_std_workspace_edit::ReplacementEnding::Lf,
        mixed_endings_admitted: false,
        visible_spans: &hidden,
        visible_boundaries: &[],
        end_boundary_visible: false,
    })
    .map_err(|error| format!("partially visible snapshot: {error:?}"))
}

#[test]
// r[verify agent_harness_runtime.hashed_edit_host.snapshot.invalid]
// r[verify agent_harness_runtime.hashed_edit_host.visibility.denied]
fn malformed_hidden_stale_and_overlapping_inputs_fail_closed() -> Result<(), String> {
    let source = b"alpha\nbeta\ngamma\n";
    let snapshot = partially_visible_snapshot(source)?;
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &snapshot,
            &[animus_std_workspace_edit::EditOperationDraft::Replace {
                start_tag: TAG_SECOND,
                end_tag: TAG_SECOND,
                replacement: "hidden",
            }],
        ),
        Err(animus_std_workspace_edit::HostIssue::HiddenSpan)
    );
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &snapshot,
            &[animus_std_workspace_edit::EditOperationDraft::InsertEnd { text: "hidden" }],
        ),
        Err(animus_std_workspace_edit::HostIssue::HiddenBoundary)
    );
    let full = admitted_for(SourceCase {
        source,
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: false,
    })?;
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &full,
            &[
                animus_std_workspace_edit::EditOperationDraft::Replace {
                    start_tag: TAG_FIRST,
                    end_tag: TAG_SECOND,
                    replacement: "first",
                },
                animus_std_workspace_edit::EditOperationDraft::Insert {
                    line_tag: TAG_SECOND,
                    side: animus_std_workspace_edit::BoundarySide::After,
                    text: "conflict",
                },
            ],
        ),
        Err(animus_std_workspace_edit::HostIssue::OverlappingOperation)
    );
    assert_eq!(
        animus_std_workspace_edit::plan_edit(
            &full,
            &[animus_std_workspace_edit::EditOperationDraft::Replace {
                start_tag: TAG_FIRST,
                end_tag: TAG_FIRST,
                replacement: "bad\rtext",
            }],
        ),
        Err(animus_std_workspace_edit::HostIssue::InvalidReplacement)
    );
    Ok(())
}
