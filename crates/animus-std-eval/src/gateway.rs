//! Local gateway admission over durable turn evidence and bounded projections.

use animus_adapter_eval::{
    CoalescingEvidence, ModelCallProjection, ModelCallProjectionDraft, TerminalStreamEvidence,
    TurnCommitEvidence, admit_identical_request_coalescing, admit_terminal_after_commit,
    project_model_call,
};
use animus_eval_core::{hash_identity, validate_identity};

use crate::{EvaluationHostPolicy, GatewayBindMode, HostError};

/// Safe gateway timing class. This is not a raw timestamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayTimingClass {
    /// Call completed within its selected normal budget.
    WithinBudget,
    /// Call completed near its selected deadline.
    NearDeadline,
    /// Call reached its selected deadline.
    TimedOut,
}

impl GatewayTimingClass {
    const fn label(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::NearDeadline => "near_deadline",
            Self::TimedOut => "timed_out",
        }
    }
}

/// Durable accepted-turn evidence required before model dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcceptedTurnEvidence<'a> {
    /// Exact run identity.
    pub run_identity: &'a str,
    /// Exact canonical request identity.
    pub request_identity: &'a str,
    /// Exact active-turn identity.
    pub turn_identity: &'a str,
    /// Exact policy identity.
    pub policy_identity: &'a str,
    /// Durable acceptance transaction identity.
    pub acceptance_identity: &'a str,
    /// Whether acceptance completed before dispatch.
    pub complete: bool,
}

/// Complete local gateway invocation facts supplied by the host shell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayInvocation<'a> {
    /// Selected bind class.
    pub bind_mode: GatewayBindMode,
    /// Exact bind capability identity.
    pub bind_identity: &'a str,
    /// Scoped model capability identity.
    pub model_capability_identity: &'a str,
    /// Separate scoped task-state capability identity.
    pub state_capability_identity: &'a str,
    /// Durable accepted-turn evidence.
    pub accepted_turn: AcceptedTurnEvidence<'a>,
    /// Canonical bounded call projection input.
    pub projection: ModelCallProjectionDraft<'a>,
    /// Safe timing class.
    pub timing: GatewayTimingClass,
    /// Terminal stream evidence.
    pub terminal: TerminalStreamEvidence<'a>,
    /// Matching durable turn-commit evidence.
    pub commit: TurnCommitEvidence<'a>,
    /// Optional exact retry-coalescing evidence.
    pub coalescing: Option<CoalescingEvidence<'a>>,
    /// Bounded request and response material bytes observed by the host.
    pub body_bytes: u64,
    /// Bounded stream event count observed by the host.
    pub stream_events: u32,
    /// Whether the bind exposes a public listener.
    pub public_bind: bool,
    /// Whether network policy contains a wildcard.
    pub wildcard_network: bool,
    /// Whether any response, tool result, web result, or assistant body is rewritten.
    pub rewrites_material: bool,
    /// Whether the run was already sealed before this invocation.
    pub sealed: bool,
}

/// Validated gateway admission before provider-facing capture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayAdmission {
    /// Canonical admission identity.
    pub identity: String,
    /// Validated model-call projection.
    pub projection: ModelCallProjection,
    /// Terminal-after-commit identity.
    pub terminal_identity: String,
    /// Optional retry-coalescing identity.
    pub coalescing_identity: Option<String>,
    /// Safe timing class.
    pub timing: GatewayTimingClass,
}

/// Validate gateway bind, capabilities, bounds, durable ordering, and retry facts.
///
/// # Errors
/// Returns a typed policy, identity, bound, gateway, or sealed class before
/// model transport or provider dispatch.
pub fn validate_gateway_invocation(
    invocation: &GatewayInvocation<'_>,
    policy: &EvaluationHostPolicy,
) -> Result<GatewayAdmission, HostError> {
    for identity in [
        invocation.bind_identity,
        invocation.model_capability_identity,
        invocation.state_capability_identity,
        invocation.accepted_turn.run_identity,
        invocation.accepted_turn.request_identity,
        invocation.accepted_turn.turn_identity,
        invocation.accepted_turn.policy_identity,
        invocation.accepted_turn.acceptance_identity,
    ] {
        validate_identity(identity).map_err(|_| HostError::Identity)?;
    }
    if invocation.sealed {
        return Err(HostError::Sealed);
    }
    if !policy.gateway.bind_modes.contains(&invocation.bind_mode)
        || invocation.public_bind
        || invocation.wildcard_network
        || invocation.rewrites_material
        || invocation.model_capability_identity == invocation.state_capability_identity
    {
        return Err(HostError::Policy);
    }
    if invocation.body_bytes == 0
        || invocation.body_bytes > policy.gateway.max_body_bytes
        || invocation.stream_events == 0
        || invocation.stream_events > policy.gateway.max_stream_events
    {
        return Err(HostError::Bound);
    }
    let accepted = invocation.accepted_turn;
    let projection = invocation.projection;
    if !accepted.complete
        || accepted.run_identity != projection.run_identity
        || accepted.request_identity != projection.request_identity
        || accepted.policy_identity != projection.policy_identity
        || accepted.run_identity != invocation.terminal.run_identity
        || accepted.request_identity != invocation.terminal.request_identity
        || accepted.turn_identity != invocation.terminal.turn_identity
    {
        return Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::CommitOrder,
        ));
    }
    let projected = project_model_call(&projection).map_err(HostError::Gateway)?;
    let terminal_identity = admit_terminal_after_commit(&invocation.terminal, &invocation.commit)
        .map_err(HostError::Gateway)?;
    let coalescing_identity = invocation
        .coalescing
        .as_ref()
        .map(admit_identical_request_coalescing)
        .transpose()
        .map_err(HostError::Gateway)?;
    let identity = hash_identity(
        "animus.native-evaluation-host.gateway-admission.v1",
        &[
            invocation.bind_identity,
            invocation.model_capability_identity,
            invocation.state_capability_identity,
            accepted.acceptance_identity,
            &projected.identity,
            &terminal_identity,
            coalescing_identity.as_deref().unwrap_or("none"),
            invocation.timing.label(),
            &invocation.body_bytes.to_string(),
            &invocation.stream_events.to_string(),
        ],
    );
    Ok(GatewayAdmission {
        identity,
        projection: projected,
        terminal_identity,
        coalescing_identity,
        timing: invocation.timing,
    })
}
