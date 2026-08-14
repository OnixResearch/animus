//! Pure provider-neutral private model-request snapshot contracts.

mod canonical;
mod core;
mod data;
mod receipt;

pub use canonical::{
    decode_model_request_provider_terminal_private_payload,
    decode_model_request_snapshot_private_payload, model_request_budget_identity,
    model_request_canonical_identity, model_request_output_control_identity,
    model_request_provider_terminal_private_payload, model_request_route_identity,
    model_request_tool_schema_set_identity,
};
pub use core::{
    classify_model_request_snapshot_recovery, observe_model_request_provider_terminal,
    observe_model_request_snapshot_commit, plan_initial_model_request_dispatch,
    plan_model_request_snapshot_commit, plan_same_model_request_retry,
    validate_model_request_provider_terminal_observation, validate_model_request_snapshot,
};
pub use data::*;
pub use receipt::{
    project_model_request_snapshot_receipt, validate_model_request_snapshot_receipt,
};

#[cfg(test)]
mod tests;
