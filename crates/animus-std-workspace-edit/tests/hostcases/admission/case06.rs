#[test]
fn cancellation_and_timeout_after_lock_release_without_stage_creation() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Delete {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
    }];
    let request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    for issue in [
        animus_std_workspace_edit::HostServiceIssue::Cancelled,
        animus_std_workspace_edit::HostServiceIssue::TimedOut,
    ] {
        let mut services = FakeServices::current(&fixture.resource_identity);
        services.checkpoint_failure = Some((
            animus_std_workspace_edit::HostCheckpoint::BeforePublication,
            issue,
        ));
        let mut names = Names::default();
        let mut io = FakePublicationIo::with_failure(FailurePoint::None);
        assert_eq!(
            animus_std_workspace_edit::WorkspaceEditHost::new(
                &fixture.root,
                &mut services,
                &mut names
            )
            .apply_with_publication_io(&mut io, &request),
            Err(animus_std_workspace_edit::HostError::Service(issue))
        );
        assert!(io.calls.is_empty());
        assert_eq!(services.calls.last(), Some(&"release"));
    }
    Ok(())
}
