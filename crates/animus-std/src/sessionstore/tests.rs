use super::*;

const GENERATED_POLICY_JSON: &str =
    include_str!("../../../../config/generated/durable-session-store-policy.json");
const POLICY_HASH: &str = "cb51ed6a1173d80feeac2e39011cc74ad0a5f51b52df2082b8b5af9643b5e68c";
const JOURNAL_NAME: &str = "session.journal";
const INVALID_JOURNAL_DIRECTORY: &str = "journal.directory";
const LOCK_NAME: &str = "session.lock";
const FIRST_LEASE: &str = "lease.first";
const SECOND_LEASE: &str = "lease.second";
const FIRST_SEQUENCE: u64 = 1;
const THIRD_SEQUENCE: u64 = 3;
const PRIVATE_USER: &[u8] = b"private durable user message";
const PRIVATE_ASSISTANT: &[u8] = b"private durable assistant message";
const EXPECTED_FRAME_COUNT: usize = 2;
const EXPECTED_COMMITTED_FRAMES: u32 = 2;
const LOCK_SYNC_CALL: u32 = 1;
const COMMIT_SYNC_CALL: u32 = 2;
const PRIVATE_FILE_MODE: u32 = 0o600;
const PERMISSION_BITS_MASK: u32 = 0o777;
const READ_ONLY_DIRECTORY_MODE: u32 = 0o500;
const WRITABLE_DIRECTORY_MODE: u32 = 0o700;
const CORRUPTION_MASK: u8 = 1;

#[test]
fn generated_policy_and_file_host_match_fake_recovery_across_restart() {
    // r[verify agent_harness_runtime.durable_session_store.boundary]
    // r[verify agent_harness_runtime.durable_session_store.commit]
    // r[verify agent_harness_runtime.durable_session_store.verification]
    let root = TestRoot::new("shared-corpus");
    let policy = parse_durable_session_store_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let descriptor = descriptor();
    let transaction = transaction(&policy, &descriptor);
    let encoded = animus_runtime::durable_session_store::encode_durable_session_transaction(
        &policy,
        &transaction,
    )
    .expect("encoded");
    let fake = animus_runtime::durable_session_store::recover_durable_session_log(
        &policy,
        &descriptor,
        encoded.as_slice(),
    )
    .expect("fake recovery");
    let mut store = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::passing()),
    )
    .expect("open");
    let outcome = store.commit(&transaction).expect("commit");
    assert!(outcome.synchronized);
    assert_eq!(outcome.committed_frame_count, EXPECTED_COMMITTED_FRAMES);
    assert_eq!(store.recovery().active_leaf_id(), fake.active_leaf_id());
    store.close().expect("close");

    let reopened = open_store(
        (&root.dir, &policy, &descriptor),
        (SECOND_LEASE, TestSynchronizer::passing()),
    )
    .expect("reopen");
    assert_eq!(
        reopened.recovery().committed_head_hash(),
        fake.committed_head_hash()
    );
    assert_eq!(
        reopened.recovery().committed_frames(),
        fake.committed_frames()
    );
    let metadata = root.dir.metadata(JOURNAL_NAME).expect("journal metadata");
    let observed_mode = cap_std::fs::PermissionsExt::mode(&metadata.permissions());
    assert_eq!(observed_mode & PERMISSION_BITS_MASK, PRIVATE_FILE_MODE);
    reopened.close().expect("reopened close");
}

#[test]
fn duplicate_entry_ids_fail_before_append() {
    let root = TestRoot::new("duplicate-entry");
    let policy = policy();
    let descriptor = descriptor();
    let first = transaction(&policy, &descriptor);
    let mut store = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::passing()),
    )
    .expect("open");
    store.commit(&first).expect("first commit");
    let bytes_before = read_raw(&root.dir);
    let duplicate = duplicate_transaction(&policy, &descriptor, first.commit().commit_hash());

    let error = store.commit(&duplicate).expect_err("duplicate rejected");

    assert_eq!(error.issue(), DurableSessionHostIssue::Validation);
    assert_eq!(read_raw(&root.dir), bytes_before, "journal stays unchanged");
    assert_eq!(
        store.recovery().committed_head_hash(),
        first.commit().commit_hash()
    );
    store.close().expect("close");
}

#[test]
fn conflicting_and_stale_writer_leases_fail_before_append() {
    // r[verify agent_harness_runtime.durable_session_store.locking]
    let root = TestRoot::new("locks");
    let policy = policy();
    let descriptor = descriptor();
    let store = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::passing()),
    )
    .expect("first writer");
    let conflict = open_store(
        (&root.dir, &policy, &descriptor),
        (SECOND_LEASE, TestSynchronizer::passing()),
    )
    .expect_err("conflicting writer");
    let stale = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::passing()),
    )
    .expect_err("stale lease");

    assert_eq!(conflict.issue(), DurableSessionHostIssue::LockConflict);
    assert_eq!(stale.issue(), DurableSessionHostIssue::StaleLease);
    assert_eq!(store.recovery().committed_frames().len(), 0);
    store.close().expect("close");
}

#[test]
fn failed_lock_setup_removes_the_unowned_lease() {
    let root = TestRoot::new("lock-setup-failure");
    let policy = policy();
    let descriptor = descriptor();
    let failure = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::failing_on(LOCK_SYNC_CALL)),
    )
    .expect_err("lock synchronization failure");

    assert_eq!(
        failure.issue(),
        DurableSessionHostIssue::SynchronizationFailed
    );
    let missing_lock = root.dir.metadata(LOCK_NAME).expect_err("lease removed");
    assert_eq!(missing_lock.kind(), std::io::ErrorKind::NotFound);
    let reopened = open_store(
        (&root.dir, &policy, &descriptor),
        (SECOND_LEASE, TestSynchronizer::passing()),
    )
    .expect("lock can be reacquired");
    reopened.close().expect("close reacquired lock");

    root.dir
        .create_dir(INVALID_JOURNAL_DIRECTORY)
        .expect("invalid journal directory fixture");
    let open_failure = open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy: &policy,
            descriptor: &descriptor,
            location: DurableSessionStoreLocation {
                root: &root.dir,
                journal_name: INVALID_JOURNAL_DIRECTORY,
                lock_name: LOCK_NAME,
            },
            lease_id: "lease.open-failure",
        },
        TestSynchronizer::passing(),
    )
    .expect_err("journal open failure");
    assert_eq!(open_failure.issue(), DurableSessionHostIssue::IoFailure);
    let missing_lock = root
        .dir
        .metadata(LOCK_NAME)
        .expect_err("open failure lease removed");
    assert_eq!(missing_lock.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn uncommitted_file_tail_is_ignored_but_committed_corruption_fails() {
    // r[verify agent_harness_runtime.durable_session_store.recovery]
    let root = TestRoot::new("recovery");
    let policy = policy();
    let descriptor = descriptor();
    let transaction = transaction(&policy, &descriptor);
    let mut store = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::passing()),
    )
    .expect("open");
    store.commit(&transaction).expect("commit");
    store.close().expect("close");

    let second = next_transaction(&policy, &descriptor, transaction.commit().commit_hash());
    let second_bytes =
        animus_runtime::durable_session_store::encode_durable_session_transaction(&policy, &second)
            .expect("second bytes");
    let commit_start = last_magic_offset(second_bytes.as_slice());
    append_raw(&root.dir, &second_bytes[..commit_start]);
    let reopened = open_store(
        (&root.dir, &policy, &descriptor),
        (SECOND_LEASE, TestSynchronizer::passing()),
    )
    .expect("reopen with ignored tail");
    assert_eq!(
        reopened.recovery().status(),
        animus_runtime::durable_session_store::DurableSessionRecoveryStatus::IncompleteTailTransaction
    );
    assert_eq!(
        reopened.recovery().committed_frames().len(),
        EXPECTED_FRAME_COUNT
    );
    reopened.close().expect("close ignored tail");

    let mut bytes = read_raw(&root.dir);
    let payload_offset = find_subslice(bytes.as_slice(), PRIVATE_USER);
    bytes[payload_offset] ^= CORRUPTION_MASK;
    overwrite_raw(&root.dir, bytes.as_slice());
    let corruption = open_store(
        (&root.dir, &policy, &descriptor),
        ("lease.corrupt", TestSynchronizer::passing()),
    )
    .expect_err("committed corruption");
    assert_eq!(corruption.issue(), DurableSessionHostIssue::Validation);
}

#[test]
fn synchronization_encryption_location_and_permission_failures_are_bounded() {
    let root = TestRoot::new("negative-host");
    let policy = policy();
    let descriptor = descriptor();
    let transaction = transaction(&policy, &descriptor);
    let mut store = open_store(
        (&root.dir, &policy, &descriptor),
        (FIRST_LEASE, TestSynchronizer::failing_on(COMMIT_SYNC_CALL)),
    )
    .expect("open before sync failure");
    let sync_failure = store.commit(&transaction).expect_err("sync failure");
    assert_eq!(
        sync_failure.issue(),
        DurableSessionHostIssue::SynchronizationFailed
    );
    store.close().expect("release lock after failed commit");

    let unsafe_location = open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy: &policy,
            descriptor: &descriptor,
            location: DurableSessionStoreLocation {
                root: &root.dir,
                journal_name: "/home/private/session",
                lock_name: "unsafe.lock",
            },
            lease_id: "lease.unsafe",
        },
        TestSynchronizer::passing(),
    )
    .expect_err("ambient location");
    assert_eq!(
        unsafe_location.issue(),
        DurableSessionHostIssue::InvalidAdmission
    );

    let mut encryption = descriptor.clone();
    encryption.encryption =
        animus_runtime::durable_session_store::DurableSessionEncryptionPosture::ExternalRequired;
    encryption.encryption_service_available = false;
    let encryption_error = open_store(
        (&root.dir, &policy, &encryption),
        ("lease.encryption", TestSynchronizer::passing()),
    )
    .expect_err("required encryption");
    assert_eq!(
        encryption_error.issue(),
        DurableSessionHostIssue::EncryptionUnavailable
    );

    root.set_mode(READ_ONLY_DIRECTORY_MODE);
    let permission = open_store(
        (&root.dir, &policy, &descriptor),
        ("lease.permission", TestSynchronizer::passing()),
    );
    root.set_mode(WRITABLE_DIRECTORY_MODE);
    #[cfg(unix)]
    assert_eq!(
        permission.expect_err("permission denial").issue(),
        DurableSessionHostIssue::IoFailure
    );
    #[cfg(not(unix))]
    drop(permission);
}

#[test]
fn malformed_generated_policy_and_bounded_reader_fail_closed() {
    let malformed =
        parse_durable_session_store_policy_json("{").expect_err("malformed generated policy");
    let unknown = GENERATED_POLICY_JSON.replacen('{', "{\n  \"private_path\": true,", 1);
    let unknown = parse_durable_session_store_policy_json(unknown.as_str())
        .expect_err("unknown generated field");
    assert!(matches!(
        malformed,
        DurableSessionStoreConfigError::Json {
            kind: DurableSessionStoreJsonErrorKind::InvalidJson,
        }
    ));
    assert!(matches!(
        unknown,
        DurableSessionStoreConfigError::Json {
            kind: DurableSessionStoreJsonErrorKind::InvalidShape,
        }
    ));

    let root = TestRoot::new("reader");
    let policy = policy();
    let descriptor = descriptor();
    let missing = read_durable_session_snapshot(&root.dir, JOURNAL_NAME, &policy, &descriptor)
        .expect_err("missing journal");
    assert_eq!(missing.issue(), DurableSessionHostIssue::IoFailure);
}

fn policy() -> DurableSessionStorePolicy {
    parse_durable_session_store_policy_json(GENERATED_POLICY_JSON).expect("policy")
}

fn descriptor() -> DurableSessionStoreDescriptor {
    DurableSessionStoreDescriptor {
        store_id: String::from("store.local"),
        location_capability: String::from("session.store.reviewed"),
        session_id: String::from("session.durable"),
        branch_id: String::from("branch.main"),
        policy_hash: String::from(POLICY_HASH),
        durability:
            animus_runtime::durable_session_store::DurableSessionDurability::SyncAllEachCommit,
        locking:
            animus_runtime::durable_session_store::DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data:
            animus_runtime::durable_session_store::DurableSessionPrivacyPosture::ReplayPrivate,
        encryption:
            animus_runtime::durable_session_store::DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from("metadata.only"),
    }
}

fn transaction(
    policy: &DurableSessionStorePolicy,
    descriptor: &DurableSessionStoreDescriptor,
) -> DurableSessionTransaction {
    let entries = [
        animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
            entry_id: "entry.user",
            parent_id: None,
            kind: animus_runtime::durable_session_store::DurableSessionRecordKind::User,
            payload: PRIVATE_USER,
        },
        animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
            entry_id: "entry.assistant",
            parent_id: Some("entry.user"),
            kind: animus_runtime::durable_session_store::DurableSessionRecordKind::Assistant,
            payload: PRIVATE_ASSISTANT,
        },
    ];
    animus_runtime::durable_session_store::plan_durable_session_transaction(
        policy,
        &animus_runtime::durable_session_store::DurableSessionTransactionDraft {
            descriptor,
            prior_head_hash: animus_runtime::durable_session_store::DURABLE_SESSION_EMPTY_HEAD_HASH,
            prior_active_leaf_id: None,
            first_sequence: FIRST_SEQUENCE,
            active_leaf_id: "entry.assistant",
            entries: &entries,
        },
    )
    .expect("transaction")
}

fn duplicate_transaction(
    policy: &DurableSessionStorePolicy,
    descriptor: &DurableSessionStoreDescriptor,
    prior_head_hash: &str,
) -> DurableSessionTransaction {
    let entries = [
        animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
            entry_id: "entry.user",
            parent_id: Some("entry.assistant"),
            kind: animus_runtime::durable_session_store::DurableSessionRecordKind::User,
            payload: b"duplicate user entry",
        },
    ];
    animus_runtime::durable_session_store::plan_durable_session_transaction(
        policy,
        &animus_runtime::durable_session_store::DurableSessionTransactionDraft {
            descriptor,
            prior_head_hash,
            prior_active_leaf_id: Some("entry.assistant"),
            first_sequence: THIRD_SEQUENCE,
            active_leaf_id: "entry.user",
            entries: &entries,
        },
    )
    .expect("duplicate transaction fixture")
}

fn next_transaction(
    policy: &DurableSessionStorePolicy,
    descriptor: &DurableSessionStoreDescriptor,
    prior_head_hash: &str,
) -> DurableSessionTransaction {
    let entries = [
        animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
            entry_id: "entry.repair",
            parent_id: Some("entry.assistant"),
            kind: animus_runtime::durable_session_store::DurableSessionRecordKind::Repair,
            payload: b"private repair",
        },
    ];
    animus_runtime::durable_session_store::plan_durable_session_transaction(
        policy,
        &animus_runtime::durable_session_store::DurableSessionTransactionDraft {
            descriptor,
            prior_head_hash,
            prior_active_leaf_id: Some("entry.assistant"),
            first_sequence: THIRD_SEQUENCE,
            active_leaf_id: "entry.repair",
            entries: &entries,
        },
    )
    .expect("next transaction")
}

fn open_store<'store>(
    context: (
        &'store CapabilityDir,
        &'store DurableSessionStorePolicy,
        &'store DurableSessionStoreDescriptor,
    ),
    lease_and_sync: (&'store str, TestSynchronizer),
) -> Result<DurableSessionLocalStore<'store, TestSynchronizer>, DurableSessionHostError> {
    let (root, policy, descriptor) = context;
    let (lease_id, synchronizer) = lease_and_sync;
    open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy,
            descriptor,
            location: DurableSessionStoreLocation {
                root,
                journal_name: JOURNAL_NAME,
                lock_name: LOCK_NAME,
            },
            lease_id,
        },
        synchronizer,
    )
}

#[derive(Debug)]
struct TestSynchronizer {
    calls: u32,
    fail_on_call: Option<u32>,
}

impl TestSynchronizer {
    const fn passing() -> Self {
        Self {
            calls: 0,
            fail_on_call: None,
        }
    }

    const fn failing_on(call: u32) -> Self {
        Self {
            calls: 0,
            fail_on_call: Some(call),
        }
    }
}

impl DurableSessionSynchronizer for TestSynchronizer {
    fn synchronize(&mut self, file: &CapabilityFile) -> Result<(), std::io::Error> {
        self.calls = self
            .calls
            .checked_add(LOCK_SYNC_CALL)
            .expect("bounded sync calls");
        if self.fail_on_call == Some(self.calls) {
            return Err(std::io::Error::other("injected synchronization failure"));
        }
        file.sync_all()
    }
}

struct TestRoot {
    path: std::path::PathBuf,
    dir: CapabilityDir,
}

impl TestRoot {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "animus-durable-session-{label}-{}",
            std::process::id()
        ));
        let _ignored = std::fs::remove_dir_all(path.as_path());
        std::fs::create_dir_all(path.as_path()).expect("test root");
        let dir = CapabilityDir::open_ambient_dir(path.as_path(), cap_std::ambient_authority())
            .expect("capability root");
        Self { path, dir }
    }

    #[cfg(unix)]
    fn set_mode(&self, mode: u32) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(self.path.as_path(), std::fs::Permissions::from_mode(mode))
            .expect("test root mode");
    }

    #[cfg(not(unix))]
    fn set_mode(&self, _mode: u32) {}
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ignored = std::fs::remove_dir_all(self.path.as_path());
    }
}

fn append_raw(root: &CapabilityDir, bytes: &[u8]) {
    let mut options = cap_std::fs::OpenOptions::new();
    options.append(true);
    let mut file = root
        .open_with(JOURNAL_NAME, &options)
        .expect("append journal");
    file.write_all(bytes).expect("append raw bytes");
    file.sync_all().expect("sync raw bytes");
}

fn read_raw(root: &CapabilityDir) -> Vec<u8> {
    let mut file = root.open(JOURNAL_NAME).expect("read journal");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("journal bytes");
    bytes
}

fn overwrite_raw(root: &CapabilityDir, bytes: &[u8]) {
    let mut options = cap_std::fs::OpenOptions::new();
    options.write(true).truncate(true);
    let mut file = root
        .open_with(JOURNAL_NAME, &options)
        .expect("rewrite journal fixture");
    file.write_all(bytes).expect("corrupt journal fixture");
    file.sync_all().expect("sync corrupt fixture");
}

fn last_magic_offset(bytes: &[u8]) -> usize {
    bytes
        .windows(8)
        .rposition(|window| window == b"ANIMSJ01")
        .expect("commit record")
}

fn find_subslice(bytes: &[u8], needle: &[u8]) -> usize {
    bytes
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("payload")
}
