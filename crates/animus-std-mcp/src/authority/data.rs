use serde_json::Value;

use crate::{DiscoveredTool, McpLimits};

/// Current decision from an existing Animus authority owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateDecision {
    /// The exact current observation allows the operation.
    Allowed,
    /// The exact current observation denies the operation.
    Denied,
}

/// Exact immutable call binding selected before dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallBinding {
    /// Exact server identity.
    pub server_id: String,
    /// Exact server generation.
    pub server_generation: u64,
    /// Exact mapped tool name.
    pub tool_name: String,
    /// Exact schema identity.
    pub schema_identity: String,
    /// Exact model invocation-plan identity.
    pub model_plan_identity: String,
    /// Exact runtime authority-seal identity.
    pub authority_seal_identity: String,
    /// Exact approval identity.
    pub approval_identity: String,
    /// Exact capability-grant identity.
    pub capability_grant_identity: String,
    /// Exact mutation-plan identity.
    pub mutation_plan_identity: String,
    /// Exact output-bound identity.
    pub output_bound_identity: String,
    /// Parent capability labels.
    pub parent_capabilities: Vec<String>,
    /// Current caller capability labels.
    pub caller_capabilities: Vec<String>,
}

/// Current owner observations checked immediately before transport dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallAuthorityObservation {
    /// Current server generation.
    pub server_generation: u64,
    /// Current schema identity.
    pub schema_identity: String,
    /// Current model invocation-plan identity.
    pub model_plan_identity: String,
    /// Current runtime authority-seal identity.
    pub authority_seal_identity: String,
    /// Current approval identity.
    pub approval_identity: String,
    /// Current capability-grant identity.
    pub capability_grant_identity: String,
    /// Current mutation-plan identity.
    pub mutation_plan_identity: String,
    /// Current output-bound identity.
    pub output_bound_identity: String,
    /// Existing strict argument-validation result.
    pub arguments: GateDecision,
    /// Existing executor readiness.
    pub executor: GateDecision,
    /// Existing capability decision.
    pub capability: GateDecision,
    /// Current approval decision.
    pub approval: GateDecision,
    /// Existing mutation coordination result.
    pub mutation: GateDecision,
    /// Existing output-boundary result.
    pub output: GateDecision,
    /// Current cancellation state.
    pub cancellation: GateDecision,
}

/// Borrowed data needed to plan one bounded MCP call.
#[derive(Clone, Copy)]
pub struct CallRequestView<'a> {
    /// Exact discovered tool.
    pub tool: &'a DiscoveredTool,
    /// Untrusted JSON object arguments.
    pub arguments: &'a Value,
    /// Immutable call binding.
    pub binding: &'a CallBinding,
    /// Current owner observations.
    pub observation: &'a CallAuthorityObservation,
    /// Calls already completed on this generation.
    pub completed_call_count: u32,
    /// Finite configured limits.
    pub limits: McpLimits,
}

/// Metadata-only call plan authorized for one transport dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallPlan {
    pub(crate) identity: String,
    pub(crate) server_generation: u64,
    pub(crate) tool_name: String,
    pub(crate) schema_identity: String,
    pub(crate) argument_identity: String,
    pub(crate) output_bound_identity: String,
}

impl CallPlan {
    /// Returns the exact call-plan identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the exact bound server generation.
    #[must_use]
    pub const fn server_generation(&self) -> u64 {
        self.server_generation
    }

    /// Returns the exact tool name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }

    /// Returns the exact schema identity.
    #[must_use]
    pub fn schema_identity(&self) -> &str {
        self.schema_identity.as_str()
    }

    /// Returns the argument payload identity without retaining arguments.
    #[must_use]
    pub fn argument_identity(&self) -> &str {
        self.argument_identity.as_str()
    }

    /// Returns the exact output-bound identity.
    #[must_use]
    pub fn output_bound_identity(&self) -> &str {
        self.output_bound_identity.as_str()
    }
}

/// Bounded call-planning denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallPlanError {
    /// An exact current identity drifted.
    StaleIdentity,
    /// Arguments were malformed or over budget.
    InvalidArguments,
    /// Existing argument, executor, or output admission denied the call.
    ExistingGateDenied,
    /// Current approval was absent or denied.
    ApprovalDenied,
    /// Parent, caller, and server capability intersection denied the call.
    CapabilityDenied,
    /// Existing mutation coordination denied the call.
    MutationDenied,
    /// Cancellation was already active.
    Cancelled,
    /// The generation call budget is exhausted.
    CallBudgetExhausted,
}
