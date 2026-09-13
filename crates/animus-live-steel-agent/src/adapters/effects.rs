//! Concrete authority-scoped host composition helpers.

use animus_std_rlm_host::CurrentAuthorityObserver;

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
    values: std::collections::BTreeMap<String, String>,
}

impl ExplicitContextMap {
    /// Builds a context map from explicit key and value pairs.
    ///
    /// # Errors
    ///
    /// Returns [`LiveHostError::Context`] for an empty, duplicate, or unsafe key.
    pub fn new(values: impl IntoIterator<Item = (String, String)>) -> Result<Self, LiveHostError> {
        let mut map = std::collections::BTreeMap::new();
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
    expected: animus_std_rlm_host::ExpectedAuthority,
    cancelled: bool,
    timed_out: bool,
}

impl LiveAuthorityObserver {
    /// Builds an admitted current observer from one sealed authority unit.
    #[must_use]
    pub const fn admitted(expected: animus_std_rlm_host::ExpectedAuthority) -> Self {
        Self {
            expected,
            cancelled: false,
            timed_out: false,
        }
    }

    /// Builds a cancelled observer for negative lifecycle checks.
    #[must_use]
    pub const fn cancelled(expected: animus_std_rlm_host::ExpectedAuthority) -> Self {
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
        _route: animus_std_rlm_host::RouteClass,
        binding: &animus_std_rlm_host::RouteBinding,
        request_scope: &animus_std_rlm_host::BindingIdentity,
    ) -> Result<
        animus_std_rlm_host::CurrentAuthorityObservation,
        animus_std_rlm_host::DispatchFailure,
    > {
        let unit = &self.expected.unit;
        Ok(animus_std_rlm_host::CurrentAuthorityObservation {
            session_lineage_identity: String::from(unit.session_lineage_identity()),
            authority_profile_identity: String::from(unit.authority_profile_identity()),
            durable_binding_identity: String::from(unit.durable_binding_identity()),
            enforcement_observation_identity: String::from(unit.enforcement_observation_identity()),
            operation_set_identity: String::from(unit.operation_set_identity()),
            adapter_cohort_identity: String::from(unit.adapter_cohort_identity()),
            resource_profile_identity: String::from(unit.resource_profile_identity()),
            resource_budget_identity: String::from(unit.resource_budget_identity()),
            runtime_services_identity: String::from(unit.runtime_services_identity()),
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
) -> Result<animus_std_rlm_host::ExpectedAuthority, LiveHostError> {
    let view_identity = |class| {
        match class {
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::SessionLineage
        | animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::AuthorityProfile
        | animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::AdapterCohort
        | animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::RuntimeServices => config.authority_component_identity(),
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::DurableBinding => config.route_binding_identity(),
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::EnforcementObservation => {
            config.authority_authorization_identity()
        }
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::OperationSet => config.operation_identity(),
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::ResourceProfile
        | animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::ResourceBudget => config.resource_identity(),
    }
    };
    let expected_views = animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::ALL
        .map(
            |class| animus_runtime::runtime_authority_seal::RuntimeAuthorityExpectedViewDraft {
                class,
                identity: view_identity(class),
            },
        );
    let observed_views =
        animus_runtime::runtime_authority_seal::RuntimeAuthorityViewClass::ALL.map(|class| animus_runtime::runtime_authority_seal::RuntimeAuthorityObservedViewDraft {
            class,
            identity: view_identity(class),
            status: animus_runtime::runtime_authority_seal::RuntimeAuthorityObservationStatus::Current,
        });
    let operations = [config.operation_identity()];
    let resources = [config.resource_identity()];
    let plan = animus_runtime::runtime_authority_seal::prepare_runtime_authority_seal(
        animus_runtime::runtime_authority_seal::RuntimeAuthorityUnitDraft {
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
        animus_runtime::runtime_authority_seal::RuntimeAuthorityReceiptDraft {
            sensitive: empty_sensitive(),
            semantic_claims: &[],
        },
    )
    .map_err(|_error| LiveHostError::Authority)?;
    let unit = animus_runtime::runtime_authority_seal::commit_runtime_authority_seal(
        &plan,
        plan.receipt().receipt_identity.as_str(),
    )
    .map_err(|_error| LiveHostError::Authority)?;
    debug_assert_eq!(unit.operation_identities(), operations);
    debug_assert_eq!(unit.resource_identities(), resources);
    Ok(animus_std_rlm_host::ExpectedAuthority { unit })
}

/// Builds route bindings for the exact configured operation set.
///
/// # Errors
///
/// Returns [`LiveHostError::Binding`] when a binding is malformed.
pub fn build_bindings(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<Vec<animus_std_rlm_host::RouteBinding>, LiveHostError> {
    config
        .admitted_operations()
        .iter()
        .map(|operation| {
            let route = match operation.as_str() {
                "context_read" => animus_std_rlm_host::RouteClass::ContextRead,
                "subagent" => animus_std_rlm_host::RouteClass::Subagent,
                _other => return Err(LiveHostError::Binding),
            };
            animus_std_rlm_host::RouteBinding::new(
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
pub fn context_route(
    context: &ExplicitContextMap,
) -> animus_std_rlm_host::ResolverContextRoute<'_> {
    animus_std_rlm_host::ResolverContextRoute {
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
) -> Result<animus_std_rlm_host::RlmHostExecutor<'a, LiveAuthorityObserver>, LiveHostError> {
    let binding_values = build_bindings(config)?;
    debug_assert_eq!(binding_values.len(), config.admitted_operations().len());
    debug_assert!(!binding_values.is_empty());
    let bindings = animus_std_rlm_host::BindingSet::new(binding_values)
        .map_err(|_error| LiveHostError::Binding)?;
    let expected = build_expected_authority(config)?;
    animus_std_rlm_host::RlmHostExecutor::new(animus_std_rlm_host::RlmHostExecutorConfig {
        bindings,
        expected_authority: expected.clone(),
        authority: LiveAuthorityObserver::admitted(expected),
        limits: config.executor_limits(),
        packs: animus_std_rlm_host::CapabilityPacks {
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

fn empty_sensitive()
-> animus_runtime::runtime_authority_seal::RuntimeAuthoritySensitiveDraft<'static> {
    animus_runtime::runtime_authority_seal::RuntimeAuthoritySensitiveDraft {
        raw_grant: None,
        descriptor: None,
        host_path: None,
        credential: None,
        prompt: None,
        provider_payload: None,
        tool_body: None,
        model_output: None,
        environment_value: None,
    }
}
