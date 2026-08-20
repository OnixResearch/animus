use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

const SUCCESS_EXIT_CODE: i32 = 0;
const NULL_INPUT_MAX_BYTES: usize = 1;

/// Exact bounded process request sent to the injected pueue process service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PueueProcessRequest {
    pub(crate) program: PathBuf,
    pub(crate) args: Vec<String>,
    pub(crate) current_dir: PathBuf,
    pub(crate) environment: Vec<(String, String)>,
    pub(crate) timeout_ms: u64,
    pub(crate) stdout_max_bytes: u64,
    pub(crate) stderr_max_bytes: u64,
    pub(crate) poll_interval_ms: u64,
    pub(crate) teardown_timeout_ms: u64,
}

impl PueueProcessRequest {
    /// Returns the exact absolute executable.
    #[must_use]
    pub fn program(&self) -> &Path {
        self.program.as_path()
    }

    /// Returns the exact arguments excluding argv zero.
    #[must_use]
    pub fn args(&self) -> &[String] {
        self.args.as_slice()
    }

    /// Returns the cleared-environment entries.
    #[must_use]
    pub fn environment(&self) -> &[(String, String)] {
        self.environment.as_slice()
    }
}

/// Stable bounded process result class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PueueProcessDisposition {
    /// Process exited with an admitted code and bounded output.
    Succeeded,
    /// Process exited with a denied code.
    ExitFailed,
    /// Process exceeded its deadline and its process group was torn down.
    TimedOut,
    /// Explicit cancellation stopped the process group.
    Cancelled,
    /// Captured output exceeded a declared byte bound.
    OutputLimitExceeded,
}

/// Bounded pueue process observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PueueProcessOutput {
    /// Stable completion class.
    pub disposition: PueueProcessDisposition,
    /// Bounded standard output bytes.
    pub stdout: Vec<u8>,
    /// Observed standard error byte count without durable raw text.
    pub stderr_observed_bytes: u64,
    /// Portable exit code when available.
    pub exit_code: Option<i32>,
}

/// Injected pueue process service failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PueueProcessError {
    /// Request bounds cannot fit the current platform.
    BoundConversion,
    /// Bounded process mechanics failed before a typed completion observation.
    RunnerFailed,
}

/// Explicit bounded process port used by the pueue host.
pub trait PueueProcessService {
    /// Runs one exact direct-exec request without a shell.
    ///
    /// # Errors
    ///
    /// Returns a typed infrastructure failure without raw process output.
    fn run(
        &mut self,
        request: &PueueProcessRequest,
    ) -> Result<PueueProcessOutput, PueueProcessError>;
}

/// Production pueue process service over the pinned `bounded-exec` cohort.
#[derive(Clone, Debug)]
pub struct BoundedExecPueueProcessService {
    cancellation: Arc<AtomicBool>,
}

impl BoundedExecPueueProcessService {
    /// Builds the service around one caller-owned cancellation flag.
    #[must_use]
    pub const fn new(cancellation: Arc<AtomicBool>) -> Self {
        Self { cancellation }
    }

    /// Returns the caller-owned cancellation flag.
    #[must_use]
    pub fn cancellation(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancellation)
    }
}

impl PueueProcessService for BoundedExecPueueProcessService {
    fn run(
        &mut self,
        request: &PueueProcessRequest,
    ) -> Result<PueueProcessOutput, PueueProcessError> {
        let stdout_max_bytes = usize::try_from(request.stdout_max_bytes)
            .map_err(|_error| PueueProcessError::BoundConversion)?;
        let stderr_max_bytes = usize::try_from(request.stderr_max_bytes)
            .map_err(|_error| PueueProcessError::BoundConversion)?;
        let policy = bounded_exec::OutcomePolicy::new(vec![SUCCESS_EXIT_CODE], true, true)
            .map_err(|_error| PueueProcessError::RunnerFailed)?;
        let output = bounded_exec::run_with_cancellation(
            bounded_exec::RunRequest {
                command: bounded_exec::CommandSpec {
                    program: request.program.clone(),
                    args: request.args.iter().map(OsString::from).collect(),
                    current_dir: request.current_dir.clone(),
                    environment_mode: bounded_exec::EnvironmentMode::Clear,
                    environment: request
                        .environment
                        .iter()
                        .map(|(key, value)| (OsString::from(key), OsString::from(value)))
                        .collect(),
                    input: bounded_exec::Input::Null,
                },
                limits: bounded_exec::ExecutionLimits {
                    timeout_ms: request.timeout_ms,
                    stdin_max_bytes: NULL_INPUT_MAX_BYTES,
                    stdout_max_bytes,
                    stderr_max_bytes,
                    poll_interval_ms: request.poll_interval_ms.min(request.timeout_ms),
                    teardown_timeout_ms: request.teardown_timeout_ms,
                },
                termination_scope: bounded_exec::TerminationScope::ProcessGroup,
                outcome_policy: policy,
            },
            &self.cancellation,
        )
        .map_err(|_error| PueueProcessError::RunnerFailed)?;
        let stderr_observed_bytes = u64::try_from(output.stderr.observed_bytes)
            .map_err(|_error| PueueProcessError::BoundConversion)?;
        Ok(PueueProcessOutput {
            disposition: map_disposition(output.disposition),
            stdout: output.stdout.bytes,
            stderr_observed_bytes,
            exit_code: output.exit_code,
        })
    }
}

const fn map_disposition(disposition: bounded_exec::Disposition) -> PueueProcessDisposition {
    match disposition {
        bounded_exec::Disposition::Succeeded => PueueProcessDisposition::Succeeded,
        bounded_exec::Disposition::ExitFailed => PueueProcessDisposition::ExitFailed,
        bounded_exec::Disposition::TimedOut => PueueProcessDisposition::TimedOut,
        bounded_exec::Disposition::Cancelled => PueueProcessDisposition::Cancelled,
        bounded_exec::Disposition::OutputLimitExceeded(_streams) => {
            PueueProcessDisposition::OutputLimitExceeded
        }
    }
}
