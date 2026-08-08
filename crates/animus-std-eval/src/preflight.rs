//! Pure host preflight over checked plans, policy, services, and destinations.

use animus_adapter_eval::{
    ServiceAdmission, ServiceKind, ServiceRequirement, admit_service, validate_service_catalog,
};
use animus_eval_core::{hash_identity, validate_identity};

use crate::{
    AuxiliaryServiceKind, GatewayAdmission, HostError, HostRunRequest, validate_artifact_manifest,
    validate_evaluation_host_policy, validate_gateway_invocation, validate_storage_target,
};

/// Number of separate evaluation adapter services required by the complete host slice.
pub const REQUIRED_HOST_SERVICE_COUNT: usize = 9;
/// Number of separate host-only auxiliary services.
pub const REQUIRED_AUXILIARY_SERVICE_COUNT: usize = 4;

const REQUIRED_SERVICES: [ServiceKind; REQUIRED_HOST_SERVICE_COUNT] = [
    ServiceKind::TaskSource,
    ServiceKind::AgentExecution,
    ServiceKind::VerifierExecution,
    ServiceKind::ArtifactCollect,
    ServiceKind::ArtifactRestore,
    ServiceKind::ModelCallCapture,
    ServiceKind::ResultAppend,
    ServiceKind::ResultReplace,
    ServiceKind::ResultReopen,
];

/// Fully admitted host plan before any effect dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostPreflight {
    /// Canonical preflight identity.
    pub identity: String,
    /// Canonical validated service-catalog identity.
    pub service_catalog_identity: String,
    /// Nine exact service admissions.
    pub admissions: Vec<ServiceAdmission>,
    /// Canonical auxiliary service-catalog identity.
    pub auxiliary_catalog_identity: String,
    /// Validated gateway admission.
    pub gateway: GatewayAdmission,
    /// Canonical artifact-manifest identity.
    pub artifact_manifest_identity: String,
    /// Canonical storage-target identity.
    pub storage_target_identity: String,
    /// Requested outer concurrency.
    pub episode_parallelism: u32,
    /// Requested inner concurrency.
    pub agent_parallelism: u32,
}

impl HostPreflight {
    /// Return one exact service admission.
    ///
    /// # Errors
    /// Returns admission absence if the preflight is internally incomplete.
    pub fn admission(&self, kind: ServiceKind) -> Result<&ServiceAdmission, HostError> {
        self.admissions
            .iter()
            .find(|admission| admission.kind() == kind)
            .ok_or(HostError::Admission(
                animus_adapter_eval::AdmissionError::Absent,
            ))
    }
}

/// Admit a complete host request before any service call.
///
/// # Errors
/// Returns policy, identity, bound, service, gateway, artifact, or storage
/// failures without ambient discovery or partial dispatch.
pub fn preflight_host(request: &HostRunRequest<'_>) -> Result<HostPreflight, HostError> {
    validate_evaluation_host_policy(request.policy).map_err(|_| HostError::Policy)?;
    validate_request_identities(request)?;
    validate_request_bounds(request)?;
    let service_catalog_identity =
        validate_service_catalog(&request.service_catalog).map_err(HostError::Admission)?;
    let auxiliary_catalog_identity = validate_auxiliary_services(request)?;
    let gateway = validate_gateway_invocation(&request.gateway, request.policy)?;
    if !request.recipe_plan.seats.iter().any(|seat| {
        seat.run_identity == request.gateway.projection.run_identity
            && seat.role != animus_eval_core::EvaluationSeatRole::Verifier
    }) {
        return Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::CommitOrder,
        ));
    }
    let artifact_manifest_identity =
        validate_artifact_manifest(&request.artifacts, request.policy)?;
    let storage_target_identity = validate_storage_target(&request.storage, request.policy)?;
    if request.artifacts.identity != request.bindings.artifact_identity {
        return Err(HostError::Artifact);
    }
    let mut admissions = Vec::with_capacity(REQUIRED_HOST_SERVICE_COUNT);
    for kind in REQUIRED_SERVICES {
        let requirement = ServiceRequirement {
            kind,
            implementation_identity: None,
            required_items: request.service_items,
            required_bytes: request.service_bytes,
            policy_identity: request.bindings.service_policy_identity,
        };
        admissions.push(
            admit_service(&request.service_catalog, &requirement).map_err(HostError::Admission)?,
        );
    }
    let identity = hash_identity(
        "animus.native-evaluation-host.preflight.v1",
        &[
            request.bindings.evaluation_identity,
            &request.recipe_plan.identity,
            request.bindings.task_identity,
            request.bindings.episode_identity,
            &service_catalog_identity,
            &auxiliary_catalog_identity,
            &gateway.identity,
            &artifact_manifest_identity,
            &storage_target_identity,
            &request.requested_episode_parallelism.to_string(),
            &request.requested_agent_parallelism.to_string(),
        ],
    );
    Ok(HostPreflight {
        identity,
        service_catalog_identity,
        admissions,
        auxiliary_catalog_identity,
        gateway,
        artifact_manifest_identity,
        storage_target_identity,
        episode_parallelism: request.requested_episode_parallelism,
        agent_parallelism: request.requested_agent_parallelism,
    })
}

fn validate_auxiliary_services(request: &HostRunRequest<'_>) -> Result<String, HostError> {
    validate_identity(request.auxiliary_services.identity).map_err(|_| HostError::Identity)?;
    if request.auxiliary_services.services.len() != REQUIRED_AUXILIARY_SERVICE_COUNT {
        return Err(HostError::Admission(
            animus_adapter_eval::AdmissionError::Absent,
        ));
    }
    let required = [
        AuxiliaryServiceKind::Tool,
        AuxiliaryServiceKind::Timer,
        AuxiliaryServiceKind::Cancellation,
        AuxiliaryServiceKind::Cleanup,
    ];
    let mut material = Vec::with_capacity(REQUIRED_AUXILIARY_SERVICE_COUNT);
    for kind in required {
        let matches: Vec<_> = request
            .auxiliary_services
            .services
            .iter()
            .filter(|service| service.kind == kind)
            .collect();
        if matches.len() != 1 {
            return Err(HostError::Admission(
                animus_adapter_eval::AdmissionError::Catalog,
            ));
        }
        let service = matches[0];
        validate_identity(service.implementation_identity).map_err(|_| HostError::Identity)?;
        validate_identity(service.policy_identity).map_err(|_| HostError::Identity)?;
        if service.policy_identity != request.bindings.service_policy_identity {
            return Err(HostError::Admission(
                animus_adapter_eval::AdmissionError::Mismatch,
            ));
        }
        if !service.available {
            return Err(HostError::Admission(
                animus_adapter_eval::AdmissionError::Unavailable,
            ));
        }
        material.push(hash_identity(
            "animus.native-evaluation-host.auxiliary-service.v1",
            &[
                kind.label(),
                service.implementation_identity,
                service.policy_identity,
            ],
        ));
    }
    Ok(hash_identity(
        "animus.native-evaluation-host.auxiliary-catalog.v1",
        &[request.auxiliary_services.identity, &material.join(",")],
    ))
}

fn validate_request_identities(request: &HostRunRequest<'_>) -> Result<(), HostError> {
    for identity in [
        request.recipe_plan.identity.as_str(),
        request.bindings.evaluation_identity,
        request.bindings.task_identity,
        request.bindings.episode_identity,
        request.bindings.role_identity,
        request.bindings.model_identity,
        request.bindings.harness_identity,
        request.bindings.solver_environment_identity,
        request.bindings.verifier_environment_identity,
        request.bindings.tool_identity,
        request.bindings.verifier_identity,
        request.bindings.rubric_identity,
        request.bindings.artifact_identity,
        request.bindings.service_policy_identity,
        request.bindings.cancellation_identity,
        request.bindings.deadline_identity,
        request.bindings.attempt_lineage_identity,
        request.bindings.correlation_identity,
    ] {
        validate_identity(identity).map_err(|_| HostError::Identity)?;
    }
    if request.recipe_plan.seats.is_empty()
        || request.recipe_plan.actions.is_empty()
        || request.bindings.solver_environment_identity
            == request.bindings.verifier_environment_identity
        || request.bindings.attempt == 0
    {
        return Err(HostError::Identity);
    }
    Ok(())
}

fn validate_request_bounds(request: &HostRunRequest<'_>) -> Result<(), HostError> {
    if request.requested_episode_parallelism == 0
        || request.requested_episode_parallelism > request.policy.concurrency.episodes
        || request.requested_agent_parallelism == 0
        || request.requested_agent_parallelism > request.policy.concurrency.agents_per_episode
        || request.service_items == 0
        || request.service_bytes == 0
        || request.service_bytes > request.storage.max_bytes
    {
        return Err(HostError::Bound);
    }
    Ok(())
}
