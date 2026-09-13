//! Accepted and rejected public handoff bounds.

#[cfg(test)]
mod controls {
    const FILE: &str = "src/bounded.rs";
    const IDENTITY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HUNK: animus_runtime::change_scope_handoff::ChangeScopeHunkDraft<'static> =
        animus_runtime::change_scope_handoff::ChangeScopeHunkDraft {
            file_path: FILE,
            hunk_identity: IDENTITY,
            old_start: 1,
            old_lines: 1,
            new_start: 1,
            new_lines: 1,
        };

    fn slots(count: u32) -> usize {
        usize::try_from(count).expect("bounded fixture allocation")
    }

    fn draft() -> animus_runtime::change_scope_handoff::ChangeScopeDraft<'static> {
        animus_runtime::change_scope_handoff::ChangeScopeDraft {
            driving_task: "task.boundary",
            files: &[FILE],
            hunks: &[HUNK],
            obligations: &["gate.boundary"],
        }
    }

    #[test]
    fn each_collection_accepts_its_bound_and_rejects_one_more() {
        let mut files = vec![FILE; slots(animus_runtime::change_scope_handoff::scope_file_bound())];
        let mut hunks = vec![HUNK; slots(animus_runtime::change_scope_handoff::scope_hunk_bound())];
        let mut obligations =
            vec![
                "gate.boundary";
                slots(animus_runtime::change_scope_handoff::scope_obligation_bound())
            ];
        let valid = animus_runtime::change_scope_handoff::validate_change_scope(
            animus_runtime::change_scope_handoff::ChangeScopeDraft {
                files: &files,
                hunks: &hunks,
                obligations: &obligations,
                ..draft()
            },
        )
        .expect("all exact collection bounds");
        assert_eq!(valid.files().len(), files.len());
        assert_eq!(valid.hunks().len(), hunks.len());
        assert_eq!(valid.obligations().len(), obligations.len());
        files.push(FILE);
        hunks.push(HUNK);
        obligations.push("gate.boundary");
        let cases = [
            (
                animus_runtime::change_scope_handoff::ChangeScopeDraft {
                    files: &files,
                    ..draft()
                },
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::MissingScopeFiles,
            ),
            (
                animus_runtime::change_scope_handoff::ChangeScopeDraft {
                    hunks: &hunks,
                    ..draft()
                },
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::MissingScopeHunks,
            ),
            (
                animus_runtime::change_scope_handoff::ChangeScopeDraft {
                    obligations: &obligations,
                    ..draft()
                },
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::MissingScopeObligations,
            ),
        ];
        for (candidate, expected) in cases {
            let issue = animus_runtime::change_scope_handoff::validate_change_scope(candidate)
                .expect_err("overbound collection");
            assert_eq!(issue.issue(), expected);
        }
    }

    #[test]
    fn label_admission_keeps_its_byte_bound() {
        let limit: u32 = animus_runtime::change_scope_handoff::scope_label_char_bound();
        let exact = "x".repeat(slots(limit));
        let valid = animus_runtime::change_scope_handoff::validate_change_scope(
            animus_runtime::change_scope_handoff::ChangeScopeDraft {
                driving_task: &exact,
                ..draft()
            },
        )
        .expect("exact byte bound");
        assert_eq!(valid.driving_task(), exact);
        let excess = format!("{exact}x");
        let multibyte = "λ".repeat(slots(limit));
        for label in [&excess, &multibyte] {
            let issue = animus_runtime::change_scope_handoff::validate_change_scope(
                animus_runtime::change_scope_handoff::ChangeScopeDraft {
                    driving_task: label,
                    ..draft()
                },
            )
            .expect_err("excess bytes are not truncated");
            assert_eq!(
                issue.issue(),
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::LabelBoundExceeded
            );
        }
    }

    #[test]
    fn bounded_artifacts_preserve_count_and_byte_limits() {
        let bounded = animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            kind: animus_runtime::change_scope_handoff::ChangeHandoffArtifactKind::DiffIdentity,
            artifact_identity: IDENTITY,
            declared_byte_count: animus_runtime::change_scope_handoff::artifact_declared_byte_bound(
            ),
        };
        let mut artifacts =
            vec![bounded; slots(animus_runtime::change_scope_handoff::scope_artifact_bound())];
        let admitted =
            animus_runtime::change_scope_handoff::validate_change_handoff_artifacts(&artifacts)
                .expect("exact bounds");
        assert_eq!(admitted.len(), artifacts.len());
        assert!(
            admitted
                .iter()
                .all(|artifact| artifact.declared_byte_count() == bounded.declared_byte_count)
        );
        artifacts.push(bounded);
        let overbound_bytes = animus_runtime::change_scope_handoff::ChangeHandoffArtifactDraft {
            declared_byte_count: u64::MAX,
            ..bounded
        };
        for candidates in [artifacts.as_slice(), &[overbound_bytes]] {
            let issue =
                animus_runtime::change_scope_handoff::validate_change_handoff_artifacts(candidates)
                    .expect_err("excess artifacts");
            assert_eq!(
                issue.issue(),
                animus_runtime::change_scope_handoff::ChangeHandoffIssue::ArtifactBoundExceeded
            );
        }
    }
}
