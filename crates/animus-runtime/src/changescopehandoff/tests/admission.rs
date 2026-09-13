#[test]
fn complete_scope_is_admitted_with_deterministic_identity() {
    // r[verify animus.change_scope_handoff.scope]
    let fixture = super::fixtures::ScopeFixture::new();
    let scope = fixture.scope();
    assert_eq!(scope.files().len(), super::fixtures::SCOPE_ITEM_COUNT);
    assert_eq!(scope.hunks().len(), super::fixtures::SCOPE_ITEM_COUNT);
    assert_eq!(scope.obligations().len(), super::fixtures::SCOPE_ITEM_COUNT);
    assert!(super::super::digest::is_blake3_identity(
        scope.scope_identity()
    ));
    let replay = fixture.scope();
    assert_eq!(scope, replay);
    assert_eq!(scope.scope_identity(), replay.scope_identity());
}

#[test]
fn malformed_scope_facts_are_rejected_at_each_declared_boundary() {
    // r[verify animus.change_scope_handoff.scope]
    let fixture = super::fixtures::ScopeFixture::new();
    let draft = fixture.draft();
    assert!(super::super::binding::validate_change_scope(draft).is_ok());
    let undeclared = super::super::data::ChangeScopeHunkDraft {
        file_path: "src/undeclared.rs",
        hunk_identity: super::fixtures::hash("hunk-undeclared"),
        old_start: 1,
        old_lines: 1,
        new_start: 1,
        new_lines: 1,
    };
    let mut stray_hunks = draft.hunks.to_vec();
    stray_hunks.push(undeclared);
    let malformed = super::super::data::ChangeScopeHunkDraft {
        file_path: super::fixtures::FILE_ALPHA,
        hunk_identity: "not-a-blake3-identity",
        ..undeclared
    };
    let malformed_hunks = [malformed];
    let cases = [
        (
            super::super::data::ChangeScopeDraft {
                driving_task: "",
                ..draft
            },
            super::super::data::ChangeHandoffIssue::MissingDrivingTask,
        ),
        (
            super::super::data::ChangeScopeDraft {
                files: &[],
                ..draft
            },
            super::super::data::ChangeHandoffIssue::MissingScopeFiles,
        ),
        (
            super::super::data::ChangeScopeDraft {
                hunks: &[],
                ..draft
            },
            super::super::data::ChangeHandoffIssue::MissingScopeHunks,
        ),
        (
            super::super::data::ChangeScopeDraft {
                obligations: &[],
                ..draft
            },
            super::super::data::ChangeHandoffIssue::MissingScopeObligations,
        ),
        (
            super::super::data::ChangeScopeDraft {
                hunks: &stray_hunks,
                ..draft
            },
            super::super::data::ChangeHandoffIssue::UndeclaredScopeFile,
        ),
        (
            super::super::data::ChangeScopeDraft {
                hunks: &malformed_hunks,
                ..draft
            },
            super::super::data::ChangeHandoffIssue::InvalidIdentity,
        ),
    ];
    for (candidate, expected) in cases {
        let issue =
            super::super::binding::validate_change_scope(candidate).expect_err("malformed scope");
        assert_eq!(issue.issue(), expected);
    }
}
