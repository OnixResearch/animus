//! Injected session-persistence adapter contracts.
//!
//! The functions in this module are deterministic state transformers over safe
//! records. Hosts own filesystem, database, encryption, locking, clock, daemon,
//! and recovery effects.

// Cairn adaptation compatibility module keeps DTOs, validators, and fixtures compact while the change is drained.
#![allow(missing_docs)]
#![allow(
    unknown_lints,
    ambiguous_params,
    assertion_density,
    bool_naming,
    borrowed_argument_types
)]
#![allow(
    compound_condition,
    excessive_file_length,
    function_length,
    non_trait_imports,
    path_segment_repetition,
    underscore_in_module_filename,
    dead_code
)]
#![allow(numeric_units, raw_arithmetic_overflow, unbounded_collection_growth)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

const HASH_DOMAIN: &str = "animus.session-persistence.v1";
const HASH_SEPARATOR: &str = "|";
const SUPPORTED_SCHEMA_VERSION: u16 = 1;
const EMPTY_COUNT: u32 = 0;
const MIN_COUNT: u32 = 1;
const MIN_FRESHNESS_SEQUENCE: u64 = 1;
const MAX_LABEL_CHARS: usize = 96;
const HASH_HEX_CHARS: usize = 64;
const GLOBAL_STORE_LABEL: &str = "global-default";
const UNSAFE_MARKERS: &[&str] = &[
    "RAW_PROMPT:",
    "RAW_MODEL_OUTPUT:",
    "RAW_TOOL_BODY:",
    "PROVIDER_PAYLOAD:",
    "BEGIN PRIVATE KEY",
    "API_KEY=",
    "sk-",
    "/home/",
    "UNBOUNDED_LOG:",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistenceServiceState {
    Available,
    Missing,
    Unavailable,
    PartialWrite,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionStoreDescriptor {
    pub store_label: String,
    pub schema_version: u16,
    pub service_state: PersistenceServiceState,
    pub global_default: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistenceAdmission {
    pub store: Option<SessionStoreDescriptor>,
    pub session_id: String,
    pub branch_id: String,
    pub redaction_policy: String,
    pub receipt_policy: String,
    pub minimum_snapshot_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafeSessionRecord {
    pub schema_version: u16,
    pub session_id: String,
    pub branch_id: String,
    pub parent_record_hash: Option<String>,
    pub record_hash: String,
    pub redaction_class: String,
    pub event_count: u32,
    pub token_count: u32,
    pub status_class: SessionRecordStatus,
    pub adapter_label: String,
    pub replay_sequence: u64,
    pub leak_marker: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRecordStatus {
    Active,
    Snapshot,
    Restored,
    Compacted,
    Tombstoned,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FakeSessionState {
    pub records: Vec<SafeSessionRecord>,
    pub tombstoned: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySummary {
    pub session_id: String,
    pub branch_id: String,
    pub record_count: usize,
    pub terminal_status: SessionRecordStatus,
    pub replay_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompactionSummary {
    pub session_id: String,
    pub branch_id: String,
    pub input_records: usize,
    pub summary_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistenceIssue {
    MissingStore,
    UnavailableBackend,
    PartialWrite,
    GlobalStoreDefault,
    SchemaMismatch,
    UnsafeRecord,
    LeakRejected,
    ReplayGap,
    RestoreConflict,
    TombstoneConflict,
    StaleSnapshot,
    EmptyField,
    InvalidHash,
}

impl PersistenceIssue {
    pub const fn code(self) -> &'static str {
        match self {
            Self::MissingStore => "missing-store",
            Self::UnavailableBackend => "unavailable-backend",
            Self::PartialWrite => "partial-write",
            Self::GlobalStoreDefault => "global-store-default",
            Self::SchemaMismatch => "schema-mismatch",
            Self::UnsafeRecord => "unsafe-record",
            Self::LeakRejected => "leak-rejected",
            Self::ReplayGap => "replay-gap",
            Self::RestoreConflict => "restore-conflict",
            Self::TombstoneConflict => "tombstone-conflict",
            Self::StaleSnapshot => "stale-snapshot",
            Self::EmptyField => "empty-field",
            Self::InvalidHash => "invalid-hash",
        }
    }
}

pub type PersistenceResult<T> = Result<T, Vec<PersistenceIssue>>;

pub fn admit_persistence(admission: &PersistenceAdmission) -> PersistenceResult<()> {
    let mut issues = Vec::new();
    validate_label(admission.session_id.as_str(), &mut issues);
    validate_label(admission.branch_id.as_str(), &mut issues);
    validate_label(admission.redaction_policy.as_str(), &mut issues);
    validate_label(admission.receipt_policy.as_str(), &mut issues);
    if admission.minimum_snapshot_sequence < MIN_FRESHNESS_SEQUENCE {
        issues.push(PersistenceIssue::StaleSnapshot);
    }
    match &admission.store {
        None => issues.push(PersistenceIssue::MissingStore),
        Some(store) => validate_store(store, &mut issues),
    }
    result_from_issues(issues, ())
}

pub fn validate_session_record(record: &SafeSessionRecord) -> PersistenceResult<()> {
    let mut issues = Vec::new();
    if record.schema_version != SUPPORTED_SCHEMA_VERSION {
        issues.push(PersistenceIssue::SchemaMismatch);
    }
    validate_label(record.session_id.as_str(), &mut issues);
    validate_label(record.branch_id.as_str(), &mut issues);
    validate_label(record.redaction_class.as_str(), &mut issues);
    validate_label(record.adapter_label.as_str(), &mut issues);
    validate_hash(record.record_hash.as_str(), &mut issues);
    if let Some(parent_hash) = &record.parent_record_hash {
        validate_hash(parent_hash.as_str(), &mut issues);
    }
    if record.event_count == EMPTY_COUNT {
        issues.push(PersistenceIssue::UnsafeRecord);
    }
    if let Some(marker) = &record.leak_marker {
        if UNSAFE_MARKERS
            .iter()
            .any(|unsafe_marker| marker.contains(unsafe_marker))
        {
            issues.push(PersistenceIssue::LeakRejected);
        }
    }
    result_from_issues(issues, ())
}

pub fn append_fake_record(
    admission: &PersistenceAdmission,
    state: &FakeSessionState,
    record: SafeSessionRecord,
) -> PersistenceResult<FakeSessionState> {
    admit_persistence(admission)?;
    validate_session_record(&record)?;
    if state.tombstoned {
        return Err(vec![PersistenceIssue::TombstoneConflict]);
    }
    if matches_store_state(admission, PersistenceServiceState::PartialWrite) {
        return Err(vec![PersistenceIssue::PartialWrite]);
    }
    let mut records = state.records.clone();
    records.push(record);
    Ok(FakeSessionState {
        records,
        tombstoned: false,
    })
}

pub fn replay_fake_session(
    admission: &PersistenceAdmission,
    state: &FakeSessionState,
) -> PersistenceResult<ReplaySummary> {
    admit_persistence(admission)?;
    let filtered = records_for_scope(admission, state)?;
    validate_replay_sequence(&filtered)?;
    let terminal_status = filtered
        .last()
        .map_or(SessionRecordStatus::Active, |record| record.status_class);
    Ok(ReplaySummary {
        session_id: admission.session_id.clone(),
        branch_id: admission.branch_id.clone(),
        record_count: filtered.len(),
        terminal_status,
        replay_hash: state_hash(&filtered),
    })
}

pub fn restore_fake_snapshot(
    admission: &PersistenceAdmission,
    state: &FakeSessionState,
    snapshot_sequence: u64,
) -> PersistenceResult<ReplaySummary> {
    if snapshot_sequence < admission.minimum_snapshot_sequence {
        return Err(vec![PersistenceIssue::StaleSnapshot]);
    }
    let summary = replay_fake_session(admission, state)?;
    let has_snapshot = state.records.iter().any(|record| {
        record.session_id == admission.session_id
            && record.branch_id == admission.branch_id
            && record.replay_sequence == snapshot_sequence
            && record.status_class == SessionRecordStatus::Snapshot
    });
    if !has_snapshot {
        return Err(vec![PersistenceIssue::RestoreConflict]);
    }
    Ok(summary)
}

pub fn compact_fake_records(
    admission: &PersistenceAdmission,
    state: &FakeSessionState,
) -> PersistenceResult<CompactionSummary> {
    let filtered = records_for_scope(admission, state)?;
    Ok(CompactionSummary {
        session_id: admission.session_id.clone(),
        branch_id: admission.branch_id.clone(),
        input_records: filtered.len(),
        summary_hash: state_hash(&filtered),
    })
}

pub fn tombstone_fake_session(
    admission: &PersistenceAdmission,
    state: &FakeSessionState,
) -> PersistenceResult<FakeSessionState> {
    admit_persistence(admission)?;
    if state.tombstoned {
        return Err(vec![PersistenceIssue::TombstoneConflict]);
    }
    Ok(FakeSessionState {
        records: state.records.clone(),
        tombstoned: true,
    })
}

pub fn session_record_hash(label: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(HASH_DOMAIN.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(label.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}

fn validate_store(store: &SessionStoreDescriptor, issues: &mut Vec<PersistenceIssue>) {
    validate_label(store.store_label.as_str(), issues);
    if store.schema_version != SUPPORTED_SCHEMA_VERSION {
        issues.push(PersistenceIssue::SchemaMismatch);
    }
    match store.service_state {
        PersistenceServiceState::Available => {}
        PersistenceServiceState::Missing => issues.push(PersistenceIssue::MissingStore),
        PersistenceServiceState::Unavailable => issues.push(PersistenceIssue::UnavailableBackend),
        PersistenceServiceState::PartialWrite => issues.push(PersistenceIssue::PartialWrite),
    }
    if store.global_default || store.store_label == GLOBAL_STORE_LABEL {
        issues.push(PersistenceIssue::GlobalStoreDefault);
    }
}

fn matches_store_state(admission: &PersistenceAdmission, state: PersistenceServiceState) -> bool {
    admission
        .store
        .as_ref()
        .is_some_and(|store| store.service_state == state)
}

fn records_for_scope<'a>(
    admission: &PersistenceAdmission,
    state: &'a FakeSessionState,
) -> PersistenceResult<Vec<&'a SafeSessionRecord>> {
    admit_persistence(admission)?;
    let records = state
        .records
        .iter()
        .filter(|record| {
            record.session_id == admission.session_id && record.branch_id == admission.branch_id
        })
        .collect::<Vec<_>>();
    if records.is_empty() {
        return Err(vec![PersistenceIssue::ReplayGap]);
    }
    for record in &records {
        validate_session_record(record)?;
    }
    Ok(records)
}

fn validate_replay_sequence(records: &[&SafeSessionRecord]) -> PersistenceResult<()> {
    for (expected, record) in (MIN_FRESHNESS_SEQUENCE..).zip(records.iter()) {
        if record.replay_sequence != expected {
            return Err(vec![PersistenceIssue::ReplayGap]);
        }
    }
    Ok(())
}

fn state_hash(records: &[&SafeSessionRecord]) -> String {
    let canonical = records
        .iter()
        .map(|record| record.record_hash.as_str())
        .collect::<Vec<_>>()
        .join(HASH_SEPARATOR);
    session_record_hash("state", canonical.as_bytes())
}

fn validate_label(value: &str, issues: &mut Vec<PersistenceIssue>) {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_LABEL_CHARS {
        issues.push(PersistenceIssue::EmptyField);
    }
    if UNSAFE_MARKERS.iter().any(|marker| trimmed.contains(marker)) {
        issues.push(PersistenceIssue::UnsafeRecord);
    }
}

fn validate_hash(value: &str, issues: &mut Vec<PersistenceIssue>) {
    if value.len() != HASH_HEX_CHARS || !value.as_bytes().iter().all(u8::is_ascii_hexdigit) {
        issues.push(PersistenceIssue::InvalidHash);
    }
}

fn result_from_issues<T>(mut issues: Vec<PersistenceIssue>, value: T) -> PersistenceResult<T> {
    issues.sort_by_key(|issue| issue.code());
    issues.dedup();
    if issues.is_empty() {
        Ok(value)
    } else {
        Err(issues)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION_ID: &str = "session.a";
    const BRANCH_ID: &str = "branch.main";
    const REDACTION: &str = "safe-summary-only";
    const RECEIPT_POLICY: &str = "receipt-hash-only";
    const STORE_LABEL: &str = "in-memory-fake-store";
    const ADAPTER_LABEL: &str = "fake-model";
    const FIRST_RECORD_BYTES: &[u8] = b"first safe record";
    const SECOND_RECORD_BYTES: &[u8] = b"second safe record";
    const THIRD_RECORD_BYTES: &[u8] = b"third safe record";
    const EVENT_COUNT: u32 = 2;
    const TOKEN_COUNT: u32 = 5;
    const FIRST_SEQUENCE: u64 = 1;
    const SECOND_SEQUENCE: u64 = 2;
    const THIRD_SEQUENCE: u64 = 3;

    #[test]
    fn fake_store_appends_replays_restores_compacts_and_tombstones() {
        // r[verify agent_harness_runtime.session_persistence_adapter.service]
        // r[verify agent_harness_runtime.session_persistence_adapter.fake_store]
        let admission_request = admission(PersistenceServiceState::Available);
        let state = FakeSessionState {
            records: Vec::new(),
            tombstoned: false,
        };
        let first = record(
            FIRST_SEQUENCE,
            SessionRecordStatus::Active,
            FIRST_RECORD_BYTES,
            None,
        );
        let second = record(
            SECOND_SEQUENCE,
            SessionRecordStatus::Snapshot,
            SECOND_RECORD_BYTES,
            Some(first.record_hash.clone()),
        );
        let third = record(
            THIRD_SEQUENCE,
            SessionRecordStatus::Compacted,
            THIRD_RECORD_BYTES,
            Some(second.record_hash.clone()),
        );

        let state = append_fake_record(&admission_request, &state, first).expect("first append");
        let state = append_fake_record(&admission_request, &state, second).expect("second append");
        let state = append_fake_record(&admission_request, &state, third).expect("third append");
        let replay = replay_fake_session(&admission_request, &state).expect("replay");
        let restored =
            restore_fake_snapshot(&admission_request, &state, SECOND_SEQUENCE).expect("restore");
        let compacted = compact_fake_records(&admission_request, &state).expect("compact");
        let tombstoned = tombstone_fake_session(&admission_request, &state).expect("tombstone");

        assert_eq!(
            replay.record_count,
            usize::try_from(THIRD_SEQUENCE).expect("fits")
        );
        assert_eq!(replay.terminal_status, SessionRecordStatus::Compacted);
        assert_eq!(restored.session_id, SESSION_ID);
        assert_eq!(compacted.input_records, replay.record_count);
        assert!(tombstoned.tombstoned);
    }

    #[test]
    fn admission_and_records_fail_closed_for_missing_store_and_unsafe_data() {
        // r[verify agent_harness_runtime.session_persistence_adapter.admission]
        // r[verify agent_harness_runtime.session_persistence_adapter.safe_records]
        let mut missing = admission(PersistenceServiceState::Available);
        missing.store = None;
        assert_eq!(
            admit_persistence(&missing),
            Err(vec![PersistenceIssue::MissingStore])
        );

        let global = PersistenceAdmission {
            store: Some(SessionStoreDescriptor {
                store_label: GLOBAL_STORE_LABEL.to_owned(),
                schema_version: SUPPORTED_SCHEMA_VERSION,
                service_state: PersistenceServiceState::Available,
                global_default: true,
            }),
            ..admission(PersistenceServiceState::Available)
        };
        assert!(
            admit_persistence(&global)
                .expect_err("global default")
                .contains(&PersistenceIssue::GlobalStoreDefault)
        );

        let unsafe_record = SafeSessionRecord {
            leak_marker: Some("RAW_PROMPT:secret".to_owned()),
            ..record(
                FIRST_SEQUENCE,
                SessionRecordStatus::Active,
                FIRST_RECORD_BYTES,
                None,
            )
        };
        assert!(
            validate_session_record(&unsafe_record)
                .expect_err("leak")
                .contains(&PersistenceIssue::LeakRejected)
        );
    }

    #[test]
    fn replay_restore_and_backend_failures_are_typed() {
        // r[verify agent_harness_runtime.session_persistence_adapter.failures]
        // r[verify agent_harness_runtime.session_persistence_adapter.verification]
        let admission_request = admission(PersistenceServiceState::Available);
        let gap_state = FakeSessionState {
            records: vec![record(
                SECOND_SEQUENCE,
                SessionRecordStatus::Active,
                SECOND_RECORD_BYTES,
                None,
            )],
            tombstoned: false,
        };
        assert_eq!(
            replay_fake_session(&admission_request, &gap_state),
            Err(vec![PersistenceIssue::ReplayGap])
        );
        let no_snapshot_state = FakeSessionState {
            records: vec![record(
                FIRST_SEQUENCE,
                SessionRecordStatus::Active,
                FIRST_RECORD_BYTES,
                None,
            )],
            tombstoned: false,
        };
        assert_eq!(
            restore_fake_snapshot(&admission_request, &no_snapshot_state, FIRST_SEQUENCE),
            Err(vec![PersistenceIssue::RestoreConflict])
        );

        let partial = admission(PersistenceServiceState::PartialWrite);
        let append_result = append_fake_record(
            &partial,
            &FakeSessionState {
                records: Vec::new(),
                tombstoned: false,
            },
            record(
                FIRST_SEQUENCE,
                SessionRecordStatus::Active,
                FIRST_RECORD_BYTES,
                None,
            ),
        );
        assert!(
            append_result
                .expect_err("partial write")
                .contains(&PersistenceIssue::PartialWrite)
        );

        let stale = PersistenceAdmission {
            minimum_snapshot_sequence: EMPTY_COUNT.into(),
            ..admission(PersistenceServiceState::Available)
        };
        assert!(
            admit_persistence(&stale)
                .expect_err("stale")
                .contains(&PersistenceIssue::StaleSnapshot)
        );
    }

    fn admission(state: PersistenceServiceState) -> PersistenceAdmission {
        PersistenceAdmission {
            store: Some(SessionStoreDescriptor {
                store_label: STORE_LABEL.to_owned(),
                schema_version: SUPPORTED_SCHEMA_VERSION,
                service_state: state,
                global_default: false,
            }),
            session_id: SESSION_ID.to_owned(),
            branch_id: BRANCH_ID.to_owned(),
            redaction_policy: REDACTION.to_owned(),
            receipt_policy: RECEIPT_POLICY.to_owned(),
            minimum_snapshot_sequence: FIRST_SEQUENCE,
        }
    }

    fn record(
        sequence: u64,
        status: SessionRecordStatus,
        bytes: &[u8],
        parent_record_hash: Option<String>,
    ) -> SafeSessionRecord {
        SafeSessionRecord {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            session_id: SESSION_ID.to_owned(),
            branch_id: BRANCH_ID.to_owned(),
            parent_record_hash,
            record_hash: session_record_hash("record", bytes),
            redaction_class: REDACTION.to_owned(),
            event_count: EVENT_COUNT,
            token_count: TOKEN_COUNT,
            status_class: status,
            adapter_label: ADAPTER_LABEL.to_owned(),
            replay_sequence: sequence,
            leak_marker: None,
        }
    }
}
