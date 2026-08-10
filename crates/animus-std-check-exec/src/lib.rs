#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! Optional bounded selected-check host for Animus completion evidence.

mod cairn;
mod executor;

use std::path::PathBuf;

pub use cairn::{CairnServiceError, PinnedCairnFactory, PinnedCairnService};
pub use executor::{
    AllowedCheck, BoundedCheckExecutor, BoundedCheckExecutorConfig, CheckExecutionBudget,
    ExecutorError,
};

const ARG_REPOSITORY_ROOT: &str = "--repository-root";
const ARG_EXECUTION_PATH: &str = "--execution-path";
const ARG_EXECUTION_HOME: &str = "--execution-home";
const ARG_EXECUTION_TMP: &str = "--execution-tmp";
const ARG_ROLE_POLICY: &str = "--role-policy";
const ARG_MODEL_POLICY: &str = "--model-policy";
const ARG_CHECK_REGISTRY: &str = "--check-registry";
const ARG_SUBJECT: &str = "--subject";
const ARG_HEAD_STATE: &str = "--head-state";
const HEAD_CLEAN: &str = "clean";
const HEAD_DIRTY: &str = "dirty";
const CHECK_NICKEL_CONFIG: &str = "nickel-config";
const CHECK_CAIRN_VALIDATE: &str = "cairn-validate";
const NIX_PROGRAM: &str = "nix";
const HEADER: &str = "ANIMUS_REAL_CHECK_COMPLETION_SMOKE";
const SCHEMA: &str = "animus.real-check-completion-host-smoke.v1";
const SHORT_TIMEOUT_MS: u64 = 120_000;
const MEDIUM_TIMEOUT_MS: u64 = 300_000;
const SMALL_OUTPUT_BYTES: usize = 65_536;
const MEDIUM_OUTPUT_BYTES: usize = 262_144;
const POLL_INTERVAL_MS: u64 = 25;
const TEARDOWN_TIMEOUT_MS: u64 = 2_000;
const NON_CLAIMS: &[&str] = &[
    "check-oracle-soundness",
    "executable-authenticity",
    "sandboxing",
    "hermeticity",
    "child-correctness",
    "side-effect-safety",
    "live-provider",
    "release-readiness",
    "production-readiness",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct RealSmokeArgs {
    repository_root: PathBuf,
    execution_path: Vec<PathBuf>,
    execution_home: PathBuf,
    execution_tmp: PathBuf,
    completion_args: Vec<String>,
    head_state: String,
}

/// Run the packaged real-check completion smoke from explicit CLI arguments.
///
/// # Errors
///
/// Returns a stable metadata-only failure marker for malformed input, rejected
/// process configuration, bounded execution failure, or blocked completion.
pub fn run_packaged_real_check_completion_smoke(args: &[String]) -> Result<String, String> {
    let parsed = parse_args(args)?;
    if parsed.head_state == HEAD_DIRTY {
        return Err(error("head-dirty"));
    }
    let config = BoundedCheckExecutorConfig {
        repository_root: parsed.repository_root,
        executable_search_path: parsed.execution_path,
        home: parsed.execution_home,
        temporary_directory: parsed.execution_tmp,
        allowed_checks: live_allowed_checks(),
    };
    let mut executor =
        BoundedCheckExecutor::new(config).map_err(|failure| error(failure.issue()))?;
    let mut cairn_factory = PinnedCairnFactory;
    let report = animus_std::evaluate_packaged_authoritative_completion_with_services(
        parsed.completion_args.as_slice(),
        &mut executor,
        &mut cairn_factory,
    )
    .map_err(|_failure| error("completion-workflow"))?;
    Ok(render_report(&report))
}

fn live_allowed_checks() -> Vec<AllowedCheck> {
    vec![
        AllowedCheck {
            check_id: CHECK_NICKEL_CONFIG.to_string(),
            argv: [NIX_PROGRAM, "run", ".#check-nickel-configs"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            budget: CheckExecutionBudget {
                timeout_ms: SHORT_TIMEOUT_MS,
                stdout_max_bytes: SMALL_OUTPUT_BYTES,
                stderr_max_bytes: SMALL_OUTPUT_BYTES,
                poll_interval_ms: POLL_INTERVAL_MS,
                teardown_timeout_ms: TEARDOWN_TIMEOUT_MS,
            },
        },
        AllowedCheck {
            check_id: CHECK_CAIRN_VALIDATE.to_string(),
            argv: [
                NIX_PROGRAM,
                "run",
                ".#cairn",
                "--",
                "validate",
                "--root",
                ".",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            budget: CheckExecutionBudget {
                timeout_ms: MEDIUM_TIMEOUT_MS,
                stdout_max_bytes: MEDIUM_OUTPUT_BYTES,
                stderr_max_bytes: MEDIUM_OUTPUT_BYTES,
                poll_interval_ms: POLL_INTERVAL_MS,
                teardown_timeout_ms: TEARDOWN_TIMEOUT_MS,
            },
        },
    ]
}

fn parse_args(args: &[String]) -> Result<RealSmokeArgs, String> {
    let mut cursor = ArgCursor::new(args);
    let mut repository_root = None;
    let mut execution_path = None;
    let mut execution_home = None;
    let mut execution_tmp = None;
    let mut role_policy = None;
    let mut model_policy = None;
    let mut check_registry = None;
    let mut subject = None;
    let mut head_state = None;
    while let Some(argument) = cursor.next() {
        match argument.as_str() {
            ARG_REPOSITORY_ROOT => repository_root = Some(cursor.next_value()?),
            ARG_EXECUTION_PATH => execution_path = Some(cursor.next_value()?),
            ARG_EXECUTION_HOME => execution_home = Some(cursor.next_value()?),
            ARG_EXECUTION_TMP => execution_tmp = Some(cursor.next_value()?),
            ARG_ROLE_POLICY => role_policy = Some(cursor.next_value()?),
            ARG_MODEL_POLICY => model_policy = Some(cursor.next_value()?),
            ARG_CHECK_REGISTRY => check_registry = Some(cursor.next_value()?),
            ARG_SUBJECT => subject = Some(cursor.next_value()?),
            ARG_HEAD_STATE => head_state = Some(parse_head_state(cursor.next_value()?)?),
            _unknown => return Err(error("unknown-argument")),
        }
    }
    let repository_root = absolute_path(required(repository_root)?)?;
    let execution_home = absolute_path(required(execution_home)?)?;
    let execution_tmp = absolute_path(required(execution_tmp)?)?;
    let execution_path_value = required(execution_path)?;
    let execution_path = parse_execution_path(execution_path_value.as_str())?;
    let role_policy = required(role_policy)?;
    let model_policy = required(model_policy)?;
    let check_registry = required(check_registry)?;
    let subject = required(subject)?;
    let head_state = required(head_state)?;
    let completion_args = vec![
        ARG_ROLE_POLICY.to_string(),
        role_policy,
        ARG_MODEL_POLICY.to_string(),
        model_policy,
        ARG_CHECK_REGISTRY.to_string(),
        check_registry,
        ARG_SUBJECT.to_string(),
        subject,
        ARG_HEAD_STATE.to_string(),
        head_state.clone(),
    ];
    Ok(RealSmokeArgs {
        repository_root,
        execution_path,
        execution_home,
        execution_tmp,
        completion_args,
        head_state,
    })
}

fn parse_head_state(value: String) -> Result<String, String> {
    if matches!(value.as_str(), HEAD_CLEAN | HEAD_DIRTY) {
        return Ok(value);
    }
    Err(error("head-state"))
}

fn absolute_path(value: String) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        return Ok(path);
    }
    Err(error("absolute-path"))
}

fn parse_execution_path(value: &str) -> Result<Vec<PathBuf>, String> {
    let paths = std::env::split_paths(value).collect::<Vec<_>>();
    if paths.is_empty() || paths.iter().any(|path| !path.is_absolute()) {
        return Err(error("execution-path"));
    }
    Ok(paths)
}

fn required<T>(value: Option<T>) -> Result<T, String> {
    value.ok_or_else(|| error("missing-argument"))
}

fn render_report(report: &animus_std::AuthoritativeCompletionSmokeReport) -> String {
    // r[impl agent_harness_runtime.real_check_completion_host_smoke.receipt]
    let mut lines = vec![format!(
        "{HEADER} schema={SCHEMA} status=completed authoritative_completion={} bounded_external_execution=true judge_decision={} required_checks={} passed_checks={} check_executor_calls={} cairn_validator_calls={} role_advisory_hash={} registry_hash={} selection_hash={} final_subject_hash={} execution_evidence_hash={} cairn_receipt_hash={} candidate_hash={} completion_receipt_hash={}",
        report.authoritative_completion(),
        report.judge_decision(),
        report.required_check_count(),
        report.passed_check_count(),
        report.check_executor_calls(),
        report.cairn_validator_calls(),
        report.role_advisory_hash(),
        report.registry_hash(),
        report.selection_hash(),
        report.final_subject_hash(),
        report.execution_evidence_hash(),
        report.cairn_receipt_hash(),
        report.candidate_hash(),
        report.completion_receipt_hash(),
    )];
    for surface in NON_CLAIMS {
        lines.push(format!("NON_CLAIM surface={surface}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn error(issue: &str) -> String {
    format!("{HEADER} status=failed issue={issue}")
}

struct ArgCursor<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> ArgCursor<'a> {
    const fn new(args: &'a [String]) -> Self {
        Self { args, index: 0 }
    }

    fn next(&mut self) -> Option<&'a String> {
        let argument = self.args.get(self.index)?;
        self.index = self.index.saturating_add(1);
        Some(argument)
    }

    fn next_value(&mut self) -> Result<String, String> {
        self.next()
            .cloned()
            .ok_or_else(|| error("missing-argument-value"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_accepts_explicit_absolute_context() {
        let args = [
            ARG_REPOSITORY_ROOT,
            "/repo",
            ARG_EXECUTION_PATH,
            "/bin:/usr/bin",
            ARG_EXECUTION_HOME,
            "/home/test",
            ARG_EXECUTION_TMP,
            "/tmp/test",
            ARG_ROLE_POLICY,
            "/repo/roles.json",
            ARG_MODEL_POLICY,
            "/repo/models.json",
            ARG_CHECK_REGISTRY,
            "/repo/checks.json",
            ARG_SUBJECT,
            "subject",
            ARG_HEAD_STATE,
            HEAD_CLEAN,
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        let parsed = parse_args(args.as_slice()).expect("explicit arguments are valid");

        assert_eq!(parsed.repository_root, PathBuf::from("/repo"));
        assert_eq!(parsed.execution_path.len(), 2);
        assert_eq!(parsed.head_state, HEAD_CLEAN);
        assert_eq!(parsed.completion_args.len(), 10);
    }

    #[test]
    fn parser_rejects_missing_unknown_relative_and_bad_head_values() {
        let missing = parse_args(&[]).expect_err("missing arguments");
        let unknown = parse_args(&["--unknown".to_string()]).expect_err("unknown argument");
        let relative = absolute_path("relative".to_string()).expect_err("relative path");
        let bad_head = parse_head_state("unknown".to_string()).expect_err("bad head state");

        assert!(missing.contains("missing-argument"));
        assert!(unknown.contains("unknown-argument"));
        assert!(relative.contains("absolute-path"));
        assert!(bad_head.contains("head-state"));
    }
}
