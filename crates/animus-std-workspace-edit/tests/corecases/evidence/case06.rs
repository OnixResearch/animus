type StaleCase<'a> = (
    animus_std_workspace_edit::FreshnessObservation<'a>,
    animus_std_workspace_edit::StaleField,
);

fn stale_cases(current: animus_std_workspace_edit::FreshnessObservation<'_>) -> [StaleCase<'_>; 7] {
    assert_eq!(current.root_identity, ROOT_ID);
    assert_eq!(current.writer_generation, WRITER_GENERATION);
    [
        (
            animus_std_workspace_edit::FreshnessObservation {
                root_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::Root,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                resource_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::Resource,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                revision_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::Revision,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                content_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::Content,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                metadata_policy_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::MetadataPolicy,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                file_metadata_identity: OTHER_ID,
                ..current
            },
            animus_std_workspace_edit::StaleField::FileMetadata,
        ),
        (
            animus_std_workspace_edit::FreshnessObservation {
                writer_generation: RELEASED_GENERATION,
                ..current
            },
            animus_std_workspace_edit::StaleField::WriterGeneration,
        ),
    ]
}

#[test]
// r[verify agent_harness_runtime.hashed_edit_host.freshness.current]
// r[verify agent_harness_runtime.hashed_edit_host.freshness.stale]
fn every_freshness_field_is_exact_and_typed() -> Result<(), String> {
    let snapshot = admitted_for(SourceCase {
        source: b"alpha\nbeta\ngamma\n",
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: false,
    })?;
    let plan = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Delete {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
        }],
    )
    .map_err(|error| format!("plan: {error:?}"))?;
    let current = animus_std_workspace_edit::FreshnessObservation {
        root_identity: ROOT_ID,
        resource_identity: RESOURCE_ID,
        revision_identity: REVISION_ID,
        content_identity: snapshot.content_identity(),
        metadata_policy_identity: POLICY_ID,
        file_metadata_identity: METADATA_ID,
        writer_generation: WRITER_GENERATION,
    };
    assert_eq!(animus_std_workspace_edit::validate_freshness(&plan, current), Ok(()));
    for (observation, field) in stale_cases(current) {
        assert_eq!(
            animus_std_workspace_edit::validate_freshness(&plan, observation),
            Err(animus_std_workspace_edit::HostIssue::Stale(field))
        );
    }
    Ok(())
}
