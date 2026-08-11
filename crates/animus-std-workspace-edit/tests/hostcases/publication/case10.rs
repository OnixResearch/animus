#[test]
fn cleanup_collision_and_unsupported_capability_remain_prepublication() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::Write);
    io.cleanup_succeeds = false;
    let receipt = apply_delete(&fixture, &mut services, &mut names, &mut io)
        .map_err(|error| format!("cleanup failure classification: {error:?}"))?;
    assert_eq!(receipt.publication_state(), "failed-before-publication");
    assert!(io.stage_exists);
    assert_eq!(receipt.released_writer_generation(), WRITER_GENERATION);

    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut io = FakePublicationIo::with_failure(FailurePoint::Collision);
    let receipt = apply_delete(&fixture, &mut services, &mut names, &mut io)
        .map_err(|error| format!("collision exhaustion classification: {error:?}"))?;
    assert_eq!(receipt.publication_state(), "failed-before-publication");
    assert_eq!(
        io.calls.iter().filter(|call| **call == "stage").count(),
        COLLISION_LIMIT
    );
    assert!(!io.stage_exists);

    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut io = FakePublicationIo::with_failure(FailurePoint::None);
    io.parent_sync_supported = false;
    let receipt = apply_delete(&fixture, &mut services, &mut names, &mut io)
        .map_err(|error| format!("unsupported parent sync classification: {error:?}"))?;
    assert_eq!(receipt.publication_state(), "failed-before-publication");
    assert!(io.calls.is_empty());
    Ok(())
}
