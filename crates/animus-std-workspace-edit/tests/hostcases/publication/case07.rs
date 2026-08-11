#[test]
// r[verify agent_harness_runtime.hashed_edit_host.publication.before]
// r[verify agent_harness_runtime.hashed_edit_host.publication.after]
// r[verify agent_harness_runtime.hashed_edit_host.publication.external_race]
fn publication_failures_retain_precommit_and_uncertain_boundaries() -> Result<(), String> {
    let fixture = Fixture::new()?;
    let spans = [animus_std_workspace_edit::VisibleSpanDraft {
        start_tag: TAG_FIRST,
        end_tag: TAG_THIRD,
    }];
    let operations = [animus_std_workspace_edit::EditOperationDraft::Replace {
        start_tag: TAG_SECOND,
        end_tag: TAG_SECOND,
        replacement: "BETA",
    }];
    let request = build_request(RequestInput {
        base: &fixture.base,
        resource_identity: &fixture.resource_identity,
        logical_resource: std::path::Path::new(RESOURCE),
        visible_spans: &spans,
        visible_boundaries: &[],
        operations: &operations,
    });

    for failure in [
        FailurePoint::Stage,
        FailurePoint::Write,
        FailurePoint::Permission,
        FailurePoint::PayloadSync,
        FailurePoint::Rename,
    ] {
        let mut services = FakeServices::current(&fixture.resource_identity);
        let mut names = Names::default();
        let mut io = FakePublicationIo::with_failure(failure);
        let receipt = animus_std_workspace_edit::WorkspaceEditHost::new(
            &fixture.root,
            &mut services,
            &mut names,
        )
        .apply_with_publication_io(&mut io, &request)
        .map_err(|error| format!("classified pre-publication failure: {error:?}"))?;
        assert_eq!(receipt.publication_state(), "failed-before-publication");
        assert_eq!(receipt.released_writer_generation(), WRITER_GENERATION);
    }

    let mut services = FakeServices::current(&fixture.resource_identity);
    let mut names = Names::default();
    let mut io = FakePublicationIo::with_failure(FailurePoint::ParentSync);
    let receipt =
        animus_std_workspace_edit::WorkspaceEditHost::new(&fixture.root, &mut services, &mut names)
            .apply_with_publication_io(&mut io, &request)
            .map_err(|error| format!("post-publication uncertainty receipt: {error:?}"))?;
    assert_eq!(receipt.publication_state(), "uncertain-after-publication");
    assert_eq!(receipt.durability_state(), "unknown");
    assert_eq!(receipt.released_writer_generation(), NEXT_WRITER_GENERATION);
    assert!(
        receipt
            .non_claims()
            .contains(&"not-universal-writer-exclusion")
    );
    Ok(())
}
