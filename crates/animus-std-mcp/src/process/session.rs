use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, sync_channel};
use std::time::{Duration, Instant};

use super::reader::{LineEvent, start_readers};
use super::terminate::{configure_scope, terminate_owned, wait_until};
use super::{ProcessError, ProcessLaunch, ProcessScope, StderrObservation, TeardownObservation};
use crate::McpLimits;

pub(crate) struct SessionTeardown {
    pub(crate) observation: TeardownObservation,
    pub(crate) scope: ProcessScope,
    pub(crate) stderr: Option<StderrObservation>,
}

pub(crate) struct StdioSession {
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<LineEvent>,
    stderr: Receiver<StderrObservation>,
    stdout_done: Receiver<()>,
    stderr_done: Receiver<()>,
    stdout_thread: Option<std::thread::JoinHandle<()>>,
    stderr_thread: Option<std::thread::JoinHandle<()>>,
    scope: ProcessScope,
    limits: McpLimits,
    stopped: bool,
}

impl StdioSession {
    pub(crate) fn spawn(launch: ProcessLaunch) -> Result<Self, ProcessError> {
        let mut command = Command::new(&launch.executable);
        command
            .args(&launch.arguments)
            .current_dir(&launch.current_directory)
            .env_clear()
            .envs(launch.environment)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let scope = configure_scope(&mut command);
        let mut child = command.spawn().map_err(|_| ProcessError::Launch)?;
        let Some(stdin) = child.stdin.take() else {
            return Err(cleanup_incomplete_spawn(
                &mut child,
                scope,
                launch.limits,
                ProcessError::MissingPipe,
            ));
        };
        let Some(stdout) = child.stdout.take() else {
            return Err(cleanup_incomplete_spawn(
                &mut child,
                scope,
                launch.limits,
                ProcessError::MissingPipe,
            ));
        };
        let Some(stderr) = child.stderr.take() else {
            return Err(cleanup_incomplete_spawn(
                &mut child,
                scope,
                launch.limits,
                ProcessError::MissingPipe,
            ));
        };
        let workers = match start_readers(
            stdout,
            stderr,
            launch.limits.max_frame_bytes,
            launch.limits.max_diagnostic_bytes,
        ) {
            Ok(workers) => workers,
            Err(error) => {
                return Err(cleanup_incomplete_spawn(
                    &mut child,
                    scope,
                    launch.limits,
                    error,
                ));
            }
        };
        Ok(Self {
            child,
            stdin: Some(stdin),
            lines: workers.lines,
            stderr: workers.stderr,
            stdout_done: workers.stdout_done,
            stderr_done: workers.stderr_done,
            stdout_thread: Some(workers.stdout_thread),
            stderr_thread: Some(workers.stderr_thread),
            scope,
            limits: launch.limits,
            stopped: false,
        })
    }

    pub(crate) fn exchange(
        &mut self,
        frame: &[u8],
        timeout_ms: u64,
        cancellation: &AtomicBool,
    ) -> Result<Vec<u8>, ProcessError> {
        self.write_frame(frame)?;
        self.receive_frame(timeout_ms, cancellation)
    }

    pub(crate) fn notify(&mut self, frame: &[u8]) -> Result<(), ProcessError> {
        self.write_frame(frame)
    }

    fn write_frame(&mut self, frame: &[u8]) -> Result<(), ProcessError> {
        if frame.is_empty() || frame.len() > self.limits.max_frame_bytes || frame.contains(&b'\n') {
            return Err(ProcessError::OversizedFrame);
        }
        let stdin = self.stdin.as_mut().ok_or(ProcessError::EarlyExit)?;
        stdin.write_all(frame).map_err(|_| ProcessError::Write)?;
        stdin.write_all(b"\n").map_err(|_| ProcessError::Write)?;
        stdin.flush().map_err(|_| ProcessError::Flush)
    }

    fn receive_frame(
        &self,
        timeout_ms: u64,
        cancellation: &AtomicBool,
    ) -> Result<Vec<u8>, ProcessError> {
        let timeout = Duration::from_millis(timeout_ms);
        let poll = Duration::from_millis(self.limits.cancellation_poll_ms);
        let started = Instant::now();
        loop {
            if cancellation.load(Ordering::Acquire) {
                return Err(ProcessError::Cancelled);
            }
            let remaining = timeout.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return Err(ProcessError::Timeout);
            }
            let wait = remaining.min(poll);
            match self.lines.recv_timeout(wait) {
                Ok(LineEvent::Frame(frame)) => return Ok(frame),
                Ok(LineEvent::Error(error)) => return Err(error),
                Ok(LineEvent::Eof) | Err(RecvTimeoutError::Disconnected) => {
                    return Err(ProcessError::EarlyExit);
                }
                Err(RecvTimeoutError::Timeout) => {}
            }
        }
    }

    pub(crate) fn stop(mut self) -> SessionTeardown {
        self.stdin.take();
        let poll = Duration::from_millis(self.limits.cancellation_poll_ms);
        let graceful = wait_until(
            &mut self.child,
            Duration::from_millis(self.limits.shutdown_timeout_ms),
            poll,
        )
        .unwrap_or_default();
        let observation = if graceful {
            if terminate_owned(&mut self.child, self.scope).is_ok() {
                TeardownObservation::Clean
            } else {
                TeardownObservation::Uncertain
            }
        } else {
            forced_teardown(&mut self.child, self.scope, self.limits, poll)
        };
        let process_terminal = observation.terminal();
        let worker_deadline =
            Instant::now() + Duration::from_millis(self.limits.teardown_timeout_ms);
        let stderr = if process_terminal {
            let remaining = worker_deadline.saturating_duration_since(Instant::now());
            self.stderr.recv_timeout(remaining).ok()
        } else {
            None
        };
        let joined = process_terminal && self.finish_workers(worker_deadline);
        self.stopped = process_terminal;
        SessionTeardown {
            observation: if process_terminal && joined {
                observation
            } else {
                TeardownObservation::Uncertain
            },
            scope: self.scope,
            stderr,
        }
    }

    pub(crate) const fn scope(&self) -> ProcessScope {
        self.scope
    }

    fn finish_workers(&mut self, deadline: Instant) -> bool {
        self.disconnect_output_receivers();
        let stdout_finished = wait_for_worker(&self.stdout_done, deadline);
        let stderr_finished = wait_for_worker(&self.stderr_done, deadline);
        stdout_finished && stderr_finished && self.join_workers()
    }

    fn disconnect_output_receivers(&mut self) {
        let (_line_sender, replacement_lines) = sync_channel(1);
        let old_lines = std::mem::replace(&mut self.lines, replacement_lines);
        drop(old_lines);
        let (_stderr_sender, replacement_stderr) = sync_channel(1);
        let old_stderr = std::mem::replace(&mut self.stderr, replacement_stderr);
        drop(old_stderr);
    }

    fn join_workers(&mut self) -> bool {
        let stdout_joined = self
            .stdout_thread
            .take()
            .is_none_or(|worker| worker.join().is_ok());
        let stderr_joined = self
            .stderr_thread
            .take()
            .is_none_or(|worker| worker.join().is_ok());
        stdout_joined && stderr_joined
    }
}

impl Drop for StdioSession {
    fn drop(&mut self) {
        if !self.stopped {
            self.stdin.take();
            let _terminated = terminate_owned(&mut self.child, self.scope);
            let _reaped = self.child.try_wait();
        }
        self.disconnect_output_receivers();
        let stdout_finished = self.stdout_done.try_recv().is_ok();
        let stderr_finished = self.stderr_done.try_recv().is_ok();
        if stdout_finished && stderr_finished {
            let _joined = self.join_workers();
        }
    }
}

fn wait_for_worker(receiver: &Receiver<()>, deadline: Instant) -> bool {
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return false;
    }
    receiver.recv_timeout(remaining).is_ok()
}

fn cleanup_incomplete_spawn(
    child: &mut Child,
    scope: ProcessScope,
    limits: McpLimits,
    cause: ProcessError,
) -> ProcessError {
    if terminate_owned(child, scope).is_err() {
        return ProcessError::StartCleanupUncertain;
    }
    let poll = Duration::from_millis(limits.cancellation_poll_ms);
    match wait_until(
        child,
        Duration::from_millis(limits.teardown_timeout_ms),
        poll,
    ) {
        Ok(true) if cause == ProcessError::MissingPipe => ProcessError::StartCleanup,
        _ => ProcessError::StartCleanupUncertain,
    }
}

fn forced_teardown(
    child: &mut Child,
    scope: ProcessScope,
    limits: McpLimits,
    poll: Duration,
) -> TeardownObservation {
    if terminate_owned(child, scope).is_err() {
        return TeardownObservation::Uncertain;
    }
    match wait_until(
        child,
        Duration::from_millis(limits.teardown_timeout_ms),
        poll,
    ) {
        Ok(true) => TeardownObservation::Forced,
        Ok(false) | Err(_) => TeardownObservation::Uncertain,
    }
}
