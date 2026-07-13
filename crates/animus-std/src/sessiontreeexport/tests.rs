use super::*;

type SessionTreeExportEntryInput = animus_runtime::session_tree_export::SessionTreeExportEntryInput;
type SessionTreeExportEntryKind = animus_runtime::session_tree_export::SessionTreeExportEntryKind;
type SessionTreeExportFormat = animus_runtime::session_tree_export::SessionTreeExportFormat;
type SessionTreeExportSourceInput =
    animus_runtime::session_tree_export::SessionTreeExportSourceInput;

const GENERATED_POLICY_JSON: &str =
    include_str!("../../../../config/generated/session-tree-export-policy.json");
const DESTINATION_CAPABILITY: &str = "export.reviewed";
const ROOT_ID: &str = "entry.root";
const LEAF_ID: &str = "entry.leaf";
const FIRST_SEQUENCE: u32 = 1;
const SECOND_SEQUENCE: u32 = 2;
const WRITE_CALL_COUNT: u32 = 1;
const INVALID_FILE_MODE: u32 = 0o644;

#[test]
fn generated_policy_loads_and_injected_writer_commits_once() {
    // r[verify agent_harness_runtime.session_tree_export.policy]
    // r[verify agent_harness_runtime.session_tree_export.host_write]
    let policy = parse_session_tree_export_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let artifact = artifact(&policy);
    let mut writer = FakeWriter::matching(&artifact);
    let request = write_request(&artifact);
    let outcome = execute_session_tree_export_write(Some(&mut writer), request).expect("write");
    let evidence = project_session_tree_export_write_evidence(&request, &outcome, "committed")
        .expect("write evidence");

    assert_eq!(writer.calls, WRITE_CALL_COUNT);
    assert_eq!(outcome.artifact_hash, artifact.artifact_hash());
    assert_eq!(outcome.bytes_written, artifact.byte_count());
    assert_eq!(evidence.artifact_hash, artifact.artifact_hash());
    assert!(evidence.synchronized);
}

#[test]
fn malformed_policy_and_unknown_fields_fail_closed() {
    let malformed =
        parse_session_tree_export_policy_json("{").expect_err("malformed generated policy");
    let unknown = GENERATED_POLICY_JSON.replacen('{', "{\n  \"unknown_private_field\": true,", 1);
    let unknown = parse_session_tree_export_policy_json(unknown.as_str())
        .expect_err("unknown generated field");

    assert!(matches!(
        malformed,
        SessionTreeExportConfigError::Json {
            kind: SessionTreeExportJsonErrorKind::InvalidJson,
        }
    ));
    assert!(matches!(
        unknown,
        SessionTreeExportConfigError::Json {
            kind: SessionTreeExportJsonErrorKind::InvalidShape,
        }
    ));
}

#[test]
fn missing_service_invalid_admission_and_partial_outcome_never_claim_success() {
    let policy = parse_session_tree_export_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let artifact = artifact(&policy);
    let missing = execute_session_tree_export_write::<FakeWriter>(None, write_request(&artifact));
    let mut writer = FakeWriter::matching(&artifact);
    let invalid = execute_session_tree_export_write(
        Some(&mut writer),
        SessionTreeExportWriteRequest {
            file_mode: INVALID_FILE_MODE,
            ..write_request(&artifact)
        },
    );
    let mut no_destination = FakeWriter::matching(&artifact);
    let missing_destination = execute_session_tree_export_write(
        Some(&mut no_destination),
        SessionTreeExportWriteRequest {
            destination_capability: "",
            ..write_request(&artifact)
        },
    );
    let mut partial = FakeWriter::matching(&artifact);
    partial.outcome.committed = false;
    let partial_result =
        execute_session_tree_export_write(Some(&mut partial), write_request(&artifact));
    let mut unsynchronized = FakeWriter::matching(&artifact);
    unsynchronized.outcome.synchronized = false;
    let unsynchronized_result =
        execute_session_tree_export_write(Some(&mut unsynchronized), write_request(&artifact));

    assert_eq!(missing, Err(SessionTreeExportWriteError::MissingService));
    assert_eq!(invalid, Err(SessionTreeExportWriteError::InvalidWrite));
    assert_eq!(writer.calls, 0);
    assert_eq!(
        missing_destination,
        Err(SessionTreeExportWriteError::InvalidWrite)
    );
    assert_eq!(no_destination.calls, 0);
    assert_eq!(
        partial_result,
        Err(SessionTreeExportWriteError::InvalidWrite)
    );
    assert_eq!(partial.calls, WRITE_CALL_COUNT);
    assert_eq!(
        unsynchronized_result,
        Err(SessionTreeExportWriteError::InvalidWrite)
    );
}

#[test]
fn overwrite_denial_and_identity_or_size_mismatch_remain_host_failures() {
    let policy = parse_session_tree_export_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let artifact = artifact(&policy);
    let mut overwrite_denied = FakeWriter::matching(&artifact);
    overwrite_denied.expected_overwrite = true;
    overwrite_denied.should_fail = true;
    let overwrite_denial = execute_session_tree_export_write(
        Some(&mut overwrite_denied),
        SessionTreeExportWriteRequest {
            allow_overwrite: true,
            ..write_request(&artifact)
        },
    );
    let mut mismatch = FakeWriter::matching(&artifact);
    mismatch.outcome.artifact_hash = blake3::hash(b"mismatch").to_hex().to_string();
    let mismatch = execute_session_tree_export_write(Some(&mut mismatch), write_request(&artifact));
    let mut short_write = FakeWriter::matching(&artifact);
    short_write.outcome.bytes_written = 0;
    let short_write =
        execute_session_tree_export_write(Some(&mut short_write), write_request(&artifact));

    assert_eq!(
        overwrite_denial,
        Err(SessionTreeExportWriteError::Service(FakeWriteError))
    );
    assert_eq!(mismatch, Err(SessionTreeExportWriteError::InvalidWrite));
    assert_eq!(short_write, Err(SessionTreeExportWriteError::InvalidWrite));
}

fn artifact(
    policy: &SessionTreeExportPolicy,
) -> animus_runtime::session_tree_export::SessionTreeExportArtifact {
    let source = SessionTreeExportSourceInput {
        session_id: String::from("session.safe"),
        snapshot_hash: hash("snapshot"),
        replay_hash: hash("replay"),
        active_leaf_id: String::from(LEAF_ID),
        entries: vec![
            SessionTreeExportEntryInput {
                entry_id: String::from(ROOT_ID),
                parent_id: None,
                storage_sequence: FIRST_SEQUENCE,
                kind: SessionTreeExportEntryKind::SystemStatus,
                repair_target_id: None,
                safe_summary: String::from("session started"),
                entry_hash: hash(ROOT_ID),
                status: String::from("accepted"),
                omitted_field_count: 0,
            },
            SessionTreeExportEntryInput {
                entry_id: String::from(LEAF_ID),
                parent_id: Some(String::from(ROOT_ID)),
                storage_sequence: SECOND_SEQUENCE,
                kind: SessionTreeExportEntryKind::AssistantOmitted,
                repair_target_id: None,
                safe_summary: String::from("assistant body omitted"),
                entry_hash: hash(LEAF_ID),
                status: String::from("accepted"),
                omitted_field_count: 1,
            },
        ],
        safe_receipt_hashes: Vec::new(),
    };
    let projection =
        animus_runtime::session_tree_export::project_session_tree_export(policy, &source)
            .expect("projection");
    animus_runtime::session_tree_export::render_session_tree_export(
        policy,
        &projection,
        SessionTreeExportFormat::Html,
    )
    .expect("artifact")
}

fn write_request(
    artifact: &animus_runtime::session_tree_export::SessionTreeExportArtifact,
) -> SessionTreeExportWriteRequest<'_> {
    SessionTreeExportWriteRequest {
        destination_capability: DESTINATION_CAPABILITY,
        allow_overwrite: false,
        file_mode: PRIVATE_FILE_MODE,
        max_bytes: artifact.byte_count(),
        artifact,
    }
}

fn hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FakeWriteError;

struct FakeWriter {
    calls: u32,
    should_fail: bool,
    expected_overwrite: bool,
    expected_bytes: u32,
    outcome: SessionTreeExportWriteOutcome,
}

impl FakeWriter {
    fn matching(artifact: &animus_runtime::session_tree_export::SessionTreeExportArtifact) -> Self {
        Self {
            calls: 0,
            should_fail: false,
            expected_overwrite: false,
            expected_bytes: artifact.byte_count(),
            outcome: SessionTreeExportWriteOutcome {
                artifact_hash: String::from(artifact.artifact_hash()),
                bytes_written: artifact.byte_count(),
                committed: true,
                synchronized: true,
            },
        }
    }
}

impl SessionTreeExportWriteService for FakeWriter {
    type Error = FakeWriteError;

    fn write(
        &mut self,
        call: &SessionTreeExportWriteCall<'_>,
    ) -> Result<SessionTreeExportWriteOutcome, Self::Error> {
        self.calls = self
            .calls
            .checked_add(WRITE_CALL_COUNT)
            .expect("bounded write call count");
        assert_eq!(call.destination_capability, DESTINATION_CAPABILITY);
        assert_eq!(call.allow_overwrite, self.expected_overwrite);
        assert_eq!(call.file_mode, PRIVATE_FILE_MODE);
        assert!(!call.artifact_hash.is_empty());
        assert_eq!(
            u32::try_from(call.artifact_bytes.len()).expect("bounded artifact size"),
            self.expected_bytes
        );
        if self.should_fail {
            Err(FakeWriteError)
        } else {
            Ok(self.outcome.clone())
        }
    }
}
