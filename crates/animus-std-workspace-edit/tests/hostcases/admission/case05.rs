#[test]
fn path_escape_wrong_root_and_directory_never_reach_staging() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Delete {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
    }];
    let mut request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new("../outside"),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::PathDenied)
    );
    assert!(services.calls.is_empty());

    request.logical_resource = std::path::Path::new(RESOURCE);
    request.root_identity = OTHER_ID;
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::PathDenied)
    );
    assert!(services.calls.is_empty());

    let directory_resource = "nested/directory";
    std::fs::create_dir(fixture.temporary.path().join(directory_resource))
        .map_err(|error| format!("directory: {error:?}"))?;
    let directory_identity = animus_std_workspace_edit::logical_resource_identity(
        ROOT_ID,
        std::path::Path::new(directory_resource),
    )
    .map_err(|error| format!("directory identity: {error:?}"))?;
    let directory_base = base_snapshot(&directory_identity, SOURCE)?;
    let directory_request = build_request(RequestInput {
        base: &directory_base,
        resource_identity: &directory_identity,
        logical_resource: std::path::Path::new(directory_resource),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    let mut directory_services = FakeServices::current(&directory_identity);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(
            &fixture.root,
            &mut directory_services,
            &mut names
        )
        .apply_with_publication_io(&mut io, &directory_request),
        Err(animus_std_workspace_edit::HostError::UnsupportedFileKind)
    );
    assert!(io.calls.is_empty());
    Ok(())
}
