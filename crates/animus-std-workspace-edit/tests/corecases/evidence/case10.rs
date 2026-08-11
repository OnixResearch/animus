#[test]
fn receipt_generation_must_match_the_terminal_publication_state() -> Result<(), String> {
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
    let complete = animus_std_workspace_edit::reduce_publication(&complete_classification()?)
        .map_err(|error| format!("complete: {error:?}"))?;
    for released_writer_generation in [WRITER_GENERATION, FUTURE_RELEASED_GENERATION] {
        assert_eq!(
            animus_std_workspace_edit::project_receipt(animus_std_workspace_edit::ReceiptDraft {
                plan: &plan,
                publication: &complete,
                released_writer_generation,
                leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::None,
            }),
            Err(animus_std_workspace_edit::ReceiptIssue::InvalidWriterGeneration)
        );
    }

    let request = durable_file_publication::core::PublicationRequest {
        destination_leaf: String::from("file.txt"),
        payload_bytes: PAYLOAD_BYTES,
        maximum_payload_bytes: MAXIMUM_PAYLOAD_BYTES,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
        replacement: durable_file_publication::core::ReplacementMode::Replace,
        durability: durable_file_publication::core::DurabilityMode::DurabilityRequired,
    };
    let failed = durable_file_publication::core::classify_publication(
        &request,
        &[durable_file_publication::core::PhaseObservation::Failed(
            durable_file_publication::core::OperationPhase::StageCreate,
        )],
    )
    .map_err(|error| format!("failed classification: {error:?}"))?;
    let failed = animus_std_workspace_edit::reduce_publication(&failed)
        .map_err(|error| format!("failed outcome: {error:?}"))?;
    let receipt =
        animus_std_workspace_edit::project_receipt(animus_std_workspace_edit::ReceiptDraft {
            plan: &plan,
            publication: &failed,
            released_writer_generation: WRITER_GENERATION,
            leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::None,
        })
        .map_err(|error| format!("failed-before-publication receipt: {error:?}"))?;
    assert_eq!(receipt.publication_state(), "failed-before-publication");
    assert_eq!(
        animus_std_workspace_edit::project_receipt(animus_std_workspace_edit::ReceiptDraft {
            plan: &plan,
            publication: &failed,
            released_writer_generation: RELEASED_GENERATION,
            leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::None,
        }),
        Err(animus_std_workspace_edit::ReceiptIssue::InvalidWriterGeneration)
    );
    Ok(())
}
