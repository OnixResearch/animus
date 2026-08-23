//! Focused positive and negative delegation tests for the bounded host.
//!
//! These tests execute real POSIX subprocesses with explicit bounds. They do
//! not prove sandboxing, executable authorization, or release eligibility;
//! they prove the host delegates bounded requests and classifies observed
//! completions, deadlines, cancellations, and startup failures.

#![cfg(unix)]

use animus_adapter_zeron_harness::{
    BoundedAgentProcessService, BoundedProcessOutcome, BoundedProcessRequest, EnvironmentMode,
};
use animus_std_zeron_harness::BoundedExecAgentProcessService;

const SH: &str = "/bin/sh";
const PROGRAM_BOUND: usize = 512;
const STDIN_BOUND: usize = 128;
const OUTPUT_BOUND: usize = 4_096;
const OUTPUT_FLOOD_CAP: usize = 256;
const TIMEOUT_MS: u64 = 5_000;
const SHORT_TIMEOUT_MS: u64 = 50;
const TEARDOWN_MS: u64 = 500;
const POLL_MS: u64 = 10;

fn request(
    program: &str,
    args: &[&str],
    timeout_ms: u64,
    stdout_bound: usize,
) -> BoundedProcessRequest {
    BoundedProcessRequest::new(
        program,
        args,
        None,
        EnvironmentMode::Clear,
        &[],
        STDIN_BOUND,
        stdout_bound,
        OUTPUT_BOUND,
        timeout_ms,
        TEARDOWN_MS,
        POLL_MS,
    )
    .expect("explicit bounds validate")
}

#[test]
fn bounded_program_succeeds_with_observed_counts() {
    let service = BoundedExecAgentProcessService;
    let outcome = service
        .run(&request(
            SH,
            &["-c", "printf hello"],
            TIMEOUT_MS,
            OUTPUT_BOUND,
        ))
        .expect("process starts");
    match outcome {
        BoundedProcessOutcome::Completed {
            exit_code,
            stdout_bytes,
            stderr_bytes,
            timed_out,
        } => {
            assert_eq!(exit_code, 0);
            assert_eq!(stdout_bytes, 5);
            assert_eq!(stderr_bytes, 0);
            assert!(!timed_out);
        }
        other => panic!("expected completed observation, got {other:?}"),
    }
}

#[test]
fn missing_executable_is_service_unavailable() {
    let service = BoundedExecAgentProcessService;
    let result = service.run(&request(
        "/nonexistent/definitely-not-a-real-program",
        &[],
        TIMEOUT_MS,
        OUTPUT_BOUND,
    ));
    assert_eq!(
        result,
        Err(animus_adapter_zeron_harness::ServiceUnavailable)
    );
}

#[test]
fn deadline_produces_timed_out_class() {
    let service = BoundedExecAgentProcessService;
    // A builtin busy loop avoids PATH and external-command dependencies under
    // a cleared environment, so the deadline is the only stop condition.
    let outcome = service
        .run(&request(
            SH,
            &["-c", "while :; do :; done"],
            SHORT_TIMEOUT_MS,
            OUTPUT_BOUND,
        ))
        .expect("process starts");
    assert_eq!(outcome, BoundedProcessOutcome::TimedOut);
}

#[test]
fn output_flood_respects_retention_bound() {
    let service = BoundedExecAgentProcessService;
    let outcome = service
        .run(&request(
            SH,
            &["-c", "yes | head -c 100000"],
            TIMEOUT_MS,
            OUTPUT_FLOOD_CAP,
        ))
        .expect("process starts");
    match outcome {
        BoundedProcessOutcome::Completed { stdout_bytes, .. } => {
            assert!(stdout_bytes <= OUTPUT_FLOOD_CAP);
        }
        other => panic!("expected completed observation, got {other:?}"),
    }
}

#[test]
fn clear_environment_still_starts() {
    let service = BoundedExecAgentProcessService;
    let outcome = service
        .run(&request(SH, &["-c", ":"], TIMEOUT_MS, OUTPUT_BOUND))
        .expect("process starts");
    assert!(matches!(
        outcome,
        BoundedProcessOutcome::Completed { exit_code: 0, .. }
    ));
}

#[test]
fn program_byte_bound_is_enforced_at_admission() {
    let long_program = "x".repeat(PROGRAM_BOUND + 1);
    let result = BoundedProcessRequest::new(
        &long_program,
        &[],
        None,
        EnvironmentMode::Clear,
        &[],
        STDIN_BOUND,
        OUTPUT_BOUND,
        OUTPUT_BOUND,
        TIMEOUT_MS,
        TEARDOWN_MS,
        POLL_MS,
    );
    assert_eq!(
        result,
        Err(animus_adapter_zeron_harness::Error::ValueTooLong)
    );
}
