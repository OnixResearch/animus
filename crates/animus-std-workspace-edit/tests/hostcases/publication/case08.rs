#[test]
fn invalid_leases_release_and_denied_acquisition_never_stages() -> Result<(), String> {
    for behavior in [
        LeaseBehavior::WrongResource,
        LeaseBehavior::WrongRevision,
        LeaseBehavior::WrongGeneration,
    ] {
        let fixture = Fixture::new()?;
        let mut services = FakeServices::current(&fixture.resource_identity);
        services.lease_behavior = behavior;
        let mut names = Names::default();
        let mut io = FakePublicationIo::with_failure(FailurePoint::None);
        assert_eq!(
            apply_delete(&fixture, &mut services, &mut names, &mut io),
            Err(animus_std_workspace_edit::HostError::Service(
                animus_std_workspace_edit::HostServiceIssue::Stale
            ))
        );
        assert!(io.calls.is_empty());
        assert_eq!(services.calls.last(), Some(&"release"));
    }

    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.lease_behavior = LeaseBehavior::Denied;
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Service(
            animus_std_workspace_edit::HostServiceIssue::Denied
        ))
    );
    assert!(!services.calls.contains(&"release"));
    assert!(io.calls.is_empty());
    Ok(())
}
