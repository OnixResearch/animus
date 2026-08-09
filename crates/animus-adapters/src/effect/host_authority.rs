//! Implementation-neutral authority-provider contracts for guarded host actions.

mod decision;
mod receipt;
mod request;
mod service;

#[cfg(test)]
mod tests;

pub use decision::{
    EffectiveAuthorityDecision, HostAuthorityDecision, HostAuthorityDecisionDraft,
    HostAuthorityFreshnessClass, HostAuthorityIssue, HostAuthorityReplayClass, HostAuthorityStatus,
    HostGateStatus, RequiredHostGates, build_host_authority_decision, intersect_host_authority,
};
pub use receipt::{
    HOST_AUTHORITY_NON_CLAIMS, HostAuthorityReceipt, HostAuthorityReceiptDraft,
    HostAuthorityReceiptLeak, build_host_authority_receipt, validate_host_authority_receipt_text,
};
pub use request::{
    AgentAuthorityBinding, AgentAuthorityBindingDraft, BindingFreshness, HostAuthorityBounds,
    HostAuthorityRequest, HostAuthorityRequestDraft, admit_agent_authority_binding,
    admit_host_authority_request, validate_agent_authority_binding_set,
};
pub use service::HostAuthorityService;
