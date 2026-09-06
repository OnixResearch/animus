//! Real-child capture, failure, compatibility, and cancellation controls.
#![cfg(test)]

use animus_std::AgentCheckExecutionService as _;

const LIMIT_BYTES: usize = 64;
const TIMEOUT_MS: u64 = 100;
// SIGABRT can include kernel core-dump work. This fixture budget is distinct
// from the timeout control and is not a production budget.
const SIGNAL_TIMEOUT_MS: u64 = 1_000;
const POLL_MS: u64 = 5;
const TEARDOWN_MS: u64 = 500;
const CHECK_ID: &str = "capture-check";
const SUBJECT: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

struct Case {
    root: tempfile::TempDir,
    config: animus_std_check_exec::BoundedCheckExecutorConfig,
    selection: animus_runtime::authoritative_completion::AgentCheckSelection,
}

impl Case {
    fn new(mode: &str, limit_bytes: usize) -> Self {
        let root = tempfile::TempDir::new().expect("explicit test root");
        let binary = std::path::Path::new(env!("CARGO_BIN_EXE_animus-check-exec-fixture"));
        let name = binary
            .file_name()
            .expect("name")
            .to_str()
            .expect("UTF-8 name");
        let mut argv = vec![name.to_string(), mode.to_string()];
        if matches!(mode, "marker" | "marker-wait") {
            argv.push(
                root.path()
                    .join("marker")
                    .to_str()
                    .expect("path")
                    .to_string(),
            );
        }
        let config = animus_std_check_exec::BoundedCheckExecutorConfig {
            repository_root: root.path().to_path_buf(),
            home: root.path().to_path_buf(),
            temporary_directory: root.path().to_path_buf(),
            executable_search_path: vec![binary.parent().expect("directory").to_path_buf()],
            allowed_checks: vec![animus_std_check_exec::AllowedCheck {
                check_id: CHECK_ID.to_string(),
                argv: argv.clone(),
                budget: animus_std_check_exec::CheckExecutionBudget {
                    timeout_ms: TIMEOUT_MS,
                    stdout_max_bytes: limit_bytes,
                    stderr_max_bytes: limit_bytes,
                    poll_interval_ms: POLL_MS,
                    teardown_timeout_ms: TEARDOWN_MS,
                },
            }],
        };
        let selection = selection(&argv);
        assert!(
            selection.blockers().is_empty(),
            "{:?}",
            selection.blockers()
        );
        assert_eq!(selection.required_checks().len(), 1);
        Self {
            root,
            config,
            selection,
        }
    }

    fn executor(&self) -> animus_std_check_exec::BoundedCheckExecutor {
        animus_std_check_exec::BoundedCheckExecutor::new(self.config.clone()).expect("config")
    }

    fn request(&self) -> animus_std::AgentCheckExecutionRequest<'_> {
        animus_std::AgentCheckExecutionRequest {
            check: &self.selection.required_checks()[0],
            registry_hash: self.selection.registry_hash(),
            selection_hash: self.selection.selection_hash(),
            policy_hash: self.selection.policy_hash(),
            final_subject_hash: self.selection.final_subject_hash(),
        }
    }

    fn capture(&self) -> animus_std_check_exec::CapturedCheck {
        self.executor()
            .execute_captured(self.request())
            .expect("actual capture")
    }
}

fn selection(argv: &[String]) -> animus_runtime::authoritative_completion::AgentCheckSelection {
    let json = serde_json::json!({
        "schema": "animus.agent-check-registry.v1", "repository_id": "onixresearch/animus",
        "registry_id": "capture-fixture", "supported_cairn_schema": "cairn.agent-completion.v2",
        "cairn_policy_hash": animus_runtime::authoritative_completion::PINNED_CAIRN_POLICY_HASH,
        "fallback_check_ids": [CHECK_ID],
        "checks": [{"check_id": CHECK_ID, "profile": "capture-fixture", "argv": argv,
            "affected_paths": ["docs"], "affected_crates": [], "affected_surfaces": ["documentation"],
            "prerequisites": [], "capability": "nix", "tier": "quick", "risk": "low",
            "timeout_class": "short", "output_budget_class": "small", "expected_artifacts": ["target/capture-observation"],
            "reproduction": {"argv": argv, "working_root": "<repository-root>"},
            "documentation_refs": ["README.md"], "positive_contract": "README.md", "negative_contract": "README.md"}]
    });
    let registry =
        animus_std::parse_agent_check_registry_json(&json.to_string()).expect("registry");
    animus_runtime::authoritative_completion::select_required_agent_checks(
        animus_runtime::authoritative_completion::AgentCheckSelectionRequest {
            registry: &registry,
            changed_artifacts: &[
                animus_runtime::authoritative_completion::ChangedArtifactDraft {
                    path: "docs/capture.md",
                    surface: "documentation",
                    content_hash: SUBJECT,
                },
            ],
            available_capabilities: &[
                animus_runtime::authoritative_completion::CheckCapability::Nix,
            ],
            maximum_tier: animus_runtime::authoritative_completion::CheckTier::Quick,
            final_subject_hash: SUBJECT,
        },
    )
}

#[test]
fn binary_streams_are_exact_and_debug_remains_metadata_only() {
    // r[verify agent_harness_runtime.check_capture.retention]
    // r[verify agent_harness_runtime.check_capture.compatibility]
    let case = Case::new("binary", LIMIT_BYTES);
    let captured = case.capture();
    assert_eq!(captured.output().stdout.bytes, b"private-capture\0\xff\r\n");
    assert_eq!(captured.output().stderr.bytes, b"private-error\0\xfe\r\n");
    assert_eq!(
        captured.output().disposition,
        animus_std_check_exec::Disposition::Succeeded
    );
    assert!(!captured.output().stdout.truncated);
    assert!(!format!("{captured:?}").contains("private"));
    assert!(!format!("{:?}", captured.observation()).contains("private"));
    let ordinary = case
        .executor()
        .execute(case.request())
        .expect("ordinary service");
    assert_eq!(captured.into_observation(), ordinary);
}

#[test]
fn nonzero_signal_and_timeout_keep_their_actual_facts() {
    // r[verify agent_harness_runtime.check_capture.retention]
    let failed = Case::new("nonzero", LIMIT_BYTES).capture();
    assert_eq!(failed.output().exit_code, Some(7));
    assert_eq!(
        failed.output().disposition,
        animus_std_check_exec::Disposition::ExitFailed
    );
    assert_eq!(failed.output().stdout.bytes, b"private-fixture-output\n");
    assert_eq!(failed.output().stderr.bytes, b"private-fixture-output\n");
    let mut signal_case = Case::new("signal", LIMIT_BYTES);
    signal_case.config.allowed_checks[0].budget.timeout_ms = SIGNAL_TIMEOUT_MS;
    let signal = signal_case.capture();
    assert!(signal.output().signal.is_some());
    assert_eq!(signal.output().exit_code, None);
    assert_eq!(
        signal.output().disposition,
        animus_std_check_exec::Disposition::ExitFailed
    );
    let timeout = Case::new("timeout", LIMIT_BYTES).capture();
    assert_eq!(
        timeout.output().completion,
        animus_std_check_exec::Completion::TimedOut
    );
    assert_eq!(
        timeout.output().disposition,
        animus_std_check_exec::Disposition::TimedOut
    );
    assert!(timeout.observation().timed_out);
}

#[test]
fn overflow_preserves_bounded_prefixes_and_both_stream_flags() {
    // r[verify agent_harness_runtime.check_capture.retention]
    for (mode, expected) in [
        ("overflow", animus_std_check_exec::OverflowedStreams::Stdout),
        (
            "stderr-overflow",
            animus_std_check_exec::OverflowedStreams::Stderr,
        ),
        (
            "both-overflow",
            animus_std_check_exec::OverflowedStreams::Both,
        ),
    ] {
        let captured = Case::new(mode, LIMIT_BYTES).capture();
        assert_eq!(
            captured.output().disposition,
            animus_std_check_exec::Disposition::OutputLimitExceeded(expected)
        );
        for stream in [&captured.output().stdout, &captured.output().stderr] {
            assert!(stream.bytes.len() <= LIMIT_BYTES);
            assert_eq!(stream.truncated, stream.observed_bytes > stream.bytes.len());
        }
        assert_eq!(
            captured.observation().status,
            animus_runtime::authoritative_completion::CheckStatus::InfrastructureError
        );
    }
}

#[test]
fn exact_capture_boundary_does_not_inflate_payload_capacity() {
    let length = b"private-capture\0\xff\r\n".len();
    let exact = Case::new("binary", length).capture();
    assert_eq!(
        exact.output().disposition,
        animus_std_check_exec::Disposition::Succeeded
    );
    let short = Case::new("binary", length.checked_sub(1).expect("nonempty")).capture();
    assert!(short.output().stdout.truncated);
    assert_ne!(
        short.output().disposition,
        animus_std_check_exec::Disposition::Succeeded
    );
}

#[test]
fn drift_missing_executable_and_unknown_check_do_not_create_a_marker() {
    // r[verify agent_harness_runtime.check_capture.compatibility]
    let case = Case::new("marker", LIMIT_BYTES);
    let mut drift = case.config.clone();
    drift.allowed_checks[0]
        .argv
        .push("not-admitted".to_string());
    let mut executor = animus_std_check_exec::BoundedCheckExecutor::new(drift).expect("config");
    assert_eq!(
        executor.execute_captured(case.request()).unwrap_err(),
        animus_std_check_exec::ExecutorError::CommandDrift
    );
    let mut missing = case.config.clone();
    missing.executable_search_path = vec![case.root.path().to_path_buf()];
    let mut executor = animus_std_check_exec::BoundedCheckExecutor::new(missing).expect("config");
    assert_eq!(
        executor.execute_captured(case.request()).unwrap_err(),
        animus_std_check_exec::ExecutorError::ExecutableMissing
    );
    let mut unknown = case.config.clone();
    unknown.allowed_checks[0].check_id = "another-check".to_string();
    let mut executor = animus_std_check_exec::BoundedCheckExecutor::new(unknown).expect("config");
    assert_eq!(
        executor.execute_captured(case.request()).unwrap_err(),
        animus_std_check_exec::ExecutorError::UnsupportedCheck
    );
    assert!(!case.root.path().join("marker").exists());
}

#[test]
fn spawn_failure_has_no_fabricated_capture_or_child_effect() {
    // r[verify agent_harness_runtime.check_capture.retention]
    let mut case = Case::new("marker", LIMIT_BYTES);
    let name = &case.config.allowed_checks[0].argv[0];
    std::fs::write(case.root.path().join(name), b"not an executable").expect("invalid executable");
    case.config.executable_search_path = vec![case.root.path().to_path_buf()];
    let error = case
        .executor()
        .execute_captured(case.request())
        .unwrap_err();
    assert_eq!(error, animus_std_check_exec::ExecutorError::RunnerFailed);
    assert!(!case.root.path().join("marker").exists());
}

#[test]
fn pre_cancelled_request_never_spawns_or_retries() {
    // r[verify agent_harness_runtime.check_capture.cancellation]
    let case = Case::new("marker", LIMIT_BYTES);
    let cancelled = std::sync::atomic::AtomicBool::new(true);
    let result = case
        .executor()
        .execute_captured_with_cancellation(case.request(), &cancelled);
    assert_eq!(
        result.unwrap_err(),
        animus_std_check_exec::ExecutorError::CancelledBeforeSpawn
    );
    assert!(!case.root.path().join("marker").exists());
    assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
    let captured = case.capture();
    assert_eq!(
        captured.output().disposition,
        animus_std_check_exec::Disposition::Succeeded
    );
    let _observation = captured.into_observation();
    assert_eq!(
        std::fs::read(case.root.path().join("marker")).expect("one marker"),
        b"one execution"
    );
}

#[test]
fn running_child_cancellation_retains_nonpassing_outcome() {
    // r[verify agent_harness_runtime.check_capture.cancellation]
    const CANCELLATION_DEADLINE_MS: u64 = 5_000;
    const STARTUP_ATTEMPTS: usize = 200;
    let mut case = Case::new("marker-wait", LIMIT_BYTES);
    case.config.allowed_checks[0].budget.timeout_ms = CANCELLATION_DEADLINE_MS;
    let flag = std::sync::atomic::AtomicBool::new(false);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _attempt in 0..STARTUP_ATTEMPTS {
                if case.root.path().join("marker").exists() {
                    flag.store(true, std::sync::atomic::Ordering::Release);
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(POLL_MS));
            }
            panic!("child did not reach its startup marker");
        });
        let captured = case
            .executor()
            .execute_captured_with_cancellation(case.request(), &flag)
            .expect("cancelled capture");
        assert_eq!(
            captured.output().completion,
            animus_std_check_exec::Completion::Cancelled
        );
        assert_eq!(
            captured.output().disposition,
            animus_std_check_exec::Disposition::Cancelled
        );
        assert_eq!(
            captured.observation().status,
            animus_runtime::authoritative_completion::CheckStatus::Cancelled
        );
    });
}

#[test]
fn null_stdin_and_only_the_four_explicit_environment_keys_reach_the_child() {
    // r[verify agent_harness_runtime.check_capture.compatibility]
    let captured = Case::new("context", LIMIT_BYTES).capture();
    assert_eq!(
        captured.output().disposition,
        animus_std_check_exec::Disposition::Succeeded
    );
    assert_eq!(captured.output().stdout.bytes, b"explicit-context");
    assert!(captured.output().stderr.bytes.is_empty());
}
