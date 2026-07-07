#![allow(non_trait_imports)]

const QUORUM_FIELD: crate::FieldName = crate::FieldName::new("router_quorum_class");
const DURATION_FIELD: crate::FieldName = crate::FieldName::new("router_duration_bucket");
const REDACTION_FIELD: crate::FieldName = crate::FieldName::new("router_redaction_marker");

const QUORUM_NONE: &str = "none";
const QUORUM_MET: &str = "met";
const QUORUM_MISSING: &str = "missing";
const DURATION_FAST: &str = "fast";
const DURATION_BOUNDED: &str = "bounded";
const DURATION_SLOW: &str = "slow";
const REDACTION_PROMPT_OMITTED: &str = "prompt_omitted";
const REDACTION_PROVIDER_PAYLOAD_OMITTED: &str = "provider_payload_omitted";
const REDACTION_WORKER_OUTPUT_OMITTED: &str = "worker_output_omitted";
const REDACTION_JUDGE_RATIONALE_OMITTED: &str = "judge_rationale_omitted";
const REDACTION_HIDDEN_REASONING_OMITTED: &str = "hidden_reasoning_omitted";
const REDACTION_CREDENTIAL_OMITTED: &str = "credential_omitted";
const REDACTION_PATH_REDACTED: &str = "path_redacted";
const REDACTION_TRACE_BOUNDED: &str = "trace_bounded";

/// Quorum class recorded in router receipts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumClass {
    /// Quorum was not required.
    None,
    /// Quorum was met.
    Met,
    /// Quorum was missing.
    Missing,
}

impl QuorumClass {
    /// Parses a quorum class.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] for unsupported labels.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(QUORUM_FIELD, value, |normalized| match normalized {
            QUORUM_NONE => Some(Self::None),
            QUORUM_MET => Some(Self::Met),
            QUORUM_MISSING => Some(Self::Missing),
            _ => None,
        })
    }

    /// Returns stable quorum label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => QUORUM_NONE,
            Self::Met => QUORUM_MET,
            Self::Missing => QUORUM_MISSING,
        }
    }
}

/// Duration bucket recorded in router receipts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurationBucket {
    /// Fast host-reported duration bucket.
    Fast,
    /// Bounded host-reported duration bucket.
    Bounded,
    /// Slow host-reported duration bucket.
    Slow,
}

impl DurationBucket {
    /// Parses a duration bucket.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] for unsupported labels.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(DURATION_FIELD, value, |normalized| match normalized {
            DURATION_FAST => Some(Self::Fast),
            DURATION_BOUNDED => Some(Self::Bounded),
            DURATION_SLOW => Some(Self::Slow),
            _ => None,
        })
    }

    /// Returns stable duration label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fast => DURATION_FAST,
            Self::Bounded => DURATION_BOUNDED,
            Self::Slow => DURATION_SLOW,
        }
    }
}

/// Router receipt redaction marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Redaction {
    /// Prompt text omitted.
    PromptOmitted,
    /// Provider payload omitted.
    ProviderPayloadOmitted,
    /// Worker output omitted.
    WorkerOutputOmitted,
    /// Judge rationale omitted.
    JudgeRationaleOmitted,
    /// Hidden reasoning omitted.
    HiddenReasoningOmitted,
    /// Credentials omitted.
    CredentialOmitted,
    /// Full paths redacted.
    PathRedacted,
    /// Trace details bounded.
    TraceBounded,
}

impl Redaction {
    /// Parses a redaction marker.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] for unsupported labels.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(REDACTION_FIELD, value, |normalized| match normalized {
            REDACTION_PROMPT_OMITTED => Some(Self::PromptOmitted),
            REDACTION_PROVIDER_PAYLOAD_OMITTED => Some(Self::ProviderPayloadOmitted),
            REDACTION_WORKER_OUTPUT_OMITTED => Some(Self::WorkerOutputOmitted),
            REDACTION_JUDGE_RATIONALE_OMITTED => Some(Self::JudgeRationaleOmitted),
            REDACTION_HIDDEN_REASONING_OMITTED => Some(Self::HiddenReasoningOmitted),
            REDACTION_CREDENTIAL_OMITTED => Some(Self::CredentialOmitted),
            REDACTION_PATH_REDACTED => Some(Self::PathRedacted),
            REDACTION_TRACE_BOUNDED => Some(Self::TraceBounded),
            _ => None,
        })
    }

    /// Returns stable redaction label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptOmitted => REDACTION_PROMPT_OMITTED,
            Self::ProviderPayloadOmitted => REDACTION_PROVIDER_PAYLOAD_OMITTED,
            Self::WorkerOutputOmitted => REDACTION_WORKER_OUTPUT_OMITTED,
            Self::JudgeRationaleOmitted => REDACTION_JUDGE_RATIONALE_OMITTED,
            Self::HiddenReasoningOmitted => REDACTION_HIDDEN_REASONING_OMITTED,
            Self::CredentialOmitted => REDACTION_CREDENTIAL_OMITTED,
            Self::PathRedacted => REDACTION_PATH_REDACTED,
            Self::TraceBounded => REDACTION_TRACE_BOUNDED,
        }
    }
}

const SINGLE_ROLES: &[crate::Role] = &[crate::Role::Primary];
const CONFIDENCE_ROLES: &[crate::Role] = &[crate::Role::Primary, crate::Role::Escalation];
const RATINGS_ROLES: &[crate::Role] = &[crate::Role::Primary, crate::Role::Rater];
const REMOM_ROLES: &[crate::Role] = &[crate::Role::Worker, crate::Role::Synthesizer];
const FUSION_ROLES: &[crate::Role] = &[crate::Role::Judge, crate::Role::Finalizer];
const WORKFLOW_ROLES: &[crate::Role] = &[crate::Role::Worker, crate::Role::Finalizer];

pub(super) fn roles_for_family(family: crate::RouteFamily) -> &'static [crate::Role] {
    match family {
        crate::RouteFamily::Single => SINGLE_ROLES,
        crate::RouteFamily::Confidence => CONFIDENCE_ROLES,
        crate::RouteFamily::Ratings => RATINGS_ROLES,
        crate::RouteFamily::Remom => REMOM_ROLES,
        crate::RouteFamily::Fusion => FUSION_ROLES,
        crate::RouteFamily::Workflow => WORKFLOW_ROLES,
    }
}

pub(super) fn collect_role_candidates(
    selected: &crate::SelectedPlan,
    selected_label: &str,
    role: crate::Role,
    catalog: &[crate::CatalogEntry],
    budget: &crate::Budget,
) -> Result<alloc::vec::Vec<alloc::string::String>, crate::ExecutionPlanError> {
    assert!(selected.bounds().max_candidates() > 0, "candidate cap must be positive");
    assert!(budget.max_candidates() > 0, "budget cap must be positive");
    let mut eligible_labels = alloc::vec::Vec::with_capacity(catalog.len());
    for entry in catalog {
        if entry.role() != role {
            continue;
        }
        if !plan_entry_allowed(entry, budget, selected.privacy()) {
            continue;
        }
        if eligible_labels.iter().any(|label| *label == entry.label()) {
            continue;
        }
        eligible_labels.push(entry.label());
    }

    if !eligible_labels.iter().any(|label| *label == selected_label) {
        return Err(crate::ExecutionPlanError::MissingRoleMapping);
    }

    let candidate_cap = match usize::try_from(selected.bounds().max_candidates()) {
        Ok(value) => value,
        Err(_) => return Err(crate::ExecutionPlanError::ImpossibleQuorum),
    };
    let mut candidate_labels = alloc::vec::Vec::with_capacity(candidate_cap);
    candidate_labels.push(alloc::string::String::from(selected_label));
    for label in eligible_labels {
        if candidate_labels.len() >= candidate_cap {
            break;
        }
        if label != selected_label {
            candidate_labels.push(alloc::string::String::from(label));
        }
    }

    Ok(candidate_labels)
}

pub(super) fn build_fallback_chain(
    selected: &crate::SelectedPlan,
    catalog: &[crate::CatalogEntry],
    budget: &crate::Budget,
    route_labels: &[alloc::string::String],
) -> Result<crate::FallbackChain, crate::ExecutionPlanError> {
    assert!(selected.bounds().max_candidates() > 0, "candidate cap must be positive");
    assert!(budget.max_candidates() > 0, "budget cap must be positive");
    if selected.family() == crate::RouteFamily::Single
        || selected.bounds().fallback_policy() == crate::FallbackPolicy::Deny
    {
        return Ok(crate::FallbackChain { steps: alloc::vec::Vec::new() });
    }

    if selected.fallback_eligibility() == crate::FallbackEligibility::Ineligible {
        return Err(crate::ExecutionPlanError::FallbackModelNotEligible);
    }

    let mut steps = alloc::vec::Vec::with_capacity(catalog.len());
    let mut fallback_labels = alloc::vec::Vec::with_capacity(catalog.len());
    let candidate_cap = match usize::try_from(selected.bounds().max_candidates()) {
        Ok(value) => value,
        Err(_) => return Err(crate::ExecutionPlanError::ImpossibleQuorum),
    };

    for entry in catalog {
        if entry.role() != crate::Role::Fallback {
            continue;
        }

        if !plan_entry_allowed(entry, budget, selected.privacy()) {
            continue;
        }

        if slice_contains_str(route_labels, entry.label())
            || slice_contains_str(&fallback_labels, entry.label())
        {
            return Err(crate::ExecutionPlanError::CyclicFallback);
        }

        if steps.len() >= candidate_cap {
            continue;
        }

        fallback_labels.push(alloc::string::String::from(entry.label()));
        steps.push(crate::FallbackStep {
            role: crate::Role::Fallback,
            model_label: alloc::string::String::from(entry.label()),
            backend: alloc::string::String::from(entry.backend()),
            local: entry.is_local(),
        });
    }

    if steps.is_empty() {
        return Err(crate::ExecutionPlanError::FallbackModelNotEligible);
    }

    Ok(crate::FallbackChain { steps })
}

fn slice_contains_str(labels: &[alloc::string::String], target: &str) -> bool {
    labels.iter().any(|label| label.as_str() == target)
}

fn plan_entry_allowed(
    entry: &crate::CatalogEntry,
    budget: &crate::Budget,
    privacy: crate::PrivacyBand,
) -> bool {
    budget.allows_backend(entry.backend())
        && (privacy == crate::PrivacyBand::RemoteAllowed || entry.is_local())
}
