#[test]
fn mixed_line_endings_require_explicit_policy() -> Result<(), String> {
    let mixed = b"alpha\r\nbeta\ngamma\r\n";
    let mixed_base = base_snapshot(BaseCase {
        source: mixed,
        normalization: "mixed_normalized_lf",
        resource: RESOURCE_ID,
    })?;
    assert_eq!(mixed_base.line_count(), LINE_COUNT);
    assert_eq!(
        animus_std_workspace_edit::admit_snapshot(&animus_std_workspace_edit::SnapshotDraft {
            base: &mixed_base,
            root_identity: ROOT_ID,
            resource_identity: RESOURCE_ID,
            revision_identity: REVISION_ID,
            metadata_policy_identity: POLICY_ID,
            file_metadata_identity: METADATA_ID,
            writer_generation: WRITER_GENERATION,
            source_bytes: mixed,
            line_ending_mode: animus_std_workspace_edit::LineEndingMode::Mixed,
            final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
            replacement_ending: animus_std_workspace_edit::ReplacementEnding::Lf,
            mixed_endings_admitted: false,
            visible_spans: &[animus_std_workspace_edit::VisibleSpanDraft {
                start_tag: TAG_FIRST,
                end_tag: TAG_THIRD,
            }],
            visible_boundaries: &[],
            end_boundary_visible: true,
        }),
        Err(animus_std_workspace_edit::HostIssue::MixedLineEndingsDenied)
    );
    Ok(())
}
