#[test]
fn bounded_artifacts_project_and_round_trip_a_validated_receipt() {
    // r[verify animus.change_scope_handoff.artifacts]
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = super::fixtures::ScopeFixture::accept_all(&pending);
    let artifacts = super::super::core::validate_change_handoff_artifacts(
        &super::fixtures::bounded_artifacts(),
    )
    .expect("artifacts");
    let receipt = super::super::evidence::project_change_handoff_receipt(&recorded, &artifacts)
        .expect("receipt");
    super::super::evidence::validate_change_handoff_receipt(&receipt).expect("valid receipt");
    assert_eq!(
        receipt.schema(),
        super::super::data::CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA,
        "schema"
    );
    assert_eq!(
        receipt.status(),
        super::super::data::ChangeHandoffStatus::DecisionRecorded
    );
    assert_eq!(
        receipt.artifact_identities().len(),
        super::fixtures::SCOPE_ITEM_COUNT
    );
    assert_eq!(
        receipt.completion_decision_identity(),
        receipt.decision_identity()
    );
    assert_eq!(
        receipt.redaction_marker(),
        super::super::data::CHANGE_SCOPE_HANDOFF_REDACTION_MARKER
    );
    assert_eq!(
        receipt.non_claims(),
        super::super::data::CHANGE_SCOPE_HANDOFF_NON_CLAIMS
    );
    let pending_receipt =
        super::super::evidence::project_change_handoff_receipt(&pending, &artifacts)
            .expect("pending receipt");
    assert_eq!(
        pending_receipt.status(),
        super::super::data::ChangeHandoffStatus::PendingDecision
    );
    assert!(pending_receipt.decision_identity().is_none());
    super::super::evidence::validate_change_handoff_receipt(&pending_receipt)
        .expect("valid pending receipt");
}

#[test]
fn artifact_leaks_and_overbound_candidates_are_rejected_before_persistence() {
    // r[verify animus.change_scope_handoff.artifacts]
    const RAW_DECLARED_BYTE_COUNT: u64 = 16;
    let overbound_byte_count = super::super::data::artifact_declared_byte_bound()
        .checked_add(1)
        .expect("overbound fixture");
    let cases = [
        (
            super::super::data::ChangeHandoffArtifactKind::RawTranscript,
            super::fixtures::hash("transcript"),
            RAW_DECLARED_BYTE_COUNT,
            super::super::data::ChangeHandoffIssue::RawArtifactKind,
        ),
        (
            super::super::data::ChangeHandoffArtifactKind::RawToolOutput,
            super::fixtures::hash("tool-output"),
            RAW_DECLARED_BYTE_COUNT,
            super::super::data::ChangeHandoffIssue::RawArtifactKind,
        ),
        (
            super::super::data::ChangeHandoffArtifactKind::RawPromptResponse,
            super::fixtures::hash("prompt"),
            RAW_DECLARED_BYTE_COUNT,
            super::super::data::ChangeHandoffIssue::RawArtifactKind,
        ),
        (
            super::super::data::ChangeHandoffArtifactKind::DiffIdentity,
            super::fixtures::hash("overbound"),
            overbound_byte_count,
            super::super::data::ChangeHandoffIssue::ArtifactBoundExceeded,
        ),
        (
            super::super::data::ChangeHandoffArtifactKind::DiffIdentity,
            "deadbeef",
            RAW_DECLARED_BYTE_COUNT,
            super::super::data::ChangeHandoffIssue::InvalidIdentity,
        ),
    ];
    for (kind, artifact_identity, declared_byte_count, expected) in cases {
        let candidate = super::super::data::ChangeHandoffArtifactDraft {
            kind,
            artifact_identity,
            declared_byte_count,
        };
        let issue = super::super::core::validate_change_handoff_artifacts(&[candidate])
            .expect_err("invalid artifact");
        assert_eq!(issue.issue(), expected);
    }
    let over_count: Vec<super::super::data::ChangeHandoffArtifactDraft<'static>> = (0
        ..=super::super::data::scope_artifact_bound())
        .map(|index| super::super::data::ChangeHandoffArtifactDraft {
            kind: super::super::data::ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: super::fixtures::hash(index.to_string().as_str()),
            declared_byte_count: 1,
        })
        .collect();
    assert_eq!(
        super::super::core::validate_change_handoff_artifacts(&over_count)
            .expect_err("artifact count")
            .issue(),
        super::super::data::ChangeHandoffIssue::ArtifactBoundExceeded
    );
}

#[test]
fn overclaim_labels_fail_non_claim_validation() {
    // r[verify animus.change_scope_handoff.nonclaims]
    for claim in super::super::data::FORBIDDEN_HANDOFF_CLAIM_LABELS {
        assert_eq!(
            super::super::core::validate_change_handoff_claims(&[claim])
                .expect_err("overclaim")
                .issue(),
            super::super::data::ChangeHandoffIssue::NonClaimViolation
        );
    }
    super::super::core::validate_change_handoff_claims(&["scope-recorded", "review-presented"])
        .expect("safe labels");
    for claim in super::super::data::CHANGE_SCOPE_HANDOFF_NON_CLAIMS {
        super::super::core::validate_change_handoff_claims(&[claim])
            .expect("non-claims stay non-claims");
    }
}

#[test]
fn tampered_receipts_fail_validation() {
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = super::fixtures::ScopeFixture::accept_all(&pending);
    let artifacts = super::super::core::validate_change_handoff_artifacts(
        &super::fixtures::bounded_artifacts(),
    )
    .expect("artifacts");
    let mut receipt = super::super::evidence::project_change_handoff_receipt(&recorded, &artifacts)
        .expect("receipt");
    receipt.driving_task = String::from("task.tampered");
    assert_eq!(
        super::super::evidence::validate_change_handoff_receipt(&receipt)
            .expect_err("tampered label")
            .issue(),
        super::super::data::ChangeHandoffIssue::ReceiptFieldMismatch
    );
    let mut claim_drift =
        super::super::evidence::project_change_handoff_receipt(&recorded, &artifacts)
            .expect("receipt");
    claim_drift.non_claims = vec!["change-correctness"];
    assert_eq!(
        super::super::evidence::validate_change_handoff_receipt(&claim_drift)
            .expect_err("claim drift")
            .issue(),
        super::super::data::ChangeHandoffIssue::ReceiptFieldMismatch
    );
}
