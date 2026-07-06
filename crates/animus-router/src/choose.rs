const MODEL_LABEL_FIELD: animus_adapters::FieldName =
    animus_adapters::FieldName::new("router_model_label");
const BACKEND_FIELD: animus_adapters::FieldName =
    animus_adapters::FieldName::new("router_backend");

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
    /// Returns [`animus_adapters::AdapterContractError`] when labels or roles are invalid.
    pub fn new(draft: CatalogEntryDraft<'_>) -> Result<Self, animus_adapters::AdapterContractError> {
        Ok(Self {
            label: animus_adapters::normalized_adapter_field(MODEL_LABEL_FIELD, draft.label)?,
            backend: animus_adapters::normalized_adapter_field(BACKEND_FIELD, draft.backend)?,
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
    /// Returns [`animus_adapters::AdapterContractError`] when backend labels are invalid.
    pub fn new(
        max_candidates: u32,
        allowed_backends: &[&str],
    ) -> Result<Self, animus_adapters::AdapterContractError> {
        let allowed_backends = allowed_backends
            .iter()
            .map(|backend| {
                animus_adapters::normalized_adapter_field(BACKEND_FIELD, backend)
            })
            .collect::<Result<alloc::vec::Vec<_>, _>>()?;
        Ok(Self {
            max_candidates,
            allowed_backends,
        })
    }

    fn allows_backend(&self, backend: &str) -> bool {
        self.allowed_backends
            .iter()
            .any(|allowed| allowed == backend)
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

    let selected_models = select_entries(recipe, catalog, budget, signals)?;
    Ok(SelectedPlan {
        recipe_id: alloc::string::String::from(recipe.id()),
        family: recipe.family(),
        selected_models,
        output_contract: recipe.output_contract(),
        bounds: recipe.bounds().clone(),
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

fn select_entries(
    recipe: &crate::Recipe,
    catalog: &[CatalogEntry],
    budget: &Budget,
    signals: &crate::Signals,
) -> Result<alloc::vec::Vec<alloc::string::String>, SelectionDenial> {
    let mut selected = alloc::vec::Vec::with_capacity(recipe.roles().len());
    for role in recipe.roles() {
        let entry = catalog
            .iter()
            .find(|entry| entry_allowed(entry, *role, budget, signals))
            .ok_or(SelectionDenial::NoEligibleModel)?;
        selected.push(alloc::string::String::from(entry.label()));
    }
    Ok(selected)
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