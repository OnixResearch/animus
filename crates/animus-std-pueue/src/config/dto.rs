use animus_runtime::{
    NamedProcessJobProfile, ProcessJobBackend, ProcessJobEnvironmentVariable,
    ProcessJobMetadataField, ProcessJobNotificationPolicy, ProcessJobProfile,
    ProcessJobProfileManifest, ProcessJobProfilePolicy, ProcessJobProfileSource,
    ProcessJobProfileSourcePrecedence, ProcessJobResourceLimits,
};
use serde::Deserialize;

use crate::config::{
    PUEUE_HOST_CONFIG_SCHEMA, PueueHostConfig, PueueHostConfigError, PueueHostLimits,
    validate_absolute_path, validate_config_leaf, validate_label, validate_non_claims,
};

mod error;

use error::{invalid_profile, invalid_value, map_json_error};

const RESERVED_ENVIRONMENT_KEYS: &[&str] = &["HOME", "PUEUE_CONFIG_PATH"];
const WAIT_INTERACTION: &str = "wait";
const POLL_INTERACTION: &str = "poll";
const CURRENT_INTERACTION_COUNT: usize = 2;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigJson {
    schema: String,
    namespace: NamespaceJson,
    executables: ExecutablesJson,
    authority: AuthorityJson,
    capabilities: CapabilitiesJson,
    bounds: BoundsJson,
    process_policy: ProcessPolicyJson,
    profiles: Vec<ProfileJson>,
    non_claims: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NamespaceJson {
    id: String,
    root: String,
    config_leaf: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExecutablesJson {
    pueue: String,
    pueued: String,
    timeout: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityJson {
    owner_label: String,
    profile_source_label: String,
    environment_policy: String,
    timeout_policy: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilitiesJson {
    interactions: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BoundsJson {
    command_timeout_ms: u64,
    daemon_timeout_ms: u64,
    poll_interval_ms: u64,
    wait_poll_attempts: u32,
    task_kill_after_seconds: u32,
    teardown_timeout_ms: u64,
    max_backend_stdout_bytes: usize,
    max_backend_stderr_bytes: usize,
    max_log_lines: u32,
    max_log_bytes: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProcessPolicyJson {
    allowed_env_prefixes: Vec<String>,
    allowed_cwd_prefixes: Vec<String>,
    max_timeout_seconds: u32,
    max_log_bytes: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileJson {
    name: String,
    backend: String,
    program: String,
    args: Vec<String>,
    cwd: String,
    env: Vec<EnvironmentJson>,
    timeout_seconds: u32,
    log_bytes: u64,
    notify_on_complete: bool,
    metadata: Vec<MetadataJson>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentJson {
    key: String,
    value: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MetadataJson {
    key: String,
    value: String,
}

pub(crate) fn decode_config(text: &str) -> Result<PueueHostConfig, PueueHostConfigError> {
    let decoded: ConfigJson = serde_json::from_str(text).map_err(|error| map_json_error(&error))?;
    validate_top_level(&decoded)?;
    let namespace_root = validate_absolute_path(decoded.namespace.root.as_str())?;
    let config_leaf = validate_config_leaf(decoded.namespace.config_leaf.as_str())?;
    let client_program = validate_absolute_path(decoded.executables.pueue.as_str())?;
    let daemon_program = validate_absolute_path(decoded.executables.pueued.as_str())?;
    let deadline_executable = validate_absolute_path(decoded.executables.timeout.as_str())?;
    let is_executable_set_distinct = client_program != daemon_program
        && client_program != deadline_executable
        && daemon_program != deadline_executable;
    if !is_executable_set_distinct {
        return Err(invalid_value());
    }
    let profile_policy = process_policy(&decoded)?;
    let profile_sources = profile_sources(&decoded, &profile_policy)?;
    Ok(PueueHostConfig {
        namespace_id: validate_label(decoded.namespace.id.as_str())?,
        config_path: namespace_root.join(config_leaf),
        namespace_root,
        pueue_program: client_program,
        pueued_program: daemon_program,
        timeout_program: deadline_executable,
        owner_label: validate_label(decoded.authority.owner_label.as_str())?,
        environment_policy: validate_label(decoded.authority.environment_policy.as_str())?,
        timeout_policy: validate_label(decoded.authority.timeout_policy.as_str())?,
        limits: limits(&decoded.bounds)?,
        wait_enabled: true,
        poll_enabled: true,
        profile_sources,
        profile_policy,
    })
}

fn validate_top_level(decoded: &ConfigJson) -> Result<(), PueueHostConfigError> {
    let interactions = decoded.capabilities.interactions.as_slice();
    let is_interaction_set_current = interactions.len() == CURRENT_INTERACTION_COUNT
        && interactions.iter().any(|value| value == WAIT_INTERACTION)
        && interactions.iter().any(|value| value == POLL_INTERACTION);
    if decoded.schema != PUEUE_HOST_CONFIG_SCHEMA
        || decoded.profiles.is_empty()
        || !is_interaction_set_current
    {
        return Err(invalid_value());
    }
    validate_label(decoded.authority.profile_source_label.as_str())?;
    validate_non_claims(decoded.non_claims.as_slice())
}

fn limits(bounds: &BoundsJson) -> Result<PueueHostLimits, PueueHostConfigError> {
    let has_zero = bounds.command_timeout_ms == 0
        || bounds.daemon_timeout_ms == 0
        || bounds.poll_interval_ms == 0
        || bounds.wait_poll_attempts == 0
        || bounds.task_kill_after_seconds == 0
        || bounds.teardown_timeout_ms == 0
        || bounds.max_backend_stdout_bytes == 0
        || bounds.max_backend_stderr_bytes == 0
        || bounds.max_log_lines == 0
        || bounds.max_log_bytes == 0;
    if has_zero {
        return Err(invalid_value());
    }
    Ok(PueueHostLimits {
        command_timeout_ms: bounds.command_timeout_ms,
        daemon_timeout_ms: bounds.daemon_timeout_ms,
        poll_interval_ms: bounds.poll_interval_ms,
        wait_poll_attempts: bounds.wait_poll_attempts,
        task_kill_after_seconds: bounds.task_kill_after_seconds,
        teardown_timeout_ms: bounds.teardown_timeout_ms,
        max_backend_stdout_bytes: bounds.max_backend_stdout_bytes,
        max_backend_stderr_bytes: bounds.max_backend_stderr_bytes,
        max_log_lines: bounds.max_log_lines,
        max_log_bytes: bounds.max_log_bytes,
    })
}

fn process_policy(decoded: &ConfigJson) -> Result<ProcessJobProfilePolicy, PueueHostConfigError> {
    if decoded.process_policy.max_timeout_seconds == 0 || decoded.process_policy.max_log_bytes == 0
    {
        return Err(invalid_value());
    }
    let policy = ProcessJobProfilePolicy {
        default_backend: ProcessJobBackend::Pueue,
        allowed_backends: vec![ProcessJobBackend::Pueue],
        allowed_env_prefixes: decoded.process_policy.allowed_env_prefixes.clone(),
        allowed_cwd_prefixes: decoded.process_policy.allowed_cwd_prefixes.clone(),
        allowed_writable_path_prefixes: Vec::new(),
        max_timeout_seconds: Some(decoded.process_policy.max_timeout_seconds),
        max_memory_bytes: None,
        max_cpu_millis: None,
        max_log_bytes: Some(decoded.process_policy.max_log_bytes),
        policy_source: decoded.authority.profile_source_label.clone(),
        policy_hash: String::new(),
    };
    Ok(policy.with_computed_hash())
}

fn profile_sources(
    decoded: &ConfigJson,
    policy: &ProcessJobProfilePolicy,
) -> Result<Vec<ProcessJobProfileSource>, PueueHostConfigError> {
    let profiles = decoded
        .profiles
        .iter()
        .map(profile)
        .collect::<Result<Vec<_>, _>>()?;
    let manifest = ProcessJobProfileManifest::current(profiles);
    let source = ProcessJobProfileSource::new(
        ProcessJobProfileSourcePrecedence::Explicit,
        decoded.authority.profile_source_label.clone(),
        manifest,
    )
    .map_err(|_error| invalid_profile())?;
    for current in &decoded.profiles {
        animus_runtime::resolve_process_job_profile(
            animus_runtime::ProcessJobProfileResolveRequest {
                sources: core::slice::from_ref(&source),
                profile_name: current.name.as_str(),
                owner_label: decoded.authority.owner_label.as_str(),
                policy,
            },
        )
        .map_err(|_error| invalid_profile())?;
    }
    Ok(vec![source])
}

fn profile(decoded: &ProfileJson) -> Result<NamedProcessJobProfile, PueueHostConfigError> {
    if decoded.backend != ProcessJobBackend::Pueue.as_str()
        || decoded.timeout_seconds == 0
        || decoded.log_bytes == 0
    {
        return Err(invalid_profile());
    }
    validate_absolute_path(decoded.program.as_str())?;
    validate_absolute_path(decoded.cwd.as_str())?;
    let env = decoded
        .env
        .iter()
        .map(environment)
        .collect::<Result<Vec<_>, _>>()?;
    let metadata = decoded
        .metadata
        .iter()
        .map(|field| ProcessJobMetadataField {
            key: field.key.clone(),
            value: field.value.clone(),
        })
        .collect();
    Ok(NamedProcessJobProfile {
        name: validate_label(decoded.name.as_str())?,
        profile: ProcessJobProfile {
            backend: Some(ProcessJobBackend::Pueue),
            command: None,
            program: Some(decoded.program.clone()),
            args: decoded.args.clone(),
            cwd: Some(decoded.cwd.clone()),
            writable_paths: Vec::new(),
            env,
            resources: ProcessJobResourceLimits {
                timeout_seconds: Some(decoded.timeout_seconds),
                memory_bytes: None,
                cpu_millis: None,
                log_bytes: Some(decoded.log_bytes),
            },
            notification: ProcessJobNotificationPolicy {
                notify_on_complete: decoded.notify_on_complete,
            },
            metadata,
        },
    })
}

fn environment(
    decoded: &EnvironmentJson,
) -> Result<ProcessJobEnvironmentVariable, PueueHostConfigError> {
    if RESERVED_ENVIRONMENT_KEYS.contains(&decoded.key.as_str()) {
        return Err(invalid_profile());
    }
    Ok(ProcessJobEnvironmentVariable {
        key: decoded.key.clone(),
        value: decoded.value.clone(),
    })
}
