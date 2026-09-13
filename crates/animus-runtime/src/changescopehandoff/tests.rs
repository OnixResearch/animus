use super::core::is_blake3_identity;
use super::*;

const DRIVING_TASK: &str = "task.scope-handoff-fixtures";
const FILE_ALPHA: &str = "src/alpha.rs";
const FILE_BETA: &str = "src/beta.rs";
const OBLIGATION_TEST: &str = "cargo-test-alpha";
const OBLIGATION_CLIPPY: &str = "clippy-alpha";

struct ScopeFixture {
    files: Vec<&'static str>,
    hunks: Vec<ChangeScopeHunkDraft<'static>>,
    obligations: Vec<&'static str>,
}

impl ScopeFixture {
    fn new() -> Self {
        Self {
            files: vec![FILE_ALPHA, FILE_BETA],
            hunks: vec![
                ChangeScopeHunkDraft {
                    file_path: FILE_ALPHA,
                    hunk_identity: hash("hunk-alpha"),
                    old_start: 1,
                    old_lines: 4,
                    new_start: 1,
                    new_lines: 9,
                },
                ChangeScopeHunkDraft {
                    file_path: FILE_BETA,
                    hunk_identity: hash("hunk-beta"),
                    old_start: 10,
                    old_lines: 2,
                    new_start: 10,
                    new_lines: 5,
                },
            ],
            obligations: vec![OBLIGATION_TEST, OBLIGATION_CLIPPY],
        }
    }

    fn draft(&self) -> ChangeScopeDraft<'_> {
        ChangeScopeDraft {
            driving_task: DRIVING_TASK,
            files: self.files.as_slice(),
            hunks: self.hunks.as_slice(),
            obligations: self.obligations.as_slice(),
        }
    }

    fn scope(&self) -> ChangeScope {
        validate_change_scope(self.draft()).expect("scope")
    }

    fn handoff(&self) -> ChangeHandoffState {
        open_change_handoff(self.scope())
    }

    fn accept_all(state: &ChangeHandoffState) -> ChangeHandoffState {
        record_change_handoff_decision(
            state,
            ChangeHandoffDecisionDraft {
                scope_identity: state.scope().scope_identity(),
                kind: ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[0, 1],
            },
        )
        .expect("decision")
    }
}

fn bounded_artifacts() -> Vec<ChangeHandoffArtifactDraft<'static>> {
    vec![
        ChangeHandoffArtifactDraft {
            kind: ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: hash("diff"),
            declared_byte_count: 128,
        },
        ChangeHandoffArtifactDraft {
            kind: ChangeHandoffArtifactKind::DecisionIdentity,
            artifact_identity: hash("decision-artifact"),
            declared_byte_count: 64,
        },
    ]
}

fn hash(label: &str) -> &'static str {
    Box::leak(
        blake3::hash(label.as_bytes())
            .to_hex()
            .to_string()
            .into_boxed_str(),
    )
}

#[test]
fn complete_scope_is_admitted_with_deterministic_identity() {
    // r[verify animus.change_scope_handoff.scope]
    let fixture = ScopeFixture::new();
    let scope = fixture.scope();
    assert_eq!(scope.files().len(), 2);
    assert_eq!(scope.hunks().len(), 2);
    assert_eq!(scope.obligations().len(), 2);
    assert!(is_blake3_identity(scope.scope_identity()));
    let replay = fixture.scope();
    assert_eq!(scope, replay);
    assert_eq!(scope.scope_identity(), replay.scope_identity());
}

#[test]
fn recorded_acceptance_completes_by_citing_the_decision() {
    // r[verify animus.change_scope_handoff.handoff]
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(pending.status(), ChangeHandoffStatus::PendingDecision);
    let recorded = ScopeFixture::accept_all(&pending);
    assert_eq!(recorded.status(), ChangeHandoffStatus::DecisionRecorded);
    let completion = evaluate_change_handoff_completion(&recorded).expect("completion");
    assert_eq!(
        completion.scope_identity(),
        recorded.scope().scope_identity()
    );
    assert_eq!(
        completion.decision_identity(),
        recorded.decision().expect("decision").decision_identity()
    );
    assert_eq!(completion.approved_hunks(), [0, 1]);
}

#[test]
fn continuation_is_subset_bounded_and_rejected_items_open_a_fresh_scope() {
    // r[verify animus.change_scope_handoff.continuation]
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = record_change_handoff_decision(
        &pending,
        ChangeHandoffDecisionDraft {
            scope_identity: pending.scope().scope_identity(),
            kind: ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0],
        },
    )
    .expect("decision");
    let plan = plan_change_continuation(&recorded).expect("plan");
    assert_eq!(plan.approved_hunks(), [0]);
    let fresh = plan.fresh_scope().expect("fresh scope");
    assert_eq!(fresh.hunks().len(), 1);
    assert_eq!(fresh.hunks()[0].hunk_identity(), hash("hunk-beta"));
    assert_eq!(fresh.files(), [String::from(FILE_BETA)]);
    assert_ne!(fresh.scope_identity(), recorded.scope().scope_identity());
    let fresh_handoff = open_change_handoff(fresh.clone());
    assert_eq!(fresh_handoff.status(), ChangeHandoffStatus::PendingDecision);
    let full_pending = fixture.handoff();
    let full_completion = ScopeFixture::accept_all(&full_pending);
    let full_plan = plan_change_continuation(&full_completion).expect("plan");
    assert!(full_plan.fresh_scope().is_none());
}

#[test]
fn bounded_artifacts_project_and_round_trip_a_validated_receipt() {
    // r[verify animus.change_scope_handoff.artifacts]
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = ScopeFixture::accept_all(&pending);
    let artifacts =
        validate_change_handoff_artifacts(bounded_artifacts().as_slice()).expect("artifacts");
    let receipt = project_change_handoff_receipt(&recorded, artifacts.as_slice()).expect("receipt");
    validate_change_handoff_receipt(&receipt).expect("valid receipt");
    assert_eq!(
        receipt.schema(),
        CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA,
        "schema"
    );
    assert_eq!(receipt.status(), ChangeHandoffStatus::DecisionRecorded);
    assert_eq!(receipt.artifact_identities().len(), 2);
    assert_eq!(
        receipt.completion_decision_identity(),
        receipt.decision_identity()
    );
    assert_eq!(
        receipt.redaction_marker(),
        CHANGE_SCOPE_HANDOFF_REDACTION_MARKER
    );
    assert_eq!(receipt.non_claims(), CHANGE_SCOPE_HANDOFF_NON_CLAIMS);
    let pending_receipt =
        project_change_handoff_receipt(&pending, artifacts.as_slice()).expect("pending receipt");
    assert_eq!(
        pending_receipt.status(),
        ChangeHandoffStatus::PendingDecision
    );
    assert!(pending_receipt.decision_identity().is_none());
    validate_change_handoff_receipt(&pending_receipt).expect("valid pending receipt");
}

#[test]
fn malformed_scope_facts_are_rejected_at_each_declared_boundary() {
    // r[verify animus.change_scope_handoff.scope]
    let fixture = ScopeFixture::new();
    let empty_task = ChangeScopeDraft {
        driving_task: "",
        files: fixture.files.as_slice(),
        hunks: fixture.hunks.as_slice(),
        obligations: fixture.obligations.as_slice(),
    };
    assert_eq!(
        validate_change_scope(empty_task)
            .expect_err("empty task")
            .issue(),
        ChangeHandoffIssue::MissingDrivingTask
    );
    let missing_files = ChangeScopeDraft {
        driving_task: DRIVING_TASK,
        files: &[],
        hunks: fixture.hunks.as_slice(),
        obligations: fixture.obligations.as_slice(),
    };
    assert_eq!(
        validate_change_scope(missing_files)
            .expect_err("missing files")
            .issue(),
        ChangeHandoffIssue::MissingScopeFiles
    );
    let missing_hunks = ChangeScopeDraft {
        driving_task: DRIVING_TASK,
        files: fixture.files.as_slice(),
        hunks: &[],
        obligations: fixture.obligations.as_slice(),
    };
    assert_eq!(
        validate_change_scope(missing_hunks)
            .expect_err("missing hunks")
            .issue(),
        ChangeHandoffIssue::MissingScopeHunks
    );
    let missing_obligations = ChangeScopeDraft {
        driving_task: DRIVING_TASK,
        files: fixture.files.as_slice(),
        hunks: fixture.hunks.as_slice(),
        obligations: &[],
    };
    assert_eq!(
        validate_change_scope(missing_obligations)
            .expect_err("missing obligations")
            .issue(),
        ChangeHandoffIssue::MissingScopeObligations
    );
    let undeclared = ChangeScopeHunkDraft {
        file_path: "src/undeclared.rs",
        hunk_identity: hash("hunk-undeclared"),
        old_start: 1,
        old_lines: 1,
        new_start: 1,
        new_lines: 1,
    };
    let mut stray_hunks = fixture.hunks.clone();
    stray_hunks.push(undeclared);
    let stray = ChangeScopeDraft {
        driving_task: DRIVING_TASK,
        files: fixture.files.as_slice(),
        hunks: stray_hunks.as_slice(),
        obligations: fixture.obligations.as_slice(),
    };
    assert_eq!(
        validate_change_scope(stray)
            .expect_err("undeclared file")
            .issue(),
        ChangeHandoffIssue::UndeclaredScopeFile
    );
    let malformed = ChangeScopeHunkDraft {
        file_path: FILE_ALPHA,
        hunk_identity: "not-a-blake3-identity",
        old_start: 1,
        old_lines: 1,
        new_start: 1,
        new_lines: 1,
    };
    let single = ChangeScopeDraft {
        driving_task: DRIVING_TASK,
        files: fixture.files.as_slice(),
        hunks: &[malformed],
        obligations: fixture.obligations.as_slice(),
    };
    assert_eq!(
        validate_change_scope(single).expect_err("identity").issue(),
        ChangeHandoffIssue::InvalidIdentity
    );
}

#[test]
fn missing_or_non_acceptance_decisions_block_completion() {
    // r[verify animus.change_scope_handoff.handoff]
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(
        evaluate_change_handoff_completion(&pending).expect_err("no decision yet"),
        ChangeHandoffCompletionBlock::DecisionMissing
    );
    let refined = record_change_handoff_decision(
        &pending,
        ChangeHandoffDecisionDraft {
            scope_identity: pending.scope().scope_identity(),
            kind: ChangeHandoffDecisionKind::RefineRequested,
            approved_hunks: &[],
        },
    )
    .expect("refine decision");
    assert_eq!(
        evaluate_change_handoff_completion(&refined).expect_err("refined is not acceptance"),
        ChangeHandoffCompletionBlock::DecisionNotAcceptance
    );
    let rejected = record_change_handoff_decision(
        &pending,
        ChangeHandoffDecisionDraft {
            scope_identity: pending.scope().scope_identity(),
            kind: ChangeHandoffDecisionKind::Rejected,
            approved_hunks: &[],
        },
    )
    .expect("reject decision");
    assert_eq!(
        evaluate_change_handoff_completion(&rejected).expect_err("rejected is not acceptance"),
        ChangeHandoffCompletionBlock::DecisionNotAcceptance
    );
}

#[test]
fn unbounded_or_mismatched_continuation_facts_are_denied() {
    // r[verify animus.change_scope_handoff.continuation]
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(
        plan_change_continuation(&pending)
            .expect_err("no decision yet")
            .issue(),
        ChangeHandoffIssue::DecisionMissing
    );
    let mismatched = record_change_handoff_decision(
        &pending,
        ChangeHandoffDecisionDraft {
            scope_identity: hash("other-scope"),
            kind: ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0],
        },
    )
    .expect_err("scope mismatch");
    assert_eq!(
        mismatched.issue(),
        ChangeHandoffIssue::ScopeIdentityMismatch
    );
    let unsorted = ChangeHandoffDecisionDraft {
        scope_identity: pending.scope().scope_identity(),
        kind: ChangeHandoffDecisionKind::Approved,
        approved_hunks: &[1, 0],
    };
    assert_eq!(
        record_change_handoff_decision(&pending, unsorted)
            .expect_err("unsorted subset")
            .issue(),
        ChangeHandoffIssue::InvalidApprovedSubset
    );
    let out_of_range = ChangeHandoffDecisionDraft {
        scope_identity: pending.scope().scope_identity(),
        kind: ChangeHandoffDecisionKind::Approved,
        approved_hunks: &[7],
    };
    assert_eq!(
        record_change_handoff_decision(&pending, out_of_range)
            .expect_err("out of range subset")
            .issue(),
        ChangeHandoffIssue::InvalidApprovedSubset
    );
    let empty_approval = ChangeHandoffDecisionDraft {
        scope_identity: pending.scope().scope_identity(),
        kind: ChangeHandoffDecisionKind::Approved,
        approved_hunks: &[],
    };
    assert_eq!(
        record_change_handoff_decision(&pending, empty_approval)
            .expect_err("empty approval")
            .issue(),
        ChangeHandoffIssue::InvalidApprovedSubset
    );
    let rejecting_subset = ChangeHandoffDecisionDraft {
        scope_identity: pending.scope().scope_identity(),
        kind: ChangeHandoffDecisionKind::Rejected,
        approved_hunks: &[0],
    };
    assert_eq!(
        record_change_handoff_decision(&pending, rejecting_subset)
            .expect_err("rejected subset")
            .issue(),
        ChangeHandoffIssue::InvalidApprovedSubset
    );
    let recorded = ScopeFixture::accept_all(&pending);
    let duplicated = record_change_handoff_decision(
        &recorded,
        ChangeHandoffDecisionDraft {
            scope_identity: recorded.scope().scope_identity(),
            kind: ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0, 1],
        },
    )
    .expect_err("decision is immutable");
    assert_eq!(
        duplicated.issue(),
        ChangeHandoffIssue::DecisionAlreadyRecorded
    );
}

#[test]
fn artifact_leaks_and_overbound_candidates_are_rejected_before_persistence() {
    // r[verify animus.change_scope_handoff.artifacts]
    let raw_transcript = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::RawTranscript,
        artifact_identity: hash("transcript"),
        declared_byte_count: 16,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(raw_transcript.as_slice())
            .expect_err("raw transcript")
            .issue(),
        ChangeHandoffIssue::RawArtifactKind
    );
    let raw_tool_output = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::RawToolOutput,
        artifact_identity: hash("tool-output"),
        declared_byte_count: 16,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(raw_tool_output.as_slice())
            .expect_err("raw tool output")
            .issue(),
        ChangeHandoffIssue::RawArtifactKind
    );
    let raw_prompt_response = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::RawPromptResponse,
        artifact_identity: hash("prompt"),
        declared_byte_count: 16,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(raw_prompt_response.as_slice())
            .expect_err("raw prompt")
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
            .expect_err("overbound")
            .issue(),
        ChangeHandoffIssue::ArtifactBoundExceeded
    );
    let malformed_identity = [ChangeHandoffArtifactDraft {
        kind: ChangeHandoffArtifactKind::DiffIdentity,
        artifact_identity: "deadbeef",
        declared_byte_count: 16,
    }];
    assert_eq!(
        validate_change_handoff_artifacts(malformed_identity.as_slice())
            .expect_err("identity")
            .issue(),
        ChangeHandoffIssue::InvalidIdentity
    );
    let over_count: Vec<ChangeHandoffArtifactDraft<'static>> = (0..=scope_artifact_bound())
        .map(|index| ChangeHandoffArtifactDraft {
            kind: ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: hash(index.to_string().leak()),
            declared_byte_count: 1,
        })
        .collect();
    assert_eq!(
        validate_change_handoff_artifacts(over_count.as_slice())
            .expect_err("artifact count")
            .issue(),
        ChangeHandoffIssue::ArtifactBoundExceeded
    );
}

#[test]
fn overclaim_labels_fail_non_claim_validation() {
    // r[verify animus.change_scope_handoff.nonclaims]
    assert_eq!(
        validate_change_handoff_claims(&["change-correctness"])
            .expect_err("change correctness")
            .issue(),
        ChangeHandoffIssue::NonClaimViolation
    );
    assert_eq!(
        validate_change_handoff_claims(&["policy-enforcement"])
            .expect_err("policy enforcement")
            .issue(),
        ChangeHandoffIssue::NonClaimViolation
    );
    assert_eq!(
        validate_change_handoff_claims(&["vcs-completion"])
            .expect_err("vcs completion")
            .issue(),
        ChangeHandoffIssue::NonClaimViolation
    );
    validate_change_handoff_claims(&["scope-recorded", "review-presented"]).expect("safe labels");
    for claim in CHANGE_SCOPE_HANDOFF_NON_CLAIMS {
        validate_change_handoff_claims(&[claim]).expect("non-claims stay non-claims");
    }
}

#[test]
fn tampered_receipts_fail_validation() {
    let fixture = ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = ScopeFixture::accept_all(&pending);
    let artifacts =
        validate_change_handoff_artifacts(bounded_artifacts().as_slice()).expect("artifacts");
    let mut receipt =
        project_change_handoff_receipt(&recorded, artifacts.as_slice()).expect("receipt");
    receipt.driving_task = String::from("task.tampered");
    assert_eq!(
        validate_change_handoff_receipt(&receipt)
            .expect_err("tampered label")
            .issue(),
        ChangeHandoffIssue::ReceiptFieldMismatch
    );
    let mut claim_drift =
        project_change_handoff_receipt(&recorded, artifacts.as_slice()).expect("receipt");
    claim_drift.non_claims = vec!["change-correctness"];
    assert_eq!(
        validate_change_handoff_receipt(&claim_drift)
            .expect_err("claim drift")
            .issue(),
        ChangeHandoffIssue::ReceiptFieldMismatch
    );
}
