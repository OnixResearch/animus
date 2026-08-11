#[test]
fn every_leased_prepublication_failure_releases_without_staging() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.second_observation = Some(Err(
        animus_std_workspace_edit::HostServiceIssue::Unavailable,
    ));
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Service(
            animus_std_workspace_edit::HostServiceIssue::Unavailable
        ))
    );
    assert_eq!(services.calls.last(), Some(&"release"));
    assert!(io.calls.is_empty());

    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.mutation_at_second_read = Some(SecondReadMutation::Remove(fixture.resource_path()));
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert_eq!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::ReadFailed)
    );
    assert_eq!(services.calls.last(), Some(&"release"));
    assert!(io.calls.is_empty());

    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.mutation_at_second_read = Some(SecondReadMutation::Replace(
        fixture.resource_path(),
        SOURCE.to_vec(),
    ));
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert!(matches!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Core(
            animus_std_workspace_edit::HostIssue::Stale(
                animus_std_workspace_edit::StaleField::FileMetadata
            )
        ))
    ));
    assert_eq!(services.calls.last(), Some(&"release"));
    assert!(io.calls.is_empty());

    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    services.second_observation = Some(Ok(animus_std_workspace_edit::HostObservation {
        revision_identity: String::from(REVISION_ID),
        metadata_policy_identity: String::from(POLICY_ID),
        writer_generation: NEXT_WRITER_GENERATION,
    }));
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    assert!(matches!(
        apply_delete(&fixture, &mut services, &mut names, &mut io),
        Err(animus_std_workspace_edit::HostError::Core(
            animus_std_workspace_edit::HostIssue::Stale(
                animus_std_workspace_edit::StaleField::WriterGeneration
            )
        ))
    ));
    assert_eq!(services.calls.last(), Some(&"release"));
    assert!(io.calls.is_empty());
    Ok(())
}
