#[test]
fn release_failure_and_generation_drift_block_receipts() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.release_behavior = ReleaseBehavior::Override(DRIFTED_WRITER_GENERATION);
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Service(
            animus_std_workspace_edit::HostServiceIssue::ReleaseFailed
        ))
    );
    assert_eq!(services.calls.last(), Some(&"release"));

    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.release_behavior = ReleaseBehavior::Fail;
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Service(
            animus_std_workspace_edit::HostServiceIssue::ReleaseFailed
        ))
    );
    assert_eq!(services.calls.last(), Some(&"release"));
    Ok(())
}
