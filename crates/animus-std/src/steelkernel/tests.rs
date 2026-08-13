use super::*;

const GENERATED_POLICY: &str =
    include_str!("../../../../config/generated/durable-session-store-policy.json");
const PRIME_AGENT_REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const SESSION_IDENTITY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const OTHER_SESSION_IDENTITY: &str =
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const CELL_IDENTITY: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const KERNEL_GENERATION: u64 = 7;
const JOURNAL_NAME: &str = "steel-kernel.journal";
const LOCK_NAME: &str = "steel-kernel.lock";
const FIRST_LEASE: &str = "lease.steel.first";
const SECOND_LEASE: &str = "lease.steel.second";
const STORE_ID: &str = "store.steel-kernel";
const LOCATION_CAPABILITY: &str = "capability.steel-kernel";
const BRANCH_ID: &str = "branch.steel-kernel";
const RECEIPT_POLICY: &str = "metadata-only";
const CONTEXT_LIMIT: u32 = 16_384;
const SOURCE_LIMIT: u32 = 4_096;
const COUNT_LIMIT: u32 = 16;
const FUEL_LIMIT: u64 = 100_000;
const RECURSION_LIMIT: u32 = 64;
const ALLOCATION_LIMIT: u64 = 1_048_576;
const REQUEST_LIMIT: u32 = 8_192;
const CHILD_DEPTH_LIMIT: u32 = 2;
const CHILD_FAN_OUT_LIMIT: u32 = 4;
const CHILD_TOKEN_LIMIT: u64 = 16_000;
const CHILD_WALL_LIMIT_MS: u64 = 60_000;
const CHILD_RESULT_LIMIT: u32 = 8_192;
const HOST_CALL_LIMIT: u32 = 4;
const HOST_RESULT_LIMIT: u32 = 32_768;
const CUMULATIVE_RESULT_LIMIT: u32 = 65_536;
const BRIDGE_NESTING_LIMIT: u32 = 4;
const BRIDGE_FUEL_LIMIT: u64 = 500_000;
const BRIDGE_WALL_LIMIT_MS: u64 = 120_000;
const TASK_CONTEXT: &str = "private task context";
const CAPTURED_RESULT: &str = "private captured result";
const RESTORED_VALUE: &str = "private captured result-suffix";
const CONTEXT_NAME: &str = "task_context";
const CHILD_PROFILE: &str = "focused-child";
const FIRST_NESTING_DEPTH: u32 = 1;

#[derive(Default)]
struct FakeControl;

impl animus_runtime::rlm_kernel::RlmBridgeControl for FakeControl {
    fn observe(
        &mut self,
        _checkpoint: animus_runtime::rlm_kernel::RlmBridgeCheckpoint,
    ) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
        animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            cancelled: false,
            elapsed_ms: 0,
        }
    }
}

#[derive(Default)]
struct CapturedExecutor {
    calls: u32,
}

impl animus_runtime::rlm_kernel::RlmKernelHostExecutor for CapturedExecutor {
    type Error = &'static str;

    fn execute(
        &mut self,
        _request: &animus_adapter_rlm_kernel::HostRequest,
    ) -> Result<String, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        Ok(String::from(CAPTURED_RESULT))
    }
}

#[test]
fn committed_helpers_context_clear_and_captured_results_restore_without_effect_replay() {
    // r[verify agent_harness_runtime.steel_kernel_restore.verification]
    // r[verify agent_harness_runtime.steel_kernel_restore.replay.no_effect]
    let policy = rlm_policy();
    let mut kernel = animus_steel::SteelRlmKernel::new(policy.clone()).expect("kernel");
    kernel
        .evaluate("(define stale-before-clear 1)")
        .expect("stale definition");
    kernel.clear_explicit().expect("clear");
    kernel
        .bind_prompt_context(&[animus_adapter_rlm_kernel::ContextBinding::new(
            CONTEXT_NAME,
            TASK_CONTEXT,
            CONTEXT_LIMIT,
        )
        .expect("context")])
        .expect("bind context");
    kernel
        .evaluate("(define (suffix value) (string-append value \"-suffix\"))")
        .expect("helper");
    let mut executor = CapturedExecutor::default();
    let mut control = FakeControl;
    animus_runtime::rlm_kernel::execute_bridged_cell(
        &mut kernel,
        animus_runtime::rlm_kernel::RlmBridgeExecutionRequest {
            source: "(define restored-value (suffix (animus-context-read \"captured\"))) restored-value",
            session_identity: SESSION_IDENTITY,
            cell_identity: CELL_IDENTITY,
            nesting_depth: FIRST_NESTING_DEPTH,
        },
        &mut executor,
        &mut control,
    )
    .expect("bridged cell");
    assert_eq!(executor.calls, 1);
    let snapshot = kernel
        .private_journal_snapshot(SESSION_IDENTITY, KERNEL_GENERATION)
        .expect("private snapshot");
    assert!(!format!("{snapshot:?}").contains(TASK_CONTEXT));
    assert!(!format!("{snapshot:?}").contains(CAPTURED_RESULT));
    let missing_result = String::from_utf8(snapshot.private_bytes().to_vec())
        .expect("private JSON")
        .replace(CAPTURED_RESULT, "");
    assert!(
        animus_steel::SteelRlmKernel::restore_private_journal(
            &policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: snapshot.committed_head_identity(),
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::Clean,
            },
            missing_result.as_bytes(),
        )
        .is_err()
    );

    let tempdir = tempfile::tempdir().expect("tempdir");
    let root = cap_std::fs::Dir::open_ambient_dir(tempdir.path(), cap_std::ambient_authority())
        .expect("cap root");
    let store_policy = super::super::parse_durable_session_store_policy_json(GENERATED_POLICY)
        .expect("store policy");
    let descriptor = descriptor(&store_policy);
    let store = open_store(&root, &store_policy, &descriptor, FIRST_LEASE);
    let mut persistence = DurableSteelKernelPersistence::new(store, &store_policy, &descriptor);
    persistence
        .commit_snapshot(&snapshot)
        .expect("snapshot commit");
    persistence.close().expect("first close");

    let reopened = open_store(&root, &store_policy, &descriptor, SECOND_LEASE);
    let persistence = DurableSteelKernelPersistence::new(reopened, &store_policy, &descriptor);
    let (mut restored, receipt) = persistence
        .restore_latest(
            &policy,
            DurableSteelKernelRestoreRequest {
                session_identity: SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: snapshot.committed_head_identity(),
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::InterruptedCell,
                active_turn_action: Some(
                    animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::PlanInterruptedToolRepair,
                ),
            },
        )
        .expect("restore");
    assert_eq!(
        executor.calls, 1,
        "restore must not invoke the old executor"
    );
    assert_eq!(
        receipt.status(),
        animus_steel::SteelKernelRestoreStatus::PriorHeadRestored
    );
    assert!(!receipt.non_claim().is_empty());
    assert!(restored.evaluate("stale-before-clear").is_err());
    assert!(matches!(
        restored.evaluate("restored-value").expect("restored value"),
        animus_steel::SteelRlmEvaluation::Value { value, .. } if value == RESTORED_VALUE
    ));
    assert!(matches!(
        restored.evaluate(CONTEXT_NAME).expect("restored context"),
        animus_steel::SteelRlmEvaluation::Value { value, .. } if value == TASK_CONTEXT
    ));
    persistence.close().expect("second close");
}

#[test]
fn restore_rejects_wrong_session_policy_head_corruption_and_private_receipt_content() {
    // r[verify agent_harness_runtime.steel_kernel_restore.verification]
    // r[verify agent_harness_runtime.steel_kernel_restore.evidence.leak]
    let policy = rlm_policy();
    let mut kernel = animus_steel::SteelRlmKernel::new(policy.clone()).expect("kernel");
    kernel.evaluate("(define durable-value 9)").expect("cell");
    let snapshot = kernel
        .private_journal_snapshot(SESSION_IDENTITY, KERNEL_GENERATION)
        .expect("snapshot");

    assert!(matches!(
        animus_steel::SteelRlmKernel::restore_private_journal(
            &policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: OTHER_SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: snapshot.committed_head_identity(),
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::Clean,
            },
            snapshot.private_bytes(),
        ),
        Err(animus_steel::SteelKernelJournalError::SessionMismatch)
    ));

    let other_policy = restricted_policy();
    assert!(matches!(
        animus_steel::SteelRlmKernel::restore_private_journal(
            &other_policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: snapshot.committed_head_identity(),
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::Clean,
            },
            snapshot.private_bytes(),
        ),
        Err(animus_steel::SteelKernelJournalError::PolicyMismatch)
    ));

    let wrong_head = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
    assert!(matches!(
        animus_steel::SteelRlmKernel::restore_private_journal(
            &policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: wrong_head,
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::Clean,
            },
            snapshot.private_bytes(),
        ),
        Err(animus_steel::SteelKernelJournalError::HeadMismatch)
    ));

    let mut corrupt = snapshot.private_bytes().to_vec();
    let corrupt_index = corrupt.len().saturating_sub(1);
    corrupt[corrupt_index] ^= 1;
    assert!(
        animus_steel::SteelRlmKernel::restore_private_journal(
            &policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: SESSION_IDENTITY,
                kernel_generation: KERNEL_GENERATION,
                committed_head_identity: snapshot.committed_head_identity(),
                recovery_posture: animus_steel::SteelKernelRecoveryPosture::Clean,
            },
            corrupt.as_slice(),
        )
        .is_err()
    );

    assert!(matches!(
        animus_steel::project_steel_kernel_restore_receipt(
            &snapshot,
            animus_steel::SteelKernelRestoreStatus::Restored,
            animus_steel::SteelKernelRestoreSensitiveDraft {
                source: Some("define durable-value"),
                ..animus_steel::SteelKernelRestoreSensitiveDraft::default()
            },
        ),
        Err(animus_steel::SteelKernelJournalError::ReceiptLeak)
    ));
    let private_debug = format!("{snapshot:?}");
    assert!(!private_debug.contains("define durable-value"));
}

fn descriptor(
    policy: &DurableSessionStorePolicy,
) -> animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
    animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(STORE_ID),
        location_capability: String::from(LOCATION_CAPABILITY),
        session_id: String::from(SESSION_IDENTITY),
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
    policy: &'a DurableSessionStorePolicy,
    descriptor: &'a DurableSessionStoreDescriptor,
    lease: &'a str,
) -> super::super::DurableSessionLocalStore<'a, super::super::FileSynchronizer> {
    super::super::open_durable_session_store(
        &super::super::DurableSessionStoreOpenRequest {
            policy,
            descriptor,
            location: super::super::DurableSessionStoreLocation {
                root,
                journal_name: JOURNAL_NAME,
                lock_name: LOCK_NAME,
            },
            lease_id: lease,
        },
        super::super::FileSynchronizer,
    )
    .expect("store")
}

fn rlm_policy() -> animus_adapter_rlm_kernel::RlmKernelPolicy {
    policy_with_operations(&[animus_adapter_rlm_kernel::HostOperationClass::ContextRead])
}

fn restricted_policy() -> animus_adapter_rlm_kernel::RlmKernelPolicy {
    policy_with_operations(&[animus_adapter_rlm_kernel::HostOperationClass::KernelClear])
}

fn policy_with_operations(
    operations: &[animus_adapter_rlm_kernel::HostOperationClass],
) -> animus_adapter_rlm_kernel::RlmKernelPolicy {
    animus_adapter_rlm_kernel::RlmKernelPolicy::from_draft(
        &animus_adapter_rlm_kernel::RlmKernelPolicyDraft {
            schema: animus_adapter_rlm_kernel::RLM_KERNEL_POLICY_SCHEMA,
            cohort: "prime-agent-rlm-2026-08-06",
            cohort_revision: PRIME_AGENT_REVISION,
            profile: "model-facing-steel",
            bounds: animus_adapter_rlm_kernel::ResourceBounds {
                max_source_chars: SOURCE_LIMIT,
                max_environment_chars: CONTEXT_LIMIT,
                max_bindings: COUNT_LIMIT,
                max_definitions: COUNT_LIMIT,
                max_evaluation_fuel: FUEL_LIMIT,
                max_recursion_depth: RECURSION_LIMIT,
                max_allocation_bytes: ALLOCATION_LIMIT,
                max_host_request_chars: REQUEST_LIMIT,
                max_child_depth: CHILD_DEPTH_LIMIT,
                max_child_fan_out: CHILD_FAN_OUT_LIMIT,
                max_child_tokens: CHILD_TOKEN_LIMIT,
                max_child_wall_time_ms: CHILD_WALL_LIMIT_MS,
                max_child_result_chars: CHILD_RESULT_LIMIT,
                max_host_calls_per_cell: HOST_CALL_LIMIT,
                max_host_result_chars: HOST_RESULT_LIMIT,
                max_cumulative_host_result_chars: CUMULATIVE_RESULT_LIMIT,
                max_bridge_nesting: BRIDGE_NESTING_LIMIT,
                max_bridge_evaluation_fuel: BRIDGE_FUEL_LIMIT,
                max_bridge_wall_time_ms: BRIDGE_WALL_LIMIT_MS,
            },
            admitted_operations: operations,
            admitted_child_profiles: &[CHILD_PROFILE],
            non_claims: animus_adapter_rlm_kernel::REQUIRED_NON_CLAIMS,
        },
    )
    .expect("RLM policy")
}
