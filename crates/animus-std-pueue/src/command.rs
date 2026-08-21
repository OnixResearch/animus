use animus_adapters::{QueueControlRequest, QueueEnqueueRequest};
use animus_runtime::{ProcessJobBackend, ResolvedProcessJobProfile};

use crate::{PueueHostConfig, PueueProcessRequest};

mod control;

const COLOR_OPTION: &str = "--color";
const COLOR_NEVER: &str = "never";
const STATUS_COMMAND: &str = "status";
const JSON_OPTION: &str = "--json";
const DEFAULT_GROUP: &str = "default";
const PUEUE_CONFIG_PATH_KEY: &str = "PUEUE_CONFIG_PATH";
const HOME_KEY: &str = "HOME";
const TIMEOUT_SIGNAL_ARGUMENT: &str = "--signal=TERM";
const TIMEOUT_KILL_AFTER_PREFIX: &str = "--kill-after=";
const SECONDS_SUFFIX: &str = "s";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PueueCommandError {
    Denied,
    MissingProfileField,
    BoundConversion,
    NonUtf8Path,
}

struct CommandDraft {
    program: std::path::PathBuf,
    args: Vec<String>,
    environment: Vec<(String, String)>,
    timeout_ms: u64,
}

pub(crate) fn health(config: &PueueHostConfig) -> Result<PueueProcessRequest, PueueCommandError> {
    build_request(
        config,
        CommandDraft {
            program: config.pueue_program.clone(),
            args: vec![
                String::from(COLOR_OPTION),
                String::from(COLOR_NEVER),
                String::from(STATUS_COMMAND),
                String::from(JSON_OPTION),
            ],
            environment: Vec::new(),
            timeout_ms: config.limits.daemon_timeout_ms,
        },
    )
}

pub(crate) fn enqueue(
    config: &PueueHostConfig,
    request: &QueueEnqueueRequest,
    resolved: &ResolvedProcessJobProfile,
) -> Result<PueueProcessRequest, PueueCommandError> {
    validate_enqueue(config, request, resolved)?;
    let program = resolved
        .request
        .program
        .as_deref()
        .ok_or(PueueCommandError::MissingProfileField)?;
    let cwd = resolved
        .request
        .cwd
        .as_deref()
        .ok_or(PueueCommandError::MissingProfileField)?;
    let deadline_executable = config
        .timeout_program
        .to_str()
        .ok_or(PueueCommandError::NonUtf8Path)?;
    let deadline_secs = resolved
        .request
        .resources
        .timeout_seconds
        .ok_or(PueueCommandError::MissingProfileField)?;
    let kill_after_seconds = config.limits.task_kill_after_seconds;
    let mut args = base_args("add");
    args.extend([
        String::from("--print-task-id"),
        String::from("--escape"),
        String::from("--working-directory"),
        String::from(cwd),
        String::from("--label"),
        String::from(request.command_label()),
    ]);
    if let Some(group) = request.group() {
        args.extend([String::from("--group"), String::from(group)]);
    }
    args.push(String::from("--"));
    args.push(String::from(deadline_executable));
    args.push(String::from(TIMEOUT_SIGNAL_ARGUMENT));
    args.push(format!(
        "{TIMEOUT_KILL_AFTER_PREFIX}{kill_after_seconds}{SECONDS_SUFFIX}"
    ));
    args.push(format!("{deadline_secs}{SECONDS_SUFFIX}"));
    args.push(String::from(program));
    args.extend(resolved.request.args.iter().cloned());
    let environment = resolved
        .request
        .env
        .iter()
        .map(|entry| (entry.key.clone(), entry.value.clone()))
        .collect();
    build_request(
        config,
        CommandDraft {
            program: config.pueue_program.clone(),
            args,
            environment,
            timeout_ms: config.limits.command_timeout_ms,
        },
    )
}

pub(crate) fn status(config: &PueueHostConfig) -> Result<PueueProcessRequest, PueueCommandError> {
    build_request(
        config,
        CommandDraft {
            program: config.pueue_program.clone(),
            args: base_args_with_json(STATUS_COMMAND),
            environment: Vec::new(),
            timeout_ms: config.limits.command_timeout_ms,
        },
    )
}

pub(crate) fn logs(
    config: &PueueHostConfig,
    task_id: &str,
) -> Result<PueueProcessRequest, PueueCommandError> {
    let task_id = validated_task_id(task_id)?;
    build_request(
        config,
        CommandDraft {
            program: config.pueue_program.clone(),
            args: [base_args_with_json("log"), vec![task_id]].concat(),
            environment: Vec::new(),
            timeout_ms: config.limits.command_timeout_ms,
        },
    )
}

pub(crate) fn control(
    config: &PueueHostConfig,
    request: &QueueControlRequest,
) -> Result<PueueProcessRequest, PueueCommandError> {
    let args = control::control_args(request)?;
    build_request(
        config,
        CommandDraft {
            program: config.pueue_program.clone(),
            args,
            environment: Vec::new(),
            timeout_ms: config.limits.command_timeout_ms,
        },
    )
}

fn validate_enqueue(
    config: &PueueHostConfig,
    request: &QueueEnqueueRequest,
    resolved: &ResolvedProcessJobProfile,
) -> Result<(), PueueCommandError> {
    let bounds = request.output_bounds();
    let host_bounds = config.limits;
    let is_label_set_matching = request.environment_policy() == config.environment_policy
        && request.timeout_policy() == config.timeout_policy;
    let is_output_bounded = bounds.max_visible_lines() <= host_bounds.max_log_lines
        && bounds.max_visible_bytes() <= host_bounds.max_log_bytes;
    let is_admitted_profile = resolved.request.backend == ProcessJobBackend::Pueue
        && resolved.request.command.is_none()
        && resolved.request.program.is_some()
        && resolved.request.cwd.is_some()
        && resolved.request.writable_paths.is_empty()
        && resolved.request.resources.timeout_seconds.is_some()
        && resolved.request.resources.log_bytes.is_some()
        && resolved.request.resources.memory_bytes.is_none()
        && resolved.request.resources.cpu_millis.is_none();
    if is_label_set_matching && is_output_bounded && is_admitted_profile {
        Ok(())
    } else {
        Err(PueueCommandError::Denied)
    }
}

fn build_request(
    config: &PueueHostConfig,
    draft: CommandDraft,
) -> Result<PueueProcessRequest, PueueCommandError> {
    let config_path = config
        .config_path
        .to_str()
        .ok_or(PueueCommandError::NonUtf8Path)?;
    let namespace_root = config
        .namespace_root
        .to_str()
        .ok_or(PueueCommandError::NonUtf8Path)?;
    let mut environment = vec![
        (
            String::from(PUEUE_CONFIG_PATH_KEY),
            String::from(config_path),
        ),
        (String::from(HOME_KEY), String::from(namespace_root)),
    ];
    environment.extend(draft.environment);
    Ok(PueueProcessRequest {
        program: draft.program,
        args: draft.args,
        current_dir: config.namespace_root.clone(),
        environment,
        timeout_ms: draft.timeout_ms,
        stdout_max_bytes: u64::try_from(config.limits.max_backend_stdout_bytes)
            .map_err(|_error| PueueCommandError::BoundConversion)?,
        stderr_max_bytes: u64::try_from(config.limits.max_backend_stderr_bytes)
            .map_err(|_error| PueueCommandError::BoundConversion)?,
        poll_interval_ms: config.limits.poll_interval_ms,
        teardown_timeout_ms: config.limits.teardown_timeout_ms,
    })
}

fn base_args(command: &str) -> Vec<String> {
    vec![
        String::from(COLOR_OPTION),
        String::from(COLOR_NEVER),
        String::from(command),
    ]
}

fn base_args_with_json(command: &str) -> Vec<String> {
    [base_args(command), vec![String::from(JSON_OPTION)]].concat()
}

fn validated_task_id(value: &str) -> Result<String, PueueCommandError> {
    let is_task_id_valid = !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit());
    if is_task_id_valid {
        Ok(String::from(value))
    } else {
        Err(PueueCommandError::Denied)
    }
}
