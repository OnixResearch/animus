//! Concrete authority-scoped host composition helpers.

use std::collections::BTreeMap;

use animus_std_rlm_host::{
    BindingIdentity, BindingSet, CapabilityPacks, CurrentAuthorityObservation,
    CurrentAuthorityObserver, DispatchFailure, ExpectedAuthority,
    ResolverContextRoute, RlmHostExecutor, RlmHostExecutorConfig, RouteBinding, RouteClass,
};

const AUTHORITY_SESSION_ID: &str = "live-steel-agent.authority";
const AUTHORITY_DISPATCH_ID: &str = "live-steel-agent.dispatch";
const CONTEXT_PREFIX: &str = "@";

/// Safe host-composition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveHostError {
    /// Exact context input was malformed.
    Context,
    /// Runtime-authority sealing failed.
    Authority,
    /// Route binding construction failed.
    Binding,
    /// Executor construction failed.
    Executor,
}

impl core::fmt::Display for LiveHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live host composition rejected: {self:?}")
    }
}

impl std::error::Error for LiveHostError {}

/// Explicit immutable context values for one live run.
#[derive(Clone, Debug, Default)]
pub struct ExplicitContextMap {
    values: BTreeMap<String, String>,
}

impl ExplicitContextMap {
    /// Builds a context map from explicit key and value pairs.
    ///
    /// # Errors
    ///
    /// Returns [`LiveHostError::Context`] for an empty, duplicate, or unsafe key.
    pub fn new(
        values: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, LiveHostError> {
        let mut map = BTreeMap::new();
        for (key, value) in values {
            let key = key.strip_prefix(CONTEXT_PREFIX).unwrap_or(key.as_str());
            if key.is_empty() || key.contains('\0') || value.contains('\0') {
                return Err(LiveHostError::Context);
            }
            if map.insert(key.to_owned(), value).is_some() {
                return Err(LiveHostError::Context);
            }
        }
        Ok(Self { values: map })
    }
}

impl animus_adapters::FileContextResolver for ExplicitContextMap {
    fn resolve_file(
        &self,
        reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        let key = reference.normalized_target();
        let value = self
            .values
            .get(key)
            .ok_or(animus_adapters::ContextReferenceError::ResolverUnavailable)?;
        Ok(animus_adapters::ResolvedContextCandidate::new(
            reference.clone(),
            animus_adapters::ContextResolverKind::File,
            value.clone(),
        ))
    }
}

/// Current authority facts observed before each host effect.
#[derive(Clone)]
pub struct LiveAuthorityObserver {
    expected: ExpectedAuthority,
    cancelled: bool,
    timed_out: bool,
}

impl LiveAuthorityObserver {
    /// Builds an admitted current observer from one sealed authority unit.
    #[must_use]
    pub const fn admitted(expected: ExpectedAuthority) -> Self {
        Self {
            expected,
            cancelled: false,
            timed_out: false,
        }
    }

    /// Builds a cancelled observer for negative lifecycle checks.
    #[must_use]
    pub const fn cancelled(expected: ExpectedAuthority) -> Self {
        Self {
            expected,
            cancelled: true,
            timed_out: false,
        }
    }
}

impl CurrentAuthorityObserver for LiveAuthorityObserver {
    fn observe(
        &mut self,
        _route: RouteClass,
        binding: &RouteBinding,
        request_scope: &BindingIdentity,
    ) -> Result<CurrentAuthorityObservation, DispatchFailure> {
        let unit = &self.expected.unit;
        Ok(CurrentAuthorityObservation {
            session_lineage_identity: unit.session_lineage_identity().to_owned(),
            authority_profile_identity: unit.authority_profile_identity().to_owned(),
            durable_binding_identity: unit.durable_binding_identity().to_owned(),
            enforcement_observation_identity: unit.enforcement_observation_identity().to_owned(),
            operation_set_identity: unit.operation_set_identity().to_owned(),
            adapter_cohort_identity: unit.adapter_cohort_identity().to_owned(),
            resource_profile_identity: unit.resource_profile_identity().to_owned(),
            resource_budget_identity: unit.resource_budget_identity().to_owned(),
            runtime_services_identity: unit.runtime_services_identity().to_owned(),
            operation_identities: unit.operation_identities().to_vec(),
            resource_identities: unit.resource_identities().to_vec(),
            binding_identity: binding.identity().clone(),
            binding_scope_identity: binding.scope_identity().clone(),
            request_scope_identity: request_scope.clone(),
            generation: binding.generation(),
            admitted: true,
            cancelled: self.cancelled,
            timed_out: self.timed_out,
        })
    }
}

/// Builds the exact runtime-authority seal selected by configuration.
///
/// # Errors
///
/// Returns [`LiveHostError::Authority`] when the seal cannot be prepared or committed.
pub fn build_expected_authority(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<ExpectedAuthority, LiveHostError> {
    use animus_runtime::runtime_authority_seal as seal;

    let expected_views = seal::RuntimeAuthorityViewClass::ALL.map(|class| {
        seal::RuntimeAuthorityExpectedViewDraft {
            class,
            identity: config.authority_component_identity(),
        }
    });
    let observed_views = seal::RuntimeAuthorityViewClass::ALL.map(|class| {
        seal::RuntimeAuthorityObservedViewDraft {
            class,
            identity: config.authority_component_identity(),
            status: seal::RuntimeAuthorityObservationStatus::Current,
        }
    });
    let operations = [config.operation_identity()];
    let resources = [config.resource_identity()];
    let plan = seal::prepare_runtime_authority_seal(
        seal::RuntimeAuthorityUnitDraft {
            session_id: AUTHORITY_SESSION_ID,
            dispatch_id: AUTHORITY_DISPATCH_ID,
            session_lineage_identity: config.authority_component_identity(),
            durable_binding_identity: config.route_binding_identity(),
            authority_profile_identity: config.authority_component_identity(),
            enforcement_observation_identity: config.authority_authorization_identity(),
            operation_set_identity: config.operation_identity(),
            adapter_cohort_identity: config.authority_component_identity(),
            resource_profile_identity: config.resource_identity(),
            resource_budget_identity: config.resource_identity(),
            runtime_services_identity: config.authority_component_identity(),
            operation_identities: &operations,
            resource_identities: &resources,
            expected_views: &expected_views,
            observed_views: &observed_views,
        },
        seal::RuntimeAuthorityReceiptDraft {
            sensitive: seal::RuntimeAuthoritySensitiveDraft::default(),
            semantic_claims: &[],
        },
    )
    .map_err(|_error| LiveHostError::Authority)?;
    let unit = seal::commit_runtime_authority_seal(&plan, plan.receipt().receipt_identity.as_str())
        .map_err(|_error| LiveHostError::Authority)?;
    Ok(ExpectedAuthority { unit })
}

/// Builds route bindings for the exact configured operation set.
///
/// # Errors
///
/// Returns [`LiveHostError::Binding`] when a binding is malformed.
pub fn build_bindings(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<Vec<RouteBinding>, LiveHostError> {
    config
        .admitted_operations()
        .iter()
        .map(|operation| {
            let route = match operation.as_str() {
                "context_read" => RouteClass::ContextRead,
                "subagent" => RouteClass::Subagent,
                _other => return Err(LiveHostError::Binding),
            };
            RouteBinding::new(
                route,
                config.route_binding_identity(),
                config.route_scope_identity(),
                config.authority_generation(),
            )
            .map_err(|_error| LiveHostError::Binding)
        })
        .collect()
}

/// Binds explicit context values to the existing resolver route.
#[must_use]
pub fn context_route(context: &ExplicitContextMap) -> ResolverContextRoute<'_> {
    ResolverContextRoute {
        resolvers: animus_adapters::ContextResolverSet::empty().with_file(context),
        policy: animus_adapters::ContextReferencePolicy::allow_local(),
    }
}

/// Builds the concrete narrow executor used by the visible composition root.
///
/// # Errors
///
/// Returns [`LiveHostError`] when authority, binding, or pack construction fails.
pub fn build_executor<'a>(
    config: &crate::ValidatedLiveSteelAgentConfig,
    context: &'a mut dyn animus_std_rlm_host::ContextRoute,
    child: &'a mut dyn animus_std_rlm_host::ChildRoute,
) -> Result<RlmHostExecutor<'a, LiveAuthorityObserver>, LiveHostError> {
    let binding_values = build_bindings(config)?;
    let bindings = BindingSet::new(binding_values).map_err(|_error| LiveHostError::Binding)?;
    let expected = build_expected_authority(config)?;
    RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings,
        expected_authority: expected.clone(),
        authority: LiveAuthorityObserver::admitted(expected),
        limits: config.executor_limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: None,
            context: Some(context),
            child: Some(child),
            kernel: None,
        },
    })
    .map_err(|_error| LiveHostError::Executor)
}
