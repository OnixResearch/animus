//! Actual bounded fixture-child and pinned-Cairn completion tests.

use std::path::{Path, PathBuf};

use animus_std::{
    CairnCompletionSummary, CairnCompletionValidationRequest, CairnCompletionValidationService,
    CompletionCairnFactoryRequest, CompletionCairnServiceFactory,
};
use animus_std_check_exec::{
    AllowedCheck, BoundedCheckExecutor, BoundedCheckExecutorConfig, CairnServiceError,
    CheckExecutionBudget, PinnedCairnFactory, PinnedCairnService,
};
use serde_json::json;
use tempfile::TempDir;

const POLICY_HASH: &str = animus_runtime::authoritative_completion::PINNED_CAIRN_POLICY_HASH;
const CHECK_ID: &str = "fixture-check";
const PRIVATE_OUTPUT: &str = "private-fixture-output";
const SUBJECT: &str = "private real-check completion subject";
const FIXTURE_TIMEOUT_MS: u64 = 100;
const FIXTURE_OUTPUT_BYTES: usize = 64;
const FIXTURE_POLL_MS: u64 = 10;
const FIXTURE_TEARDOWN_MS: u64 = 500;
const EXPECTED_CHECK_COUNT: usize = 1;

fn fixture_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_animus-check-exec-fixture"))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate is under workspace crates")
        .to_path_buf()
}

fn registry_value(program: &str, mode: &str) -> serde_json::Value {
    json!({
        "schema": "animus.agent-check-registry.v1",
        "repository_id": "onixresearch/animus",
        "registry_id": "animus-agent-checks",
        "supported_cairn_schema": "cairn.agent-completion.v2",
        "cairn_policy_hash": POLICY_HASH,
        "fallback_check_ids": [CHECK_ID],
        "checks": [{
            "check_id": CHECK_ID,
            "profile": "fixture-check",
            "argv": [program, mode],
            "affected_paths": ["docs"],
            "affected_crates": [],
            "affected_surfaces": ["documentation"],
            "prerequisites": [],
            "capability": "nix",
            "tier": "quick",
            "risk": "low",
            "timeout_class": "short",
            "output_budget_class": "small",
            "expected_artifacts": ["target/check-exec-fixture.receipt"],
            "reproduction": {
                "argv": [program, mode],
                "working_root": "<repository-root>"
            },
            "documentation_refs": ["README.md"],
            "positive_contract": "README.md",
            "negative_contract": "README.md"
        }]
    })
}

fn budget() -> CheckExecutionBudget {
    CheckExecutionBudget {
        timeout_ms: FIXTURE_TIMEOUT_MS,
        stdout_max_bytes: FIXTURE_OUTPUT_BYTES,
        stderr_max_bytes: FIXTURE_OUTPUT_BYTES,
        poll_interval_ms: FIXTURE_POLL_MS,
        teardown_timeout_ms: FIXTURE_TEARDOWN_MS,
    }
}

#[derive(Clone, Copy)]
struct FixtureCase<'a> {
    registry_mode: &'a str,
    allowed_mode: &'a str,
    head_state: &'a str,
    include_executable: bool,
}

fn run_fixture<F>(
    fixture_case: FixtureCase<'_>,
    cairn_factory: &mut F,
) -> Result<animus_std::AuthoritativeCompletionSmokeReport, String>
where
    F: CompletionCairnServiceFactory,
{
    let root = workspace_root();
    let fixture = fixture_binary();
    let program = fixture
        .file_name()
        .and_then(|value| value.to_str())
        .expect("fixture binary name");
    let temporary = TempDir::new().expect("temporary execution root");
    let registry_path = temporary.path().join("registry.json");
    let registry_json =
        serde_json::to_string_pretty(&registry_value(program, fixture_case.registry_mode))
            .expect("registry JSON");
    animus_std::parse_agent_check_registry_json(registry_json.as_str())
        .expect("fixture registry must satisfy the production parser");
    std::fs::write(registry_path.as_path(), registry_json.as_bytes()).expect("registry fixture");
    let search_path = if fixture_case.include_executable {
        vec![fixture.parent().expect("fixture parent").to_path_buf()]
    } else {
        vec![temporary.path().to_path_buf()]
    };
    let mut executor = BoundedCheckExecutor::new(BoundedCheckExecutorConfig {
        repository_root: root.clone(),
        executable_search_path: search_path,
        home: temporary.path().to_path_buf(),
        temporary_directory: temporary.path().to_path_buf(),
        allowed_checks: vec![AllowedCheck {
            check_id: CHECK_ID.to_string(),
            argv: vec![program.to_string(), fixture_case.allowed_mode.to_string()],
            budget: budget(),
        }],
    })
    .expect("bounded fixture executor");
    let args = vec![
        "--role-policy".to_string(),
        root.join("config/generated/agent-role-profiles.json")
            .display()
            .to_string(),
        "--model-policy".to_string(),
        root.join("config/generated/model-runtime-profiles.json")
            .display()
            .to_string(),
        "--check-registry".to_string(),
        registry_path.display().to_string(),
        "--subject".to_string(),
        SUBJECT.to_string(),
        "--head-state".to_string(),
        fixture_case.head_state.to_string(),
    ];
    animus_std::evaluate_packaged_authoritative_completion_with_services(
        args.as_slice(),
        &mut executor,
        cairn_factory,
    )
}

#[test]
fn actual_bounded_fixture_child_and_pinned_cairn_complete() {
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.execution.pass]
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.cairn.pass]
    let mut cairn = PinnedCairnFactory;
    let report = run_fixture(
        FixtureCase {
            registry_mode: "success",
            allowed_mode: "success",
            head_state: "clean",
            include_executable: true,
        },
        &mut cairn,
    )
    .expect("bounded child and pinned Cairn should complete");

    assert!(report.authoritative_completion());
    assert_eq!(report.required_check_count(), EXPECTED_CHECK_COUNT);
    assert_eq!(report.passed_check_count(), EXPECTED_CHECK_COUNT);
    assert_eq!(report.check_executor_calls(), EXPECTED_CHECK_COUNT);
    assert_eq!(report.cairn_validator_calls(), EXPECTED_CHECK_COUNT);
    assert_eq!(report.judge_decision(), "select");
}

#[test]
fn nonzero_timeout_and_output_overflow_block_without_output_leaks() {
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.failure.nonzero]
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.failure.timeout]
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.failure.output]
    for mode in ["nonzero", "timeout", "overflow", "signal"] {
        let mut cairn = PinnedCairnFactory;
        let failure = run_fixture(
            FixtureCase {
                registry_mode: mode,
                allowed_mode: mode,
                head_state: "clean",
                include_executable: true,
            },
            &mut cairn,
        )
        .expect_err("non-passing child must block completion");
        assert!(failure.contains("issue=completion"), "{mode}: {failure}");
        assert!(!failure.contains(PRIVATE_OUTPUT), "{mode}: {failure}");
        assert!(!failure.contains(SUBJECT), "{mode}: {failure}");
    }
}

#[test]
fn command_drift_missing_executable_and_dirty_state_fail_closed() {
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.execution.drift]
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.verification.fixtures]
    let mut cairn = PinnedCairnFactory;
    let drift = run_fixture(
        FixtureCase {
            registry_mode: "success",
            allowed_mode: "nonzero",
            head_state: "clean",
            include_executable: true,
        },
        &mut cairn,
    )
    .expect_err("command drift must fail");
    let missing = run_fixture(
        FixtureCase {
            registry_mode: "success",
            allowed_mode: "success",
            head_state: "clean",
            include_executable: false,
        },
        &mut cairn,
    )
    .expect_err("missing executable must fail");
    let dirty = run_fixture(
        FixtureCase {
            registry_mode: "success",
            allowed_mode: "success",
            head_state: "dirty",
            include_executable: true,
        },
        &mut cairn,
    )
    .expect_err("dirty state must fail before execution");

    assert!(drift.contains("issue=check-execution"));
    assert!(missing.contains("issue=check-execution"));
    assert!(dirty.contains("issue=head-dirty"));
    assert!(!drift.contains(PRIVATE_OUTPUT));
    assert!(!missing.contains(SUBJECT));
}

struct StaleCairnFactory;

impl CompletionCairnServiceFactory for StaleCairnFactory {
    type Error = CairnServiceError;
    type Service = StaleCairnService;

    fn create(
        &mut self,
        request: CompletionCairnFactoryRequest<'_>,
    ) -> Result<Self::Service, Self::Error> {
        let mut pinned = PinnedCairnFactory;
        Ok(StaleCairnService {
            inner: pinned.create(request)?,
        })
    }
}

struct StaleCairnService {
    inner: PinnedCairnService,
}

impl CairnCompletionValidationService for StaleCairnService {
    type Error = CairnServiceError;

    fn validate_completion(
        &mut self,
        request: CairnCompletionValidationRequest<'_>,
    ) -> Result<CairnCompletionSummary, Self::Error> {
        let mut summary = self.inner.validate_completion(request)?;
        summary.final_subject_hash = "stale".to_string();
        Ok(summary)
    }
}

#[test]
fn stale_pinned_cairn_summary_blocks_completion() {
    // r[verify agent_harness_runtime.real_check_completion_host_smoke.cairn.stale]
    let mut cairn = StaleCairnFactory;
    let failure = run_fixture(
        FixtureCase {
            registry_mode: "success",
            allowed_mode: "success",
            head_state: "clean",
            include_executable: true,
        },
        &mut cairn,
    )
    .expect_err("stale Cairn summary must block completion");

    assert!(failure.contains("issue=completion"));
    assert!(!failure.contains(SUBJECT));
}
