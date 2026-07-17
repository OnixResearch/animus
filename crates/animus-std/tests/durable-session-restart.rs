//! Real child-process restart coverage for the durable session-store host.

use std::io::{self, Write};
use std::process::{Command, ExitStatus};

use animus_runtime::durable_session_store::{
    DURABLE_SESSION_EMPTY_HEAD_HASH, DurableSessionDurability, DurableSessionEncryptionPosture,
    DurableSessionLockPolicy, DurableSessionPrivacyPosture, DurableSessionPrivateEntryDraft,
    DurableSessionRecordKind, DurableSessionRecoveryStatus, DurableSessionStoreDescriptor,
    DurableSessionTransactionDraft, encode_durable_session_transaction,
    plan_durable_session_transaction,
};
use animus_std::{
    DurableSessionLocalStore, DurableSessionStoreLocation, DurableSessionStoreOpenRequest,
    DurableSessionSynchronizer, open_durable_session_store,
    parse_durable_session_store_policy_json,
};
use cap_std::fs::{Dir as CapabilityDir, File as CapabilityFile};

const GENERATED_POLICY_JSON: &str =
    include_str!("../../../config/generated/durable-session-store-policy.json");
const POLICY_HASH: &str = "cb51ed6a1173d80feeac2e39011cc74ad0a5f51b52df2082b8b5af9643b5e68c";
const CHILD_PHASE_ENV: &str = "ANIMUS_DURABLE_SESSION_RESTART_PHASE";
const CHILD_ROOT_ENV: &str = "ANIMUS_DURABLE_SESSION_RESTART_ROOT";
const WRITE_PHASE: &str = "write";
const READ_PHASE: &str = "read";
const TEST_FILTER: &str = "durable_session_restart_survives_child_processes";
const JOURNAL_NAME: &str = "session.journal";
const LOCK_NAME: &str = "session.lock";
const SESSION_ID: &str = "session.restart";
const BRANCH_ID: &str = "branch.main";
const USER_ENTRY_ID: &str = "entry.user";
const ASSISTANT_ENTRY_ID: &str = "entry.assistant";
const UNCOMMITTED_REPAIR_ENTRY_ID: &str = "entry.uncommitted-repair";
const WRITE_LEASE_ID: &str = "lease.writer-child";
const READ_LEASE_ID: &str = "lease.reader-child";
const FIRST_SEQUENCE: u64 = 1;
const THIRD_SEQUENCE: u64 = 3;
const FRAME_MAGIC_BYTES: usize = 8;
const EXPECTED_FRAME_COUNT: usize = 2;
const PRIVATE_USER: &[u8] = b"restart-private-user";
const PRIVATE_ASSISTANT: &[u8] = b"restart-private-assistant";
const PRIVATE_UNCOMMITTED_REPAIR: &[u8] = b"restart-private-uncommitted-repair";

#[test]
fn durable_session_restart_survives_child_processes() {
    // r[verify agent_harness_runtime.durable_session_store.commit]
    // r[verify agent_harness_runtime.durable_session_store.recovery]
    // r[verify agent_harness_runtime.durable_session_store.verification]
    let Some(phase) = std::env::var_os(CHILD_PHASE_ENV) else {
        run_parent();
        return;
    };
    let root_path = std::env::var_os(CHILD_ROOT_ENV).expect("child root capability path");
    let root = CapabilityDir::open_ambient_dir(root_path, cap_std::ambient_authority())
        .expect("open test capability root");
    match phase.to_str() {
        Some(WRITE_PHASE) => write_child(&root),
        Some(READ_PHASE) => read_child(&root),
        _ => panic!("unsupported restart fixture phase"),
    }
}

fn run_parent() {
    let root_path = std::env::temp_dir().join(format!(
        "animus-durable-session-restart-{}",
        std::process::id()
    ));
    let _ignored = std::fs::remove_dir_all(root_path.as_path());
    std::fs::create_dir_all(root_path.as_path()).expect("create restart fixture root");

    let writer_status = run_child(WRITE_PHASE, root_path.as_os_str());
    assert!(
        writer_status.success(),
        "writer child failed: {writer_status}"
    );
    let reader_status = run_child(READ_PHASE, root_path.as_os_str());
    assert!(
        reader_status.success(),
        "reader child failed: {reader_status}"
    );

    std::fs::remove_dir_all(root_path).expect("remove restart fixture root");
}

fn run_child(phase: &str, root_path: &std::ffi::OsStr) -> ExitStatus {
    Command::new(std::env::current_exe().expect("current restart test executable"))
        .arg("--exact")
        .arg(TEST_FILTER)
        .arg("--nocapture")
        .env(CHILD_PHASE_ENV, phase)
        .env(CHILD_ROOT_ENV, root_path)
        .status()
        .expect("run restart fixture child")
}

fn write_child(root: &CapabilityDir) {
    let policy = parse_durable_session_store_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let descriptor = descriptor();
    let entries = [
        DurableSessionPrivateEntryDraft {
            entry_id: USER_ENTRY_ID,
            parent_id: None,
            kind: DurableSessionRecordKind::User,
            payload: PRIVATE_USER,
        },
        DurableSessionPrivateEntryDraft {
            entry_id: ASSISTANT_ENTRY_ID,
            parent_id: Some(USER_ENTRY_ID),
            kind: DurableSessionRecordKind::Assistant,
            payload: PRIVATE_ASSISTANT,
        },
    ];
    let transaction = plan_durable_session_transaction(
        &policy,
        &DurableSessionTransactionDraft {
            descriptor: &descriptor,
            prior_head_hash: DURABLE_SESSION_EMPTY_HEAD_HASH,
            prior_active_leaf_id: None,
            first_sequence: FIRST_SEQUENCE,
            active_leaf_id: ASSISTANT_ENTRY_ID,
            entries: &entries,
        },
    )
    .expect("plan restart transaction");
    let mut store = open(root, &policy, &descriptor, WRITE_LEASE_ID);
    let outcome = store
        .commit(&transaction)
        .expect("commit restart transaction");
    assert!(outcome.synchronized);
    store.close().expect("close writer child");

    let uncommitted_entries = [DurableSessionPrivateEntryDraft {
        entry_id: UNCOMMITTED_REPAIR_ENTRY_ID,
        parent_id: Some(ASSISTANT_ENTRY_ID),
        kind: DurableSessionRecordKind::Repair,
        payload: PRIVATE_UNCOMMITTED_REPAIR,
    }];
    let uncommitted = plan_durable_session_transaction(
        &policy,
        &DurableSessionTransactionDraft {
            descriptor: &descriptor,
            prior_head_hash: transaction.commit().commit_hash(),
            prior_active_leaf_id: Some(ASSISTANT_ENTRY_ID),
            first_sequence: THIRD_SEQUENCE,
            active_leaf_id: UNCOMMITTED_REPAIR_ENTRY_ID,
            entries: &uncommitted_entries,
        },
    )
    .expect("plan uncommitted restart tail");
    let uncommitted_bytes = encode_durable_session_transaction(&policy, &uncommitted)
        .expect("encode uncommitted restart tail");
    let commit_start = uncommitted_bytes
        .windows(FRAME_MAGIC_BYTES)
        .rposition(|window| window == b"ANIMSJ01")
        .expect("uncommitted commit record");
    let mut options = cap_std::fs::OpenOptions::new();
    options.append(true);
    let mut journal = root
        .open_with(JOURNAL_NAME, &options)
        .expect("open restart journal tail");
    journal
        .write_all(&uncommitted_bytes[..commit_start])
        .expect("append uncommitted restart tail");
    journal.sync_all().expect("sync uncommitted restart tail");
}

fn read_child(root: &CapabilityDir) {
    let policy = parse_durable_session_store_policy_json(GENERATED_POLICY_JSON).expect("policy");
    let descriptor = descriptor();
    let store = open(root, &policy, &descriptor, READ_LEASE_ID);
    let recovery = store.recovery();
    assert_eq!(
        recovery.status(),
        DurableSessionRecoveryStatus::IncompleteTailTransaction
    );
    assert_eq!(recovery.committed_frames().len(), EXPECTED_FRAME_COUNT);
    assert_eq!(recovery.active_leaf_id(), Some(ASSISTANT_ENTRY_ID));
    assert_eq!(recovery.committed_frames()[0].entry_id(), USER_ENTRY_ID);
    assert_eq!(recovery.committed_frames()[0].parent_id(), None);
    assert_eq!(
        recovery.committed_frames()[0].private_payload(),
        PRIVATE_USER
    );
    assert_eq!(
        recovery.committed_frames()[1].entry_id(),
        ASSISTANT_ENTRY_ID
    );
    assert_eq!(
        recovery.committed_frames()[1].parent_id(),
        Some(USER_ENTRY_ID)
    );
    assert_eq!(
        recovery.committed_frames()[1].private_payload(),
        PRIVATE_ASSISTANT
    );
    assert!(
        recovery
            .committed_frames()
            .iter()
            .all(|frame| frame.entry_id() != UNCOMMITTED_REPAIR_ENTRY_ID)
    );
    store.close().expect("close reader child");
}

fn open<'store>(
    root: &'store CapabilityDir,
    policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'store DurableSessionStoreDescriptor,
    lease_id: &'store str,
) -> DurableSessionLocalStore<'store, SyncAll> {
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
        SyncAll,
    )
    .expect("open durable session store")
}

fn descriptor() -> DurableSessionStoreDescriptor {
    DurableSessionStoreDescriptor {
        store_id: String::from("store.local"),
        location_capability: String::from("session.store.restart.fixture"),
        session_id: String::from(SESSION_ID),
        branch_id: String::from(BRANCH_ID),
        policy_hash: String::from(POLICY_HASH),
        durability: DurableSessionDurability::SyncAllEachCommit,
        locking: DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data: DurableSessionPrivacyPosture::ReplayPrivate,
        encryption: DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from("metadata.only"),
    }
}

#[derive(Clone, Copy)]
struct SyncAll;

impl DurableSessionSynchronizer for SyncAll {
    fn synchronize(&mut self, file: &CapabilityFile) -> Result<(), io::Error> {
        file.sync_all()
    }
}
