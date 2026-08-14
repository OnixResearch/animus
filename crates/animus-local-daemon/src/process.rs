//! Explicit worker process-tree launcher and observation shell.

use std::collections::BTreeMap;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const PROCESS_ID_DOMAIN: &str = "animus.local-daemon-process.v1";
const MAX_ARGUMENTS: usize = 32;
const MAX_ENVIRONMENT_ENTRIES: usize = 16;
const MAX_FIELD_BYTES: usize = 4096;
const POLL_SLEEP_MS: u64 = 10;

/// Exact request for one worker process tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerLaunchRequest {
    /// Executable selected by the composition root.
    pub executable: PathBuf,
    /// Explicit argument vector.
    pub arguments: Vec<String>,
    /// Explicit environment. No parent environment is inherited.
    pub environment: BTreeMap<String, String>,
    /// Explicit current directory.
    pub current_directory: PathBuf,
    /// Root session identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Monotonic generation.
    pub generation: u64,
    /// Endpoint identity expected during readiness.
    pub endpoint_identity: String,
}

/// Host-observed process identity facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessIdentity {
    /// Process identifier.
    pub process_id: u32,
    /// BLAKE3 identity over admitted process facts.
    pub identity: String,
}

/// Current bounded process observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessObservation {
    /// Process remains live.
    Running,
    /// Process exited and was reaped.
    Exited(ExitStatus),
    /// Deadline expired before exit.
    Deadline,
}

/// Spawned worker process tree owner.
pub struct SpawnedWorker {
    child: Child,
    identity: ProcessIdentity,
}

impl SpawnedWorker {
    /// Returns host-observed process identity.
    #[must_use]
    pub const fn identity(&self) -> &ProcessIdentity {
        &self.identity
    }

    /// Observes process state without blocking.
    ///
    /// # Errors
    ///
    /// Returns [`WorkerProcessError`] when the host cannot observe the process.
    pub fn observe(&mut self) -> Result<ProcessObservation, WorkerProcessError> {
        self.child
            .try_wait()
            .map(|status| status.map_or(ProcessObservation::Running, ProcessObservation::Exited))
            .map_err(|_error| WorkerProcessError::Io)
    }

    /// Continues bounded stop finalization until exit and reap or deadline.
    ///
    /// # Errors
    ///
    /// Returns [`WorkerProcessError`] for signal, wait, or deadline failure.
    pub fn stop_and_reap(
        &mut self,
        deadline: Duration,
    ) -> Result<ProcessObservation, WorkerProcessError> {
        if deadline.is_zero() {
            return Err(WorkerProcessError::InvalidRequest);
        }
        match self.observe()? {
            ProcessObservation::Exited(status) => return Ok(ProcessObservation::Exited(status)),
            ProcessObservation::Running | ProcessObservation::Deadline => {}
        }
        terminate_process_group(self.child.id())?;
        let started = Instant::now();
        while started.elapsed() < deadline {
            match self.observe()? {
                ProcessObservation::Running | ProcessObservation::Deadline => {
                    std::thread::sleep(Duration::from_millis(POLL_SLEEP_MS));
                }
                ProcessObservation::Exited(status) => {
                    return Ok(ProcessObservation::Exited(status));
                }
            }
        }
        Err(WorkerProcessError::Deadline)
    }
}

/// Explicit worker process launcher.
pub struct ProcessTreeLauncher;

impl ProcessTreeLauncher {
    /// Starts one worker with closed standard streams and no inherited environment.
    ///
    /// # Errors
    ///
    /// Returns [`WorkerProcessError`] for invalid inputs or spawn failure.
    pub fn launch(request: &WorkerLaunchRequest) -> Result<SpawnedWorker, WorkerProcessError> {
        validate_request(request)?;
        let mut command = Command::new(request.executable.as_path());
        command
            .args(request.arguments.iter())
            .env_clear()
            .envs(request.environment.iter())
            .current_dir(request.current_directory.as_path())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0);
        let child = command.spawn().map_err(|_error| WorkerProcessError::Io)?;
        let process_id = child.id();
        if process_id == 0 {
            return Err(WorkerProcessError::Identity);
        }
        let material = format!(
            "{PROCESS_ID_DOMAIN}\0{}\0{}\0{}\0{}\0{process_id}",
            request.root_id, request.worker_id, request.generation, request.endpoint_identity
        );
        let identity = ProcessIdentity {
            process_id,
            identity: blake3::hash(material.as_bytes()).to_hex().to_string(),
        };
        Ok(SpawnedWorker { child, identity })
    }
}

/// Worker process shell failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerProcessError {
    /// Launch request is missing a finite explicit input.
    InvalidRequest,
    /// Process identity could not be established.
    Identity,
    /// Process operation exceeded its deadline.
    Deadline,
    /// Process I/O failed.
    Io,
}

impl core::fmt::Display for WorkerProcessError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "worker process rejected: {self:?}")
    }
}

impl std::error::Error for WorkerProcessError {}

fn validate_request(request: &WorkerLaunchRequest) -> Result<(), WorkerProcessError> {
    if !absolute_file_candidate(request.executable.as_path())
        || !absolute_directory_candidate(request.current_directory.as_path())
        || request.arguments.len() > MAX_ARGUMENTS
        || request.environment.len() > MAX_ENVIRONMENT_ENTRIES
        || request.generation == 0
        || !valid_field(request.root_id.as_str())
        || !valid_field(request.worker_id.as_str())
        || request.endpoint_identity.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
        || !request
            .endpoint_identity
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || request.arguments.iter().any(|value| !valid_field(value))
        || request
            .environment
            .iter()
            .any(|(key, value)| !valid_field(key) || !valid_field(value) || key.contains('='))
    {
        return Err(WorkerProcessError::InvalidRequest);
    }
    Ok(())
}

fn terminate_process_group(process_id: u32) -> Result<(), WorkerProcessError> {
    use nix::errno::Errno;
    use nix::sys::signal::{Signal, killpg};
    use nix::unistd::Pid;

    let process_id = i32::try_from(process_id).map_err(|_error| WorkerProcessError::Identity)?;
    match killpg(Pid::from_raw(process_id), Signal::SIGKILL) {
        Ok(()) | Err(Errno::ESRCH) => Ok(()),
        Err(_error) => Err(WorkerProcessError::Io),
    }
}

fn absolute_file_candidate(path: &Path) -> bool {
    path.is_absolute() && path.file_name().is_some()
}

fn absolute_directory_candidate(path: &Path) -> bool {
    path.is_absolute()
}

fn valid_field(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FIELD_BYTES
        && !value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
}
