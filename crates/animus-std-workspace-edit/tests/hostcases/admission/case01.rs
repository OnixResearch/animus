#[test]
// r[verify agent_harness_runtime.hashed_edit_host.boundary.selected]
// r[verify agent_harness_runtime.hashed_edit_host.freshness.current]
// r[verify agent_harness_runtime.hashed_edit_host.publication.complete]
// r[verify agent_harness_runtime.hashed_edit_host.verification.complete]
fn concrete_capability_host_revalidates_then_publishes_and_syncs() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let boundaries = [animus_std_workspace_edit::VisibleBoundaryDraft {
        line_tag: TAG_SECOND,
        side: animus_std_workspace_edit::BoundarySide::After,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Replace {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
        replacement: "BETA",
    }];
    let request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &boundaries,
        operations: &operations,
    });
    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut names = Names::default();
    let receipt =
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply(&request)
            .map_err(|error| format!("concrete edit: {error:?}"))?;

    let expected_bytes = b"alpha\nBETA\ngamma\n";
    assert_eq!(receipt.after_identity(), hash(expected_bytes));
    assert_eq!(
        std::fs::read(fixture.resource_path())
            .map_err(|error| format!("published file: {error:?}"))?,
        expected_bytes
    );
    assert_eq!(receipt.publication_state(), "durability-complete");
    assert_eq!(receipt.released_writer_generation(), NEXT_WRITER_GENERATION);
    assert_eq!(
        services.calls,
        [
            "observe",
            "acquire",
            "checkpoint-second-read",
            "observe",
            "checkpoint-publication",
            "release",
        ]
    );
    Ok(())
}
