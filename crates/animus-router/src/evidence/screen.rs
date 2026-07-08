#![allow(non_trait_imports)]

use alloc::string::String;
use alloc::vec::Vec;

/// Materialized router execution plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouterExecutionPlan {
    pub(super) recipe_id: String,
    pub(super) family: crate::RouteFamily,
    pub(super) selected_models: Vec<String>,
    pub(super) output_contract: crate::OutputContract,
    pub(super) bounds: crate::Bounds,
    pub(super) privacy: crate::PrivacyBand,
    pub(super) fallback_eligibility: crate::FallbackEligibility,
    pub(super) role_steps: Vec<RoleStep>,
    pub(super) remaining_synthesis_attempts: u32,
    pub(super) remaining_repair_attempts: u32,
    pub(super) fallback_chain: FallbackChain,
}

impl RouterExecutionPlan {
    /// Returns the selected recipe identifier.
    #[must_use]
    pub fn recipe_id(&self) -> &str {
        self.recipe_id.as_str()
    }

    /// Returns the route family.
    #[must_use]
    pub const fn family(&self) -> crate::RouteFamily {
        self.family
    }

    /// Returns selected safe model labels.
    #[must_use]
    pub fn selected_models(&self) -> &[String] {
        self.selected_models.as_slice()
    }

    /// Returns the final output contract.
    #[must_use]
    pub const fn output_contract(&self) -> crate::OutputContract {
        self.output_contract
    }

    /// Returns the validated execution bounds.
    #[must_use]
    pub const fn bounds(&self) -> &crate::Bounds {
        &self.bounds
    }

    /// Returns the projected privacy band used for step eligibility.
    #[must_use]
    pub const fn privacy(&self) -> crate::PrivacyBand {
        self.privacy
    }

    /// Returns the projected fallback eligibility band.
    #[must_use]
    pub const fn fallback_eligibility(&self) -> crate::FallbackEligibility {
        self.fallback_eligibility
    }

    /// Returns ordered role steps.
    #[must_use]
    pub fn role_steps(&self) -> &[RoleStep] {
        self.role_steps.as_slice()
    }

    /// Returns remaining synthesis attempts.
    #[must_use]
    pub const fn remaining_synthesis_attempts(&self) -> u32 {
        self.remaining_synthesis_attempts
    }

    /// Returns remaining repair attempts.
    #[must_use]
    pub const fn remaining_repair_attempts(&self) -> u32 {
        self.remaining_repair_attempts
    }

    /// Returns the ordered fallback chain.
    #[must_use]
    pub fn fallback_chain(&self) -> &FallbackChain {
        &self.fallback_chain
    }
}

/// One ordered role step in a materialized router execution plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleStep {
    role: crate::Role,
    candidate_labels: Vec<String>,
    parallelism_cap: u32,
    quorum: u32,
}

impl RoleStep {
    /// Returns the role for this step.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns the ordered candidate labels for this step.
    #[must_use]
    pub fn candidate_labels(&self) -> &[String] {
        self.candidate_labels.as_slice()
    }

    /// Returns the parallelism cap for this step.
    #[must_use]
    pub const fn parallelism_cap(&self) -> u32 {
        self.parallelism_cap
    }

    /// Returns the quorum threshold for this step.
    #[must_use]
    pub const fn quorum(&self) -> u32 {
        self.quorum
    }
}

/// Ordered fallback chain materialized from the catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FallbackChain {
    pub(super) steps: Vec<FallbackStep>,
}

impl FallbackChain {
    /// Returns the ordered fallback steps.
    #[must_use]
    pub fn steps(&self) -> &[FallbackStep] {
        self.steps.as_slice()
    }
}

/// One fallback step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FallbackStep {
    pub(super) role: crate::Role,
    pub(super) model_label: String,
    pub(super) backend: String,
    pub(super) local: bool,
}

impl FallbackStep {
    /// Returns the fallback role.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns the fallback model label.
    #[must_use]
    pub fn model_label(&self) -> &str {
        self.model_label.as_str()
    }

    /// Returns the fallback backend label.
    #[must_use]
    pub fn backend(&self) -> &str {
        self.backend.as_str()
    }

    /// Returns whether the fallback model is local-only.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.local
    }
}

/// Typed router execution-plan denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionPlanError {
    /// The quorum threshold was zero.
    ZeroQuorum,
    /// The quorum threshold exceeded the available candidate labels.
    ImpossibleQuorum,
    /// Collaborative repair budget was not available.
    UnboundedRepair,
    /// A recipe role could not be mapped to a selected model step.
    MissingRoleMapping,
    /// The fallback chain revisited a model already used in the route.
    CyclicFallback,
    /// No fallback model satisfied backend or privacy constraints.
    FallbackModelNotEligible,
}

/// Materializes a deterministic router execution plan.
// r[impl agent_harness_runtime.model_router.execution_plan]
pub fn materialize_execution_plan(
    selected: &crate::SelectedPlan,
    catalog: &[crate::CatalogEntry],
    budget: &crate::Budget,
) -> Result<RouterExecutionPlan, ExecutionPlanError> {
    let roles = super::label::roles_for_family(selected.family());
    validate_plan_constraints(selected, roles.len())?;
    let (role_steps, route_labels) = build_role_steps(selected, catalog, budget, roles)?;
    let fallback_chain =
        super::label::build_fallback_chain(selected, catalog, budget, &route_labels)?;

    Ok(super::record::assemble_execution_plan(
        selected,
        role_steps,
        fallback_chain,
    ))
}

fn validate_plan_constraints(
    selected: &crate::SelectedPlan,
    roles_len: usize,
) -> Result<(), ExecutionPlanError> {
    assert!(roles_len > 0, "role mapping must exist");
    assert!(
        selected.bounds().max_candidates() > crate::limit::ZERO_COUNT,
        "candidate cap must be positive"
    );
    assert!(
        selected.bounds().max_parallelism() > crate::limit::ZERO_COUNT,
        "parallelism cap must be positive"
    );

    if selected.bounds().min_success_quorum() == crate::limit::ZERO_COUNT {
        return Err(ExecutionPlanError::ZeroQuorum);
    }

    if selected.family() != crate::RouteFamily::Single
        && selected.bounds().max_repair_attempts() == crate::limit::ZERO_COUNT
    {
        return Err(ExecutionPlanError::UnboundedRepair);
    }

    if selected.selected_models().len() != roles_len {
        return Err(ExecutionPlanError::MissingRoleMapping);
    }

    Ok(())
}

fn build_role_steps(
    selected: &crate::SelectedPlan,
    catalog: &[crate::CatalogEntry],
    budget: &crate::Budget,
    roles: &[crate::Role],
) -> Result<(Vec<RoleStep>, Vec<String>), ExecutionPlanError> {
    assert!(roles.len() > 0, "role mapping must exist");
    assert!(
        selected.bounds().max_candidates() > crate::limit::ZERO_COUNT,
        "candidate cap must be positive"
    );
    assert!(
        selected.bounds().max_parallelism() > crate::limit::ZERO_COUNT,
        "parallelism cap must be positive"
    );
    let mut role_steps = Vec::with_capacity(roles.len());
    let mut route_labels = Vec::with_capacity(roles.len());
    for (index, role) in roles.iter().copied().enumerate() {
        let selected_label = match selected.selected_models().get(index) {
            Some(label) => label.as_str(),
            None => return Err(ExecutionPlanError::MissingRoleMapping),
        };
        let candidate_labels =
            super::label::collect_role_candidates(selected, selected_label, role, catalog, budget)?;
        let quorum = selected.bounds().min_success_quorum();
        let quorum_index = match usize::try_from(quorum) {
            Ok(value) => value,
            Err(_) => return Err(ExecutionPlanError::ImpossibleQuorum),
        };
        if quorum_index > candidate_labels.len() {
            return Err(ExecutionPlanError::ImpossibleQuorum);
        }
        route_labels.extend(candidate_labels.iter().cloned());
        role_steps.push(RoleStep {
            role,
            candidate_labels,
            parallelism_cap: selected.bounds().max_parallelism(),
            quorum,
        });
    }

    Ok((role_steps, route_labels))
}
