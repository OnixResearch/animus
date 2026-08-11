#[test]
fn invalid_publication_input_and_generation_overflow_release_the_lease() -> Result<(), String> {
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
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    request.collision_attempt_limit = 0;
    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::PublicationFailed)
    );
    assert_eq!(services.calls.last(), Some(&"release"));
    assert!(io.calls.is_empty());

    request.collision_attempt_limit = COLLISION_LIMIT;
    request.writer_generation = MAX_WRITER_GENERATION;
    services = FakeServices::current(&fixture.resource_identity);
    services.observation.writer_generation = MAX_WRITER_GENERATION;
    io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::Service(
            animus_std_workspace_edit::HostServiceIssue::ReleaseFailed
        ))
    );
    assert_eq!(services.calls.last(), Some(&"release"));
    Ok(())
}
