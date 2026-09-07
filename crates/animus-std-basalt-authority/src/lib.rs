//! Optional standard-library Basalt UCAN authority gate for Animus host effects.
//!
//! The pure core translates exact generic Animus requests through a closed
//! supported-action table. The shell calls Basalt with caller-owned key, proof,
//! revocation, caveat, replay, time, and limit services. It dispatches no effect
//! until Basalt admission, every existing Animus gate, and safe-decision commit pass.
//!
//! This crate and Basalt use `AGPL-3.0-or-later`. Adoption is explicit and
//! non-default. Consumers must complete their own license review.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]

mod core;
mod shell;

pub use core::{
    BASALT_LICENSE, BASALT_POLICY_IDENTITY, BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
    BASALT_SOURCE_REVISION, BASALT_UCAN_CATALOG_IDENTITY, BASALT_UCAN_REVISION, BasaltRequestPlan,
    basalt_provider_identity, map_basalt_reason, normalize_basalt_identity,
    operation_profile_identity, plan_basalt_request, principal_hash, required_policy_identity,
    resource_hash,
};
pub use shell::{
    BasaltAuthorityFailure, BasaltAuthorityInvocation, BasaltAuthorityService,
    BasaltAuthorizedDispatch, BasaltDispatchContext, HostAuthorityDecisionCommit,
    dispatch_with_basalt,
};

#[cfg(test)]
mod tests;
