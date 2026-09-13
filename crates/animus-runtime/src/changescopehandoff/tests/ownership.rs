#[test]
fn recorded_acceptance_completes_by_citing_the_decision() {
    // r[verify animus.change_scope_handoff.handoff]
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(
        pending.status(),
        super::super::data::ChangeHandoffStatus::PendingDecision
    );
    let recorded = super::fixtures::ScopeFixture::accept_all(&pending);
    assert_eq!(
        recorded.status(),
        super::super::data::ChangeHandoffStatus::DecisionRecorded
    );
    let completion =
        super::super::core::evaluate_change_handoff_completion(&recorded).expect("completion");
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
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    let recorded = super::super::core::record_change_handoff_decision(
        &pending,
        super::super::data::ChangeHandoffDecisionDraft {
            scope_identity: pending.scope().scope_identity(),
            kind: super::super::data::ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0],
        },
    )
    .expect("decision");
    let plan = super::super::core::plan_change_continuation(&recorded).expect("plan");
    assert_eq!(plan.approved_hunks(), [0]);
    let fresh = plan.fresh_scope().expect("fresh scope");
    assert_eq!(fresh.hunks().len(), 1);
    assert_eq!(
        fresh.hunks()[0].hunk_identity(),
        super::fixtures::hash("hunk-beta")
    );
    assert_eq!(fresh.files(), [String::from(super::fixtures::FILE_BETA)]);
    assert_ne!(fresh.scope_identity(), recorded.scope().scope_identity());
    let fresh_handoff = super::super::core::open_change_handoff(fresh.clone());
    assert_eq!(
        fresh_handoff.status(),
        super::super::data::ChangeHandoffStatus::PendingDecision
    );
    let full_pending = fixture.handoff();
    let full_completion = super::fixtures::ScopeFixture::accept_all(&full_pending);
    let full_plan = super::super::core::plan_change_continuation(&full_completion).expect("plan");
    assert!(full_plan.fresh_scope().is_none());
}

#[test]
fn missing_or_non_acceptance_decisions_block_completion() {
    // r[verify animus.change_scope_handoff.handoff]
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(
        super::super::core::evaluate_change_handoff_completion(&pending)
            .expect_err("no decision yet"),
        super::super::data::ChangeHandoffCompletionBlock::DecisionMissing
    );
    for kind in [
        super::super::data::ChangeHandoffDecisionKind::RefineRequested,
        super::super::data::ChangeHandoffDecisionKind::Rejected,
    ] {
        let recorded = super::super::core::record_change_handoff_decision(
            &pending,
            super::super::data::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind,
                approved_hunks: &[],
            },
        )
        .expect("non-acceptance decision");
        assert_eq!(
            super::super::core::evaluate_change_handoff_completion(&recorded)
                .expect_err("not acceptance"),
            super::super::data::ChangeHandoffCompletionBlock::DecisionNotAcceptance
        );
    }
}

#[test]
fn unbounded_or_mismatched_continuation_facts_are_denied() {
    // r[verify animus.change_scope_handoff.continuation]
    const OUT_OF_RANGE_INDEX: u32 = 7;
    let fixture = super::fixtures::ScopeFixture::new();
    let pending = fixture.handoff();
    assert_eq!(
        super::super::core::plan_change_continuation(&pending)
            .expect_err("no decision yet")
            .issue(),
        super::super::data::ChangeHandoffIssue::DecisionMissing
    );
    let mismatched = super::super::core::record_change_handoff_decision(
        &pending,
        super::super::data::ChangeHandoffDecisionDraft {
            scope_identity: super::fixtures::hash("other-scope"),
            kind: super::super::data::ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0],
        },
    )
    .expect_err("scope mismatch");
    assert_eq!(
        mismatched.issue(),
        super::super::data::ChangeHandoffIssue::ScopeIdentityMismatch
    );
    let out_of_range = [OUT_OF_RANGE_INDEX];
    let cases: &[(super::super::data::ChangeHandoffDecisionKind, &[u32])] = &[
        (
            super::super::data::ChangeHandoffDecisionKind::Approved,
            &[1, 0],
        ),
        (
            super::super::data::ChangeHandoffDecisionKind::Approved,
            &out_of_range,
        ),
        (super::super::data::ChangeHandoffDecisionKind::Approved, &[]),
        (
            super::super::data::ChangeHandoffDecisionKind::Rejected,
            &[0],
        ),
    ];
    for (kind, approved_hunks) in cases {
        let issue = super::super::core::record_change_handoff_decision(
            &pending,
            super::super::data::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: *kind,
                approved_hunks,
            },
        )
        .expect_err("invalid subset");
        assert_eq!(
            issue.issue(),
            super::super::data::ChangeHandoffIssue::InvalidApprovedSubset
        );
    }
    let recorded = super::fixtures::ScopeFixture::accept_all(&pending);
    let duplicated = super::super::core::record_change_handoff_decision(
        &recorded,
        super::super::data::ChangeHandoffDecisionDraft {
            scope_identity: recorded.scope().scope_identity(),
            kind: super::super::data::ChangeHandoffDecisionKind::Approved,
            approved_hunks: &[0, 1],
        },
    )
    .expect_err("decision is immutable");
    assert_eq!(
        duplicated.issue(),
        super::super::data::ChangeHandoffIssue::DecisionAlreadyRecorded
    );
}
