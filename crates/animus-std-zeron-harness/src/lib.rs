//! Optional bounded external-agent subprocess host.
//!
//! This host implements the `animus-adapter-zeron-harness`
//! [`BoundedAgentProcessService`] port over the pinned `bounded-exec` surface.
//! It owns process spawn, bounded standard I/O, deadlines, cancellation
//! shaping, owned teardown, working-directory resolution, and environment
//! translation. It does not own executable discovery, sandboxing, output
//! meaning, credentials, or release policy.
//!
//! License: the host depends on `bounded-exec`, which is `AGPL-3.0-or-later`.
//! Keep this crate a separate opt-in member so reusable `no_std` contracts do
//! not inherit that dependency.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

use std::ffi::OsString;
use std::path::PathBuf;

use animus_adapter_zeron_harness::{
    BoundedAgentProcessService, BoundedProcessOutcome, BoundedProcessRequest, EnvironmentMode,
    ServiceUnavailable,
};
use bounded_exec::{
    CommandSpec, Completion, ExecutionLimits, Input, OutcomePolicy, RunRequest, TerminationScope,
    run,
};

/// The accepted exit code for a successful agent CLI subprocess.
pub const ACCEPTED_EXIT_CODE: i32 = 0;
/// The exit code recorded when the process stopped without a portable code.
pub const UNKNOWN_EXIT_CODE: i32 = -1;

/// The default bounded-process host.
///
/// It carries no request state and is safe to share across calls.
#[derive(Clone, Debug, Default)]
pub struct BoundedExecAgentProcessService;

impl BoundedAgentProcessService for BoundedExecAgentProcessService {
    fn run(
        &self,
        request: &BoundedProcessRequest,
    ) -> Result<BoundedProcessOutcome, ServiceUnavailable> {
        let run_request = translate(request)?;
        let output = run(run_request).map_err(|_| ServiceUnavailable)?;
        Ok(translate_output(&output))
    }
}

fn translate(request: &BoundedProcessRequest) -> Result<RunRequest, ServiceUnavailable> {
    if request.program.is_empty() {
        return Err(ServiceUnavailable);
    }
    let program = PathBuf::from(&request.program);
    let args: Vec<OsString> = request.args.iter().map(OsString::from).collect();
    let current_dir = match &request.current_dir {
        Some(directory) => PathBuf::from(directory),
        None => std::env::current_dir().map_err(|_| ServiceUnavailable)?,
    };
    let environment_mode = match request.environment_mode {
        EnvironmentMode::Clear => bounded_exec::EnvironmentMode::Clear,
        EnvironmentMode::Inherit => bounded_exec::EnvironmentMode::Inherit,
    };
    let environment: Vec<(OsString, OsString)> = request
        .environment
        .iter()
        .map(|(key, value)| (OsString::from(key), OsString::from(value)))
        .collect();
    let limits = ExecutionLimits {
        timeout_ms: request.timeout_ms,
        stdin_max_bytes: request.stdin_max_bytes,
        stdout_max_bytes: request.stdout_max_bytes,
        stderr_max_bytes: request.stderr_max_bytes,
        poll_interval_ms: request.poll_interval_ms,
        teardown_timeout_ms: request.teardown_timeout_ms,
    };
    let outcome_policy =
        OutcomePolicy::new(vec![ACCEPTED_EXIT_CODE], true, true).map_err(|_| ServiceUnavailable)?;
    Ok(RunRequest {
        command: CommandSpec {
            program,
            args,
            current_dir,
            environment_mode,
            environment,
            input: Input::Null,
        },
        limits,
        termination_scope: TerminationScope::ProcessGroup,
        outcome_policy,
    })
}

fn translate_output(output: &bounded_exec::ExecutionOutput) -> BoundedProcessOutcome {
    let stdout_bytes = output.stdout.observed_bytes;
    let stderr_bytes = output.stderr.observed_bytes;
    match output.completion {
        Completion::TimedOut => BoundedProcessOutcome::TimedOut,
        Completion::Cancelled => BoundedProcessOutcome::Cancelled,
        Completion::Exited => BoundedProcessOutcome::Completed {
            exit_code: output.exit_code.unwrap_or(UNKNOWN_EXIT_CODE),
            stdout_bytes,
            stderr_bytes,
            timed_out: false,
        },
    }
}
