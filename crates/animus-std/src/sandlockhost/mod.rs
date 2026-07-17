//! Optional fail-closed Linux Sandlock host adapter.
//!
//! Planning, authorization binding, capability admission, and receipt
//! projection are pure over explicit values. Concrete root resolution, upstream
//! policy construction, probing, process execution, timeout/cancellation, and
//! cleanup remain in the Linux shell.

mod core;
mod model;
#[cfg(target_os = "linux")]
mod shell;
#[cfg(not(target_os = "linux"))]
mod unsupported;

pub use core::{
    evaluate_sandlock_preflight, execution_facts_hash, plan_sandlock_launch,
    project_sandlock_receipt, sandlock_cohort_hash, sandlock_command_hash,
    validate_sandlock_receipt_text, verify_sandlock_receipt,
};
pub use model::{
    SANDLOCK_COHORT_LICENSE, SANDLOCK_COHORT_REVISION, SANDLOCK_COHORT_VERSION,
    SANDLOCK_HOST_RECEIPT_SCHEMA, SANDLOCK_LAUNCH_PLAN_SCHEMA, SANDLOCK_RECEIPT_NON_CLAIMS,
    SandlockAuthorizationBinding, SandlockAuthorizationBindingDraft, SandlockAuthorizationDecision,
    SandlockCapability, SandlockCleanupStatus, SandlockCowOutcome, SandlockExecutionFacts,
    SandlockExecutionStatus, SandlockHostError, SandlockHostPolicy, SandlockHostPolicyDraft,
    SandlockHostReceipt, SandlockLaunchPlan, SandlockLaunchRequest, SandlockPreflightDecision,
    SandlockPreflightStatus, SandlockProbeReport, SandlockRootAccess, SandlockRootBinding,
};
#[cfg(target_os = "linux")]
pub use shell::{
    SandlockCancellation, SandlockCapabilityRootGrant, SandlockHostExecutionRequest,
    SandlockHostExecutionResult, SandlockMechanismProbe, SandlockResolvedRoot,
    execute_sandlock_host, resolve_sandlock_roots,
};
#[cfg(not(target_os = "linux"))]
pub use unsupported::{
    SandlockCancellation, SandlockCapabilityRootGrant, SandlockHostExecutionRequest,
    SandlockHostExecutionResult, SandlockMechanismProbe, SandlockResolvedRoot,
    execute_sandlock_host, resolve_sandlock_roots,
};

#[cfg(test)]
mod tests;
