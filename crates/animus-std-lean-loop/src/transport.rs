use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::Ordering;
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel};
use std::thread::JoinHandle;

use crate::core::domain_hash;
use crate::{
    ARTIFACT_DOMAIN, LeanCleanupStatus, LeanLaunchDescriptor, LeanLoopError, LeanLoopFailureClass,
    LeanLoopProfile, LeanSandboxObservation, LeanSandboxPosture, LeanTransportObservation,
};

const FIELD_EXECUTABLE: &str = "executable";
const FIELD_SANDBOX: &str = "sandbox";
const FIELD_TRANSPORT: &str = "transport";
const FIELD_CLEANUP: &str = "cleanup";
const CHANNEL_CAPACITY: usize = 1;
const LINE_TERMINATOR: u8 = b'\n';
const MAX_ARTIFACT_BYTES: u64 = 256 * 1024 * 1024;
const ONE_FRAME: u32 = 1;

/// Injected bounded frame transport used by the pure protocol/fabric shell.
pub trait LeanTransport {
    /// Exchanges one line-delimited request for exactly one response.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] for framing, deadline, cancellation, pipe, or terminal failures.
    fn exchange(&mut self, frame: &str, poll_count: u32) -> Result<String, LeanLoopError>;
    /// Performs bounded transport cleanup.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] when exit, reader joining, stderr bounds, or termination fails.
    fn cleanup(&mut self) -> Result<LeanCleanupStatus, LeanLoopError>;
    /// Returns a metadata-only transport observation.
    fn observation(&self) -> LeanTransportObservation;
}

#[derive(Debug)]
enum ReaderMessage {
    Frame(String),
    Failure(LeanLoopFailureClass),
}

/// Persistent explicit child process with bounded stdin/stdout/stderr ownership.
pub struct StdioLeanProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    responses: Receiver<ReaderMessage>,
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<usize>>,
    profile: LeanLoopProfile,
    descriptor: LeanLaunchDescriptor,
    terminal: bool,
    frame_count: u32,
    byte_count: u64,
    failure: Option<LeanLoopFailureClass>,
    cleanup_status: LeanCleanupStatus,
}

impl StdioLeanProcess {
    /// Validates the launch descriptor, verifies the executable, and starts bounded readers.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] when path/sandbox admission, artifact verification, spawn, or
    /// pipe acquisition fails.
    pub fn launch(
        profile: LeanLoopProfile,
        descriptor: LeanLaunchDescriptor,
    ) -> Result<(Self, String), LeanLoopError> {
        validate_launch(&profile, &descriptor)?;
        let artifact_identity = artifact_identity(descriptor.executable.as_path())?;
        if artifact_identity != descriptor.expected_artifact_identity {
            return Err(LeanLoopError::new(
                LeanLoopFailureClass::ArtifactIdentityMismatch,
                FIELD_EXECUTABLE,
            ));
        }
        let mut child = Command::new(descriptor.executable.as_os_str())
            .current_dir(descriptor.working_directory.as_path())
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_error| {
                LeanLoopError::new(LeanLoopFailureClass::SpawnFailure, FIELD_EXECUTABLE)
            })?;
        let stdin = child.stdin.take().ok_or_else(|| {
            LeanLoopError::new(LeanLoopFailureClass::SpawnFailure, FIELD_TRANSPORT)
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            LeanLoopError::new(LeanLoopFailureClass::SpawnFailure, FIELD_TRANSPORT)
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            LeanLoopError::new(LeanLoopFailureClass::SpawnFailure, FIELD_TRANSPORT)
        })?;
        let (sender, responses) = sync_channel(CHANNEL_CAPACITY);
        let frame_bytes = profile.bounds.frame_bytes;
        let stdout_thread = std::thread::spawn(move || read_stdout(stdout, frame_bytes, &sender));
        let diagnostic_bytes = profile.bounds.diagnostic_bytes;
        let stderr_thread = std::thread::spawn(move || read_stderr(stderr, diagnostic_bytes));
        Ok((
            Self {
                child,
                stdin: Some(stdin),
                responses,
                stdout_thread: Some(stdout_thread),
                stderr_thread: Some(stderr_thread),
                profile,
                descriptor,
                terminal: false,
                frame_count: 0,
                byte_count: 0,
                failure: None,
                cleanup_status: LeanCleanupStatus::Completed,
            },
            artifact_identity,
        ))
    }

    fn fail<T>(&mut self, failure: LeanLoopFailureClass) -> Result<T, LeanLoopError> {
        self.failure = Some(failure);
        self.terminal = true;
        Err(LeanLoopError::new(failure, FIELD_TRANSPORT))
    }
}

impl LeanTransport for StdioLeanProcess {
    fn exchange(&mut self, frame: &str, poll_count: u32) -> Result<String, LeanLoopError> {
        if self.terminal {
            return self.fail(LeanLoopFailureClass::TerminalTransport);
        }
        if frame.len() > self.profile.bounds.frame_bytes {
            return self.fail(LeanLoopFailureClass::FrameLimitExceeded);
        }
        if self.descriptor.cancellation.load(Ordering::Acquire) {
            return self.fail(LeanLoopFailureClass::CancellationAmbiguous);
        }
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| LeanLoopError::new(LeanLoopFailureClass::BrokenPipe, FIELD_TRANSPORT))?;
        stdin.write_all(frame.as_bytes()).map_err(|error| {
            let failure = if error.kind() == std::io::ErrorKind::BrokenPipe {
                LeanLoopFailureClass::BrokenPipe
            } else if error.kind() == std::io::ErrorKind::WouldBlock {
                LeanLoopFailureClass::Backpressure
            } else {
                LeanLoopFailureClass::WriteFailure
            };
            self.failure = Some(failure);
            self.terminal = true;
            LeanLoopError::new(failure, FIELD_TRANSPORT)
        })?;
        stdin.write_all(&[LINE_TERMINATOR]).map_err(|_error| {
            self.failure = Some(LeanLoopFailureClass::WriteFailure);
            self.terminal = true;
            LeanLoopError::new(LeanLoopFailureClass::WriteFailure, FIELD_TRANSPORT)
        })?;
        stdin.flush().map_err(|_error| {
            self.failure = Some(LeanLoopFailureClass::FlushFailure);
            self.terminal = true;
            LeanLoopError::new(LeanLoopFailureClass::FlushFailure, FIELD_TRANSPORT)
        })?;
        self.frame_count = self.frame_count.saturating_add(ONE_FRAME);
        self.byte_count = self
            .byte_count
            .saturating_add(u64::try_from(frame.len()).unwrap_or(u64::MAX));

        for _poll in 0..poll_count {
            if self.descriptor.cancellation.load(Ordering::Acquire) {
                return self.fail(LeanLoopFailureClass::CancellationAmbiguous);
            }
            match self
                .responses
                .recv_timeout(self.profile.bounds.poll_interval)
            {
                Ok(ReaderMessage::Frame(response)) => {
                    self.frame_count = self.frame_count.saturating_add(ONE_FRAME);
                    self.byte_count = self
                        .byte_count
                        .saturating_add(u64::try_from(response.len()).unwrap_or(u64::MAX));
                    return Ok(response);
                }
                Ok(ReaderMessage::Failure(failure)) => return self.fail(failure),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => {
                    return self.fail(LeanLoopFailureClass::InputEof);
                }
            }
        }
        self.fail(LeanLoopFailureClass::ResponseDeadline)
    }

    fn cleanup(&mut self) -> Result<LeanCleanupStatus, LeanLoopError> {
        self.stdin.take();
        let mut forced = false;
        let mut exited = None;
        for _poll in 0..self.profile.bounds.cleanup_poll_count {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    exited = Some(status);
                    break;
                }
                Ok(None) => std::thread::sleep(self.profile.bounds.poll_interval),
                Err(_error) => {
                    self.cleanup_status = LeanCleanupStatus::Failed;
                    return Err(LeanLoopError::new(
                        LeanLoopFailureClass::CleanupFailure,
                        FIELD_CLEANUP,
                    ));
                }
            }
        }
        if exited.is_none() {
            self.child.kill().map_err(|_error| {
                LeanLoopError::new(LeanLoopFailureClass::CleanupFailure, FIELD_CLEANUP)
            })?;
            forced = true;
            exited = Some(self.child.wait().map_err(|_error| {
                LeanLoopError::new(LeanLoopFailureClass::CleanupFailure, FIELD_CLEANUP)
            })?);
        }
        if let Some(thread) = self.stdout_thread.take() {
            thread.join().map_err(|_panic| {
                LeanLoopError::new(LeanLoopFailureClass::CleanupFailure, FIELD_CLEANUP)
            })?;
        }
        if let Some(thread) = self.stderr_thread.take() {
            let stderr_bytes = thread.join().map_err(|_panic| {
                LeanLoopError::new(LeanLoopFailureClass::CleanupFailure, FIELD_CLEANUP)
            })?;
            if stderr_bytes > self.profile.bounds.diagnostic_bytes {
                self.failure = Some(LeanLoopFailureClass::StderrLimitExceeded);
                self.cleanup_status = LeanCleanupStatus::Failed;
                return Err(LeanLoopError::new(
                    LeanLoopFailureClass::StderrLimitExceeded,
                    FIELD_CLEANUP,
                ));
            }
        }
        let status = exited.expect("cleanup exit status must be present");
        if !forced && !status.success() {
            self.failure = Some(LeanLoopFailureClass::NonZeroExit);
            self.cleanup_status = LeanCleanupStatus::Failed;
            return Err(LeanLoopError::new(
                LeanLoopFailureClass::NonZeroExit,
                FIELD_CLEANUP,
            ));
        }
        self.terminal = true;
        self.cleanup_status = if forced {
            LeanCleanupStatus::Forced
        } else {
            LeanCleanupStatus::Completed
        };
        Ok(self.cleanup_status)
    }

    fn observation(&self) -> LeanTransportObservation {
        LeanTransportObservation {
            frame_count: self.frame_count,
            byte_count: self.byte_count,
            failure: self.failure,
            cleanup: self.cleanup_status,
        }
    }
}

impl Drop for StdioLeanProcess {
    fn drop(&mut self) {
        if !self.terminal {
            let _ignored = self.child.kill();
            let _ignored = self.child.wait();
        }
    }
}

fn validate_launch(
    profile: &LeanLoopProfile,
    descriptor: &LeanLaunchDescriptor,
) -> Result<(), LeanLoopError> {
    if !descriptor.executable.is_absolute()
        || !descriptor.working_directory.is_absolute()
        || descriptor
            .executable
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        || descriptor
            .working_directory
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(LeanLoopError::new(
            LeanLoopFailureClass::ExecutableRejected,
            FIELD_EXECUTABLE,
        ));
    }
    let sandbox_valid = match profile.sandbox_posture {
        LeanSandboxPosture::Required => {
            descriptor.sandbox_observation == LeanSandboxObservation::Enforced
        }
        LeanSandboxPosture::FixtureOnly => matches!(
            descriptor.sandbox_observation,
            LeanSandboxObservation::Enforced | LeanSandboxObservation::FixtureOnly
        ),
    };
    if !sandbox_valid {
        return Err(LeanLoopError::new(
            LeanLoopFailureClass::SandboxEvidenceMissing,
            FIELD_SANDBOX,
        ));
    }
    Ok(())
}

/// Computes the domain-separated BLAKE3 identity of one bounded executable file.
///
/// # Errors
///
/// Returns [`LeanLoopError`] when the path is unreadable, not a regular non-empty file, or exceeds
/// the artifact byte bound.
pub fn artifact_identity(path: &std::path::Path) -> Result<String, LeanLoopError> {
    let metadata = std::fs::metadata(path).map_err(|_error| {
        LeanLoopError::new(LeanLoopFailureClass::ExecutableRejected, FIELD_EXECUTABLE)
    })?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > MAX_ARTIFACT_BYTES {
        return Err(LeanLoopError::new(
            LeanLoopFailureClass::ExecutableRejected,
            FIELD_EXECUTABLE,
        ));
    }
    let bytes = std::fs::read(path).map_err(|_error| {
        LeanLoopError::new(LeanLoopFailureClass::ExecutableRejected, FIELD_EXECUTABLE)
    })?;
    Ok(domain_hash(ARTIFACT_DOMAIN, bytes.as_slice()))
}

fn read_stdout(
    stdout: std::process::ChildStdout,
    frame_bytes: usize,
    sender: &SyncSender<ReaderMessage>,
) {
    let mut reader = BufReader::new(stdout);
    loop {
        let mut buffer = Vec::with_capacity(frame_bytes.min(crate::model::REDACTION_MARKER.len()));
        let limit = u64::try_from(frame_bytes.saturating_add(1)).unwrap_or(u64::MAX);
        let result = reader
            .by_ref()
            .take(limit)
            .read_until(LINE_TERMINATOR, &mut buffer);
        match result {
            Ok(0) => {
                let _ignored = sender.send(ReaderMessage::Failure(LeanLoopFailureClass::InputEof));
                return;
            }
            Ok(_read) if buffer.len() > frame_bytes => {
                let _ignored = sender.send(ReaderMessage::Failure(
                    LeanLoopFailureClass::FrameLimitExceeded,
                ));
                return;
            }
            Ok(_read) if buffer.last() != Some(&LINE_TERMINATOR) => {
                let _ignored = sender.send(ReaderMessage::Failure(
                    LeanLoopFailureClass::MalformedResponse,
                ));
                return;
            }
            Ok(_read) => {
                buffer.pop();
                match String::from_utf8(buffer) {
                    Ok(frame) => {
                        if sender.send(ReaderMessage::Frame(frame)).is_err() {
                            return;
                        }
                    }
                    Err(_error) => {
                        let _ignored = sender.send(ReaderMessage::Failure(
                            LeanLoopFailureClass::MalformedResponse,
                        ));
                        return;
                    }
                }
            }
            Err(_error) => {
                let _ignored =
                    sender.send(ReaderMessage::Failure(LeanLoopFailureClass::BrokenPipe));
                return;
            }
        }
    }
}

fn read_stderr(stderr: std::process::ChildStderr, diagnostic_bytes: usize) -> usize {
    let mut bytes = Vec::new();
    let limit = u64::try_from(diagnostic_bytes.saturating_add(1)).unwrap_or(u64::MAX);
    let _ignored = stderr.take(limit).read_to_end(&mut bytes);
    bytes.len()
}
