use std::collections::BTreeSet;
use std::path::{Component, Path};

use animus_adapters::{
    SafeHash, SandboxCowDisposition, SandboxEnforcementRequirement, SandboxFilesystemMode,
    SandboxHttpPolicy, SandboxNetworkPolicy, SandboxProcessProfile,
};

use super::model::{
    SANDLOCK_COHORT_LICENSE, SANDLOCK_COHORT_REVISION, SANDLOCK_COHORT_VERSION,
    SANDLOCK_HOST_RECEIPT_SCHEMA, SANDLOCK_LAUNCH_PLAN_SCHEMA, SandlockAuthorizationDecision,
    SandlockCapability, SandlockCleanupStatus, SandlockCowOutcome, SandlockExecutionFacts,
    SandlockExecutionStatus, SandlockHostError, SandlockHostPolicyDraft, SandlockHostReceipt,
    SandlockHostReceiptParts, SandlockLaunchPlan, SandlockLaunchPlanParts, SandlockLaunchRequest,
    SandlockPreflightDecision, SandlockPreflightStatus, SandlockProbeReport, SandlockRootAccess,
    SandlockRootBinding, descriptor_filesystem_mode, descriptor_has_minimal_environment,
    descriptor_network_policy, descriptor_requires_cow, descriptor_requires_determinism,
    descriptor_requires_http, descriptor_requires_resources,
};

const HASH_DOMAIN_COHORT: &str = "animus.sandlock-cohort.v1";
const HASH_DOMAIN_POLICY: &str = "animus.sandlock-host-policy.v1";
const HASH_DOMAIN_COMMAND: &str = "animus.sandlock-command.v1";
const HASH_DOMAIN_PLAN: &str = "animus.sandlock-launch-plan.v1";
const HASH_DOMAIN_PREFLIGHT: &str = "animus.sandlock-preflight.v1";
const HASH_DOMAIN_EXECUTION: &str = "animus.sandlock-execution.v1";
const HASH_DOMAIN_RECEIPT: &str = "animus.sandlock-host-receipt.v1";
const HASH_FIELD_SEPARATOR: &[u8] = b"\0";
const MAX_SAFE_LABEL_BYTES: usize = 128;
const MAX_CPU_PERCENT: u8 = 100;
const EMPTY_COUNT: u64 = 0;
const MAX_RULE_BYTES: usize = 512;
const RECEIPT_LEAK_MARKERS: &[&str] = &[
    "raw_prompt",
    "api_key=",
    "authorization: bearer",
    "/home/",
    "/tmp/",
    "raw_network",
    "raw_file",
    "raw_process_output",
    "sandlock_debug",
    "unbounded_events",
    "stdout_body",
    "stderr_body",
];

#[derive(Clone, Copy)]
pub(crate) struct HostPolicyHashInput<'a> {
    pub(crate) draft: &'a SandlockHostPolicyDraft<'a>,
    pub(crate) egress_allow: &'a [String],
    pub(crate) http_allow: &'a [String],
    pub(crate) http_deny: &'a [String],
}

/// Computes the exact BLAKE3 identity of a command without retaining its body.
///
/// # Errors
///
/// Returns [`SandlockHostError::InvalidCommand`] for an empty command, empty
/// argument, or argument containing NUL.
pub fn sandlock_command_hash(command: &[&str]) -> Result<SafeHash, SandlockHostError> {
    if command.is_empty()
        || command
            .iter()
            .any(|argument| argument.is_empty() || argument.contains('\0'))
    {
        return Err(SandlockHostError::InvalidCommand);
    }
    let mut hasher = domain_hasher(HASH_DOMAIN_COMMAND);
    hash_count(&mut hasher, command.len());
    for argument in command {
        hash_text(&mut hasher, argument);
    }
    Ok(finish_hash(&hasher))
}

/// Returns the exact reviewed Sandlock cohort identity.
#[must_use]
pub fn sandlock_cohort_hash() -> SafeHash {
    let mut hasher = domain_hasher(HASH_DOMAIN_COHORT);
    hash_text(&mut hasher, SANDLOCK_COHORT_VERSION);
    hash_text(&mut hasher, SANDLOCK_COHORT_REVISION);
    hash_text(&mut hasher, SANDLOCK_COHORT_LICENSE);
    finish_hash(&hasher)
}

/// Purely validates authorization, roots, policy, and capability requirements.
///
/// # Errors
///
/// Returns [`SandlockHostError`] when any exact identity differs, enforcement is
/// best-effort, a root escapes its grant, or requested constraints are not fully
/// representable by the reviewed cohort.
pub fn plan_sandlock_launch(
    request: SandlockLaunchRequest<'_>,
) -> Result<SandlockLaunchPlan, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.translation]
    // r[impl agent_harness_runtime.sandlock_host_adapter.authority]
    reject_best_effort(request.descriptor.enforcement_requirement())?;
    validate_authorization(&request)?;
    validate_root_bindings(request.descriptor, request.policy, request.root_bindings)?;
    validate_policy_compatibility(request.descriptor, request.policy)?;
    let required_capabilities = required_capabilities(request.descriptor);
    let cohort_hash = sandlock_cohort_hash();
    let plan_hash = launch_plan_hash(&request, &required_capabilities, &cohort_hash);
    Ok(SandlockLaunchPlan::from_parts(SandlockLaunchPlanParts {
        descriptor: request.descriptor,
        policy: request.policy,
        authorization: request.authorization,
        command_hash: request.command_hash,
        root_bindings: request.root_bindings,
        required_capabilities,
        cohort_hash,
        plan_hash,
    }))
}

/// Purely checks exact plan/cohort binding and complete requested mechanism coverage.
///
/// # Errors
///
/// Returns [`SandlockHostError`] when the probe targets another plan/cohort or
/// does not prove every requested mechanism.
pub fn evaluate_sandlock_preflight(
    plan: &SandlockLaunchPlan,
    report: &SandlockProbeReport,
) -> Result<SandlockPreflightDecision, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.preflight]
    if report.plan_hash() != plan.plan_hash() || report.cohort_hash() != plan.cohort_hash() {
        return Err(SandlockHostError::ProbeIdentityMismatch);
    }
    let proven = report
        .proven_capabilities()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let missing = plan
        .required_capabilities()
        .iter()
        .copied()
        .filter(|capability| !proven.contains(capability))
        .collect::<Vec<_>>();
    let status = if missing.is_empty() {
        SandlockPreflightStatus::Ready
    } else {
        SandlockPreflightStatus::Blocked
    };
    let preflight_hash = preflight_hash(plan, report, missing.as_slice(), status);
    Ok(SandlockPreflightDecision::new(
        status,
        missing,
        preflight_hash,
    ))
}

/// Projects exact safe host facts into a metadata-only Sandlock receipt.
///
/// # Errors
///
/// Returns [`SandlockHostError`] if preflight was not ready, execution evidence
/// contradicts the plan, output bounds were exceeded, or receipt self-verification
/// fails.
pub fn project_sandlock_receipt(
    plan: &SandlockLaunchPlan,
    preflight: &SandlockPreflightDecision,
    facts: &SandlockExecutionFacts,
) -> Result<SandlockHostReceipt, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.receipts]
    if !preflight.is_ready() {
        return Err(SandlockHostError::ExecutionEvidenceMismatch);
    }
    validate_execution_facts(plan, facts)?;
    let parts = SandlockHostReceiptParts {
        plan,
        preflight,
        facts,
    };
    let receipt_hash = receipt_hash(&parts);
    let receipt = SandlockHostReceipt::from_parts(parts, receipt_hash);
    verify_sandlock_receipt(&receipt)?;
    Ok(receipt)
}

/// Recomputes and verifies a Sandlock host receipt identity.
///
/// # Errors
///
/// Returns [`SandlockHostError::ExecutionEvidenceMismatch`] when the receipt
/// does not carry its canonical BLAKE3 identity.
pub fn verify_sandlock_receipt(receipt: &SandlockHostReceipt) -> Result<(), SandlockHostError> {
    let expected = receipt_hash_from_receipt(receipt);
    if &expected == receipt.receipt_hash() {
        Ok(())
    } else {
        Err(SandlockHostError::ExecutionEvidenceMismatch)
    }
}

/// Rejects unsafe rendered receipt evidence before persistence.
///
/// # Errors
///
/// Returns [`SandlockHostError::UnsafeReceiptEvidence`] when raw prompts,
/// credentials, host paths, payloads, process bodies, debug logs, or unbounded
/// events are present.
pub fn validate_sandlock_receipt_text(text: &str) -> Result<(), SandlockHostError> {
    let normalized = text.to_ascii_lowercase();
    if RECEIPT_LEAK_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        Err(SandlockHostError::UnsafeReceiptEvidence)
    } else {
        Ok(())
    }
}

pub(crate) fn validate_safe_label(
    field: &'static str,
    value: &str,
) -> Result<(), SandlockHostError> {
    let is_safe = !value.is_empty()
        && value.len() <= MAX_SAFE_LABEL_BYTES
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
        && !value.contains("..")
        && !value.starts_with('/');
    if is_safe {
        Ok(())
    } else {
        Err(SandlockHostError::UnsafeLabel { field })
    }
}

pub(crate) fn validate_relative_path(value: &str) -> Result<(), SandlockHostError> {
    let path = Path::new(value);
    let safe = !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir));
    if safe {
        Ok(())
    } else {
        Err(SandlockHostError::UnsafeRelativeRoot)
    }
}

pub(crate) fn canonical_strings(values: &[&str]) -> Result<Vec<String>, SandlockHostError> {
    let mut seen = BTreeSet::new();
    let mut canonical = Vec::with_capacity(values.len());
    for value in values {
        let safe = !value.is_empty()
            && value.len() <= MAX_RULE_BYTES
            && !value.chars().any(char::is_control);
        if !safe {
            return Err(SandlockHostError::InvalidPolicy {
                reason: "unsafe_rule",
            });
        }
        if !seen.insert(*value) {
            return Err(SandlockHostError::InvalidPolicy {
                reason: "duplicate_rule",
            });
        }
        canonical.push((*value).to_owned());
    }
    Ok(canonical)
}

pub(crate) fn validate_policy_draft(
    draft: &SandlockHostPolicyDraft<'_>,
) -> Result<(), SandlockHostError> {
    validate_safe_label("policy_id", draft.policy_id)?;
    if draft.timeout_millis == EMPTY_COUNT || draft.max_output_bytes == EMPTY_COUNT {
        return Err(SandlockHostError::InvalidPolicy {
            reason: "zero_bound",
        });
    }
    if draft
        .max_cpu_percent
        .is_some_and(|value| value > MAX_CPU_PERCENT)
    {
        return Err(SandlockHostError::InvalidPolicy {
            reason: "invalid_cpu_percent",
        });
    }
    for value in [
        draft.max_memory_bytes,
        draft.max_processes.map(u64::from),
        draft.max_open_files.map(u64::from),
        draft.max_cpu_percent.map(u64::from),
    ] {
        if value == Some(EMPTY_COUNT) {
            return Err(SandlockHostError::InvalidPolicy {
                reason: "zero_resource_bound",
            });
        }
    }
    if let Some(label) = draft.cow_workdir_root {
        validate_safe_label("cow_workdir_root", label)?;
    }
    let _ = canonical_strings(draft.egress_allow)?;
    let _ = canonical_strings(draft.http_allow)?;
    let _ = canonical_strings(draft.http_deny)?;
    Ok(())
}

pub(crate) fn host_policy_hash(input: HostPolicyHashInput<'_>) -> SafeHash {
    let draft = input.draft;
    let mut hasher = domain_hasher(HASH_DOMAIN_POLICY);
    hash_text(&mut hasher, draft.policy_id);
    hash_u64(&mut hasher, draft.timeout_millis);
    hash_u64(&mut hasher, draft.max_output_bytes);
    hash_optional_u64(&mut hasher, draft.max_memory_bytes);
    hash_optional_u32(&mut hasher, draft.max_processes);
    hash_optional_u32(&mut hasher, draft.max_open_files);
    hash_optional_u8(&mut hasher, draft.max_cpu_percent);
    hash_strings(&mut hasher, input.egress_allow);
    hash_strings(&mut hasher, input.http_allow);
    hash_strings(&mut hasher, input.http_deny);
    hash_optional_u64(&mut hasher, draft.deterministic_epoch_seconds);
    hash_optional_u64(&mut hasher, draft.deterministic_seed);
    hash_optional_text(&mut hasher, draft.cow_workdir_root);
    finish_hash(&hasher)
}

fn reject_best_effort(requirement: SandboxEnforcementRequirement) -> Result<(), SandlockHostError> {
    if requirement == SandboxEnforcementRequirement::Required {
        Ok(())
    } else {
        Err(SandlockHostError::BestEffortUnsupported)
    }
}

fn validate_authorization(request: &SandlockLaunchRequest<'_>) -> Result<(), SandlockHostError> {
    let authorization = request.authorization;
    if authorization.decision() != SandlockAuthorizationDecision::Allowed {
        return Err(SandlockHostError::AuthorizationDenied);
    }
    if !authorization.is_current() {
        return Err(SandlockHostError::AuthorizationStale);
    }
    exact_hash(
        authorization.action_hash(),
        request.action_hash,
        "action_hash",
    )?;
    exact_hash(
        authorization.subject_hash(),
        request.subject_hash,
        "subject_hash",
    )?;
    exact_hash(
        authorization.target_hash(),
        request.target_hash,
        "target_hash",
    )?;
    exact_hash(
        authorization.descriptor_hash(),
        request.descriptor.descriptor_hash(),
        "descriptor_hash",
    )?;
    exact_hash(
        authorization.policy_hash(),
        request.policy.policy_hash(),
        "policy_hash",
    )?;
    exact_hash(
        authorization.command_hash(),
        request.command_hash,
        "command_hash",
    )
}

fn exact_hash(
    observed: &SafeHash,
    expected: &SafeHash,
    field: &'static str,
) -> Result<(), SandlockHostError> {
    if observed == expected {
        Ok(())
    } else {
        Err(SandlockHostError::AuthorizationMismatch { field })
    }
}

fn validate_root_bindings(
    descriptor: &animus_adapters::SandboxDescriptor,
    policy: &super::model::SandlockHostPolicy,
    bindings: &[SandlockRootBinding],
) -> Result<(), SandlockHostError> {
    if bindings.is_empty() {
        return Err(SandlockHostError::MissingWritableRoot);
    }
    let mut labels = BTreeSet::new();
    for binding in bindings {
        validate_relative_path(binding.relative_path())?;
        if !labels.insert(binding.label()) {
            return Err(SandlockHostError::DuplicateRootBinding);
        }
    }
    let requested_writable = descriptor
        .writable_roots()
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for requested in &requested_writable {
        let Some(binding) = bindings
            .iter()
            .find(|binding| binding.label() == *requested)
        else {
            return Err(SandlockHostError::MissingWritableRoot);
        };
        if binding.access() != SandlockRootAccess::ReadWrite {
            return Err(SandlockHostError::MissingWritableRoot);
        }
    }
    for binding in bindings {
        if binding.access() == SandlockRootAccess::ReadWrite
            && !requested_writable.contains(binding.label())
        {
            return Err(SandlockHostError::UnexpectedWritableRoot);
        }
    }
    if descriptor_filesystem_mode(descriptor) != SandboxFilesystemMode::WorkspaceWrite
        && !requested_writable.is_empty()
    {
        return Err(SandlockHostError::UnexpectedWritableRoot);
    }
    validate_cow_root(descriptor, policy, bindings)
}

fn validate_cow_root(
    descriptor: &animus_adapters::SandboxDescriptor,
    policy: &super::model::SandlockHostPolicy,
    bindings: &[SandlockRootBinding],
) -> Result<(), SandlockHostError> {
    if descriptor_requires_cow(descriptor) {
        let Some(cow_label) = policy.cow_workdir_root() else {
            return Err(SandlockHostError::InvalidCowRoot);
        };
        let valid = bindings.iter().any(|binding| {
            binding.label() == cow_label && binding.access() == SandlockRootAccess::ReadWrite
        });
        if !valid {
            return Err(SandlockHostError::InvalidCowRoot);
        }
    } else if policy.cow_workdir_root().is_some() {
        return Err(SandlockHostError::InvalidCowRoot);
    }
    Ok(())
}

fn validate_policy_compatibility(
    descriptor: &animus_adapters::SandboxDescriptor,
    policy: &super::model::SandlockHostPolicy,
) -> Result<(), SandlockHostError> {
    if descriptor.cow_disposition() == SandboxCowDisposition::Commit {
        return Err(SandlockHostError::InvalidPolicy {
            reason: "cow_commit_not_auditable",
        });
    }
    if !descriptor_has_minimal_environment(descriptor) {
        return Err(SandlockHostError::InvalidPolicy {
            reason: "allowlisted_environment_not_supported",
        });
    }
    validate_network_policy(descriptor_network_policy(descriptor), policy)?;
    validate_http_policy(descriptor.http_policy(), policy)?;
    validate_resource_policy(descriptor.process_profile(), policy)?;
    validate_determinism_policy(descriptor, policy)
}

fn validate_network_policy(
    network: SandboxNetworkPolicy,
    policy: &super::model::SandlockHostPolicy,
) -> Result<(), SandlockHostError> {
    let rule_count = policy.egress_allow().len();
    let valid = match network {
        SandboxNetworkPolicy::EgressAllowlist => rule_count > 0,
        SandboxNetworkPolicy::DenyAll | SandboxNetworkPolicy::BindDeny => rule_count == 0,
    };
    if valid {
        Ok(())
    } else {
        Err(SandlockHostError::InvalidPolicy {
            reason: "network_rule_mismatch",
        })
    }
}

fn validate_http_policy(
    http: SandboxHttpPolicy,
    policy: &super::model::SandlockHostPolicy,
) -> Result<(), SandlockHostError> {
    let has_rules = !policy.http_allow().is_empty() || !policy.http_deny().is_empty();
    let valid = match http {
        SandboxHttpPolicy::None => !has_rules,
        SandboxHttpPolicy::MethodHostPathAllowlist => !policy.http_allow().is_empty(),
    };
    if valid {
        Ok(())
    } else {
        Err(SandlockHostError::InvalidPolicy {
            reason: "http_rule_mismatch",
        })
    }
}

fn validate_resource_policy(
    process: SandboxProcessProfile,
    policy: &super::model::SandlockHostPolicy,
) -> Result<(), SandlockHostError> {
    let values = [
        policy.max_memory_bytes().map(|_| ()),
        policy.max_processes().map(|_| ()),
        policy.max_open_files().map(|_| ()),
        policy.max_cpu_percent().map(|_| ()),
    ];
    let all_present = values.iter().all(Option::is_some);
    let all_absent = values.iter().all(Option::is_none);
    let valid = match process {
        SandboxProcessProfile::Basic => all_absent,
        SandboxProcessProfile::ResourceLimited => all_present,
    };
    if valid {
        Ok(())
    } else {
        Err(SandlockHostError::InvalidPolicy {
            reason: "resource_bound_mismatch",
        })
    }
}

fn validate_determinism_policy(
    descriptor: &animus_adapters::SandboxDescriptor,
    policy: &super::model::SandlockHostPolicy,
) -> Result<(), SandlockHostError> {
    let has_epoch = policy.deterministic_epoch_seconds().is_some();
    let has_seed = policy.deterministic_seed().is_some();
    let valid = if descriptor_requires_determinism(descriptor) {
        has_epoch && has_seed
    } else {
        !has_epoch && !has_seed
    };
    if valid {
        Ok(())
    } else {
        Err(SandlockHostError::InvalidPolicy {
            reason: "determinism_value_mismatch",
        })
    }
}

fn required_capabilities(
    descriptor: &animus_adapters::SandboxDescriptor,
) -> Vec<SandlockCapability> {
    let mut capabilities = vec![
        SandlockCapability::Filesystem,
        SandlockCapability::SyscallFilter,
        SandlockCapability::Supervisor,
        SandlockCapability::Network,
        SandlockCapability::Environment,
    ];
    if descriptor_requires_cow(descriptor) {
        capabilities.push(SandlockCapability::Cow);
    }
    if descriptor_requires_resources(descriptor) {
        capabilities.push(SandlockCapability::Resources);
    }
    if descriptor_requires_http(descriptor) {
        capabilities.push(SandlockCapability::Http);
    }
    if descriptor_requires_determinism(descriptor) {
        capabilities.push(SandlockCapability::Determinism);
    }
    capabilities.sort();
    capabilities
}

fn launch_plan_hash(
    request: &SandlockLaunchRequest<'_>,
    capabilities: &[SandlockCapability],
    cohort_hash: &SafeHash,
) -> SafeHash {
    let mut hasher = domain_hasher(HASH_DOMAIN_PLAN);
    hash_text(&mut hasher, SANDLOCK_LAUNCH_PLAN_SCHEMA);
    hash_text(&mut hasher, request.descriptor.descriptor_hash().as_str());
    hash_text(&mut hasher, request.policy.policy_hash().as_str());
    hash_text(
        &mut hasher,
        request.authorization.authorization_hash().as_str(),
    );
    hash_text(&mut hasher, request.action_hash.as_str());
    hash_text(&mut hasher, request.subject_hash.as_str());
    hash_text(&mut hasher, request.target_hash.as_str());
    hash_text(&mut hasher, request.command_hash.as_str());
    hash_text(&mut hasher, cohort_hash.as_str());
    hash_count(&mut hasher, request.root_bindings.len());
    for binding in request.root_bindings {
        hash_text(&mut hasher, binding.label());
        hash_text(&mut hasher, binding.grant_hash().as_str());
        hash_text(&mut hasher, binding.relative_path());
        hash_text(&mut hasher, binding.access().as_str());
    }
    hash_capabilities(&mut hasher, capabilities);
    finish_hash(&hasher)
}

fn preflight_hash(
    plan: &SandlockLaunchPlan,
    report: &SandlockProbeReport,
    missing: &[SandlockCapability],
    status: SandlockPreflightStatus,
) -> SafeHash {
    let mut hasher = domain_hasher(HASH_DOMAIN_PREFLIGHT);
    hash_text(&mut hasher, plan.plan_hash().as_str());
    hash_text(&mut hasher, plan.cohort_hash().as_str());
    hash_text(&mut hasher, report.probe_hash().as_str());
    hash_text(
        &mut hasher,
        if status == SandlockPreflightStatus::Ready {
            "ready"
        } else {
            "blocked"
        },
    );
    hash_capabilities(&mut hasher, plan.required_capabilities());
    hash_capabilities(&mut hasher, report.proven_capabilities());
    hash_capabilities(&mut hasher, missing);
    finish_hash(&hasher)
}

fn validate_execution_facts(
    plan: &SandlockLaunchPlan,
    facts: &SandlockExecutionFacts,
) -> Result<(), SandlockHostError> {
    let combined_output = facts.stdout_bytes.saturating_add(facts.stderr_bytes);
    if combined_output > plan.policy().max_output_bytes() {
        return Err(SandlockHostError::ExecutionEvidenceMismatch);
    }
    let expected_execution_hash = execution_facts_hash(plan, facts);
    if expected_execution_hash != facts.execution_hash {
        return Err(SandlockHostError::ExecutionEvidenceMismatch);
    }
    let enforced = facts
        .enforced_capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if enforced.len() != facts.enforced_capabilities.len() {
        return Err(SandlockHostError::ExecutionEvidenceMismatch);
    }
    if facts.status == SandlockExecutionStatus::Passed {
        let required = plan
            .required_capabilities()
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if enforced != required || facts.cleanup != SandlockCleanupStatus::Complete {
            return Err(SandlockHostError::ExecutionEvidenceMismatch);
        }
    }
    validate_cow_facts(plan, facts)
}

fn validate_cow_facts(
    plan: &SandlockLaunchPlan,
    facts: &SandlockExecutionFacts,
) -> Result<(), SandlockHostError> {
    let disposition = plan.descriptor().cow_disposition();
    let valid = match disposition {
        SandboxCowDisposition::Disabled => facts.cow == SandlockCowOutcome::NotRequested,
        SandboxCowDisposition::DryRun => facts.cow == SandlockCowOutcome::Discarded,
        SandboxCowDisposition::Commit => {
            if facts.status == SandlockExecutionStatus::Passed {
                facts.cow == SandlockCowOutcome::Committed
            } else {
                matches!(
                    facts.cow,
                    SandlockCowOutcome::Discarded | SandlockCowOutcome::Failed
                )
            }
        }
    };
    if valid {
        Ok(())
    } else {
        Err(SandlockHostError::ExecutionEvidenceMismatch)
    }
}

/// Computes the execution-facts identity expected by receipt projection.
#[must_use]
pub fn execution_facts_hash(plan: &SandlockLaunchPlan, facts: &SandlockExecutionFacts) -> SafeHash {
    let mut hasher = domain_hasher(HASH_DOMAIN_EXECUTION);
    hash_text(&mut hasher, plan.plan_hash().as_str());
    hash_text(&mut hasher, facts.status.as_str());
    hash_optional_i32(&mut hasher, facts.exit_code);
    hash_optional_i32(&mut hasher, facts.signal);
    hash_u64(&mut hasher, facts.stdout_bytes);
    hash_u64(&mut hasher, facts.stderr_bytes);
    hash_optional_hash(&mut hasher, facts.stdout_hash.as_ref());
    hash_optional_hash(&mut hasher, facts.stderr_hash.as_ref());
    hash_text(&mut hasher, facts.cow.as_str());
    hash_optional_u32(&mut hasher, facts.cow_change_count);
    hash_optional_hash(&mut hasher, facts.cow_digest.as_ref());
    hash_text(&mut hasher, facts.cleanup.as_str());
    hash_capabilities(&mut hasher, facts.enforced_capabilities.as_slice());
    finish_hash(&hasher)
}

fn receipt_hash(parts: &SandlockHostReceiptParts<'_>) -> SafeHash {
    let mut hasher = receipt_hasher(
        parts.facts.status,
        parts.plan.descriptor().descriptor_hash(),
        parts.plan.policy().policy_hash(),
        parts.plan.authorization_hash(),
        parts.plan.plan_hash(),
        parts.plan.cohort_hash(),
        parts.preflight.preflight_hash(),
        &parts.facts.execution_hash,
    );
    hash_capabilities(&mut hasher, parts.plan.required_capabilities());
    hash_capabilities(&mut hasher, parts.facts.enforced_capabilities.as_slice());
    hash_execution_tail(&mut hasher, parts.facts);
    finish_hash(&hasher)
}

fn receipt_hash_from_receipt(receipt: &SandlockHostReceipt) -> SafeHash {
    let mut hasher = receipt_hasher(
        receipt.status(),
        receipt.descriptor_hash(),
        receipt.policy_hash(),
        receipt.authorization_hash(),
        receipt.plan_hash(),
        receipt.cohort_hash(),
        receipt.preflight_hash(),
        receipt.execution_hash(),
    );
    hash_strings(&mut hasher, receipt.requested_capabilities());
    hash_strings(&mut hasher, receipt.enforced_capabilities());
    hash_u64(&mut hasher, receipt.stdout_bytes());
    hash_u64(&mut hasher, receipt.stderr_bytes());
    hash_optional_hash(&mut hasher, receipt.stdout_hash());
    hash_optional_hash(&mut hasher, receipt.stderr_hash());
    hash_text(&mut hasher, receipt.cow_outcome().as_str());
    hash_optional_u32(&mut hasher, receipt.cow_change_count());
    hash_optional_hash(&mut hasher, receipt.cow_digest());
    hash_optional_i32(&mut hasher, receipt.exit_code());
    hash_optional_i32(&mut hasher, receipt.signal());
    hash_text(&mut hasher, receipt.cleanup_status().as_str());
    finish_hash(&hasher)
}

#[allow(clippy::too_many_arguments)]
fn receipt_hasher(
    status: SandlockExecutionStatus,
    descriptor_hash: &SafeHash,
    policy_hash: &SafeHash,
    authorization_hash: &SafeHash,
    plan_hash: &SafeHash,
    cohort_hash: &SafeHash,
    preflight_hash: &SafeHash,
    execution_hash: &SafeHash,
) -> blake3::Hasher {
    let mut hasher = domain_hasher(HASH_DOMAIN_RECEIPT);
    hash_text(&mut hasher, SANDLOCK_HOST_RECEIPT_SCHEMA);
    hash_text(&mut hasher, status.as_str());
    hash_text(&mut hasher, descriptor_hash.as_str());
    hash_text(&mut hasher, policy_hash.as_str());
    hash_text(&mut hasher, authorization_hash.as_str());
    hash_text(&mut hasher, plan_hash.as_str());
    hash_text(&mut hasher, cohort_hash.as_str());
    hash_text(&mut hasher, preflight_hash.as_str());
    hash_text(&mut hasher, execution_hash.as_str());
    hasher
}

fn hash_execution_tail(hasher: &mut blake3::Hasher, facts: &SandlockExecutionFacts) {
    hash_u64(hasher, facts.stdout_bytes);
    hash_u64(hasher, facts.stderr_bytes);
    hash_optional_hash(hasher, facts.stdout_hash.as_ref());
    hash_optional_hash(hasher, facts.stderr_hash.as_ref());
    hash_text(hasher, facts.cow.as_str());
    hash_optional_u32(hasher, facts.cow_change_count);
    hash_optional_hash(hasher, facts.cow_digest.as_ref());
    hash_optional_i32(hasher, facts.exit_code);
    hash_optional_i32(hasher, facts.signal);
    hash_text(hasher, facts.cleanup.as_str());
}

fn domain_hasher(domain: &str) -> blake3::Hasher {
    let mut hasher = blake3::Hasher::new();
    hash_text(&mut hasher, domain);
    hasher
}

fn hash_text(hasher: &mut blake3::Hasher, value: &str) {
    hasher.update(value.as_bytes());
    hasher.update(HASH_FIELD_SEPARATOR);
}

fn hash_u64(hasher: &mut blake3::Hasher, value: u64) {
    hash_text(hasher, value.to_string().as_str());
}

fn hash_count(hasher: &mut blake3::Hasher, value: usize) {
    hash_text(hasher, value.to_string().as_str());
}

fn hash_optional_text(hasher: &mut blake3::Hasher, value: Option<&str>) {
    hash_text(hasher, value.unwrap_or("none"));
}

fn hash_optional_hash(hasher: &mut blake3::Hasher, value: Option<&SafeHash>) {
    hash_optional_text(hasher, value.map(SafeHash::as_str));
}

fn hash_optional_u64(hasher: &mut blake3::Hasher, value: Option<u64>) {
    hash_text(
        hasher,
        value
            .map_or_else(|| "none".to_owned(), |number| number.to_string())
            .as_str(),
    );
}

fn hash_optional_u32(hasher: &mut blake3::Hasher, value: Option<u32>) {
    hash_optional_u64(hasher, value.map(u64::from));
}

fn hash_optional_u8(hasher: &mut blake3::Hasher, value: Option<u8>) {
    hash_optional_u64(hasher, value.map(u64::from));
}

fn hash_optional_i32(hasher: &mut blake3::Hasher, value: Option<i32>) {
    hash_text(
        hasher,
        value
            .map_or_else(|| "none".to_owned(), |number| number.to_string())
            .as_str(),
    );
}

fn hash_strings(hasher: &mut blake3::Hasher, values: &[String]) {
    hash_count(hasher, values.len());
    for value in values {
        hash_text(hasher, value);
    }
}

fn hash_capabilities(hasher: &mut blake3::Hasher, values: &[SandlockCapability]) {
    hash_count(hasher, values.len());
    for value in values {
        hash_text(hasher, value.as_str());
    }
}

fn finish_hash(hasher: &blake3::Hasher) -> SafeHash {
    SafeHash::from_blake3_bytes(hasher.finalize().as_bytes())
}
