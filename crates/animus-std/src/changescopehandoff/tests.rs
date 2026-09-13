#[path = "tests/support.rs"]
mod support;

const ARTIFACT_COUNT: usize = 2;
const OUT_OF_RANGE_INDEX: u32 = 7;
const RAW_DECLARED_BYTE_COUNT: u64 = 8;

#[test]
fn recorded_review_completes_by_citing_the_recorded_acceptance() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.artifacts]
    let pending = support::pending_handoff();
    let mut reviewer = support::ScriptedReview {
        decision: Ok(support::approved(&[0, 1])),
    };
    let recorded = super::record_presented_change_handoff_review(&mut reviewer, &pending)
        .expect("recorded review");
    assert_eq!(
        recorded.status(),
        animus_runtime::change_scope_handoff::ChangeHandoffStatus::DecisionRecorded
    );
    let artifacts = animus_runtime::change_scope_handoff::validate_change_handoff_artifacts(
        support::bounded_artifacts().as_slice(),
    )
    .expect("bounded");
    let completed =
        super::complete_change_handoff_review(&recorded, artifacts.as_slice()).expect("completion");
    assert_eq!(
        completed.completion.decision_identity(),
        recorded
            .decision()
            .expect("recorded decision")
            .decision_identity()
    );
    assert_eq!(
        completed.receipt.status(),
        animus_runtime::change_scope_handoff::ChangeHandoffStatus::DecisionRecorded
    );
    assert_eq!(
        completed.receipt.completion_decision_identity(),
        Some(completed.completion.decision_identity())
    );
    assert_eq!(
        completed.receipt.artifact_identities().len(),
        ARTIFACT_COUNT
    );
    assert_eq!(
        completed.receipt.non_claims(),
        animus_runtime::change_scope_handoff::CHANGE_SCOPE_HANDOFF_NON_CLAIMS
    );
}

#[test]
fn unavailable_review_blocks_recording_and_leaves_the_scope_pending() {
    // r[verify animus.change_scope_handoff.handoff]
    let pending = support::pending_handoff();
    let mut reviewer = super::NoChangeHandoffReview;
    let error = super::record_presented_change_handoff_review(&mut reviewer, &pending)
        .expect_err("no invented decision");
    assert_eq!(
        error.issue(),
        super::ChangeHandoffReviewIssue::ReviewUnavailable
    );
    assert_eq!(
        pending.status(),
        animus_runtime::change_scope_handoff::ChangeHandoffStatus::PendingDecision
    );
    let blocker = super::complete_change_handoff_review(&pending, &[]).expect_err("no decision");
    assert_eq!(
        blocker.issue(),
        super::ChangeHandoffReviewIssue::DecisionMissing
    );
}

#[test]
fn malformed_reviewer_decisions_are_rejected_without_mutating_the_presentation() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.continuation]
    let pending = support::pending_handoff();
    let before = pending.clone();
    for index in [OUT_OF_RANGE_INDEX, u32::MAX] {
        let mut out_of_range = support::ScriptedReview {
            decision: Ok(support::approved(&[index])),
        };
        let error = super::record_presented_change_handoff_review(&mut out_of_range, &pending)
            .expect_err("out of range subset");
        assert_eq!(error.issue(), super::ChangeHandoffReviewIssue::CoreRejected);
        assert_eq!(pending, before);
        assert_eq!(
            pending.status(),
            animus_runtime::change_scope_handoff::ChangeHandoffStatus::PendingDecision
        );
    }
    let mut empty = support::ScriptedReview {
        decision: Ok(support::approved(&[])),
    };
    let error = super::record_presented_change_handoff_review(&mut empty, &pending)
        .expect_err("empty approval");
    assert_eq!(error.issue(), super::ChangeHandoffReviewIssue::CoreRejected);
    let mut accepting = support::ScriptedReview {
        decision: Ok(support::approved(&[0])),
    };
    let recorded = super::record_presented_change_handoff_review(&mut accepting, &pending)
        .expect("first decision");
    let duplicated = super::record_presented_change_handoff_review(&mut accepting, &recorded)
        .expect_err("immutable");
    assert_eq!(
        duplicated.issue(),
        super::ChangeHandoffReviewIssue::CoreRejected
    );
}

#[test]
fn rejected_review_blocks_completion_and_routes_rejected_items_to_a_fresh_scope() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.continuation]
    let pending = support::pending_handoff();
    let mut rejecting = support::ScriptedReview {
        decision: Ok(super::ChangeHandoffReviewDecision {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Rejected,
            approved_hunks: Vec::new(),
        }),
    };
    let recorded =
        super::record_presented_change_handoff_review(&mut rejecting, &pending).expect("rejection");
    let blocked =
        super::complete_change_handoff_review(&recorded, &[]).expect_err("not an acceptance");
    assert_eq!(
        blocked.issue(),
        super::ChangeHandoffReviewIssue::DecisionNotAcceptance
    );
    let partial = support::pending_handoff();
    let mut partial_review = support::ScriptedReview {
        decision: Ok(support::approved(&[0])),
    };
    let partially_approved =
        super::record_presented_change_handoff_review(&mut partial_review, &partial)
            .expect("partial approval");
    let plan = animus_runtime::change_scope_handoff::plan_change_continuation(&partially_approved)
        .expect("continuation plan");
    let fresh = plan.fresh_scope().expect("fresh scope for rejected items");
    assert_ne!(fresh.scope_identity(), partial.scope().scope_identity());
    assert_eq!(fresh.hunks().len(), 1);
}

#[test]
fn host_completion_keeps_artifact_and_claim_boundaries() {
    // r[verify animus.change_scope_handoff.artifacts]
    // r[verify animus.change_scope_handoff.nonclaims]
    let pending = support::pending_handoff();
    let mut reviewer = support::ScriptedReview {
        decision: Ok(support::approved(&[0, 1])),
    };
    super::record_presented_change_handoff_review(&mut reviewer, &pending).expect("recorded");
    let raw_artifact = [
        animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffArtifactKind::RawTranscript,
            artifact_identity: support::hash("raw"),
            declared_byte_count: RAW_DECLARED_BYTE_COUNT,
        },
    ];
    assert_eq!(
        animus_runtime::change_scope_handoff::validate_change_handoff_artifacts(
            raw_artifact.as_slice()
        )
        .expect_err("raw artifact leak")
        .issue(),
        animus_runtime::change_scope_handoff::ChangeHandoffIssue::RawArtifactKind
    );
    let overbound = [
        animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: support::hash("overbound"),
            declared_byte_count:
                animus_runtime::change_scope_handoff::artifact_declared_byte_bound()
                    .checked_add(1)
                    .expect("overbound fixture"),
        },
    ];
    assert_eq!(
        animus_runtime::change_scope_handoff::validate_change_handoff_artifacts(
            overbound.as_slice()
        )
        .expect_err("overbound artifact")
        .issue(),
        animus_runtime::change_scope_handoff::ChangeHandoffIssue::ArtifactBoundExceeded
    );
    assert_eq!(
        animus_runtime::change_scope_handoff::validate_change_handoff_claims(&[
            "production-readiness"
        ])
        .expect_err("forbidden ownership claim")
        .issue(),
        animus_runtime::change_scope_handoff::ChangeHandoffIssue::NonClaimViolation
    );
    animus_runtime::change_scope_handoff::validate_change_handoff_claims(
        animus_runtime::change_scope_handoff::CHANGE_SCOPE_HANDOFF_NON_CLAIMS.as_slice(),
    )
    .expect("non-claims stay non-claims");
}
