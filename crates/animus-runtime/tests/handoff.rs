//! Public-API controls for typed scope identity binding.

#[cfg(test)]
mod controls {
    use animus_runtime::change_scope_handoff;

    const TASK: &str = "task.scope-partition";
    const OTHER_TASK: &str = "task.other-partition";
    const FILE: &str = "src/alpha.rs";
    const SHIFTED_LABEL: &str = "src/beta.rs";
    const OBLIGATION: &str = "gate.alpha";
    const OTHER_OBLIGATION: &str = "gate.beta";
    const HUNK_ID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const OTHER_HUNK_ID: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const LEGACY_SCOPE_ID: &str =
        "102ebf980596c3e6ea3321ef10e0785c0fe6b7067f27d468e9265b9434ff124b";
    const APPROVED_HUNKS: &[usize] = &[0];
    const CHANGED_COORDINATE: u32 = 2;

    const HUNK: change_scope_handoff::ChangeScopeHunkDraft<'static> =
        change_scope_handoff::ChangeScopeHunkDraft {
            file_path: FILE,
            hunk_identity: HUNK_ID,
            old_start: 1,
            old_lines: 1,
            new_start: 1,
            new_lines: 1,
        };

    fn draft() -> change_scope_handoff::ChangeScopeDraft<'static> {
        change_scope_handoff::ChangeScopeDraft {
            driving_task: TASK,
            files: &[FILE, SHIFTED_LABEL],
            hunks: &[HUNK],
            obligations: &[OBLIGATION],
        }
    }

    fn scope(
        draft: change_scope_handoff::ChangeScopeDraft<'_>,
    ) -> change_scope_handoff::ChangeScope {
        change_scope_handoff::validate_change_scope(draft).expect("complete scope is admitted")
    }

    fn accept(
        pending: &change_scope_handoff::ChangeHandoffState,
        identity: &str,
    ) -> Result<change_scope_handoff::ChangeHandoffState, change_scope_handoff::ChangeHandoffError>
    {
        change_scope_handoff::record_change_handoff_decision(
            pending,
            change_scope_handoff::ChangeHandoffDecisionDraft {
                scope_identity: identity,
                kind: change_scope_handoff::ChangeHandoffDecisionKind::Approved,
                approved_hunks: APPROVED_HUNKS,
            },
        )
    }

    fn refuses_identity(changed: change_scope_handoff::ChangeScope, identity: &str) {
        let pending = change_scope_handoff::open_change_handoff(changed);
        let before = pending.clone();
        let rejected =
            accept(&pending, identity).expect_err("another scope cannot authorize this one");
        assert_eq!(
            rejected.issue(),
            change_scope_handoff::ChangeHandoffIssue::ScopeIdentityMismatch
        );
        assert_eq!(pending, before);
        assert_eq!(
            pending.status(),
            change_scope_handoff::ChangeHandoffStatus::PendingDecision
        );
        assert!(pending.decision().is_none());
        assert_eq!(
            change_scope_handoff::evaluate_change_handoff_completion(&pending),
            Err(change_scope_handoff::ChangeHandoffCompletionBlock::DecisionMissing)
        );
    }

    fn refuses_crossing(
        original: &change_scope_handoff::ChangeScope,
        changed: change_scope_handoff::ChangeScope,
    ) {
        assert_ne!(original, &changed);
        refuses_identity(original.clone(), changed.scope_identity());
        assert_ne!(original.scope_identity(), changed.scope_identity());
        refuses_identity(changed, original.scope_identity());
    }

    #[test]
    fn exact_replay_preserves_decision_completion_and_receipt() {
        // r[verify animus.change_scope_handoff.scope]
        // r[verify animus.change_scope_handoff.handoff]
        let original = scope(draft());
        let replay = scope(draft());
        assert_eq!(original, replay);
        let pending = change_scope_handoff::open_change_handoff(replay);
        let recorded =
            accept(&pending, original.scope_identity()).expect("exact decision is admitted");
        let completed = change_scope_handoff::evaluate_change_handoff_completion(&recorded)
            .expect("recorded acceptance completes");
        assert_eq!(completed.scope_identity(), original.scope_identity());
        assert_eq!(completed.approved_hunks(), APPROVED_HUNKS);
        let receipt = change_scope_handoff::project_change_handoff_receipt(&recorded, &[])
            .expect("accepted handoff projects a receipt");
        change_scope_handoff::validate_change_handoff_receipt(&receipt)
            .expect("the projected receipt validates");
        assert_eq!(receipt.scope_identity(), completed.scope_identity());
        assert_eq!(
            receipt.completion_decision_identity(),
            Some(completed.decision_identity())
        );
        assert_eq!(
            receipt.non_claims(),
            change_scope_handoff::CHANGE_SCOPE_HANDOFF_NON_CLAIMS
        );
    }

    #[test]
    fn every_admitted_partition_rejects_the_other_decision() {
        // r[verify animus.change_scope_handoff.scope]
        // Labels include possible field tags, count text, and multibyte UTF-8.
        const CROSSING_LABELS: &[&str] = &[SHIFTED_LABEL, "files", "obligations", "1", "λ"];
        for label in CROSSING_LABELS {
            let original = scope(change_scope_handoff::ChangeScopeDraft {
                files: &[FILE, label],
                ..draft()
            });
            let changed = scope(change_scope_handoff::ChangeScopeDraft {
                files: &[FILE],
                obligations: &[label, OBLIGATION],
                ..draft()
            });
            assert_ne!(original.files(), changed.files());
            assert_ne!(original.obligations(), changed.obligations());
            refuses_crossing(&original, changed);
        }
    }

    #[test]
    fn task_and_each_collection_bind_the_decision() {
        // r[verify animus.change_scope_handoff.scope]
        let original = scope(draft());
        let changes = [
            change_scope_handoff::ChangeScopeDraft {
                driving_task: OTHER_TASK,
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                files: &[FILE],
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                files: &[SHIFTED_LABEL, FILE],
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                files: &[FILE, FILE],
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                obligations: &[OTHER_OBLIGATION],
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                obligations: &[OBLIGATION, OBLIGATION],
                ..draft()
            },
            change_scope_handoff::ChangeScopeDraft {
                hunks: &[HUNK, HUNK],
                ..draft()
            },
        ];
        for changed in changes {
            refuses_crossing(&original, scope(changed));
        }
        let ordered = scope(change_scope_handoff::ChangeScopeDraft {
            obligations: &[OBLIGATION, OTHER_OBLIGATION],
            ..draft()
        });
        let reversed = scope(change_scope_handoff::ChangeScopeDraft {
            obligations: &[OTHER_OBLIGATION, OBLIGATION],
            ..draft()
        });
        refuses_crossing(&ordered, reversed);
    }

    #[test]
    fn every_hunk_field_binds_the_decision() {
        // r[verify animus.change_scope_handoff.scope]
        let original = scope(draft());
        let changes = [
            change_scope_handoff::ChangeScopeHunkDraft {
                file_path: SHIFTED_LABEL,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                hunk_identity: OTHER_HUNK_ID,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                old_start: CHANGED_COORDINATE,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                old_lines: CHANGED_COORDINATE,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                new_start: CHANGED_COORDINATE,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                new_lines: CHANGED_COORDINATE,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                old_start: u32::MAX,
                ..HUNK
            },
            change_scope_handoff::ChangeScopeHunkDraft {
                new_lines: 0,
                ..HUNK
            },
        ];
        for changed in changes {
            refuses_crossing(
                &original,
                scope(change_scope_handoff::ChangeScopeDraft {
                    hunks: &[changed],
                    ..draft()
                }),
            );
        }
    }

    #[test]
    fn hunk_order_binds_the_decision() {
        // r[verify animus.change_scope_handoff.scope]
        let other = change_scope_handoff::ChangeScopeHunkDraft {
            hunk_identity: OTHER_HUNK_ID,
            ..HUNK
        };
        let ordered = scope(change_scope_handoff::ChangeScopeDraft {
            hunks: &[HUNK, other],
            ..draft()
        });
        let reversed = scope(change_scope_handoff::ChangeScopeDraft {
            hunks: &[other, HUNK],
            ..draft()
        });
        refuses_crossing(&ordered, reversed);
    }

    #[test]
    fn uncounted_v1_scope_identity_cannot_answer_a_new_scope() {
        // r[verify animus.change_scope_handoff.scope]
        // Captured from the unchanged public-API counterexample, not the new encoder.
        refuses_identity(scope(draft()), LEGACY_SCOPE_ID);
    }
}
