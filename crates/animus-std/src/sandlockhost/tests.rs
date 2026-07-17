use animus_adapters::{SafeHash, SandboxDescriptor, SandboxDescriptorDraft};

use super::*;

const ACTION: &[u8] = b"action";
const SUBJECT: &[u8] = b"subject";
const TARGET: &[u8] = b"target";
const AUTHORIZATION: &[u8] = b"authorization";
const COMMAND: &[&str] = &["fixture-command", "--bounded"];
const POLICY_ID: &str = "sandlock:strict-fixture";
const ROOT_TOOLS: &str = "tools";
const ROOT_WORKSPACE: &str = "workspace";
const ROOT_TOOLS_PATH: &str = "store/tools";
const ROOT_WORKSPACE_PATH: &str = "worktree";
const NETWORK_RULE: &str = "api.example.invalid:443";
const HTTP_RULE: &str = "POST api.example.invalid/v1/*";
const TIMEOUT_MILLIS: u64 = 30_000;
const OUTPUT_LIMIT_BYTES: u64 = 65_536;
const MEMORY_LIMIT_BYTES: u64 = 268_435_456;
const PROCESS_LIMIT: u32 = 32;
const OPEN_FILE_LIMIT: u32 = 128;
const CPU_LIMIT_PERCENT: u8 = 50;
const DETERMINISTIC_EPOCH_SECONDS: u64 = 946_684_800;
const DETERMINISTIC_SEED: u64 = 42;
const EXPECTED_CAPABILITY_COUNT: usize = 9;
const NEGATIVE_AUTHORIZATION_CASES: usize = 3;

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.translation]
// r[verify agent_harness_runtime.sandlock_host_adapter.authority]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn complete_binding_produces_one_deterministic_path_free_plan() {
    let fixture = fixture();
    let first = plan(&fixture).expect("complete fixture should plan");
    let second = plan(&fixture).expect("identical fixture should plan");

    assert_eq!(first, second);
    assert_eq!(
        first.required_capabilities().len(),
        EXPECTED_CAPABILITY_COUNT
    );
    assert_eq!(first.command_hash(), &fixture.command_hash);
    assert!(first.root_bindings().iter().all(|binding| {
        !binding.relative_path().starts_with('/') && !binding.relative_path().contains("..")
    }));
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.preflight]
// r[verify agent_harness_runtime.sandlock_host_adapter.receipts]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn complete_probe_and_execution_project_a_self_verifying_redacted_receipt() {
    let fixture = fixture();
    let plan = plan(&fixture).expect("complete fixture should plan");
    let report = SandlockProbeReport::new(
        plan.plan_hash().clone(),
        plan.cohort_hash().clone(),
        plan.required_capabilities().to_vec(),
        SafeHash::from_blake3_bytes(b"probe"),
    );
    let preflight = evaluate_sandlock_preflight(&plan, &report)
        .expect("complete mechanism proof should admit execution");
    let mut facts = passing_facts(&plan);
    facts.execution_hash = execution_facts_hash(&plan, &facts);
    let receipt = project_sandlock_receipt(&plan, &preflight, &facts)
        .expect("matching safe execution facts should project");

    assert_eq!(receipt.status(), SandlockExecutionStatus::Passed);
    assert_eq!(receipt.plan_hash(), plan.plan_hash());
    assert_eq!(
        receipt.requested_capabilities(),
        receipt.enforced_capabilities()
    );
    assert_eq!(receipt.cow_outcome(), SandlockCowOutcome::Discarded);
    assert!(verify_sandlock_receipt(&receipt).is_ok());
    assert!(SANDLOCK_RECEIPT_NON_CLAIMS.contains(&"not_authorization"));
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.authority]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn denied_stale_and_mismatched_authorization_fail_before_planning() {
    let cases = [
        (
            SandlockAuthorizationDecision::Denied,
            true,
            None,
            SandlockHostError::AuthorizationDenied,
        ),
        (
            SandlockAuthorizationDecision::Allowed,
            false,
            None,
            SandlockHostError::AuthorizationStale,
        ),
        (
            SandlockAuthorizationDecision::Allowed,
            true,
            Some(SafeHash::from_blake3_bytes(b"wrong-command")),
            SandlockHostError::AuthorizationMismatch {
                field: "command_hash",
            },
        ),
    ];
    assert_eq!(cases.len(), NEGATIVE_AUTHORIZATION_CASES);

    for (decision, current, command_override, expected) in cases {
        let mut fixture = fixture();
        fixture.authorization = authorization(
            &fixture,
            decision,
            current,
            command_override.as_ref().unwrap_or(&fixture.command_hash),
        );
        assert_eq!(plan(&fixture), Err(expected));
    }
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.translation]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn unsafe_missing_and_widened_roots_fail_closed() {
    assert_eq!(
        SandlockRootBinding::new(
            ROOT_WORKSPACE,
            SafeHash::from_blake3_bytes(b"grant"),
            "../escape",
            SandlockRootAccess::ReadWrite,
        ),
        Err(SandlockHostError::UnsafeRelativeRoot)
    );

    let mut missing = fixture();
    missing
        .roots
        .retain(|binding| binding.label() != ROOT_WORKSPACE);
    assert_eq!(plan(&missing), Err(SandlockHostError::MissingWritableRoot));

    let mut widened = fixture();
    widened.roots.push(
        SandlockRootBinding::new(
            "extra-write",
            SafeHash::from_blake3_bytes(b"extra"),
            "other",
            SandlockRootAccess::ReadWrite,
        )
        .expect("safe fixture root"),
    );
    assert_eq!(
        plan(&widened),
        Err(SandlockHostError::UnexpectedWritableRoot)
    );
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.preflight]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn missing_or_cross_plan_probe_cannot_start_target_execution() {
    let fixture = fixture();
    let plan = plan(&fixture).expect("complete fixture should plan");
    let mut incomplete = plan.required_capabilities().to_vec();
    incomplete.retain(|capability| *capability != SandlockCapability::Network);
    let missing = SandlockProbeReport::new(
        plan.plan_hash().clone(),
        plan.cohort_hash().clone(),
        incomplete,
        SafeHash::from_blake3_bytes(b"missing-network"),
    );
    let cross_plan = SandlockProbeReport::new(
        SafeHash::from_blake3_bytes(b"other-plan"),
        plan.cohort_hash().clone(),
        plan.required_capabilities().to_vec(),
        SafeHash::from_blake3_bytes(b"cross-plan"),
    );

    let blocked = evaluate_sandlock_preflight(&plan, &missing)
        .expect("matching incomplete probe should produce a typed blocker");
    assert!(!blocked.is_ready());
    assert_eq!(
        blocked.missing_capabilities(),
        &[SandlockCapability::Network]
    );
    assert_eq!(
        evaluate_sandlock_preflight(&plan, &cross_plan),
        Err(SandlockHostError::ProbeIdentityMismatch)
    );
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.feature_gate]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn best_effort_and_incompatible_policy_values_are_rejected() {
    let mut best_effort = fixture();
    best_effort.descriptor = descriptor("best_effort");
    best_effort.authorization = authorization(
        &best_effort,
        SandlockAuthorizationDecision::Allowed,
        true,
        &best_effort.command_hash,
    );
    assert_eq!(
        plan(&best_effort),
        Err(SandlockHostError::BestEffortUnsupported)
    );

    let policy = SandlockHostPolicy::new(SandlockHostPolicyDraft {
        egress_allow: &[],
        ..valid_policy_draft()
    })
    .expect("structurally valid policy");
    let mut incompatible = fixture();
    incompatible.policy = policy;
    incompatible.authorization = authorization(
        &incompatible,
        SandlockAuthorizationDecision::Allowed,
        true,
        &incompatible.command_hash,
    );
    assert_eq!(
        plan(&incompatible),
        Err(SandlockHostError::InvalidPolicy {
            reason: "network_rule_mismatch"
        })
    );

    let mut unauditable_commit = fixture();
    unauditable_commit.descriptor = commit_descriptor();
    unauditable_commit.authorization = authorization(
        &unauditable_commit,
        SandlockAuthorizationDecision::Allowed,
        true,
        &unauditable_commit.command_hash,
    );
    assert_eq!(
        plan(&unauditable_commit),
        Err(SandlockHostError::InvalidPolicy {
            reason: "cow_commit_not_auditable"
        })
    );
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.execution]
// r[verify agent_harness_runtime.sandlock_host_adapter.receipts]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn timeout_cleanup_and_output_contradictions_never_project_as_passed() {
    let fixture = fixture();
    let plan = plan(&fixture).expect("complete fixture should plan");
    let report = SandlockProbeReport::new(
        plan.plan_hash().clone(),
        plan.cohort_hash().clone(),
        plan.required_capabilities().to_vec(),
        SafeHash::from_blake3_bytes(b"probe"),
    );
    let preflight = evaluate_sandlock_preflight(&plan, &report).expect("preflight");

    let mut timed_out = passing_facts(&plan);
    timed_out.status = SandlockExecutionStatus::TimedOut;
    timed_out.exit_code = None;
    timed_out.cow = SandlockCowOutcome::Discarded;
    timed_out.execution_hash = execution_facts_hash(&plan, &timed_out);
    let timeout_receipt = project_sandlock_receipt(&plan, &preflight, &timed_out)
        .expect("typed timeout should remain recordable");
    assert_eq!(timeout_receipt.status(), SandlockExecutionStatus::TimedOut);

    let mut setup_failed = passing_facts(&plan);
    setup_failed.status = SandlockExecutionStatus::SetupFailed;
    setup_failed.exit_code = None;
    setup_failed.enforced_capabilities.clear();
    setup_failed.execution_hash = execution_facts_hash(&plan, &setup_failed);
    let setup_receipt = project_sandlock_receipt(&plan, &preflight, &setup_failed)
        .expect("typed setup failure should remain recordable");
    assert_eq!(setup_receipt.status(), SandlockExecutionStatus::SetupFailed);

    let mut cleanup_failed = passing_facts(&plan);
    cleanup_failed.status = SandlockExecutionStatus::Failed;
    cleanup_failed.cleanup = SandlockCleanupStatus::Failed;
    cleanup_failed.execution_hash = execution_facts_hash(&plan, &cleanup_failed);
    let cleanup_receipt = project_sandlock_receipt(&plan, &preflight, &cleanup_failed)
        .expect("typed cleanup failure should remain recordable");
    assert_eq!(cleanup_receipt.status(), SandlockExecutionStatus::Failed);

    let mut false_pass = passing_facts(&plan);
    false_pass.cleanup = SandlockCleanupStatus::Failed;
    false_pass.execution_hash = execution_facts_hash(&plan, &false_pass);
    assert_eq!(
        project_sandlock_receipt(&plan, &preflight, &false_pass),
        Err(SandlockHostError::ExecutionEvidenceMismatch)
    );

    let mut oversized = passing_facts(&plan);
    oversized.stdout_bytes = OUTPUT_LIMIT_BYTES + 1;
    oversized.execution_hash = execution_facts_hash(&plan, &oversized);
    assert_eq!(
        project_sandlock_receipt(&plan, &preflight, &oversized),
        Err(SandlockHostError::ExecutionEvidenceMismatch)
    );
}

#[test]
// r[verify agent_harness_runtime.sandlock_host_adapter.receipts]
// r[verify agent_harness_runtime.sandlock_host_adapter.verification]
fn raw_receipt_evidence_is_rejected() {
    let cases = [
        "RAW_PROMPT hidden",
        "API_KEY=secret",
        "/home/operator/private",
        "RAW_NETWORK payload",
        "RAW_FILE contents",
        "RAW_PROCESS_OUTPUT body",
        "SANDLOCK_DEBUG trace",
        "UNBOUNDED_EVENTS stream",
    ];
    for case in cases {
        assert_eq!(
            validate_sandlock_receipt_text(case),
            Err(SandlockHostError::UnsafeReceiptEvidence)
        );
    }
    assert!(
        validate_sandlock_receipt_text("status=passed plan_hash=redacted cleanup=complete").is_ok()
    );
}

struct Fixture {
    descriptor: SandboxDescriptor,
    policy: SandlockHostPolicy,
    authorization: SandlockAuthorizationBinding,
    action_hash: SafeHash,
    subject_hash: SafeHash,
    target_hash: SafeHash,
    command_hash: SafeHash,
    roots: Vec<SandlockRootBinding>,
}

fn fixture() -> Fixture {
    let descriptor = descriptor("required");
    let policy = SandlockHostPolicy::new(valid_policy_draft()).expect("valid policy fixture");
    let action_hash = SafeHash::from_blake3_bytes(ACTION);
    let subject_hash = SafeHash::from_blake3_bytes(SUBJECT);
    let target_hash = SafeHash::from_blake3_bytes(TARGET);
    let command_hash = sandlock_command_hash(COMMAND).expect("valid command fixture");
    let roots = vec![
        SandlockRootBinding::new(
            ROOT_TOOLS,
            SafeHash::from_blake3_bytes(b"tools-grant"),
            ROOT_TOOLS_PATH,
            SandlockRootAccess::ReadOnly,
        )
        .expect("safe tools root"),
        SandlockRootBinding::new(
            ROOT_WORKSPACE,
            SafeHash::from_blake3_bytes(b"workspace-grant"),
            ROOT_WORKSPACE_PATH,
            SandlockRootAccess::ReadWrite,
        )
        .expect("safe workspace root"),
    ];
    let mut fixture = Fixture {
        descriptor,
        policy,
        authorization: placeholder_authorization(),
        action_hash,
        subject_hash,
        target_hash,
        command_hash,
        roots,
    };
    fixture.authorization = authorization(
        &fixture,
        SandlockAuthorizationDecision::Allowed,
        true,
        &fixture.command_hash,
    );
    fixture
}

fn descriptor(enforcement: &'static str) -> SandboxDescriptor {
    SandboxDescriptor::new(descriptor_draft(enforcement)).expect("valid descriptor fixture")
}

fn commit_descriptor() -> SandboxDescriptor {
    SandboxDescriptor::new(SandboxDescriptorDraft {
        cow_disposition: "commit",
        ..descriptor_draft("required")
    })
    .expect("commit descriptor is structurally portable")
}

fn descriptor_draft(enforcement: &'static str) -> SandboxDescriptorDraft<'static> {
    SandboxDescriptorDraft {
        descriptor_id: "sandlock:full-fixture",
        filesystem_mode: "workspace_write",
        writable_roots: &[ROOT_WORKSPACE],
        workdir_mode: "cow_staged",
        cow_disposition: "dry_run",
        process_profile: "resource_limited",
        environment_policy: "minimal",
        network_policy: "egress_allowlist",
        http_policy: "method_host_path_allowlist",
        determinism_policy: "stable_time_randomness",
        enforcement_requirement: enforcement,
    }
}

fn valid_policy_draft() -> SandlockHostPolicyDraft<'static> {
    SandlockHostPolicyDraft {
        policy_id: POLICY_ID,
        timeout_millis: TIMEOUT_MILLIS,
        max_output_bytes: OUTPUT_LIMIT_BYTES,
        max_memory_bytes: Some(MEMORY_LIMIT_BYTES),
        max_processes: Some(PROCESS_LIMIT),
        max_open_files: Some(OPEN_FILE_LIMIT),
        max_cpu_percent: Some(CPU_LIMIT_PERCENT),
        egress_allow: &[NETWORK_RULE],
        http_allow: &[HTTP_RULE],
        http_deny: &[],
        deterministic_epoch_seconds: Some(DETERMINISTIC_EPOCH_SECONDS),
        deterministic_seed: Some(DETERMINISTIC_SEED),
        cow_workdir_root: Some(ROOT_WORKSPACE),
    }
}

fn authorization(
    fixture: &Fixture,
    decision: SandlockAuthorizationDecision,
    current: bool,
    command_hash: &SafeHash,
) -> SandlockAuthorizationBinding {
    SandlockAuthorizationBinding::new(SandlockAuthorizationBindingDraft {
        authorization_hash: SafeHash::from_blake3_bytes(AUTHORIZATION),
        action_hash: fixture.action_hash.clone(),
        subject_hash: fixture.subject_hash.clone(),
        target_hash: fixture.target_hash.clone(),
        descriptor_hash: fixture.descriptor.descriptor_hash().clone(),
        policy_hash: fixture.policy.policy_hash().clone(),
        command_hash: command_hash.clone(),
        decision,
        current,
    })
}

fn placeholder_authorization() -> SandlockAuthorizationBinding {
    let placeholder = SafeHash::from_blake3_bytes(b"placeholder");
    SandlockAuthorizationBinding::new(SandlockAuthorizationBindingDraft {
        authorization_hash: placeholder.clone(),
        action_hash: placeholder.clone(),
        subject_hash: placeholder.clone(),
        target_hash: placeholder.clone(),
        descriptor_hash: placeholder.clone(),
        policy_hash: placeholder.clone(),
        command_hash: placeholder,
        decision: SandlockAuthorizationDecision::Denied,
        current: false,
    })
}

fn plan(fixture: &Fixture) -> Result<SandlockLaunchPlan, SandlockHostError> {
    plan_sandlock_launch(SandlockLaunchRequest {
        descriptor: &fixture.descriptor,
        policy: &fixture.policy,
        authorization: &fixture.authorization,
        action_hash: &fixture.action_hash,
        subject_hash: &fixture.subject_hash,
        target_hash: &fixture.target_hash,
        command_hash: &fixture.command_hash,
        root_bindings: fixture.roots.as_slice(),
    })
}

fn passing_facts(plan: &SandlockLaunchPlan) -> SandlockExecutionFacts {
    SandlockExecutionFacts {
        status: SandlockExecutionStatus::Passed,
        exit_code: Some(0),
        signal: None,
        stdout_bytes: 0,
        stderr_bytes: 0,
        stdout_hash: Some(SafeHash::from_blake3_bytes(b"")),
        stderr_hash: Some(SafeHash::from_blake3_bytes(b"")),
        cow: SandlockCowOutcome::Discarded,
        cow_change_count: None,
        cow_digest: None,
        cleanup: SandlockCleanupStatus::Complete,
        enforced_capabilities: plan.required_capabilities().to_vec(),
        execution_hash: SafeHash::from_blake3_bytes(b"pending"),
    }
}
