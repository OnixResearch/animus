const DRIVING_TASK: &str = "task.host-scope-handoff-fixtures";
const FILE_ALPHA: &str = "src/alpha.rs";
const FILE_BETA: &str = "src/beta.rs";
const OBLIGATION: &str = "cargo-test-host";
const ALPHA_OLD_LINES: u32 = 3;
const ALPHA_NEW_LINES: u32 = 8;
const BETA_START_LINE: u32 = 12;
const BETA_OLD_LINES: u32 = 2;
const BETA_NEW_LINES: u32 = 4;
const DIFF_DECLARED_BYTES: u64 = 96;
const DECISION_DECLARED_BYTES: u64 = 64;

pub(super) fn hash(label: &str) -> &'static str {
    Box::leak(
        blake3::hash(label.as_bytes())
            .to_hex()
            .to_string()
            .into_boxed_str(),
    )
}

struct ScopeFixture {
    hunks: Vec<animus_runtime::change_scope_handoff::ChangeScopeHunkDraft<'static>>,
}

impl ScopeFixture {
    fn new() -> Self {
        Self {
            hunks: vec![
                animus_runtime::change_scope_handoff::ChangeScopeHunkDraft {
                    file_path: FILE_ALPHA,
                    hunk_identity: hash("hunk-alpha"),
                    old_start: 1,
                    old_lines: ALPHA_OLD_LINES,
                    new_start: 1,
                    new_lines: ALPHA_NEW_LINES,
                },
                animus_runtime::change_scope_handoff::ChangeScopeHunkDraft {
                    file_path: FILE_BETA,
                    hunk_identity: hash("hunk-beta"),
                    old_start: BETA_START_LINE,
                    old_lines: BETA_OLD_LINES,
                    new_start: BETA_START_LINE,
                    new_lines: BETA_NEW_LINES,
                },
            ],
        }
    }

    fn draft(&self) -> animus_runtime::change_scope_handoff::ChangeScopeDraft<'_> {
        animus_runtime::change_scope_handoff::ChangeScopeDraft {
            driving_task: DRIVING_TASK,
            files: &[FILE_ALPHA, FILE_BETA],
            hunks: self.hunks.as_slice(),
            obligations: &[OBLIGATION],
        }
    }

    fn pending(&self) -> animus_runtime::change_scope_handoff::ChangeHandoffState {
        animus_runtime::change_scope_handoff::open_change_handoff(
            animus_runtime::change_scope_handoff::validate_change_scope(self.draft())
                .expect("scope"),
        )
    }
}

pub(super) fn pending_handoff() -> animus_runtime::change_scope_handoff::ChangeHandoffState {
    ScopeFixture::new().pending()
}

pub(super) fn bounded_artifacts()
-> Vec<animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft<'static>> {
    vec![
        animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: hash("host-diff"),
            declared_byte_count: DIFF_DECLARED_BYTES,
        },
        animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffArtifactKind::DecisionIdentity,
            artifact_identity: hash("host-decision"),
            declared_byte_count: DECISION_DECLARED_BYTES,
        },
    ]
}

#[derive(Clone)]
pub(super) struct ScriptedReview {
    pub(super) decision:
        Result<super::super::ChangeHandoffReviewDecision, super::super::ChangeHandoffReviewIssue>,
}

impl super::super::ChangeHandoffReviewService for ScriptedReview {
    fn decide(
        &mut self,
        _presentation: super::super::ChangeHandoffScopePresentation<'_>,
    ) -> Result<super::super::ChangeHandoffReviewDecision, super::super::ChangeHandoffReviewIssue>
    {
        self.decision.clone()
    }
}

pub(super) fn approved(hunks: &[u32]) -> super::super::ChangeHandoffReviewDecision {
    super::super::ChangeHandoffReviewDecision {
        kind: animus_runtime::change_scope_handoff::ChangeHandoffDecisionKind::Approved,
        approved_hunks: hunks.to_vec(),
    }
}
