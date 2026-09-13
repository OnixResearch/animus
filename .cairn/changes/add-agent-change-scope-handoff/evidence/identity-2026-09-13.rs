//! Public-API controls for the unchanged imported handoff implementation.

const TASK: &str = "task.scope-partition";
const FILE: &str = "src/alpha.rs";
const SHIFTED_LABEL: &str = "src/beta.rs";
const OBLIGATION: &str = "gate.alpha";
const OTHER_OBLIGATION: &str = "gate.beta";
const HUNK_ID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const APPROVED_HUNKS: &[usize] = &[0];

fn scope(
    files: &[&str],
    obligations: &[&str],
) -> animus_runtime::change_scope_handoff::ChangeScope {
    let hunks = [animus_runtime::change_scope_handoff::ChangeScopeHunkDraft {
        file_path: FILE,
        hunk_identity: HUNK_ID,
        old_start: 1,
        old_lines: 1,
        new_start: 1,
        new_lines: 1,
    }];
    animus_runtime::change_scope_handoff::validate_change_scope(
        animus_runtime::change_scope_handoff::ChangeScopeDraft {
            driving_task: TASK,
            files,
            hunks: &hunks,
            obligations,
        },
    )
    .expect("complete bounded scope is admitted")
}

#[test]
fn an_exact_scope_accepts_its_own_decision() {
    let scope = scope(&[FILE], &[OBLIGATION]);
    let pending = animus_runtime::change_scope_handoff::open_change_handoff(scope);
    let decision = animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
        scope_identity: pending.scope().scope_identity(),
        kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
        approved_hunks: APPROVED_HUNKS,
    };
    let recorded = animus_runtime::change_scope_handoff::record_change_handoff_decision(
        &pending, decision,
    )
    .expect("the exact scope accepts the decision");
    let completion =
        animus_runtime::change_scope_handoff::evaluate_change_handoff_completion(&recorded)
            .expect("recorded acceptance completes the handoff");
    assert_eq!(completion.scope_identity(), pending.scope().scope_identity());
    assert_eq!(completion.approved_hunks(), APPROVED_HUNKS);
}

#[test]
fn a_changed_obligation_rejects_the_prior_decision() {
    let original = scope(&[FILE], &[OBLIGATION]);
    let changed = scope(&[FILE], &[OTHER_OBLIGATION]);
    assert_ne!(original.scope_identity(), changed.scope_identity());
    let pending = animus_runtime::change_scope_handoff::open_change_handoff(changed);
    let decision = animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
        scope_identity: original.scope_identity(),
        kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
        approved_hunks: APPROVED_HUNKS,
    };
    let result = animus_runtime::change_scope_handoff::record_change_handoff_decision(
        &pending, decision,
    );
    assert!(matches!(result, Err(error) if error.issue()
        == animus_runtime::change_scope_handoff::ChangeHandoffIssue::ScopeIdentityMismatch));
}

#[test]
fn moving_a_label_between_fields_rejects_the_prior_decision() {
    let original = scope(&[FILE, SHIFTED_LABEL], &[OBLIGATION]);
    let changed = scope(&[FILE], &[SHIFTED_LABEL, OBLIGATION]);
    assert_ne!(original.files(), changed.files());
    assert_ne!(original.obligations(), changed.obligations());
    let pending = animus_runtime::change_scope_handoff::open_change_handoff(changed);
    let decision = animus_runtime::change_scope_handoff::ChangeHandoffDecisionDraft {
        scope_identity: original.scope_identity(),
        kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
        approved_hunks: APPROVED_HUNKS,
    };
    let result = animus_runtime::change_scope_handoff::record_change_handoff_decision(
        &pending, decision,
    );
    let rejected = result.expect_err("a decision for another scope must fail");
    assert_eq!(rejected.issue(),
        animus_runtime::change_scope_handoff::ChangeHandoffIssue::ScopeIdentityMismatch);
}
