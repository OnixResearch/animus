//! Child-process restart and incomplete-tail evidence for the coding-session host.

#![cfg(feature = "coding-session-host")]

use animus_runtime::coding_session_host::{
    CodingSessionConfig, CodingSessionQueueProfile, CodingSessionServiceSet,
};
use animus_runtime::{MessageQueuePolicy, MessageQueuePolicyDraft};
use animus_std::coding_session_host::{
    AllowCodingSessionPolicy, AvailableCodingSessionTimer, CodingSessionHost,
    CodingSessionHostIssue, CodingSessionHostServices, DenyUnusedCodingSessionTool,
    DurableCodingSessionPersistence, FakeCodingSessionProvider, FakeCodingSessionSummarizer,
    NoopCodingSessionEventSink, ObservedCodingSessionCancellation,
};
use animus_std::{
    DurableSessionStoreLocation, DurableSessionStoreOpenRequest, FileSynchronizer,
    open_durable_session_store, parse_durable_session_store_policy_json,
};
use std::io::Write;

const BINARY: &str = env!("CARGO_BIN_EXE_animus-coding-session-smoke");
const POLICY_JSON: &str =
    include_str!("../../../config/generated/durable-session-store-policy.json");
const SESSION_ID: &str = "session.packaged-smoke";
const BRANCH_ID: &str = "branch.main";
const STORE_ID: &str = "store.packaged-smoke";
const LOCATION_CAPABILITY: &str = "capability.explicit-store-root";
const RECEIPT_POLICY: &str = "metadata-only";
const JOURNAL_NAME: &str = "coding-session.log";
const LOCK_NAME: &str = "coding-session.lock";
const SECOND_LEASE: &str = "lease.restart.integration";
const THIRD_LEASE: &str = "lease.restart.incomplete";
const PROMPT: &str = "persist this child-process turn";
const FAKE_RESPONSE: &str = "unused restart provider";
const FAKE_SUMMARY: &str = "unused restart summary";
const QUEUE_POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_QUEUE_ITEMS: u32 = 8;
const EXPECTED_MESSAGES: usize = 2;
const INCOMPLETE_TAIL_BYTE: u8 = 0xff;

#[test]
fn child_process_commit_reopens_and_incomplete_tail_blocks_continuation() {
    // r[verify agent_harness_runtime.coding_session_host.durable_boundaries]
    // r[verify agent_harness_runtime.coding_session_host.recovery]
    // r[verify agent_harness_runtime.coding_session_host.conformance]
    let root = TestRoot::new();
    let output = std::process::Command::new(BINARY)
        .args([
            "--store-root",
            root.path.to_str().expect("utf8 root"),
            "--prompt",
            PROMPT,
        ])
        .output()
        .expect("spawn packaged child");
    assert!(
        output.status.success(),
        "child stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rendered = String::from_utf8(output.stdout).expect("child stdout");
    assert!(rendered.contains("status=completed"));
    assert!(!rendered.contains(PROMPT));

    let policy = parse_durable_session_store_policy_json(POLICY_JSON).expect("policy");
    let descriptor = descriptor(&policy);
    let cap_root =
        cap_std::fs::Dir::open_ambient_dir(root.path.as_path(), cap_std::ambient_authority())
            .expect("cap root");
    let store = open_store(&cap_root, &policy, &descriptor, SECOND_LEASE);
    let mut resumed = host(DurableCodingSessionPersistence::new(
        store,
        &policy,
        &descriptor,
    ));
    resumed.resume().expect("resume committed child turn");
    assert_eq!(resumed.message_snapshot().len(), EXPECTED_MESSAGES);
    let services = resumed.into_services();
    assert_eq!(services.provider.dispatch_count, 0);
    services.persistence.close().expect("close resumed store");

    let mut journal = std::fs::OpenOptions::new()
        .append(true)
        .open(root.path.join(JOURNAL_NAME))
        .expect("open journal tail");
    journal
        .write_all(&[INCOMPLETE_TAIL_BYTE])
        .expect("append incomplete tail");
    journal.sync_all().expect("sync incomplete tail");
    drop(journal);

    let tail_root =
        cap_std::fs::Dir::open_ambient_dir(root.path.as_path(), cap_std::ambient_authority())
            .expect("tail cap root");
    let tail_store = open_store(&tail_root, &policy, &descriptor, THIRD_LEASE);
    let mut blocked = host(DurableCodingSessionPersistence::new(
        tail_store,
        &policy,
        &descriptor,
    ));
    let error = blocked.resume().expect_err("incomplete tail blocks resume");
    assert_eq!(error.issue(), CodingSessionHostIssue::SessionBlocked);
    let blocked_services = blocked.into_services();
    assert_eq!(blocked_services.provider.dispatch_count, 0);
    blocked_services
        .persistence
        .close()
        .expect("close blocked store");
}

fn host<Persistence>(
    persistence: Persistence,
) -> CodingSessionHost<
    FakeCodingSessionProvider,
    DenyUnusedCodingSessionTool,
    Persistence,
    AllowCodingSessionPolicy,
    ObservedCodingSessionCancellation,
    AvailableCodingSessionTimer,
    FakeCodingSessionSummarizer,
    NoopCodingSessionEventSink,
>
where
    Persistence: animus_std::coding_session_host::CodingSessionPersistenceService,
{
    let state = animus_runtime::coding_session_host::new_coding_session_state(
        CodingSessionConfig {
            session_id: String::from(SESSION_ID),
            branch_id: String::from(BRANCH_ID),
            queue_profile: CodingSessionQueueProfile::Sequential,
            services: CodingSessionServiceSet::all_available(),
            resume_required: true,
        },
        queue_policy(),
    )
    .expect("resume state");
    CodingSessionHost::new(
        state,
        CodingSessionHostServices {
            provider: FakeCodingSessionProvider {
                response: String::from(FAKE_RESPONSE),
                dispatch_count: 0,
            },
            tool: DenyUnusedCodingSessionTool,
            persistence,
            policy: AllowCodingSessionPolicy,
            cancellation: ObservedCodingSessionCancellation,
            timer: AvailableCodingSessionTimer,
            summarizer: FakeCodingSessionSummarizer {
                summary: String::from(FAKE_SUMMARY),
            },
            events: NoopCodingSessionEventSink,
        },
    )
    .expect("host")
}

fn queue_policy() -> MessageQueuePolicy {
    MessageQueuePolicy::new(MessageQueuePolicyDraft {
        max_items: MAX_QUEUE_ITEMS,
        allowed_sources: &["coding-session-host"],
        policy_label: "coding-session-restart-queue",
        policy_hash: QUEUE_POLICY_HASH,
        allow_queue_mutations: true,
        allow_interrupts: true,
    })
    .expect("queue policy")
}

fn descriptor(
    policy: &animus_runtime::durable_session_store::DurableSessionStorePolicy,
) -> animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
    animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(STORE_ID),
        location_capability: String::from(LOCATION_CAPABILITY),
        session_id: String::from(SESSION_ID),
        branch_id: String::from(BRANCH_ID),
        policy_hash: String::from(policy.policy_hash()),
        durability:
            animus_runtime::durable_session_store::DurableSessionDurability::SyncAllEachCommit,
        locking:
            animus_runtime::durable_session_store::DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data:
            animus_runtime::durable_session_store::DurableSessionPrivacyPosture::ReplayPrivate,
        encryption:
            animus_runtime::durable_session_store::DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from(RECEIPT_POLICY),
    }
}

fn open_store<'a>(
    root: &'a cap_std::fs::Dir,
    policy: &'a animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'a animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    lease_id: &'a str,
) -> animus_std::DurableSessionLocalStore<'a, FileSynchronizer> {
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
        FileSynchronizer,
    )
    .expect("open store")
}

struct TestRoot {
    path: std::path::PathBuf,
}

impl TestRoot {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "animus-coding-session-child-restart-{}",
            std::process::id()
        ));
        let _ignored = std::fs::remove_dir_all(path.as_path());
        std::fs::create_dir_all(path.as_path()).expect("create root");
        Self { path }
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ignored = std::fs::remove_dir_all(self.path.as_path());
    }
}
