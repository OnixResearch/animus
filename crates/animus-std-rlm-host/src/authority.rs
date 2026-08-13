use crate::{BindingIdentity, DispatchFailure, DispatchFailureClass, RouteBinding, RouteClass};

/// Sealed authority unit and request-local expected binding facts.
#[derive(Clone, Debug)]
pub struct ExpectedAuthority {
    /// Existing immutable runtime-authority seal.
    pub unit: animus_runtime::runtime_authority_seal::RuntimeAuthorityUnit,
}

/// Current host observation immediately before one effect.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentAuthorityObservation {
    /// Current runtime-authority recheck fields.
    pub session_lineage_identity: String,
    /// Current authority profile identity.
    pub authority_profile_identity: String,
    /// Current durable binding identity.
    pub durable_binding_identity: String,
    /// Current enforcement observation identity.
    pub enforcement_observation_identity: String,
    /// Current operation set identity.
    pub operation_set_identity: String,
    /// Current adapter cohort identity.
    pub adapter_cohort_identity: String,
    /// Current resource profile identity.
    pub resource_profile_identity: String,
    /// Current resource budget identity.
    pub resource_budget_identity: String,
    /// Current runtime services identity.
    pub runtime_services_identity: String,
    /// Current exact operation identities.
    pub operation_identities: Vec<String>,
    /// Current exact resource identities.
    pub resource_identities: Vec<String>,
    /// Current route binding identity.
    pub binding_identity: BindingIdentity,
    /// Current capability-pack scope identity.
    pub binding_scope_identity: BindingIdentity,
    /// Current exact request scope identity.
    pub request_scope_identity: BindingIdentity,
    /// Current generation.
    pub generation: u64,
    /// Whether current policy admits this effect.
    pub admitted: bool,
    /// Whether cancellation is current.
    pub cancelled: bool,
    /// Whether the deadline expired.
    pub timed_out: bool,
}

impl CurrentAuthorityObservation {
    pub(crate) fn validate(
        &self,
        expected: &ExpectedAuthority,
        binding: &RouteBinding,
        scope_identity: &BindingIdentity,
    ) -> Result<(), DispatchFailure> {
        let route = binding.route();
        if self.cancelled {
            return Err(failure(route, DispatchFailureClass::Cancelled, "cancelled"));
        }
        if self.timed_out {
            return Err(failure(route, DispatchFailureClass::TimedOut, "deadline"));
        }
        if !self.admitted {
            return Err(failure(route, DispatchFailureClass::Denied, "authority"));
        }
        if self.binding_identity != *binding.identity()
            || self.binding_scope_identity != *binding.scope_identity()
            || self.request_scope_identity != *scope_identity
            || self.generation != binding.generation()
        {
            return Err(failure(route, DispatchFailureClass::Stale, "binding"));
        }
        let operations = self
            .operation_identities
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let resources = self
            .resource_identities
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        animus_runtime::runtime_authority_seal::start_runtime_authority_effect(
            &expected.unit,
            animus_runtime::runtime_authority_seal::RuntimeAuthorityRecheckDraft {
                session_lineage_identity: self.session_lineage_identity.as_str(),
                authority_profile_identity: self.authority_profile_identity.as_str(),
                durable_binding_identity: self.durable_binding_identity.as_str(),
                enforcement_observation_identity: self.enforcement_observation_identity.as_str(),
                operation_set_identity: self.operation_set_identity.as_str(),
                adapter_cohort_identity: self.adapter_cohort_identity.as_str(),
                resource_profile_identity: self.resource_profile_identity.as_str(),
                resource_budget_identity: self.resource_budget_identity.as_str(),
                runtime_services_identity: self.runtime_services_identity.as_str(),
                operation_identities: operations.as_slice(),
                resource_identities: resources.as_slice(),
                admitted: self.admitted,
            },
        )
        .map(|_running| ())
        .map_err(|_error| failure(route, DispatchFailureClass::Stale, "seal"))
    }
}

/// Injected current authority observer.
pub trait CurrentAuthorityObserver {
    /// Observes current seal, generation, cancellation, scope, and binding facts.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure when current facts are unavailable.
    fn observe(
        &mut self,
        route: RouteClass,
        binding: &RouteBinding,
        request_scope: &BindingIdentity,
    ) -> Result<CurrentAuthorityObservation, DispatchFailure>;
}

fn failure(route: RouteClass, class: DispatchFailureClass, source: &str) -> DispatchFailure {
    DispatchFailure::new(route, class, source)
}
