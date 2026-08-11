use std::collections::BTreeSet;
use std::ffi::OsString;
use std::sync::atomic::{AtomicBool, Ordering};

use serde_json::Value;

use super::api::{ConnectedHost, McpClientHost};
use super::{
    CallOutput, CredentialService, HostError, HostErrorKind, LaunchAuthority, ResolvedCredentials,
    StartArtifacts,
};
use crate::config::{framed_hash, validate_hash};
use crate::process::{ProcessError, ProcessLaunch, StdioSession};
use crate::protocol::{
    INITIALIZE_RESPONSE_ID, LIST_TOOLS_RESPONSE_ID, ProtocolError, encode_call, encode_cancel,
    encode_initialize, encode_initialized, encode_list_tools, parse_call_response,
    parse_initialize_response,
};
use crate::{
    CallPlan, DiscoveryRequest, LifecycleBinding, LifecycleEvent, LifecyclePhase, McpHostConfig,
    TeardownObservation, TeardownState, admit_instructions, argument_identity, initial_lifecycle,
    reduce_lifecycle, validate_discovery,
};

const PROCESS_ATTEMPT_DOMAIN: &str = "animus.mcp-client-host.process-attempt.v1";

pub(crate) fn connect(
    config: McpHostConfig,
    generation: u64,
    launch: LaunchAuthority,
    credentials: Box<dyn CredentialService>,
) -> Result<ConnectedHost, HostError> {
    validate_launch(&config, generation, &launch)?;
    let resolved = credentials
        .resolve(
            config.server().credential_refs.as_slice(),
            config.server().environment_keys.as_slice(),
        )
        .map_err(|_| not_started_error(HostErrorKind::Credential))?;
    validate_credentials(&config, &resolved).map_err(|error| not_started_error(error.kind))?;
    let binding = lifecycle_binding(&config, generation, resolved.snapshot_identity.as_str());
    let mut lifecycle = initial_lifecycle(binding.clone())
        .map_err(|_| not_started_error(HostErrorKind::LaunchAuthority))?;
    lifecycle = reduce_lifecycle(
        &lifecycle,
        LifecycleEvent::BeginStart(binding.clone()),
        config.server().limits.max_calls_per_generation,
    )
    .map_err(|_| not_started_error(HostErrorKind::LaunchAuthority))?;
    let process_launch = ProcessLaunch {
        executable: launch.executable_path,
        arguments: config
            .server()
            .arguments
            .iter()
            .map(OsString::from)
            .collect(),
        environment: resolved.environment,
        current_directory: launch.working_directory,
        limits: config.server().limits,
    };
    let mut session = StdioSession::spawn(process_launch).map_err(process_start_error)?;
    let scope = session.scope();
    lifecycle = match reduce_lifecycle(
        &lifecycle,
        LifecycleEvent::ProcessStarted(binding.clone()),
        config.server().limits.max_calls_per_generation,
    ) {
        Ok(next) => next,
        Err(_) => return Err(stop_error(session, HostErrorKind::Process)),
    };
    let artifacts = match initialize(&mut session, &config, &binding) {
        Ok(artifacts) => artifacts,
        Err(kind) => return Err(stop_error(session, kind)),
    };
    lifecycle = match reduce_lifecycle(
        &lifecycle,
        LifecycleEvent::InitializationAccepted(binding.clone()),
        config.server().limits.max_calls_per_generation,
    ) {
        Ok(next) => next,
        Err(_) => return Err(stop_error(session, HostErrorKind::Protocol)),
    };
    let catalog = match discover(&mut session, &config, &binding) {
        Ok(catalog) => catalog,
        Err(kind) => return Err(stop_error(session, kind)),
    };
    lifecycle = match reduce_lifecycle(
        &lifecycle,
        LifecycleEvent::DiscoveryAccepted {
            binding,
            catalog_identity: String::from(catalog.identity()),
            tool_count: catalog.tools().len(),
        },
        config.server().limits.max_calls_per_generation,
    ) {
        Ok(next) => next,
        Err(_) => return Err(stop_error(session, HostErrorKind::Discovery)),
    };
    Ok(ConnectedHost {
        config,
        lifecycle,
        catalog,
        session,
        credentials,
        artifacts,
        scope,
    })
}

fn initialize(
    session: &mut StdioSession,
    config: &McpHostConfig,
    binding: &LifecycleBinding,
) -> Result<StartArtifacts, HostErrorKind> {
    let limits = config.server().limits;
    let request = encode_initialize(limits.max_frame_bytes).map_err(map_protocol_error)?;
    let cancellation = AtomicBool::new(false);
    let response = session
        .exchange(&request, limits.initialize_timeout_ms, &cancellation)
        .map_err(map_process_error)?;
    let initialized = parse_initialize_response(&response, INITIALIZE_RESPONSE_ID, limits)
        .map_err(map_protocol_error)?;
    let notification = encode_initialized(limits.max_frame_bytes).map_err(map_protocol_error)?;
    session.notify(&notification).map_err(map_process_error)?;
    let instructions = initialized
        .instructions
        .map(|body| {
            admit_instructions(
                config.server().id.as_str(),
                binding.generation,
                body,
                limits.max_instruction_bytes,
            )
            .map_err(|_| HostErrorKind::Context)
        })
        .transpose()?;
    Ok(StartArtifacts {
        instructions,
        server_info_identity: initialized.server_identity,
    })
}

fn discover(
    session: &mut StdioSession,
    config: &McpHostConfig,
    binding: &LifecycleBinding,
) -> Result<crate::DiscoveredCatalog, HostErrorKind> {
    let limits = config.server().limits;
    let request = encode_list_tools(limits.max_frame_bytes).map_err(map_protocol_error)?;
    let cancellation = AtomicBool::new(false);
    let response = session
        .exchange(&request, limits.discovery_timeout_ms, &cancellation)
        .map_err(map_process_error)?;
    validate_discovery(DiscoveryRequest {
        bytes: response.as_slice(),
        expected_response_id: LIST_TOOLS_RESPONSE_ID,
        config,
        binding,
        previous: None,
    })
    .map_err(|_| HostErrorKind::Discovery)
}

pub(crate) fn call(
    host: &mut McpClientHost,
    plan: &CallPlan,
    arguments: &Value,
    cancellation: &AtomicBool,
) -> Result<CallOutput, HostError> {
    validate_call_binding(host, plan, arguments)?;
    if cancellation.load(Ordering::Acquire) {
        return Err(retained_error(HostErrorKind::Cancelled));
    }
    let resolved = host
        .credentials
        .resolve(
            host.config.server().credential_refs.as_slice(),
            host.config.server().environment_keys.as_slice(),
        )
        .map_err(|_| retained_error(HostErrorKind::Credential))?;
    validate_credentials(&host.config, &resolved)?;
    if resolved.snapshot_identity != host.lifecycle.binding().credential_snapshot_id {
        let binding = host.lifecycle.binding().clone();
        host.lifecycle = reduce_lifecycle(
            &host.lifecycle,
            LifecycleEvent::CredentialDrifted {
                binding,
                observed_credential_identity: resolved.snapshot_identity,
            },
            host.config.server().limits.max_calls_per_generation,
        )
        .map_err(|_| retained_error(HostErrorKind::CredentialDrift))?;
        let teardown = stop(host);
        return Err(HostError {
            kind: HostErrorKind::CredentialDrift,
            teardown,
        });
    }
    let request_id = host.next_request_id;
    let next_request_id = request_id
        .checked_add(1)
        .ok_or_else(|| retained_error(HostErrorKind::Authority))?;
    let request = encode_call(
        request_id,
        plan.tool_name(),
        arguments,
        host.config.server().limits.max_frame_bytes,
    )
    .map_err(|_| retained_error(HostErrorKind::Protocol))?;
    let binding = host.lifecycle.binding().clone();
    host.lifecycle = reduce_lifecycle(
        &host.lifecycle,
        LifecycleEvent::CallStarted {
            binding: binding.clone(),
            call_identity: String::from(plan.identity()),
        },
        host.config.server().limits.max_calls_per_generation,
    )
    .map_err(|_| retained_error(HostErrorKind::Authority))?;
    host.next_request_id = next_request_id;
    let response = exchange_call(host, request_id, request.as_slice(), cancellation)?;
    let parsed = parse_call_response(response.as_slice(), request_id, host.config.server().limits)
        .map_err(|_| stop_host_error(host, HostErrorKind::Protocol))?;
    host.lifecycle = reduce_lifecycle(
        &host.lifecycle,
        LifecycleEvent::CallCompleted {
            binding,
            call_identity: String::from(plan.identity()),
            output_identity: parsed.identity.clone(),
        },
        host.config.server().limits.max_calls_per_generation,
    )
    .map_err(|_| stop_host_error(host, HostErrorKind::Protocol))?;
    Ok(CallOutput {
        value: parsed.value,
        identity: parsed.identity,
        is_error: parsed.is_error,
    })
}

fn exchange_call(
    host: &mut McpClientHost,
    request_id: u64,
    request: &[u8],
    cancellation: &AtomicBool,
) -> Result<Vec<u8>, HostError> {
    let Some(session) = host.session.as_mut() else {
        return Err(retained_error(HostErrorKind::Process));
    };
    match session.exchange(
        request,
        host.config.server().limits.call_timeout_ms,
        cancellation,
    ) {
        Ok(response) => Ok(response),
        Err(ProcessError::Cancelled | ProcessError::Timeout) => {
            let cancel = encode_cancel(request_id, host.config.server().limits.max_frame_bytes)
                .map_err(|_| retained_error(HostErrorKind::Protocol))?;
            let _notification = session.notify(cancel.as_slice());
            let binding = host.lifecycle.binding().clone();
            host.lifecycle = reduce_lifecycle(
                &host.lifecycle,
                LifecycleEvent::CancellationRequested {
                    binding,
                    call_identity: host
                        .lifecycle
                        .active_call_identity
                        .clone()
                        .ok_or_else(|| retained_error(HostErrorKind::Cancelled))?,
                },
                host.config.server().limits.max_calls_per_generation,
            )
            .map_err(|_| retained_error(HostErrorKind::Cancelled))?;
            let teardown = stop(host);
            Err(HostError {
                kind: HostErrorKind::Cancelled,
                teardown,
            })
        }
        Err(_) => Err(stop_host_error(host, HostErrorKind::Process)),
    }
}

pub(crate) fn stop(host: &mut McpClientHost) -> TeardownObservation {
    let Some(session) = host.session.take() else {
        return if host.lifecycle.phase() == LifecyclePhase::Stopped {
            TeardownObservation::Clean
        } else {
            TeardownObservation::NotStarted
        };
    };
    if !matches!(
        host.lifecycle.phase(),
        LifecyclePhase::Stopping | LifecyclePhase::Cancelling
    ) {
        let binding = host.lifecycle.binding().clone();
        if let Ok(next) = reduce_lifecycle(
            &host.lifecycle,
            LifecycleEvent::StopRequested(binding),
            host.config.server().limits.max_calls_per_generation,
        ) {
            host.lifecycle = next;
        }
    }
    let teardown = session.stop();
    assert_eq!(
        teardown.scope, host.process_scope,
        "process scope is stable"
    );
    let _stderr_metadata = teardown.stderr.as_ref().map(|stderr| {
        (
            stderr.identity.as_str(),
            stderr.byte_count,
            stderr.truncated,
        )
    });
    let state = if teardown.observation.terminal() {
        TeardownState::Terminal
    } else {
        TeardownState::Uncertain
    };
    let binding = host.lifecycle.binding().clone();
    if let Ok(next) = reduce_lifecycle(
        &host.lifecycle,
        LifecycleEvent::StopObserved {
            binding,
            teardown: state,
        },
        host.config.server().limits.max_calls_per_generation,
    ) {
        host.lifecycle = next;
    }
    teardown.observation
}

fn validate_launch(
    config: &McpHostConfig,
    generation: u64,
    launch: &LaunchAuthority,
) -> Result<(), HostError> {
    if generation == 0
        || !launch.executable_path.is_absolute()
        || !launch.working_directory.is_absolute()
        || launch.executable_identity != config.server().executable_identity
        || launch.working_root_identity != config.server().working_root_identity
    {
        return Err(not_started_error(HostErrorKind::LaunchAuthority));
    }
    Ok(())
}

fn validate_credentials(
    config: &McpHostConfig,
    resolved: &ResolvedCredentials,
) -> Result<(), HostError> {
    validate_hash(resolved.snapshot_identity.as_str())
        .map_err(|_| retained_error(HostErrorKind::Credential))?;
    if resolved.environment.len() != config.server().environment_keys.len() {
        return Err(retained_error(HostErrorKind::Credential));
    }
    let expected = config
        .server()
        .environment_keys
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut observed = BTreeSet::new();
    for (key, _value) in &resolved.environment {
        let key = key
            .to_str()
            .ok_or_else(|| retained_error(HostErrorKind::Credential))?;
        if !observed.insert(key) || !expected.contains(key) {
            return Err(retained_error(HostErrorKind::Credential));
        }
    }
    Ok(())
}

fn lifecycle_binding(
    config: &McpHostConfig,
    generation: u64,
    credential_snapshot_id: &str,
) -> LifecycleBinding {
    let mut attempt_bytes = Vec::new();
    attempt_bytes.extend_from_slice(config.identity().as_bytes());
    attempt_bytes.extend_from_slice(&generation.to_le_bytes());
    let process_attempt_id = framed_hash(PROCESS_ATTEMPT_DOMAIN, attempt_bytes.as_slice());
    LifecycleBinding {
        server_id: config.server().id.clone(),
        configuration_id: String::from(config.identity()),
        protocol_id: config.server().protocol_version.clone(),
        policy_id: config.server().policy_identity.clone(),
        credential_snapshot_id: String::from(credential_snapshot_id),
        generation,
        process_attempt_id,
    }
}

fn validate_call_binding(
    host: &McpClientHost,
    plan: &CallPlan,
    arguments: &Value,
) -> Result<(), HostError> {
    let observed_argument_identity =
        argument_identity(arguments).map_err(|_| retained_error(HostErrorKind::Authority))?;
    if host.lifecycle.phase() != LifecyclePhase::Ready
        || plan.server_generation() != host.lifecycle.binding().generation
        || host.catalog.tool(plan.tool_name()).is_none()
        || plan.argument_identity() != observed_argument_identity
    {
        return Err(retained_error(HostErrorKind::Authority));
    }
    Ok(())
}

fn stop_error(session: StdioSession, kind: HostErrorKind) -> HostError {
    let teardown = session.stop();
    HostError {
        kind: if teardown.observation == TeardownObservation::Uncertain {
            HostErrorKind::TeardownUncertain
        } else {
            kind
        },
        teardown: teardown.observation,
    }
}

fn stop_host_error(host: &mut McpClientHost, kind: HostErrorKind) -> HostError {
    let teardown = stop(host);
    HostError {
        kind: if teardown == TeardownObservation::Uncertain {
            HostErrorKind::TeardownUncertain
        } else {
            kind
        },
        teardown,
    }
}

const fn retained_error(kind: HostErrorKind) -> HostError {
    HostError {
        kind,
        teardown: TeardownObservation::Retained,
    }
}

const fn not_started_error(kind: HostErrorKind) -> HostError {
    HostError {
        kind,
        teardown: TeardownObservation::NotStarted,
    }
}

const fn process_start_error(error: ProcessError) -> HostError {
    match error {
        ProcessError::Launch => HostError {
            kind: HostErrorKind::Process,
            teardown: TeardownObservation::NotStarted,
        },
        ProcessError::StartCleanup => HostError {
            kind: HostErrorKind::Process,
            teardown: TeardownObservation::Clean,
        },
        _ => HostError {
            kind: HostErrorKind::TeardownUncertain,
            teardown: TeardownObservation::Uncertain,
        },
    }
}

const fn map_process_error(error: ProcessError) -> HostErrorKind {
    match error {
        ProcessError::Timeout | ProcessError::Cancelled => HostErrorKind::Cancelled,
        ProcessError::MalformedFrame | ProcessError::OversizedFrame => HostErrorKind::Protocol,
        _ => HostErrorKind::Process,
    }
}

const fn map_protocol_error(_error: ProtocolError) -> HostErrorKind {
    HostErrorKind::Protocol
}
