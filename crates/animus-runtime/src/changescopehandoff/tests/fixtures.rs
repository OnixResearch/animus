pub(super) const DRIVING_TASK: &str = "task.scope-handoff-fixtures";
pub(super) const FILE_ALPHA: &str = "src/alpha.rs";
pub(super) const FILE_BETA: &str = "src/beta.rs";
pub(super) const SCOPE_ITEM_COUNT: usize = 2;
const OBLIGATION_TEST: &str = "cargo-test-alpha";
const OBLIGATION_CLIPPY: &str = "clippy-alpha";
const ALPHA_OLD_LINES: u32 = 4;
const ALPHA_NEW_LINES: u32 = 9;
const BETA_START_LINE: u32 = 10;
const BETA_OLD_LINES: u32 = 2;
const BETA_NEW_LINES: u32 = 5;
const DIFF_DECLARED_BYTES: u64 = 128;
const DECISION_DECLARED_BYTES: u64 = 64;

pub(super) struct ScopeFixture {
    files: Vec<&'static str>,
    hunks: Vec<super::super::data::ChangeScopeHunkDraft<'static>>,
    obligations: Vec<&'static str>,
}

impl ScopeFixture {
    pub(super) fn new() -> Self {
        Self {
            files: vec![FILE_ALPHA, FILE_BETA],
            hunks: vec![
                super::super::data::ChangeScopeHunkDraft {
                    file_path: FILE_ALPHA,
                    hunk_identity: hash("hunk-alpha"),
                    old_start: 1,
                    old_lines: ALPHA_OLD_LINES,
                    new_start: 1,
                    new_lines: ALPHA_NEW_LINES,
                },
                super::super::data::ChangeScopeHunkDraft {
                    file_path: FILE_BETA,
                    hunk_identity: hash("hunk-beta"),
                    old_start: BETA_START_LINE,
                    old_lines: BETA_OLD_LINES,
                    new_start: BETA_START_LINE,
                    new_lines: BETA_NEW_LINES,
                },
            ],
            obligations: vec![OBLIGATION_TEST, OBLIGATION_CLIPPY],
        }
    }

    pub(super) fn draft(&self) -> super::super::data::ChangeScopeDraft<'_> {
        super::super::data::ChangeScopeDraft {
            driving_task: DRIVING_TASK,
            files: self.files.as_slice(),
            hunks: self.hunks.as_slice(),
            obligations: self.obligations.as_slice(),
        }
    }

    pub(super) fn scope(&self) -> super::super::data::ChangeScope {
        super::super::binding::validate_change_scope(self.draft()).expect("scope")
    }

    pub(super) fn handoff(&self) -> super::super::data::ChangeHandoffState {
        super::super::core::open_change_handoff(self.scope())
    }

    pub(super) fn accept_all(
        state: &super::super::data::ChangeHandoffState,
    ) -> super::super::data::ChangeHandoffState {
        super::super::core::record_change_handoff_decision(
            state,
            super::super::data::ChangeHandoffDecisionDraft {
                scope_identity: state.scope().scope_identity(),
                kind: super::super::data::ChangeHandoffDecisionKind::Approved,
                approved_hunks: &[0, 1],
            },
        )
        .expect("decision")
    }
}

pub(super) fn bounded_artifacts() -> Vec<super::super::data::ChangeHandoffArtifactDraft<'static>> {
    vec![
        super::super::data::ChangeHandoffArtifactDraft {
            kind: super::super::data::ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: hash("diff"),
            declared_byte_count: DIFF_DECLARED_BYTES,
        },
        super::super::data::ChangeHandoffArtifactDraft {
            kind: super::super::data::ChangeHandoffArtifactKind::DecisionIdentity,
            artifact_identity: hash("decision-artifact"),
            declared_byte_count: DECISION_DECLARED_BYTES,
        },
    ]
}

pub(super) fn hash(label: &str) -> &'static str {
    Box::leak(
        blake3::hash(label.as_bytes())
            .to_hex()
            .to_string()
            .into_boxed_str(),
    )
}
