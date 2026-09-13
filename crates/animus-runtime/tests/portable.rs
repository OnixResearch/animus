//! Public handoff compatibility and integer-boundary controls.

#[cfg(test)]
mod controls {
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
    const HUNK: animus_runtime::change_scope_handoff::ChangeScopeHunkDraft<'static> =
        animus_runtime::change_scope_handoff::ChangeScopeHunkDraft {
            file_path: FILE,
            hunk_identity: IDENTITY,
            old_start: 1,
            old_lines: 1,
            new_start: 1,
            new_lines: 1,
        };

    fn draft() -> animus_runtime::change_scope_handoff::ChangeScopeDraft<'static> {
        animus_runtime::change_scope_handoff::ChangeScopeDraft {
            driving_task: TASK,
            files: &[FILE],
            hunks: &[HUNK],
            obligations: &[OBLIGATION],
        }
    }

    #[test]
    fn admitted_values_retain_the_v2_digest_contract() {
        let scope = animus_runtime::change_scope_handoff::validate_change_scope(draft())
            .expect("valid scope");
        let pending = animus_runtime::change_scope_handoff::open_change_handoff(scope);
        let pending_receipt =
            animus_runtime::change_scope_handoff::project_change_handoff_receipt(&pending, &[])
                .expect("pending receipt");
        let recorded = animus_runtime::change_scope_handoff::record_change_handoff_decision(
            &pending,
            animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[0],
            },
        )
        .expect("accepted review");
        let completion =
            animus_runtime::change_scope_handoff::evaluate_change_handoff_completion(&recorded)
                .expect("recorded acceptance completes");
        let receipt =
            animus_runtime::change_scope_handoff::project_change_handoff_receipt(&recorded, &[])
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
    fn maximum_hunk_scope_returns_portable_indexes_without_truncation() {
        let hunk_count: u32 = animus_runtime::change_scope_handoff::scope_hunk_bound();
        let hunk_slots = usize::try_from(hunk_count).expect("bounded allocation");
        let hunks = vec![HUNK; hunk_slots];
        let scope = animus_runtime::change_scope_handoff::validate_change_scope(
            animus_runtime::change_scope_handoff::ChangeScopeDraft {
                hunks: &hunks,
                ..draft()
            },
        )
        .expect("maximum hunk scope");
        let pending = animus_runtime::change_scope_handoff::open_change_handoff(scope);
        let last_index = hunk_count.checked_sub(1).expect("nonempty bound");
        let approved: &[u32] = &[0, last_index];
        let recorded = animus_runtime::change_scope_handoff::record_change_handoff_decision(
            &pending,
            animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: approved,
            },
        )
        .expect("first and last admitted indexes");
        let decision_indexes: &[u32] = recorded.decision().expect("recorded").approved_hunks();
        let completion =
            animus_runtime::change_scope_handoff::evaluate_change_handoff_completion(&recorded)
                .expect("completion");
        let completed_indexes: &[u32] = completion.approved_hunks();
        let plan = animus_runtime::change_scope_handoff::plan_change_continuation(&recorded)
            .expect("bounded plan");
        let continued_indexes: &[u32] = plan.approved_hunks();
        assert_eq!(decision_indexes, approved);
        assert_eq!(completed_indexes, approved);
        assert_eq!(continued_indexes, approved);
        let rejected_count = hunk_slots
            .checked_sub(approved.len())
            .expect("bounded subset");
        assert_eq!(
            plan.fresh_scope()
                .expect("rejected remainder")
                .hunks()
                .len(),
            rejected_count
        );
    }

    #[test]
    fn duplicate_descending_and_one_past_indexes_leave_the_scope_pending() {
        let hunks = [HUNK, HUNK];
        let scope = animus_runtime::change_scope_handoff::validate_change_scope(
            animus_runtime::change_scope_handoff::ChangeScopeDraft {
                hunks: &hunks,
                ..draft()
            },
        )
        .expect("valid scope");
        let one_past = u32::try_from(hunks.len()).expect("portable count");
        let pending = animus_runtime::change_scope_handoff::open_change_handoff(scope);
        let before = pending.clone();
        let candidates: &[&[u32]] = &[&[0, 0], &[1, 0], &[one_past]];
        for approved_hunks in candidates {
            let issue = animus_runtime::change_scope_handoff::record_change_handoff_decision(
                &pending,
                animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
                    scope_identity: pending.scope().scope_identity(),
                    kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                    approved_hunks,
                },
            )
            .expect_err("invalid portable subset");
            assert_eq!(
                issue.issue(),
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::InvalidApprovedSubset
            );
            assert_eq!(pending, before);
            assert!(pending.decision().is_none());
        }
    }

    #[test]
    fn an_out_of_scope_index_cannot_consume_the_pending_state() {
        let scope = animus_runtime::change_scope_handoff::validate_change_scope(draft())
            .expect("valid scope");
        let pending = animus_runtime::change_scope_handoff::open_change_handoff(scope);
        let before = pending.clone();
        let out_of_scope = u32::MAX;
        let issue = animus_runtime::change_scope_handoff::record_change_handoff_decision(
            &pending,
            animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: pending.scope().scope_identity(),
                kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[out_of_scope],
            },
        )
        .expect_err("out-of-scope index is rejected");
        assert_eq!(
            issue.issue(),
            animus_runtime::change_scope_handoff::ChangeHandoffIssue::InvalidApprovedSubset
        );
        assert_eq!(pending, before);
        assert_eq!(
            animus_runtime::change_scope_handoff::evaluate_change_handoff_completion(&pending),
            Err(
                animus_runtime::change_scope_handoff::ChangeHandoffCompletionBlock::DecisionMissing
            )
        );
    }
}
