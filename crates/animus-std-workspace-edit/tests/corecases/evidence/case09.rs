fn assert_private_rejections(draft: animus_std_workspace_edit::ReceiptDraft<'_>) {
    let private_cases = [
        animus_std_workspace_edit::PrivateMaterialClass::FileBody,
        animus_std_workspace_edit::PrivateMaterialClass::ReplacementText,
        animus_std_workspace_edit::PrivateMaterialClass::AbsolutePath,
        animus_std_workspace_edit::PrivateMaterialClass::VcsData,
        animus_std_workspace_edit::PrivateMaterialClass::ProcessData,
        animus_std_workspace_edit::PrivateMaterialClass::CredentialData,
        animus_std_workspace_edit::PrivateMaterialClass::RawError,
    ];
    assert_eq!(private_cases.len(), 7);
    for material in private_cases {
        assert_eq!(
            animus_std_workspace_edit::project_receipt(animus_std_workspace_edit::ReceiptDraft {
                leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::Private(material),
                ..draft
            }),
            Err(animus_std_workspace_edit::ReceiptIssue::PrivateMaterial)
        );
    }
}

fn assert_overclaim_rejections(draft: animus_std_workspace_edit::ReceiptDraft<'_>) {
    let overclaims = [
        animus_std_workspace_edit::UnsupportedClaimClass::UniversalExclusion,
        animus_std_workspace_edit::UnsupportedClaimClass::SemanticCorrectness,
        animus_std_workspace_edit::UnsupportedClaimClass::AtomicDurability,
        animus_std_workspace_edit::UnsupportedClaimClass::AuthorityOrSandbox,
        animus_std_workspace_edit::UnsupportedClaimClass::Readiness,
    ];
    assert_eq!(overclaims.len(), 5);
    for claim in overclaims {
        assert_eq!(
            animus_std_workspace_edit::project_receipt(animus_std_workspace_edit::ReceiptDraft {
                leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::UnsupportedClaim(claim),
                ..draft
            }),
            Err(animus_std_workspace_edit::ReceiptIssue::UnsupportedClaim)
        );
    }
}

#[test]
// r[verify agent_harness_runtime.hashed_edit_host.evidence.safe]
// r[verify agent_harness_runtime.hashed_edit_host.evidence.rejects]
// r[verify agent_harness_runtime.hashed_edit_host.verification.incomplete]
fn receipts_are_deterministic_metadata_only_and_reject_every_leak_class() -> Result<(), String> {
    let snapshot = admitted_for(SourceCase {
        source: b"alpha\nbeta\ngamma\n",
        normalization: "lf",
        mode: animus_std_workspace_edit::LineEndingMode::Lf,
        final_newline: animus_std_workspace_edit::FinalNewlineState::Present,
        mixed: false,
    })?;
    let plan = animus_std_workspace_edit::plan_edit(
        &snapshot,
        &[animus_std_workspace_edit::EditOperationDraft::Replace {
            start_tag: TAG_SECOND,
            end_tag: TAG_SECOND,
            replacement: "BETA",
        }],
    )
    .map_err(|error| format!("plan: {error:?}"))?;
    let outcome = animus_std_workspace_edit::reduce_publication(&complete_classification()?)
        .map_err(|error| format!("outcome: {error:?}"))?;
    let draft = animus_std_workspace_edit::ReceiptDraft {
        plan: &plan,
        publication: &outcome,
        released_writer_generation: RELEASED_GENERATION,
        leak_facts: animus_std_workspace_edit::ReceiptLeakFacts::None,
    };
    let first = animus_std_workspace_edit::project_receipt(draft)
        .map_err(|error| format!("receipt: {error:?}"))?;
    let second = animus_std_workspace_edit::project_receipt(draft)
        .map_err(|error| format!("repeat receipt: {error:?}"))?;
    assert_eq!(first, second);
    assert_eq!(first.root_identity(), ROOT_ID);
    assert!(first.non_claims().contains(&"not-universal-writer-exclusion"));
    assert_private_rejections(draft);
    assert_overclaim_rejections(draft);
    Ok(())
}
