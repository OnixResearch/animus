#![allow(
    dead_code,
    reason = "shared integration fixtures are selected by separate test targets"
)]

use std::collections::VecDeque;

use animus_std_pueue::{
    PueueProcessError, PueueProcessOutput, PueueProcessRequest, PueueProcessService,
};

pub const FAKE_PUEUE: &str = "/nix/store/fixture/bin/pueue";
pub const FAKE_PUEUED: &str = "/nix/store/fixture/bin/pueued";
pub const FAKE_TIMEOUT: &str = "/nix/store/fixture/bin/timeout";
pub const FAKE_JOB: &str = "/nix/store/fixture/bin/true";
pub const NAMESPACE_ID: &str = "animus-pueue-fixture";

const COMMAND_TIMEOUT_MS: u64 = 30_000;
const DAEMON_TIMEOUT_MS: u64 = 5_000;
const POLL_INTERVAL_MS: u64 = 50;
const WAIT_POLL_ATTEMPTS: u32 = 20;
const TASK_KILL_AFTER_SECONDS: u32 = 5;
const TEARDOWN_TIMEOUT_MS: u64 = 2_000;
const BACKEND_OUTPUT_BYTES: usize = 1_048_576;
const LOG_LINES: u32 = 1_000;
const LOG_BYTES: u32 = 65_536;
const PROFILE_TIMEOUT_SECONDS: u32 = 3_600;
const PROFILE_LOG_BYTES: u64 = 1_048_576;

#[derive(Clone, Copy)]
pub struct ConfigPaths<'a> {
    pub root: &'a str,
    pub pueue: &'a str,
    pub pueued: &'a str,
    pub timeout: &'a str,
    pub job: &'a str,
}

pub fn fixture_config_json(root: &str) -> String {
    config_json_from_paths(ConfigPaths {
        root,
        pueue: FAKE_PUEUE,
        pueued: FAKE_PUEUED,
        timeout: FAKE_TIMEOUT,
        job: FAKE_JOB,
    })
}

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::ambiguous_params,
        reason = "the shared fixture wrapper preserves the established four-path integration test call shape"
    )
)]
pub fn config_json(root: &str, pueue: &str, pueued: &str, job: &str) -> String {
    config_json_from_paths(ConfigPaths {
        root,
        pueue,
        pueued,
        timeout: FAKE_TIMEOUT,
        job,
    })
}

pub fn config_json_from_paths(paths: ConfigPaths<'_>) -> String {
    serde_json::json!({
        "schema": "animus.pueue-host.v1",
        "namespace": {
            "id": NAMESPACE_ID,
            "root": paths.root,
            "config_leaf": "pueue.yml",
        },
        "executables": {
            "pueue": paths.pueue,
            "pueued": paths.pueued,
            "timeout": paths.timeout,
        },
        "authority": {
            "owner_label": "animus-pueue-host",
            "profile_source_label": "animus-pueue-profiles",
            "environment_policy": "animus-pueue-env",
            "timeout_policy": "animus-pueue-timeout",
        },
        "capabilities": {
            "interactions": ["wait", "poll"],
        },
        "bounds": {
            "command_timeout_ms": COMMAND_TIMEOUT_MS,
            "daemon_timeout_ms": DAEMON_TIMEOUT_MS,
            "poll_interval_ms": POLL_INTERVAL_MS,
            "wait_poll_attempts": WAIT_POLL_ATTEMPTS,
            "task_kill_after_seconds": TASK_KILL_AFTER_SECONDS,
            "teardown_timeout_ms": TEARDOWN_TIMEOUT_MS,
            "max_backend_stdout_bytes": BACKEND_OUTPUT_BYTES,
            "max_backend_stderr_bytes": BACKEND_OUTPUT_BYTES,
            "max_log_lines": LOG_LINES,
            "max_log_bytes": LOG_BYTES,
        },
        "process_policy": {
            "allowed_env_prefixes": ["ANIMUS_"],
            "allowed_cwd_prefixes": [paths.root],
            "max_timeout_seconds": PROFILE_TIMEOUT_SECONDS,
            "max_log_bytes": PROFILE_LOG_BYTES,
        },
        "profiles": [{
            "name": "fixture-check",
            "backend": "pueue",
            "program": paths.job,
            "args": [],
            "cwd": paths.root,
            "env": [{"key": "ANIMUS_TASK", "value": "fixture"}],
            "timeout_seconds": PROFILE_TIMEOUT_SECONDS,
            "log_bytes": PROFILE_LOG_BYTES,
            "notify_on_complete": false,
            "metadata": [{"key": "identity.class", "value": "fixture"}],
        }],
        "non_claims": [
            "command_correctness",
            "scheduler_reliability",
            "sandboxing",
            "production_readiness",
            "release_eligibility",
        ],
    })
    .to_string()
}

#[derive(Default)]
pub struct FakeProcess {
    pub calls: Vec<PueueProcessRequest>,
    outputs: VecDeque<Result<PueueProcessOutput, PueueProcessError>>,
}

impl FakeProcess {
    pub fn with_outputs(outputs: Vec<PueueProcessOutput>) -> Self {
        Self {
            calls: Vec::new(),
            outputs: outputs.into_iter().map(Ok).collect(),
        }
    }
}

impl PueueProcessService for FakeProcess {
    fn run(
        &mut self,
        request: &PueueProcessRequest,
    ) -> Result<PueueProcessOutput, PueueProcessError> {
        self.calls.push(request.clone());
        match self.outputs.pop_front() {
            Some(output) => output,
            None => Err(PueueProcessError::RunnerFailed),
        }
    }
}

pub fn success(text: &str) -> PueueProcessOutput {
    PueueProcessOutput {
        disposition: animus_std_pueue::PueueProcessDisposition::Succeeded,
        stdout: text.as_bytes().to_vec(),
        stderr_observed_bytes: 0,
        exit_code: Some(0),
    }
}

pub fn exit_failed() -> PueueProcessOutput {
    PueueProcessOutput {
        disposition: animus_std_pueue::PueueProcessDisposition::ExitFailed,
        stdout: Vec::new(),
        stderr_observed_bytes: 1,
        exit_code: Some(1),
    }
}

pub fn cancelled() -> PueueProcessOutput {
    PueueProcessOutput {
        disposition: animus_std_pueue::PueueProcessDisposition::Cancelled,
        stdout: Vec::new(),
        stderr_observed_bytes: 0,
        exit_code: None,
    }
}

pub fn timed_out() -> PueueProcessOutput {
    PueueProcessOutput {
        disposition: animus_std_pueue::PueueProcessDisposition::TimedOut,
        stdout: Vec::new(),
        stderr_observed_bytes: 0,
        exit_code: None,
    }
}

pub fn output_limited() -> PueueProcessOutput {
    PueueProcessOutput {
        disposition: animus_std_pueue::PueueProcessDisposition::OutputLimitExceeded,
        stdout: Vec::new(),
        stderr_observed_bytes: 0,
        exit_code: None,
    }
}

pub fn status_fixture() -> String {
    serde_json::json!({
        "groups": {
            "default": {"status": "Running", "parallel_tasks": 1},
            "build": {"status": "Paused", "parallel_tasks": 0},
        },
        "tasks": {
            "42": {
                "id": 42,
                "command": "secret command",
                "path": "/home/private/work",
                "envs": {"API_KEY": "secret"},
                "group": "build",
                "status": {"Running": {}},
            }
        }
    })
    .to_string()
}
