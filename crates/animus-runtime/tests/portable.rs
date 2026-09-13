//! Public handoff compatibility and integer-boundary controls.

#[cfg(test)]
mod controls {
    use animus_runtime::change_scope_handoff;

    const TASK: &str = "task.portable-handoff";
    const FILE: &str = "src/portable.rs";
    const OBLIGATION: &str = "gate.portable";
    const IDENTITY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const SCOPE_ID: &str = "0b9142a782580ca4e8d4e1bd3fa5e2760984b410752fc6f3f7d4a287cc07f88f";
    const PENDING_RECEIPT_ID: &str =
        "f1e9385c6fc31860aeac18032e5b145029cff9d834aa22edab14b7672d72b2b8";
    const DECISION_ID: &str = "f26a1865fbcdd0767beaa89204b9f1398fc212c2ca2dfc84b40fac9e977c2407";
    const COMPLETED_RECEIPT_ID: &str =
        "f803a5ad391291f0ff145370abf5bac4615caf04c9686acc7bcf1be8719f87c2";
    const HUNK: change_scope_handoff::ChangeScopeHunkDraft<'static> =
        change_scope_handoff::ChangeScopeHunkDraft {
            file_path: FILE,
            hunk_identity: IDENTITY,
            old_start: 1,
            old_lines: 1,
            new_start: 1,
            new_lines: 1,
        };

    fn draft() -> change_scope_handoff::ChangeScopeDraft<'static> {
        change_scope_handoff::ChangeScopeDraft {
            driving_task: TASK,
            files: &[FILE],
            hunks: &[HUNK],
            obligations: &[OBLIGATION],
        }
    }

    #[test]
    fn admitted_values_retain_the_v2_digest_contract() {
        let scope = change_scope_handoff::validate_change_scope(draft()).expect("valid scope");
        let pending = change_scope_handoff::open_change_handoff(scope);
        let pending_receipt = change_scope_handoff::project_change_handoff_receipt(&pending, &[])
            .expect("pending receipt");
        let recorded = change_scope_handoff::record_change_handoff_decision(
            &pending,
            change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[0],
            },
        )
        .expect("accepted review");
        let completion = change_scope_handoff::evaluate_change_handoff_completion(&recorded)
            .expect("recorded acceptance completes");
        let receipt = change_scope_handoff::project_change_handoff_receipt(&recorded, &[])
            .expect("completed receipt");
        assert_eq!(pending.scope().scope_identity(), SCOPE_ID);
        assert_eq!(pending_receipt.receipt_identity(), PENDING_RECEIPT_ID);
        assert_eq!(completion.decision_identity(), DECISION_ID);
        assert_eq!(receipt.receipt_identity(), COMPLETED_RECEIPT_ID);
        assert_eq!(receipt.scope_identity(), pending.scope().scope_identity());
        assert_eq!(
            receipt.completion_decision_identity(),
            Some(completion.decision_identity())
        );
    }

    #[test]
    fn an_out_of_scope_index_cannot_consume_the_pending_state() {
        let scope = change_scope_handoff::validate_change_scope(draft()).expect("valid scope");
        let pending = change_scope_handoff::open_change_handoff(scope);
        let before = pending.clone();
        let out_of_scope = usize::try_from(u32::MAX).expect("host index represents the fixture");
        let issue = change_scope_handoff::record_change_handoff_decision(
            &pending,
            change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[out_of_scope],
            },
        )
        .expect_err("out-of-scope index is rejected");
        assert_eq!(
            issue.issue(),
            change_scope_handoff::ChangeHandoffIssue::InvalidApprovedSubset
        );
        assert_eq!(pending, before);
        assert_eq!(
            change_scope_handoff::evaluate_change_handoff_completion(&pending),
            Err(change_scope_handoff::ChangeHandoffCompletionBlock::DecisionMissing)
        );
    }
}
