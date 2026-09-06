//! Bounded exact-command executor for selected completion checks.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use animus_runtime::authoritative_completion::CheckStatus;
use animus_std::{
    AgentCheckExecutionRequest, AgentCheckExecutionService, HostCheckExecutionObservation,
};
use bounded_exec::{
    CommandSpec, Completion, Disposition, EnvironmentMode, ExecutionLimits, Input, OutcomePolicy,
    RunRequest, TerminationScope,
};

const SUCCESS_EXIT_CODE: i32 = 0;
const NULL_STDIN_MAX_BYTES: usize = 1;
const NIX_CONFIG: &str = "builders =\nsecret-key-files =";
const EVIDENCE_DOMAIN: &str = "animus.real-check-completion.execution-evidence.v1";
const RECEIPT_DOMAIN: &str = "animus.real-check-completion.execution-receipt.v1";
const OUTPUT_DOMAIN: &str = "animus.real-check-completion.output.v1";

/// Finite process limits attached to one reviewed check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckExecutionBudget {
    /// Maximum child lifetime in milliseconds.
    pub timeout_ms: u64,
    /// Maximum retained stdout bytes.
    pub stdout_max_bytes: usize,
    /// Maximum retained stderr bytes.
    pub stderr_max_bytes: usize,
    /// Child-status polling interval in milliseconds.
    pub poll_interval_ms: u64,
    /// Maximum I/O teardown wait in milliseconds.
    pub teardown_timeout_ms: u64,
}

impl CheckExecutionBudget {
    fn limits(self) -> Result<ExecutionLimits, ExecutorError> {
        let limits = ExecutionLimits {
            timeout_ms: self.timeout_ms,
            stdin_max_bytes: NULL_STDIN_MAX_BYTES,
            stdout_max_bytes: self.stdout_max_bytes,
            stderr_max_bytes: self.stderr_max_bytes,
            poll_interval_ms: self.poll_interval_ms,
            teardown_timeout_ms: self.teardown_timeout_ms,
        };
        limits
            .validate()
            .map_err(|_failure| ExecutorError::InvalidBudget)?;
        Ok(limits)
    }
}

/// Exact check identifier, argv, and limits admitted by one host profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowedCheck {
    /// Stable registry check identifier.
    pub check_id: String,
    /// Exact argv including the executable name.
    pub argv: Vec<String>,
    /// Finite child-process limits.
    pub budget: CheckExecutionBudget,
}

/// Explicit process context for bounded selected-check execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedCheckExecutorConfig {
    /// Absolute repository root used as the child working directory.
    pub repository_root: PathBuf,
    /// Ordered absolute directories used to resolve argv zero.
    pub executable_search_path: Vec<PathBuf>,
    /// Explicit HOME value supplied to children.
    pub home: PathBuf,
    /// Explicit temporary directory supplied to children.
    pub temporary_directory: PathBuf,
    /// Exact allowed checks.
    pub allowed_checks: Vec<AllowedCheck>,
}

impl BoundedCheckExecutorConfig {
    fn validate(&self) -> Result<(), ExecutorError> {
        if !self.repository_root.is_absolute()
            || !self.home.is_absolute()
            || !self.temporary_directory.is_absolute()
            || self.executable_search_path.is_empty()
            || self.allowed_checks.is_empty()
        {
            return Err(ExecutorError::InvalidConfig);
        }
        if self
            .executable_search_path
            .iter()
            .any(|path| !path.is_absolute())
            || std::env::join_paths(self.executable_search_path.iter()).is_err()
        {
            return Err(ExecutorError::InvalidConfig);
        }
        for (index, check) in self.allowed_checks.iter().enumerate() {
            if check.check_id.is_empty() || check.argv.is_empty() {
                return Err(ExecutorError::InvalidConfig);
            }
            check.budget.limits()?;
            if self.allowed_checks[..index]
                .iter()
                .any(|prior| prior.check_id == check.check_id)
            {
                return Err(ExecutorError::InvalidConfig);
            }
        }
        Ok(())
    }
}

/// Stable bounded executor failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutorError {
    /// Host configuration is malformed or incomplete.
    InvalidConfig,
    /// A named budget is invalid.
    InvalidBudget,
    /// The selected check is not in the host allowlist.
    UnsupportedCheck,
    /// Registry argv differs from the reviewed host argv.
    CommandDrift,
    /// The executable cannot be resolved from the explicit search path.
    ExecutableMissing,
    /// The bounded runner failed before it could return complete child facts.
    RunnerFailed,
    /// A byte counter cannot be represented by the completion DTO.
    ByteCountOverflow,
    /// Explicit cancellation was already raised at the pre-spawn checkpoint.
    CancelledBeforeSpawn,
}

impl ExecutorError {
    /// Return a stable safe issue label.
    #[must_use]
    pub const fn issue(self) -> &'static str {
        match self {
            Self::InvalidConfig => "invalid-config",
            Self::InvalidBudget => "invalid-budget",
            Self::UnsupportedCheck => "unsupported-check",
            Self::CommandDrift => "command-drift",
            Self::ExecutableMissing => "executable-missing",
            Self::RunnerFailed => "runner-failed",
            Self::ByteCountOverflow => "byte-count-overflow",
            Self::CancelledBeforeSpawn => "cancelled-before-spawn",
        }
    }
}

/// Bounded process shell implementing the Animus selected-check seam.
pub struct BoundedCheckExecutor {
    config: BoundedCheckExecutorConfig,
}

impl BoundedCheckExecutor {
    /// Validate and construct one executor.
    ///
    /// # Errors
    ///
    /// Returns a safe configuration or budget failure.
    pub fn new(config: BoundedCheckExecutorConfig) -> Result<Self, ExecutorError> {
        config.validate()?;
        Ok(Self { config })
    }

    fn allowed_check<'a>(
        &'a self,
        request: AgentCheckExecutionRequest<'_>,
    ) -> Result<&'a AllowedCheck, ExecutorError> {
        let allowed = self
            .config
            .allowed_checks
            .iter()
            .find(|check| check.check_id == request.check.check_id())
            .ok_or(ExecutorError::UnsupportedCheck)?;
        if allowed.argv != request.check.command() {
            return Err(ExecutorError::CommandDrift);
        }
        Ok(allowed)
    }

    fn resolve_program(&self, argv_zero: &str) -> Result<PathBuf, ExecutorError> {
        let program = Path::new(argv_zero);
        if program.components().count() != 1 {
            return Err(ExecutorError::CommandDrift);
        }
        self.config
            .executable_search_path
            .iter()
            .map(|directory| directory.join(program))
            .find(|candidate| candidate.is_file())
            .ok_or(ExecutorError::ExecutableMissing)
    }

    fn environment(&self) -> Result<Vec<(OsString, OsString)>, ExecutorError> {
        let joined_path = std::env::join_paths(self.config.executable_search_path.iter())
            .map_err(|_failure| ExecutorError::InvalidConfig)?;
        Ok(vec![
            (OsString::from("PATH"), joined_path),
            (
                OsString::from("HOME"),
                self.config.home.as_os_str().to_os_string(),
            ),
            (
                OsString::from("TMPDIR"),
                self.config.temporary_directory.as_os_str().to_os_string(),
            ),
            (OsString::from("NIX_CONFIG"), OsString::from(NIX_CONFIG)),
        ])
    }

    /// Execute once and retain private bounded bytes beside the observation.
    ///
    /// Admission and child mechanics are identical to the metadata-only service.
    /// The caller owns current authorization and output interpretation.
    ///
    /// # Errors
    ///
    /// Returns the existing admission or runner error without a partial capture.
    pub fn execute_captured(
        &mut self,
        request: AgentCheckExecutionRequest<'_>,
    ) -> Result<crate::CapturedCheck, ExecutorError> {
        self.capture_inner(request, None)
    }

    /// Execute with an explicit caller-owned cancellation flag.
    ///
    /// Cancellation is cooperative, not atomic authorization or effect rollback.
    /// No retry or fallback occurs. The flag is never modified by this host.
    ///
    /// # Errors
    ///
    /// Returns `CancelledBeforeSpawn` at an already-cancelled host checkpoint,
    /// or the existing admission or runner error. Later cancellation is retained
    /// as the runner's non-passing execution result after owned teardown.
    pub fn execute_captured_with_cancellation(
        &mut self,
        request: AgentCheckExecutionRequest<'_>,
        cancellation: &std::sync::atomic::AtomicBool,
    ) -> Result<crate::CapturedCheck, ExecutorError> {
        self.capture_inner(request, Some(cancellation))
    }

    fn capture_inner(
        &self,
        request: AgentCheckExecutionRequest<'_>,
        cancellation: Option<&std::sync::atomic::AtomicBool>,
    ) -> Result<crate::CapturedCheck, ExecutorError> {
        // r[impl agent_harness_runtime.real_check_completion_host_smoke.execution]
        // r[impl agent_harness_runtime.real_check_completion_host_smoke.failure]
        // r[impl agent_harness_runtime.check_capture.retention]
        // r[impl agent_harness_runtime.check_capture.cancellation]
        let allowed = self.allowed_check(request)?;
        if cancellation.is_some_and(|flag| flag.load(std::sync::atomic::Ordering::Acquire)) {
            return Err(ExecutorError::CancelledBeforeSpawn);
        }
        let program = self.resolve_program(allowed.argv[0].as_str())?;
        let args = allowed.argv[1..]
            .iter()
            .map(OsString::from)
            .collect::<Vec<_>>();
        let outcome_policy = OutcomePolicy::new(vec![SUCCESS_EXIT_CODE], true, true)
            .map_err(|_failure| ExecutorError::InvalidConfig)?;
        let run_request = RunRequest {
            command: CommandSpec {
                program,
                args,
                current_dir: self.config.repository_root.clone(),
                environment_mode: EnvironmentMode::Clear,
                environment: self.environment()?,
                input: Input::Null,
            },
            limits: allowed.budget.limits()?,
            termination_scope: TerminationScope::ProcessGroup,
            outcome_policy,
        };
        let output = match cancellation {
            Some(flag) => bounded_exec::run_with_cancellation(run_request, flag),
            None => bounded_exec::run(run_request),
        }
        .map_err(|_failure| ExecutorError::RunnerFailed)?;
        let observed = observation(request, &output)?;
        Ok(crate::CapturedCheck::new(observed, output))
    }
}

impl AgentCheckExecutionService for BoundedCheckExecutor {
    type Error = ExecutorError;

    fn execute(
        &mut self,
        request: AgentCheckExecutionRequest<'_>,
    ) -> Result<HostCheckExecutionObservation, Self::Error> {
        // r[impl agent_harness_runtime.check_capture.compatibility]
        self.execute_captured(request)
            .map(crate::CapturedCheck::into_observation)
    }
}

fn observation(
    request: AgentCheckExecutionRequest<'_>,
    output: &bounded_exec::ExecutionOutput,
) -> Result<HostCheckExecutionObservation, ExecutorError> {
    let stdout_bytes = u64::try_from(output.stdout.observed_bytes)
        .map_err(|_failure| ExecutorError::ByteCountOverflow)?;
    let stderr_bytes = u64::try_from(output.stderr.observed_bytes)
        .map_err(|_failure| ExecutorError::ByteCountOverflow)?;
    let status = status_for_disposition(output.disposition);
    let stdout_hash = hash_parts(OUTPUT_DOMAIN, &[b"stdout", output.stdout.bytes.as_slice()]);
    let stderr_hash = hash_parts(OUTPUT_DOMAIN, &[b"stderr", output.stderr.bytes.as_slice()]);
    let exit_code = output
        .exit_code
        .map_or_else(String::new, |value| value.to_string());
    let signal = output
        .signal
        .map_or_else(String::new, |value| value.to_string());
    let stdout_count = stdout_bytes.to_string();
    let stderr_count = stderr_bytes.to_string();
    let evidence_hash = hash_text_parts(
        EVIDENCE_DOMAIN,
        &[
            request.check.check_id(),
            request.check.command_hash(),
            request.final_subject_hash,
            status.as_str(),
            exit_code.as_str(),
            signal.as_str(),
            stdout_count.as_str(),
            stderr_count.as_str(),
            stdout_hash.as_str(),
            stderr_hash.as_str(),
        ],
    );
    let execution_receipt_hash = hash_text_parts(
        RECEIPT_DOMAIN,
        &[
            request.registry_hash,
            request.selection_hash,
            request.policy_hash,
            request.final_subject_hash,
            evidence_hash.as_str(),
        ],
    );
    Ok(HostCheckExecutionObservation {
        status,
        evidence_hash,
        execution_receipt_hash,
        exit_code: output.exit_code,
        signal: output.signal,
        timed_out: output.completion == Completion::TimedOut,
        cleanup_complete: true,
        stdout_bytes,
        stderr_bytes,
        stdout_hash,
        stderr_hash,
        artifacts: Vec::new(),
    })
}

const fn status_for_disposition(disposition: Disposition) -> CheckStatus {
    match disposition {
        Disposition::Succeeded => CheckStatus::Passed,
        Disposition::ExitFailed => CheckStatus::Failed,
        Disposition::TimedOut => CheckStatus::TimedOut,
        Disposition::Cancelled => CheckStatus::Cancelled,
        Disposition::OutputLimitExceeded(_streams) => CheckStatus::InfrastructureError,
    }
}

fn hash_text_parts(domain: &str, parts: &[&str]) -> String {
    let bytes = parts.iter().map(|part| part.as_bytes()).collect::<Vec<_>>();
    hash_parts(domain, bytes.as_slice())
}

fn hash_parts(domain: &str, parts: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    update_framed(&mut hasher, domain.as_bytes());
    for part in parts {
        update_framed(&mut hasher, part);
    }
    hasher.finalize().to_hex().to_string()
}

fn update_framed(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    let length = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    hasher.update(length.to_le_bytes().as_slice());
    hasher.update(bytes);
}
