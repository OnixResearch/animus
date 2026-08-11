#[test]
fn root_level_capability_file_uses_the_opened_root_directory() -> Result<(), String> {
    let fixture = Fixture::new()?;
    std::fs::write(fixture.temporary.path().join(ROOT_RESOURCE), SOURCE)
        .map_err(|error| format!("root file: {error:?}"))?;
    let resource_identity = animus_std_workspace_edit::logical_resource_identity(
        ROOT_ID,
        std::path::Path::new(ROOT_RESOURCE),
    )
    .map_err(|error| format!("root resource identity: {error:?}"))?;
    let base = base_snapshot(&resource_identity, SOURCE)?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Delete {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
    }];
    let request = build_request(RequestInput {
        base: &base,
        resource_identity: &resource_identity,
        logical_resource: std::path::Path::new(ROOT_RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    let mut services = FakeServices::current(&resource_identity);
    let mut names = Names::default();
    let receipt =
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply(&request)
            .map_err(|error| format!("root-level publication: {error:?}"))?;
    assert_eq!(
        std::fs::read(fixture.temporary.path().join(ROOT_RESOURCE))
            .map_err(|error| format!("published root file: {error:?}"))?,
        b"alpha\ngamma\n"
    );
    assert_eq!(receipt.publication_state(), "durability-complete");
    Ok(())
}
