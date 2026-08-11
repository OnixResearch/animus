#[test]
// r[verify agent_harness_runtime.hashed_edit_host.freshness.stale]
fn second_read_content_and_revision_drift_never_create_a_stage() -> Result<(), String> {
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
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.mutation_at_second_read = Some(SecondReadMutation::Write(
        fixture.resource_path(),
        b"external\n".to_vec(),
    ));
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    let error =
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request)
            .err()
            .ok_or_else(|| String::from("content drift: expected error"))?;
    assert!(matches!(
        error,
        animus_std_workspace_edit::HostError::Core(animus_std_workspace_edit::HostIssue::Stale(_))
    ));
    assert!(io.calls.is_empty());
    assert_eq!(services.calls.last(), Some(&"release"));

    let fixture = Fixture::new()?;
    let request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.second_observation = Some(Ok(animus_std_workspace_edit::HostObservation {
        revision_identity: String::from(OTHER_ID),
        metadata_policy_identity: String::from(POLICY_ID),
        writer_generation: WRITER_GENERATION,
    }));
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    let error =
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request)
            .err()
            .ok_or_else(|| String::from("revision drift: expected error"))?;
    assert!(matches!(
        error,
        animus_std_workspace_edit::HostError::Core(animus_std_workspace_edit::HostIssue::Stale(_))
    ));
    assert!(io.calls.is_empty());
    Ok(())
}
