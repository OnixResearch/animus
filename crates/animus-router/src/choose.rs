const MODEL_LABEL_FIELD: crate::FieldName = crate::FieldName::new("router_model_label");
const BACKEND_FIELD: crate::FieldName = crate::FieldName::new("router_backend");

/// Safe catalog projection used by recipe selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogEntry {
    label: alloc::string::String,
    backend: alloc::string::String,
    role: crate::Role,
    local: bool,
}

impl CatalogEntry {
    /// Builds a safe catalog projection.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when labels or roles are invalid.
    pub fn new(draft: CatalogEntryDraft<'_>) -> Result<Self, crate::AdapterContractError> {
        Ok(Self {
            label: crate::normalized_adapter_field(MODEL_LABEL_FIELD, draft.label)?,
            backend: crate::normalized_adapter_field(BACKEND_FIELD, draft.backend)?,
            role: crate::Role::parse(draft.role)?,
            local: draft.local,
        })
    }

    /// Returns safe model label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns safe backend label.
    #[must_use]
    pub fn backend(&self) -> &str {
        self.backend.as_str()
    }

    /// Returns catalog role.
    #[must_use]
    pub const fn role(&self) -> crate::Role {
        self.role
    }

    /// Returns whether this entry satisfies local-only privacy constraints.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.local
    }
}

/// Borrowed catalog projection fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatalogEntryDraft<'a> {
    /// Safe model label.
    pub label: &'a str,
    /// Safe backend label.
    pub backend: &'a str,
    /// Role label.
    pub role: &'a str,
    /// Whether the model satisfies local-only privacy constraints.
    pub local: bool,
}

/// Budget and backend eligibility used by recipe selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Budget {
    max_candidates: u32,
    allowed_backends: alloc::vec::Vec<alloc::string::String>,
}

impl Budget {
    /// Builds router budget state.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when backend labels are invalid.
    pub fn new(
        max_candidates: u32,
        allowed_backends: &[&str],
    ) -> Result<Self, crate::AdapterContractError> {
        let allowed_backends = allowed_backends
            .iter()
            .map(|backend| crate::normalized_adapter_field(BACKEND_FIELD, backend))
            .collect::<Result<alloc::vec::Vec<_>, _>>()?;
        Ok(Self {
            max_candidates,
            allowed_backends,
        })
    }

    pub(crate) fn allows_backend(&self, backend: &str) -> bool {
        self.allowed_backends
            .iter()
            .any(|allowed| allowed == backend)
    }

    /// Returns max candidates.
    #[must_use]
    pub const fn max_candidates(&self) -> u32 {
        self.max_candidates
    }
}

/// Selected recipe and safe model labels produced before provider dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedPlan {
    recipe_id: alloc::string::String,
    family: crate::RouteFamily,
    selected_models: alloc::vec::Vec<alloc::string::String>,
    output_contract: crate::OutputContract,
    bounds: crate::Bounds,
    privacy: crate::PrivacyBand,
    fallback_eligibility: crate::FallbackEligibility,
    rotation_decisions: alloc::vec::Vec<crate::RouterRotationDecision>,
}

impl SelectedPlan {
    /// Returns selected recipe identifier.
    #[must_use]
    pub fn recipe_id(&self) -> &str {
        self.recipe_id.as_str()
    }

    /// Returns selected route family.
    #[must_use]
    pub const fn family(&self) -> crate::RouteFamily {
        self.family
    }

    /// Returns selected safe model labels.
    #[must_use]
    pub fn selected_models(&self) -> &[alloc::string::String] {
        self.selected_models.as_slice()
    }

    /// Returns final output contract.
    #[must_use]
    pub const fn output_contract(&self) -> crate::OutputContract {
        self.output_contract
    }

    /// Returns execution bounds.
    #[must_use]
    pub const fn bounds(&self) -> &crate::Bounds {
        &self.bounds
    }

    /// Returns the projected privacy band.
    #[must_use]
    pub const fn privacy(&self) -> crate::PrivacyBand {
        self.privacy
    }

    /// Returns the projected fallback eligibility.
    #[must_use]
    pub const fn fallback_eligibility(&self) -> crate::FallbackEligibility {
        self.fallback_eligibility
    }

    /// Returns safe rotation decisions produced during selection.
    #[must_use]
    pub fn rotation_decisions(&self) -> &[crate::RouterRotationDecision] {
        self.rotation_decisions.as_slice()
    }
}

/// Typed router-selection denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionDenial {
    /// No recipe matched projected signals.
    NoRecipe,
    /// Budget limits reject the selected recipe.
    BudgetExceeded,
    /// No eligible safe model projection exists for a required role.
    NoEligibleModel,
    /// Host cursor state was stale and policy denied reset.
    StaleRotationCursor,
}

/// Selects a deterministic recipe plan before provider dispatch.
///
/// # Errors
///
/// Returns [`SelectionDenial`] when no recipe, budget, or model eligibility path exists.
///
/// # Panics
///
/// Panics only if a previously validated recipe loses its role list or candidate bound.
pub fn select_recipe(
    signals: &crate::Signals,
    recipes: &[crate::Recipe],
    catalog: &[CatalogEntry],
    budget: &Budget,
) -> Result<SelectedPlan, SelectionDenial> {
    let preferred_family = preferred_family(signals);
    let recipe = find_recipe(recipes, preferred_family)?;
    assert!(
        !recipe.roles().is_empty(),
        "validated recipe must keep roles"
    );
    assert!(
        recipe.bounds().max_candidates() > 0,
        "validated bounds must cap candidates"
    );
    if recipe.bounds().max_candidates() > budget.max_candidates {
        return Err(SelectionDenial::BudgetExceeded);
    }

    let selection = select_entries(SelectionRequest {
        recipe,
        catalog,
        budget,
        signals,
        rotation: None,
    })?;
    Ok(SelectedPlan {
        recipe_id: alloc::string::String::from(recipe.id()),
        family: recipe.family(),
        selected_models: selection.selected_models,
        output_contract: recipe.output_contract(),
        bounds: recipe.bounds().clone(),
        privacy: signals.privacy(),
        fallback_eligibility: signals.fallback_eligibility(),
        rotation_decisions: selection.rotation_decisions,
    })
}

/// Borrowed request used for rotation-aware recipe selection.
#[derive(Clone, Copy, Debug)]
pub struct RotatingSelectionRequest<'a> {
    /// Projected request signals.
    pub signals: &'a crate::Signals,
    /// Validated recipe catalog.
    pub recipes: &'a [crate::Recipe],
    /// Safe model catalog projection.
    pub catalog: &'a [CatalogEntry],
    /// Budget and backend allowlist.
    pub budget: &'a Budget,
    /// Host-owned rotation snapshot.
    pub rotation: &'a crate::RouterRotationSnapshot,
}

/// Selects a deterministic recipe plan with explicit host-owned rotation state.
///
/// # Errors
///
/// Returns [`SelectionDenial`] when no recipe, budget, model eligibility, or cursor path exists.
///
/// # Panics
///
/// Panics only if a previously validated recipe loses its role list or candidate bound.
pub fn select_recipe_with_rotation(
    request: RotatingSelectionRequest<'_>,
) -> Result<SelectedPlan, SelectionDenial> {
    let preferred_family = preferred_family(request.signals);
    let recipe = find_recipe(request.recipes, preferred_family)?;
    assert!(
        !recipe.roles().is_empty(),
        "validated recipe must keep roles"
    );
    assert!(
        recipe.bounds().max_candidates() > 0,
        "validated bounds must cap candidates"
    );
    if recipe.bounds().max_candidates() > request.budget.max_candidates {
        return Err(SelectionDenial::BudgetExceeded);
    }

    let selection = select_entries(SelectionRequest {
        recipe,
        catalog: request.catalog,
        budget: request.budget,
        signals: request.signals,
        rotation: Some(request.rotation),
    })?;
    Ok(SelectedPlan {
        recipe_id: alloc::string::String::from(recipe.id()),
        family: recipe.family(),
        selected_models: selection.selected_models,
        output_contract: recipe.output_contract(),
        bounds: recipe.bounds().clone(),
        privacy: request.signals.privacy(),
        fallback_eligibility: request.signals.fallback_eligibility(),
        rotation_decisions: selection.rotation_decisions,
    })
}

fn find_recipe(
    recipes: &[crate::Recipe],
    preferred_family: crate::RouteFamily,
) -> Result<&crate::Recipe, SelectionDenial> {
    recipes
        .iter()
        .find(|recipe| recipe.family() == preferred_family)
        .or_else(|| {
            recipes
                .iter()
                .find(|recipe| recipe.family() == crate::RouteFamily::Single)
        })
        .ok_or(SelectionDenial::NoRecipe)
}

fn preferred_family(signals: &crate::Signals) -> crate::RouteFamily {
    match (
        signals.task_shape(),
        signals.risk(),
        signals.contract_pressure(),
    ) {
        (crate::TaskShape::Workflow, _, _) => crate::RouteFamily::Workflow,
        (_, crate::SignalBand::Strict, _) => crate::RouteFamily::Fusion,
        (_, crate::SignalBand::High, crate::SignalBand::Strict | crate::SignalBand::High) => {
            crate::RouteFamily::Remom
        }
        (_, crate::SignalBand::High, _) => crate::RouteFamily::Ratings,
        (_, crate::SignalBand::Medium, _) => crate::RouteFamily::Confidence,
        _ => crate::RouteFamily::Single,
    }
}

struct SelectionOutput {
    selected_models: alloc::vec::Vec<alloc::string::String>,
    rotation_decisions: alloc::vec::Vec<crate::RouterRotationDecision>,
}

#[derive(Clone, Copy)]
struct SelectionRequest<'a> {
    recipe: &'a crate::Recipe,
    catalog: &'a [CatalogEntry],
    budget: &'a Budget,
    signals: &'a crate::Signals,
    rotation: Option<&'a crate::RouterRotationSnapshot>,
}

fn select_entries(request: SelectionRequest<'_>) -> Result<SelectionOutput, SelectionDenial> {
    let mut selected_models = alloc::vec::Vec::with_capacity(request.recipe.roles().len());
    let mut rotation_decisions = alloc::vec::Vec::new();
    for role in request.recipe.roles() {
        let eligible_labels =
            collect_eligible_labels(request.catalog, *role, request.budget, request.signals);
        if eligible_labels.is_empty() {
            return Err(SelectionDenial::NoEligibleModel);
        }
        match request.rotation {
            Some(snapshot) => {
                let decision = crate::rotate_eligible_candidates(*role, &eligible_labels, snapshot)
                    .map_err(|error| selection_denial_from_rotation(&error))?;
                selected_models.push(alloc::string::String::from(decision.selected_model()));
                rotation_decisions.push(decision);
            }
            None => selected_models.push(eligible_labels[0].clone()),
        }
    }
    Ok(SelectionOutput {
        selected_models,
        rotation_decisions,
    })
}

fn collect_eligible_labels(
    catalog: &[CatalogEntry],
    role: crate::Role,
    budget: &Budget,
    signals: &crate::Signals,
) -> alloc::vec::Vec<alloc::string::String> {
    let mut eligible_labels = alloc::vec::Vec::new();
    for entry in catalog {
        if !entry_allowed(entry, role, budget, signals) {
            continue;
        }
        if eligible_labels.iter().any(|label| label == entry.label()) {
            continue;
        }
        eligible_labels.push(alloc::string::String::from(entry.label()));
    }
    eligible_labels
}

fn selection_denial_from_rotation(error: &crate::RouterRotationError) -> SelectionDenial {
    match error {
        crate::RouterRotationError::StaleCursor => SelectionDenial::StaleRotationCursor,
        crate::RouterRotationError::NoEligibleCandidates
        | crate::RouterRotationError::Contract(_)
        | crate::RouterRotationError::ReceiptLeak(_) => SelectionDenial::NoEligibleModel,
    }
}

fn entry_allowed(
    entry: &CatalogEntry,
    role: crate::Role,
    budget: &Budget,
    signals: &crate::Signals,
) -> bool {
    entry.role() == role
        && budget.allows_backend(entry.backend())
        && (signals.privacy() == crate::PrivacyBand::RemoteAllowed || entry.is_local())
}

#[cfg(test)]
pub(crate) fn test_selected_plan(
    fixture: crate::test_support::SelectedPlanFixture<'_>,
) -> SelectedPlan {
    SelectedPlan {
        recipe_id: alloc::string::String::from(fixture.recipe_id),
        family: fixture.family,
        selected_models: fixture
            .selected_models
            .iter()
            .map(|model| alloc::string::String::from(*model))
            .collect(),
        output_contract: fixture.output_contract,
        bounds: fixture.bounds,
        privacy: fixture.privacy,
        fallback_eligibility: fixture.fallback_eligibility,
        rotation_decisions: alloc::vec::Vec::new(),
    }
}
