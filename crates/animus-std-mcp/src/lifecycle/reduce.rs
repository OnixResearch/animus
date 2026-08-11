use super::{
    FailureClass, LifecycleBinding, LifecycleError, LifecycleEvent, LifecyclePhase, LifecycleState,
    TeardownState,
};
use crate::config::{validate_hash, validate_safe_label};

/// Builds a configured pure lifecycle with no process effects.
///
/// # Errors
///
/// Returns a bounded identity error when the supplied generation binding is malformed.
pub fn initial_lifecycle(binding: LifecycleBinding) -> Result<LifecycleState, LifecycleError> {
    validate_binding(&binding)?;
    Ok(LifecycleState {
        binding,
        phase: LifecyclePhase::Configured,
        catalog_identity: None,
        active_call_identity: None,
        completed_call_count: 0,
        failure: None,
        restart_blocked: false,
    })
}

/// Applies one exact lifecycle event under a finite call budget.
///
/// # Errors
///
/// Returns a bounded transition, identity, or budget error without starting effects.
// r[impl agent_harness_runtime.mcp_client_host.lifecycle]
pub fn reduce_lifecycle(
    current: &LifecycleState,
    event: LifecycleEvent,
    max_calls_per_generation: u32,
) -> Result<LifecycleState, LifecycleError> {
    if max_calls_per_generation == 0 {
        return Err(LifecycleError::CallBudgetExhausted);
    }
    match event {
        LifecycleEvent::BeginStart(binding) => begin_start(current, &binding),
        LifecycleEvent::ProcessStarted(binding) => process_started(current, &binding),
        LifecycleEvent::InitializationAccepted(binding) => initialized(current, &binding),
        LifecycleEvent::DiscoveryAccepted {
            binding,
            catalog_identity,
            tool_count,
        } => discovered(current, &binding, catalog_identity, tool_count),
        LifecycleEvent::CallStarted {
            binding,
            call_identity,
        } => start_call(current, &binding, call_identity, max_calls_per_generation),
        LifecycleEvent::CallCompleted {
            binding,
            call_identity,
            output_identity,
        } => complete_call(
            current,
            &binding,
            call_identity.as_str(),
            output_identity.as_str(),
        ),
        LifecycleEvent::CredentialDrifted {
            binding,
            observed_credential_identity,
        } => credential_drift(current, &binding, observed_credential_identity.as_str()),
        LifecycleEvent::CancellationRequested {
            binding,
            call_identity,
        } => cancel_call(current, &binding, call_identity.as_str()),
        LifecycleEvent::StopRequested(binding) => stop_requested(current, &binding),
        LifecycleEvent::StopObserved { binding, teardown } => {
            stop_observed(current, &binding, teardown)
        }
        LifecycleEvent::Failed {
            binding,
            class,
            teardown,
        } => fail(current, &binding, class, teardown),
    }
}

fn begin_start(
    current: &LifecycleState,
    binding: &LifecycleBinding,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Configured)?;
    require_binding(current, binding)?;
    Ok(next_phase(current, LifecyclePhase::Starting))
}

fn process_started(
    current: &LifecycleState,
    binding: &LifecycleBinding,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Starting)?;
    require_binding(current, binding)?;
    Ok(next_phase(current, LifecyclePhase::Initializing))
}

fn initialized(
    current: &LifecycleState,
    binding: &LifecycleBinding,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Initializing)?;
    require_binding(current, binding)?;
    Ok(next_phase(current, LifecyclePhase::Discovering))
}

fn discovered(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    catalog_identity: String,
    tool_count: usize,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Discovering)?;
    require_binding(current, binding)?;
    validate_hash(catalog_identity.as_str()).map_err(|_| LifecycleError::InvalidIdentity)?;
    if tool_count == 0 {
        return Err(LifecycleError::InvalidTransition);
    }
    let mut next = current.clone();
    next.phase = LifecyclePhase::Ready;
    next.catalog_identity = Some(catalog_identity);
    Ok(next)
}

fn start_call(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    call_identity: String,
    max_calls_per_generation: u32,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Ready)?;
    require_binding(current, binding)?;
    validate_hash(call_identity.as_str()).map_err(|_| LifecycleError::InvalidIdentity)?;
    if current.completed_call_count >= max_calls_per_generation {
        return Err(LifecycleError::CallBudgetExhausted);
    }
    let mut next = current.clone();
    next.phase = LifecyclePhase::Calling;
    next.active_call_identity = Some(call_identity);
    Ok(next)
}

fn complete_call(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    call_identity: &str,
    output_identity: &str,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Calling)?;
    require_binding(current, binding)?;
    validate_hash(output_identity).map_err(|_| LifecycleError::InvalidIdentity)?;
    if current.active_call_identity.as_deref() != Some(call_identity) {
        return Err(LifecycleError::BindingDrift);
    }
    let Some(completed_call_count) = current.completed_call_count.checked_add(1) else {
        return Err(LifecycleError::CallBudgetExhausted);
    };
    let mut next = current.clone();
    next.phase = LifecyclePhase::Ready;
    next.active_call_identity = None;
    next.completed_call_count = completed_call_count;
    Ok(next)
}

fn credential_drift(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    observed_credential_identity: &str,
) -> Result<LifecycleState, LifecycleError> {
    require_binding(current, binding)?;
    validate_hash(observed_credential_identity).map_err(|_| LifecycleError::InvalidIdentity)?;
    if current.binding.credential_snapshot_id == observed_credential_identity {
        return Err(LifecycleError::InvalidTransition);
    }
    if matches!(
        current.phase,
        LifecyclePhase::Stopped | LifecyclePhase::Failed | LifecyclePhase::Stopping
    ) {
        return Err(LifecycleError::InvalidTransition);
    }
    let mut next = current.clone();
    next.phase = LifecyclePhase::Stopping;
    next.failure = Some(FailureClass::CredentialDrift);
    next.active_call_identity = None;
    Ok(next)
}

fn cancel_call(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    call_identity: &str,
) -> Result<LifecycleState, LifecycleError> {
    require_phase(current, LifecyclePhase::Calling)?;
    require_binding(current, binding)?;
    if current.active_call_identity.as_deref() != Some(call_identity) {
        return Err(LifecycleError::BindingDrift);
    }
    Ok(next_phase(current, LifecyclePhase::Cancelling))
}

fn stop_requested(
    current: &LifecycleState,
    binding: &LifecycleBinding,
) -> Result<LifecycleState, LifecycleError> {
    require_binding(current, binding)?;
    if matches!(
        current.phase,
        LifecyclePhase::Stopped | LifecyclePhase::Stopping
    ) {
        return Err(LifecycleError::InvalidTransition);
    }
    Ok(next_phase(current, LifecyclePhase::Stopping))
}

fn stop_observed(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    teardown: TeardownState,
) -> Result<LifecycleState, LifecycleError> {
    if !matches!(
        current.phase,
        LifecyclePhase::Stopping | LifecyclePhase::Cancelling
    ) {
        return Err(LifecycleError::InvalidTransition);
    }
    require_binding(current, binding)?;
    let mut next = current.clone();
    next.active_call_identity = None;
    match teardown {
        TeardownState::Terminal | TeardownState::NotStarted => {
            next.phase = LifecyclePhase::Stopped;
            next.restart_blocked = false;
        }
        TeardownState::Uncertain => {
            next.phase = LifecyclePhase::Failed;
            next.failure = Some(FailureClass::Teardown);
            next.restart_blocked = true;
        }
    }
    Ok(next)
}

fn fail(
    current: &LifecycleState,
    binding: &LifecycleBinding,
    class: FailureClass,
    teardown: TeardownState,
) -> Result<LifecycleState, LifecycleError> {
    require_binding(current, binding)?;
    if current.phase == LifecyclePhase::Stopped {
        return Err(LifecycleError::InvalidTransition);
    }
    let mut next = current.clone();
    next.phase = LifecyclePhase::Failed;
    next.failure = Some(class);
    next.active_call_identity = None;
    next.restart_blocked = teardown == TeardownState::Uncertain;
    Ok(next)
}

fn require_phase(current: &LifecycleState, expected: LifecyclePhase) -> Result<(), LifecycleError> {
    if current.phase != expected {
        return Err(LifecycleError::InvalidTransition);
    }
    Ok(())
}

fn require_binding(
    current: &LifecycleState,
    observed: &LifecycleBinding,
) -> Result<(), LifecycleError> {
    validate_binding(observed)?;
    if &current.binding != observed {
        return Err(LifecycleError::BindingDrift);
    }
    Ok(())
}

fn next_phase(current: &LifecycleState, phase: LifecyclePhase) -> LifecycleState {
    let mut next = current.clone();
    next.phase = phase;
    next
}

fn validate_binding(binding: &LifecycleBinding) -> Result<(), LifecycleError> {
    validate_safe_label(binding.server_id.as_str()).map_err(|_| LifecycleError::InvalidIdentity)?;
    validate_safe_label(binding.protocol_id.as_str())
        .map_err(|_| LifecycleError::InvalidIdentity)?;
    validate_hash(binding.configuration_id.as_str())
        .map_err(|_| LifecycleError::InvalidIdentity)?;
    validate_hash(binding.policy_id.as_str()).map_err(|_| LifecycleError::InvalidIdentity)?;
    validate_hash(binding.credential_snapshot_id.as_str())
        .map_err(|_| LifecycleError::InvalidIdentity)?;
    validate_hash(binding.process_attempt_id.as_str())
        .map_err(|_| LifecycleError::InvalidIdentity)?;
    if binding.generation == 0 {
        return Err(LifecycleError::InvalidIdentity);
    }
    Ok(())
}
