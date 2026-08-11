#[cfg(unix)]
fn assert_symlink_denial(
    fixture: &Fixture,
    names: &mut Names,
    io: &mut FakePublicationIo,
) -> Result<(), String> {
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Delete {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
    }];
    let link_resource = "nested/link.txt";
    std::os::unix::fs::symlink("file.txt", fixture.temporary.path().join(link_resource))
        .map_err(|error| format!("symlink: {error:?}"))?;
    let link_identity = animus_std_workspace_edit::logical_resource_identity(
        ROOT_ID,
        std::path::Path::new(link_resource),
    )
    .map_err(|error| format!("link identity: {error:?}"))?;
    let link_base = base_snapshot(&link_identity, SOURCE)?;
    let link_request = build_request(RequestInput {
        base: &link_base,
        resource_identity: &link_identity,
        logical_resource: std::path::Path::new(link_resource),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });
    let mut services = FakeServices::current(&link_identity);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(
            &fixture.root,
            &mut services,
            names,
        )
        .apply_with_publication_io(io, &link_request),
        Err(animus_std_workspace_edit::HostError::UnsupportedFileKind)
    );
    assert!(!link_identity.is_empty());
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.hashed_edit_host.boundary.ambient]
fn path_authority_symlink_permission_and_missing_gate_fail_before_publication() -> Result<(), String> {
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
        logical_resource: std::path::Path::new("/outside"),
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
    request.expected_mode = Some(0);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::PermissionDenied)
    );
    request.expected_mode = None;
    request.expected_owner = Some(NONEXISTENT_OWNER);
    assert_eq!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::OwnershipDenied)
    );
    request.expected_owner = None;
    request.gates.authority = animus_std_workspace_edit::GateDecision::Denied;
    assert!(matches!(
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request),
        Err(animus_std_workspace_edit::HostError::Core(
            animus_std_workspace_edit::HostIssue::GateDenied(_)
        ))
    ));
    #[cfg(unix)]
    assert_symlink_denial(&fixture, &mut names, &mut io)?;
    assert!(io.calls.is_empty());
    Ok(())
}
