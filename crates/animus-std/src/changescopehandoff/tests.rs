use super::*;
use animus_runtime::change_scope_handoff::{
    CHANGE_SCOPE_HANDOFF_NON_CLAIMS, ChangeHandoffArtifactDraft, ChangeHandoffArtifactKind,
    ChangeHandoffDecisionKind, ChangeHandoffIssue, ChangeHandoffStatus, ChangeScopeDraft,
    ChangeScopeHunkDraft, artifact_declared_byte_bound, open_change_handoff,
    plan_change_continuation, validate_change_handoff_artifacts, validate_change_handoff_claims,
    validate_change_scope,
};

const DRIVING_TASK: &str = "task.host-scope-handoff-fixtures";
const FILE_ALPHA: &str = "src/alpha.rs";
const FILE_BETA: &str = "src/beta.rs";
const OBLIGATION: &str = "cargo-test-host";

fn hash(label: &str) -> &'static str {
    Box::leak(
        blake3::hash(label.as_bytes())
            .to_hex()
            .to_string()
            .into_boxed_str(),
    )
}

struct ScopeFixture {
    hunks: Vec<ChangeScopeHunkDraft<'static>>,
}

impl ScopeFixture {
    fn new() -> Self {
        Self {
            hunks: vec![
                ChangeScopeHunkDraft {
                    file_path: FILE_ALPHA,
                    hunk_identity: hash("hunk-alpha"),
                    old_start: 1,
                    old_lines: 3,
                    new_start: 1,
                    new_lines: 8,
                },
                ChangeScopeHunkDraft {
                    file_path: FILE_BETA,
                    hunk_identity: hash("hunk-beta"),
                    old_start: 12,
                    old_lines: 2,
                    new_start: 12,
                    new_lines: 4,
                },
            ],
        }
    }

    fn draft(&self) -> ChangeScopeDraft<'_> {
        ChangeScopeDraft {
            driving_task: DRIVING_TASK,
            files: &[FILE_ALPHA, FILE_BETA],
            hunks: self.hunks.as_slice(),
            obligations: &[OBLIGATION],
        }
    }

    fn pending(&self) -> animus_runtime::change_scope_handoff::ChangeHandoffState {
        open_change_handoff(validate_change_scope(self.draft()).expect("scope"))
    }
}

fn pending_handoff() -> animus_runtime::change_scope_handoff::ChangeHandoffState {
    ScopeFixture::new().pending()
}

fn bounded_artifacts() -> Vec<ChangeHandoffArtifactDraft<'static>> {
    vec![
        ChangeHandoffArtifactDraft {
            kind: ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: hash("host-diff"),
            declared_byte_count: 96,
        },
        ChangeHandoffArtifactDraft {
            kind: ChangeHandoffArtifactKind::DecisionIdentity,
            artifact_identity: hash("host-decision"),
            declared_byte_count: 64,
        },
    ]
}

#[derive(Clone)]
struct ScriptedReview {
    decision: Result<ChangeHandoffReviewDecision, ChangeHandoffReviewIssue>,
}

impl ChangeHandoffReviewService for ScriptedReview {
    fn decide(
        &mut self,
        _presentation: ChangeHandoffScopePresentation<'_>,
    ) -> Result<ChangeHandoffReviewDecision, ChangeHandoffReviewIssue> {
        self.decision.clone()
    }
}

fn approved(hunks: &[usize]) -> ChangeHandoffReviewDecision {
    ChangeHandoffReviewDecision {
        kind: ChangeHandoffDecisionKind::Approved,
        approved_hunks: hunks.to_vec(),
    }
}

#[test]
fn recorded_review_completes_by_citing_the_recorded_acceptance() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.artifacts]
    let pending = pending_handoff();
    let mut reviewer = ScriptedReview {
        decision: Ok(approved(&[0, 1])),
    };
    let recorded =
        record_presented_change_handoff_review(&mut reviewer, &pending).expect("recorded review");
    assert_eq!(recorded.status(), ChangeHandoffStatus::DecisionRecorded);
    let artifacts =
        validate_change_handoff_artifacts(bounded_artifacts().as_slice()).expect("bounded");
    let completed =
        complete_change_handoff_review(&recorded, artifacts.as_slice()).expect("completion");
    assert_eq!(
        completed.completion.decision_identity(),
        recorded
            .decision()
            .expect("recorded decision")
            .decision_identity()
    );
    assert_eq!(
        completed.receipt.status(),
        ChangeHandoffStatus::DecisionRecorded
    );
    assert_eq!(
        completed.receipt.completion_decision_identity(),
        Some(completed.completion.decision_identity())
    );
    assert_eq!(completed.receipt.artifact_identities().len(), 2);
    assert_eq!(
        completed.receipt.non_claims(),
        CHANGE_SCOPE_HANDOFF_NON_CLAIMS
    );
}

#[test]
fn unavailable_review_blocks_recording_and_leaves_the_scope_pending() {
    // r[verify animus.change_scope_handoff.handoff]
    let pending = pending_handoff();
    let mut reviewer = NoChangeHandoffReview;
    let error = record_presented_change_handoff_review(&mut reviewer, &pending)
        .expect_err("no invented decision");
    assert_eq!(error.issue(), ChangeHandoffReviewIssue::ReviewUnavailable);
    assert_eq!(pending.status(), ChangeHandoffStatus::PendingDecision);
    let blocker = complete_change_handoff_review(&pending, &[]).expect_err("no decision");
    assert_eq!(blocker.issue(), ChangeHandoffReviewIssue::DecisionMissing);
}

#[test]
fn malformed_reviewer_decisions_are_rejected_without_mutating_the_presentation() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.continuation]
    let pending = pending_handoff();
    let mut out_of_range = ScriptedReview {
        decision: Ok(approved(&[7])),
    };
    let error = record_presented_change_handoff_review(&mut out_of_range, &pending)
        .expect_err("out of range subset");
    assert_eq!(error.issue(), ChangeHandoffReviewIssue::CoreRejected);
    assert_eq!(pending.status(), ChangeHandoffStatus::PendingDecision);
    let mut empty = ScriptedReview {
        decision: Ok(approved(&[])),
    };
    let error =
        record_presented_change_handoff_review(&mut empty, &pending).expect_err("empty approval");
    assert_eq!(error.issue(), ChangeHandoffReviewIssue::CoreRejected);
    let mut accepting = ScriptedReview {
        decision: Ok(approved(&[0])),
    };
    let recorded =
        record_presented_change_handoff_review(&mut accepting, &pending).expect("first decision");
    let duplicated =
        record_presented_change_handoff_review(&mut accepting, &recorded).expect_err("immutable");
    assert_eq!(duplicated.issue(), ChangeHandoffReviewIssue::CoreRejected);
}

#[test]
fn rejected_review_blocks_completion_and_routes_rejected_items_to_a_fresh_scope() {
    // r[verify animus.change_scope_handoff.handoff]
    // r[verify animus.change_scope_handoff.continuation]
    let pending = pending_handoff();
    let mut rejecting = ScriptedReview {
        decision: Ok(ChangeHandoffReviewDecision {
            kind: ChangeHandoffDecisionKind::Rejected,
            approved_hunks: Vec::new(),
        }),
    };
    let recorded =
        record_presented_change_handoff_review(&mut rejecting, &pending).expect("rejection");
    let blocked = complete_change_handoff_review(&recorded, &[]).expect_err("not an acceptance");
    assert_eq!(
        blocked.issue(),
        ChangeHandoffReviewIssue::DecisionNotAcceptance
    );
    let partial = pending_handoff();
    let mut partial_review = ScriptedReview {
        decision: Ok(approved(&[0])),
    };
    let partially_approved = record_presented_change_handoff_review(&mut partial_review, &partial)
        .expect("partial approval");
    let plan = plan_change_continuation(&partially_approved).expect("continuation plan");
    let fresh = plan.fresh_scope().expect("fresh scope for rejected items");
    assert_ne!(fresh.scope_identity(), partial.scope().scope_identity());
    assert_eq!(fresh.hunks().len(), 1);
}

#[test]
fn host_completion_keeps_artifact_and_claim_boundaries() {
    // r[verify animus.change_scope_handoff.artifacts]
    // r[verify animus.change_scope_handoff.nonclaims]
    let pending = pending_handoff();
    let mut reviewer = ScriptedReview {
        decision: Ok(approved(&[0, 1])),
    };
    record_presented_change_handoff_review(&mut reviewer, &pending).expect("recorded");
    let raw_artifact = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::RawTranscript,
        artifact_identity: hash("raw"),
        declared_byte_count: 8,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(raw_artifact.as_slice())
            .expect_err("raw artifact leak")
            .issue(),
        ChangeHandoffIssue::RawArtifactKind
    );
    let overbound = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::DiffIdentity,
        artifact_identity: hash("overbound"),
        declared_byte_count: artifact_declared_byte_bound() + 1,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(overbound.as_slice())
            .expect_err("overbound artifact")
            .issue(),
        ChangeHandoffIssue::ArtifactBoundExceeded
    );
    assert_eq!(
        validate_change_handoff_claims(&["production-readiness"])
            .expect_err("forbidden ownership claim")
            .issue(),
        ChangeHandoffIssue::NonClaimViolation
    );
    validate_change_handoff_claims(CHANGE_SCOPE_HANDOFF_NON_CLAIMS.as_slice())
        .expect("non-claims stay non-claims");
}
