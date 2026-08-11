#[test]
// r[verify agent_harness_runtime.hashed_edit_host.publication.complete]
// r[verify agent_harness_runtime.hashed_edit_host.publication.after]
fn publication_reduction_separates_visibility_durability_and_uncertainty() -> Result<(), String> {
    let complete = animus_std_workspace_edit::reduce_publication(&complete_classification()?)
        .map_err(|error| format!("complete outcome: {error:?}"))?;
    assert_eq!(
        complete.state(),
        animus_std_workspace_edit::PublicationState::DurabilityComplete
    );
    assert_eq!(
        complete.durability(),
        animus_std_workspace_edit::DurabilityState::Complete
    );

    let request = durable_file_publication::core::PublicationRequest {
        destination_leaf: String::from("file.txt"),
        payload_bytes: PAYLOAD_BYTES,
        maximum_payload_bytes: MAXIMUM_PAYLOAD_BYTES,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
        replacement: durable_file_publication::core::ReplacementMode::Replace,
        durability: durable_file_publication::core::DurabilityMode::DurabilityRequired,
    };
    let uncertain = durable_file_publication::core::classify_publication(
        &request,
        &[
            durable_file_publication::core::PhaseObservation::StageCreated {
                leaf: String::from(".stage"),
            },
            durable_file_publication::core::PhaseObservation::PayloadWritten {
                bytes: PAYLOAD_BYTES,
            },
            durable_file_publication::core::PhaseObservation::PermissionsApplied,
            durable_file_publication::core::PhaseObservation::PayloadSynchronized,
            durable_file_publication::core::PhaseObservation::Renamed,
            durable_file_publication::core::PhaseObservation::Failed(
                durable_file_publication::core::OperationPhase::ParentSync,
            ),
        ],
    )
    .map_err(|error| format!("uncertain publication: {error:?}"))?;
    let uncertain = animus_std_workspace_edit::reduce_publication(&uncertain)
        .map_err(|error| format!("uncertain outcome: {error:?}"))?;
    assert_eq!(
        uncertain.state(),
        animus_std_workspace_edit::PublicationState::UncertainAfterPublication
    );
    assert_eq!(
        uncertain.durability(),
        animus_std_workspace_edit::DurabilityState::Unknown
    );
    Ok(())
}
