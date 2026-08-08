//! Thin host shell over explicit evaluation and auxiliary service ports.

use animus_adapter_eval::{
    AgentExecutionPort, AgentExecutionRequest, ArtifactCollectPort, ArtifactCollectRequest,
    ArtifactRestorePort, ArtifactRestoreRequest, EvaluationBinding, ModelCallCapturePort,
    ModelCallCaptureRequest, ObservationDisposition, ResultAppendPort, ResultAppendRequest,
    ResultReopenPort, ResultReopenRequest, ResultReplacePort, ResultReplaceRequest, ServiceKind,
    ServiceObservation, ServiceRequest, TaskSourcePort, TaskSourceRequest, ValidatedObservation,
    VerifierExecutionPort, VerifierExecutionRequest, build_service_request, reduce_observation,
    validate_service_observation,
};
use animus_eval_core::{EvaluationSeatRole, hash_identity, validate_identity};

use crate::{
    CleanupObligation, CleanupObservation, HostError, HostPreflight, HostReceiptClaim,
    HostReceiptDraft, HostReceiptLeakFlags, HostReceiptStatus, HostRunRequest, HostRunResult,
    HostStatus, preflight_host, project_host_receipt,
};

/// Observation from one explicit host-only tool service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolObservation {
    /// Exact tool observation identity.
    pub identity: String,
    /// Whether the bounded selected tool step completed.
    pub complete: bool,
}

/// Explicit cancellation observation service.
pub trait CancellationPort {
    /// Return whether the exact cancellation scope is current.
    fn cancellation_current(&mut self, cancellation_identity: &str) -> bool;
}

/// Explicit deadline observation service.
pub trait DeadlinePort {
    /// Return whether the exact deadline is current.
    fn deadline_expired(&mut self, deadline_identity: &str) -> bool;
}

/// Explicit tool service used by the fake-backed public host slice.
pub trait ToolPort {
    /// Execute or observe one bounded selected tool step.
    fn execute_tool(&mut self, tool_identity: &str, run_identity: &str) -> ToolObservation;
}

/// Explicit cleanup service.
pub trait CleanupPort {
    /// Attempt all selected cleanup obligations.
    fn cleanup(&mut self, obligation: &CleanupObligation<'_>) -> CleanupObservation;
}

/// Complete explicit service set required by the public host entrypoint.
pub trait EvaluationHostServices:
    TaskSourcePort
    + AgentExecutionPort
    + VerifierExecutionPort
    + ArtifactCollectPort
    + ArtifactRestorePort
    + ModelCallCapturePort
    + ResultAppendPort
    + ResultReplacePort
    + ResultReopenPort
    + CancellationPort
    + DeadlinePort
    + ToolPort
    + CleanupPort
{
}

impl<T> EvaluationHostServices for T where
    T: TaskSourcePort
        + AgentExecutionPort
        + VerifierExecutionPort
        + ArtifactCollectPort
        + ArtifactRestorePort
        + ModelCallCapturePort
        + ResultAppendPort
        + ResultReplacePort
        + ResultReopenPort
        + CancellationPort
        + DeadlinePort
        + ToolPort
        + CleanupPort
{
}

/// Execute one admitted native plan through explicit services only.
///
/// # Errors
/// Returns typed preflight, cancellation, deadline, binding, service, gateway,
/// storage, artifact, sealed, or receipt failures without ambient fallback.
pub fn execute_evaluation_host<S: EvaluationHostServices>(
    request: &HostRunRequest<'_>,
    services: &mut S,
) -> Result<HostRunResult, HostError> {
    let preflight = preflight_host(request)?;
    check_control(request, services)?;
    let mut permits = PermitTracker::new(
        request.policy.concurrency.episodes,
        request.policy.concurrency.agents_per_episode,
    );
    permits.acquire_episode(request.requested_episode_parallelism)?;
    let execution = execute_admitted(request, &preflight, services, &mut permits);
    permits.release_episode(request.requested_episode_parallelism);
    let seal_identity = hash_identity(
        "animus.native-evaluation-host.run-seal.v1",
        &[
            request.bindings.evaluation_identity,
            request.bindings.episode_identity,
            &preflight.identity,
        ],
    );
    let cleanup = services.cleanup(&CleanupObligation {
        evaluation_identity: request.bindings.evaluation_identity,
        episode_identity: request.bindings.episode_identity,
        cancellation_identity: request.bindings.cancellation_identity,
        seal_identity: &seal_identity,
    });
    validate_cleanup(&cleanup, request)?;
    let observations = match execution {
        Ok(observations) => observations,
        Err(error) if cleanup.complete => return Err(error),
        Err(_) => return Err(HostError::CleanupRequired),
    };
    let status = if cleanup.complete {
        HostStatus::Completed
    } else {
        HostStatus::CleanupRequired
    };
    let receipt_status = if cleanup.complete {
        HostReceiptStatus::Completed
    } else {
        HostReceiptStatus::CleanupRequired
    };
    let observation_count = u32::try_from(observations.len()).map_err(|_| HostError::Bound)?;
    let claims = [HostReceiptClaim::BoundedObservation];
    let receipt = project_host_receipt(&HostReceiptDraft {
        profile_identity: &request.policy.profile_id,
        evaluation_identity: request.bindings.evaluation_identity,
        plan_identity: &request.recipe_plan.identity,
        preflight_identity: &preflight.identity,
        service_catalog_identity: &preflight.service_catalog_identity,
        gateway_identity: &preflight.gateway.identity,
        verifier_identity: request.bindings.verifier_identity,
        artifact_identity: &preflight.artifact_manifest_identity,
        storage_identity: &preflight.storage_target_identity,
        cleanup_identity: &cleanup.identity,
        status: receipt_status,
        observation_count,
        redaction_count: 0,
        cleanup_obligation_count: cleanup.unresolved_count,
        leaks: HostReceiptLeakFlags::default(),
        claims: &claims,
    })
    .map_err(|_| HostError::Receipt)?;
    let observation_material: Vec<&str> = observations
        .iter()
        .map(|observation| observation.identity.as_str())
        .collect();
    let identity = hash_identity(
        "animus.native-evaluation-host.run.v1",
        &[
            request.bindings.evaluation_identity,
            &request.recipe_plan.identity,
            &preflight.identity,
            &observation_material.join(","),
            &seal_identity,
            &cleanup.identity,
            &receipt.identity,
        ],
    );
    Ok(HostRunResult {
        status,
        identity,
        observations,
        peak_episode_permits: permits.peak_episodes,
        peak_agent_permits: permits.peak_agents,
        receipt,
    })
}

fn execute_admitted<S: EvaluationHostServices>(
    request: &HostRunRequest<'_>,
    preflight: &HostPreflight,
    services: &mut S,
    permits: &mut PermitTracker,
) -> Result<Vec<ValidatedObservation>, HostError> {
    let primary_run = request
        .recipe_plan
        .seats
        .first()
        .ok_or(HostError::Identity)?
        .run_identity
        .as_str();
    let mut observations = Vec::new();
    call_reopen(request, preflight, services, primary_run, &mut observations)?;
    check_control(request, services)?;
    call_task(request, preflight, services, primary_run, &mut observations)?;

    let mut artifact_handoff_complete = false;
    let mut gateway_capture_complete = false;
    for seat in &request.recipe_plan.seats {
        check_control(request, services)?;
        match seat.role {
            EvaluationSeatRole::Verifier => {
                if !artifact_handoff_complete {
                    call_artifact_collect(
                        request,
                        preflight,
                        services,
                        &seat.run_identity,
                        &mut observations,
                    )?;
                    call_artifact_restore(
                        request,
                        preflight,
                        services,
                        &seat.run_identity,
                        &mut observations,
                    )?;
                    artifact_handoff_complete = true;
                }
                call_with_agent_permit(permits, request.requested_agent_parallelism, || {
                    call_verifier(
                        request,
                        preflight,
                        services,
                        &seat.run_identity,
                        &mut observations,
                    )
                })?;
            }
            EvaluationSeatRole::Solver | EvaluationSeatRole::SimulatedUser => {
                if seat.run_identity == request.gateway.projection.run_identity {
                    call_model_capture(
                        request,
                        preflight,
                        services,
                        &seat.run_identity,
                        &mut observations,
                    )?;
                    gateway_capture_complete = true;
                }
                call_with_agent_permit(permits, request.requested_agent_parallelism, || {
                    call_agent(
                        request,
                        preflight,
                        services,
                        &seat.run_identity,
                        &mut observations,
                    )
                })?;
                let tool =
                    services.execute_tool(request.bindings.tool_identity, &seat.run_identity);
                validate_tool(&tool)?;
            }
        }
    }
    if !gateway_capture_complete {
        return Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::CommitOrder,
        ));
    }
    check_control(request, services)?;
    call_append(request, preflight, services, primary_run, &mut observations)?;
    call_replace(request, preflight, services, primary_run, &mut observations)?;
    call_reopen(request, preflight, services, primary_run, &mut observations)?;
    Ok(observations)
}

fn check_control<S: CancellationPort + DeadlinePort>(
    request: &HostRunRequest<'_>,
    services: &mut S,
) -> Result<(), HostError> {
    if services.cancellation_current(request.bindings.cancellation_identity) {
        return Err(HostError::Cancelled);
    }
    if services.deadline_expired(request.bindings.deadline_identity) {
        return Err(HostError::TimedOut);
    }
    Ok(())
}

fn call_with_agent_permit<F, T>(
    permits: &mut PermitTracker,
    requested: u32,
    call: F,
) -> Result<T, HostError>
where
    F: FnOnce() -> Result<T, HostError>,
{
    permits.acquire_agent(requested)?;
    let result = call();
    permits.release_agent(requested);
    result
}

fn service_request<'a>(
    request: &'a HostRunRequest<'a>,
    preflight: &HostPreflight,
    kind: ServiceKind,
    run_identity: &'a str,
    environment_identity: &'a str,
) -> Result<ServiceRequest<'a>, HostError> {
    let admission = preflight.admission(kind)?;
    let binding = EvaluationBinding {
        evaluation_identity: request.bindings.evaluation_identity,
        plan_identity: &request.recipe_plan.identity,
        task_identity: request.bindings.task_identity,
        episode_identity: request.bindings.episode_identity,
        run_identity,
        role_identity: request.bindings.role_identity,
        model_identity: request.bindings.model_identity,
        harness_identity: request.bindings.harness_identity,
        environment_identity,
        solver_environment_identity: request.bindings.solver_environment_identity,
        tool_identity: request.bindings.tool_identity,
        verifier_identity: request.bindings.verifier_identity,
        rubric_identity: request.bindings.rubric_identity,
        artifact_identity: request.bindings.artifact_identity,
        policy_identity: request.bindings.service_policy_identity,
        cancellation_identity: request.bindings.cancellation_identity,
        deadline_identity: request.bindings.deadline_identity,
        attempt: request.bindings.attempt,
        attempt_lineage_identity: request.bindings.attempt_lineage_identity,
        correlation_identity: request.bindings.correlation_identity,
    };
    build_service_request(
        kind,
        admission.identity(),
        admission.implementation_identity(),
        &binding,
        request.service_items,
        request.service_bytes,
        admission,
    )
    .map_err(HostError::Binding)
}

fn record_observation(
    service: &ServiceRequest<'_>,
    observation: &ServiceObservation,
    observations: &mut Vec<ValidatedObservation>,
) -> Result<(), HostError> {
    let validated =
        validate_service_observation(service, observation).map_err(HostError::Binding)?;
    match reduce_observation(&validated) {
        ObservationDisposition::Eligible => {
            observations.push(validated);
            Ok(())
        }
        ObservationDisposition::Blocked | ObservationDisposition::Failed => {
            Err(HostError::Service(validated.outcome))
        }
    }
}

macro_rules! call_port {
    ($function:ident, $trait:ident, $kind:expr, $request_type:ident, $method:ident, $verifier_environment:expr) => {
        fn $function<S: $trait>(
            request: &HostRunRequest<'_>,
            preflight: &HostPreflight,
            services: &mut S,
            run_identity: &str,
            observations: &mut Vec<ValidatedObservation>,
        ) -> Result<(), HostError> {
            let environment_identity = if $verifier_environment {
                request.bindings.verifier_environment_identity
            } else {
                request.bindings.solver_environment_identity
            };
            let service = service_request(
                request,
                preflight,
                $kind,
                run_identity,
                environment_identity,
            )?;
            let typed = $request_type::new(service).map_err(HostError::Binding)?;
            let observation = services.$method(&typed);
            record_observation(typed.service(), &observation, observations)
        }
    };
}

call_port!(
    call_task,
    TaskSourcePort,
    ServiceKind::TaskSource,
    TaskSourceRequest,
    materialize,
    false
);
call_port!(
    call_agent,
    AgentExecutionPort,
    ServiceKind::AgentExecution,
    AgentExecutionRequest,
    execute_agent,
    false
);
call_port!(
    call_verifier,
    VerifierExecutionPort,
    ServiceKind::VerifierExecution,
    VerifierExecutionRequest,
    execute_verifier,
    true
);
call_port!(
    call_artifact_collect,
    ArtifactCollectPort,
    ServiceKind::ArtifactCollect,
    ArtifactCollectRequest,
    collect_artifacts,
    false
);
call_port!(
    call_artifact_restore,
    ArtifactRestorePort,
    ServiceKind::ArtifactRestore,
    ArtifactRestoreRequest,
    restore_artifacts,
    true
);
call_port!(
    call_model_capture,
    ModelCallCapturePort,
    ServiceKind::ModelCallCapture,
    ModelCallCaptureRequest,
    capture_model_call,
    false
);
call_port!(
    call_append,
    ResultAppendPort,
    ServiceKind::ResultAppend,
    ResultAppendRequest,
    append_result,
    false
);
call_port!(
    call_replace,
    ResultReplacePort,
    ServiceKind::ResultReplace,
    ResultReplaceRequest,
    replace_result,
    false
);
call_port!(
    call_reopen,
    ResultReopenPort,
    ServiceKind::ResultReopen,
    ResultReopenRequest,
    reopen_result,
    false
);

fn validate_tool(observation: &ToolObservation) -> Result<(), HostError> {
    validate_identity(&observation.identity).map_err(|_| HostError::Identity)?;
    if !observation.complete {
        return Err(HostError::Service(
            animus_adapter_eval::ObservationClass::InfrastructureFailure,
        ));
    }
    Ok(())
}

fn validate_cleanup(
    cleanup: &CleanupObservation,
    request: &HostRunRequest<'_>,
) -> Result<(), HostError> {
    validate_identity(&cleanup.identity).map_err(|_| HostError::Identity)?;
    if cleanup.unresolved_count > request.policy.cleanup.max_obligations
        || (cleanup.complete && cleanup.unresolved_count != 0)
        || (!cleanup.complete && cleanup.unresolved_count == 0)
    {
        return Err(HostError::Bound);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PermitTracker {
    max_episodes: u32,
    max_agents: u32,
    active_episodes: u32,
    active_agents: u32,
    peak_episodes: u32,
    peak_agents: u32,
}

impl PermitTracker {
    const fn new(max_episodes: u32, max_agents: u32) -> Self {
        Self {
            max_episodes,
            max_agents,
            active_episodes: 0,
            active_agents: 0,
            peak_episodes: 0,
            peak_agents: 0,
        }
    }

    fn acquire_episode(&mut self, count: u32) -> Result<(), HostError> {
        let next = self
            .active_episodes
            .checked_add(count)
            .ok_or(HostError::Bound)?;
        if count == 0 || next > self.max_episodes {
            return Err(HostError::Bound);
        }
        self.active_episodes = next;
        self.peak_episodes = self.peak_episodes.max(next);
        Ok(())
    }

    fn release_episode(&mut self, count: u32) {
        debug_assert!(self.active_episodes >= count);
        self.active_episodes -= count;
    }

    fn acquire_agent(&mut self, count: u32) -> Result<(), HostError> {
        let next = self
            .active_agents
            .checked_add(count)
            .ok_or(HostError::Bound)?;
        if count == 0 || next > self.max_agents {
            return Err(HostError::Bound);
        }
        self.active_agents = next;
        self.peak_agents = self.peak_agents.max(next);
        Ok(())
    }

    fn release_agent(&mut self, count: u32) {
        debug_assert!(self.active_agents >= count);
        self.active_agents -= count;
    }
}
