use std::collections::BTreeSet;

use super::{CallPlan, CallPlanError, CallRequestView, GateDecision};
use crate::config::{framed_hash, validate_hash, validate_safe_label};

const ARGUMENT_DOMAIN: &str = "animus.mcp-client-host.arguments.v1";
const CALL_DOMAIN: &str = "animus.mcp-client-host.call.v1";

/// Computes the exact argument identity used by transport admission.
///
/// # Errors
///
/// Returns `InvalidArguments` when canonical JSON serialization fails.
pub fn argument_identity(arguments: &serde_json::Value) -> Result<String, CallPlanError> {
    let bytes = serde_json::to_vec(arguments).map_err(|_| CallPlanError::InvalidArguments)?;
    Ok(framed_hash(ARGUMENT_DOMAIN, bytes.as_slice()))
}

/// Plans one MCP transport dispatch after exact existing Animus gates.
///
/// # Errors
///
/// Returns a bounded denial before transport dispatch or model visibility.
// r[impl agent_harness_runtime.mcp_client_host.authority]
pub fn plan_call(request: CallRequestView<'_>) -> Result<CallPlan, CallPlanError> {
    validate_exact_identities(request)?;
    validate_arguments(request)?;
    validate_current_gates(request)?;
    validate_capability_intersection(request)?;
    if request.completed_call_count >= request.limits.max_calls_per_generation {
        return Err(CallPlanError::CallBudgetExhausted);
    }
    let argument_identity = argument_identity(request.arguments)?;
    let bridge = animus_adapters::evaluate_mcp_dispatch(animus_adapters::McpDispatchRequest {
        definition: Some(request.tool.definition()),
        argument_status: animus_adapters::McpArgumentStatus::Valid,
        executor_status: animus_adapters::tool::ExecutorStatus::Available,
        capability_status: animus_adapters::tool::CapabilityStatus::Allowed,
        mutation_status: animus_adapters::McpMutationStatus::Independent,
        output_boundary_status: animus_adapters::tool::OutputBoundaryStatus::Bounded,
    });
    if !bridge.server_adapter_called() {
        return Err(CallPlanError::ExistingGateDenied);
    }
    let identity = call_identity(request, argument_identity.as_str());
    Ok(CallPlan {
        identity,
        server_generation: request.binding.server_generation,
        tool_name: request.binding.tool_name.clone(),
        schema_identity: request.binding.schema_identity.clone(),
        argument_identity,
        output_bound_identity: request.binding.output_bound_identity.clone(),
    })
}

fn validate_exact_identities(request: CallRequestView<'_>) -> Result<(), CallPlanError> {
    validate_safe_label(request.binding.server_id.as_str())
        .map_err(|_| CallPlanError::StaleIdentity)?;
    for identity in [
        request.binding.schema_identity.as_str(),
        request.binding.model_plan_identity.as_str(),
        request.binding.authority_seal_identity.as_str(),
        request.binding.approval_identity.as_str(),
        request.binding.capability_grant_identity.as_str(),
        request.binding.mutation_plan_identity.as_str(),
        request.binding.output_bound_identity.as_str(),
    ] {
        validate_hash(identity).map_err(|_| CallPlanError::StaleIdentity)?;
    }
    if request.binding.server_generation != request.observation.server_generation
        || request.binding.schema_identity != request.observation.schema_identity
        || request.binding.model_plan_identity != request.observation.model_plan_identity
        || request.binding.authority_seal_identity != request.observation.authority_seal_identity
        || request.binding.approval_identity != request.observation.approval_identity
        || request.binding.capability_grant_identity
            != request.observation.capability_grant_identity
        || request.binding.mutation_plan_identity != request.observation.mutation_plan_identity
        || request.binding.output_bound_identity != request.observation.output_bound_identity
        || request.binding.tool_name != request.tool.mapping().tool_name()
        || request.binding.schema_identity != request.tool.mapping().schema_identity()
    {
        return Err(CallPlanError::StaleIdentity);
    }
    Ok(())
}

fn validate_arguments(request: CallRequestView<'_>) -> Result<(), CallPlanError> {
    let object = request
        .arguments
        .as_object()
        .ok_or(CallPlanError::InvalidArguments)?;
    if object.len() > request.limits.max_arguments {
        return Err(CallPlanError::InvalidArguments);
    }
    let bytes =
        serde_json::to_vec(request.arguments).map_err(|_| CallPlanError::InvalidArguments)?;
    if bytes.len() > request.limits.max_frame_bytes {
        return Err(CallPlanError::InvalidArguments);
    }
    Ok(())
}

fn validate_current_gates(request: CallRequestView<'_>) -> Result<(), CallPlanError> {
    if request.observation.cancellation != GateDecision::Allowed {
        return Err(CallPlanError::Cancelled);
    }
    if request.observation.approval != GateDecision::Allowed {
        return Err(CallPlanError::ApprovalDenied);
    }
    if request.observation.capability != GateDecision::Allowed {
        return Err(CallPlanError::CapabilityDenied);
    }
    if request.observation.mutation != GateDecision::Allowed {
        return Err(CallPlanError::MutationDenied);
    }
    if request.observation.arguments != GateDecision::Allowed
        || request.observation.executor != GateDecision::Allowed
        || request.observation.output != GateDecision::Allowed
    {
        return Err(CallPlanError::ExistingGateDenied);
    }
    Ok(())
}

fn validate_capability_intersection(request: CallRequestView<'_>) -> Result<(), CallPlanError> {
    let parent = request
        .binding
        .parent_capabilities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let caller = request
        .binding
        .caller_capabilities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if request
        .tool
        .mapping()
        .capability_labels()
        .iter()
        .any(|capability| {
            !parent.contains(capability.as_str()) || !caller.contains(capability.as_str())
        })
    {
        return Err(CallPlanError::CapabilityDenied);
    }
    Ok(())
}

fn call_identity(request: CallRequestView<'_>, argument_identity: &str) -> String {
    let mut bytes = Vec::new();
    for value in [
        request.binding.server_id.as_str(),
        request.binding.tool_name.as_str(),
        request.binding.schema_identity.as_str(),
        request.binding.model_plan_identity.as_str(),
        request.binding.authority_seal_identity.as_str(),
        request.binding.approval_identity.as_str(),
        request.binding.capability_grant_identity.as_str(),
        request.binding.mutation_plan_identity.as_str(),
        request.binding.output_bound_identity.as_str(),
        argument_identity,
    ] {
        append_framed(&mut bytes, value.as_bytes());
    }
    bytes.extend_from_slice(&request.binding.server_generation.to_le_bytes());
    framed_hash(CALL_DOMAIN, bytes.as_slice())
}

fn append_framed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u64).to_le_bytes());
    output.extend_from_slice(value);
}
