const MAX_PROVIDER_COUNT: usize = 32;
const MAX_MODEL_COUNT: usize = 128;
const NEXT_INDEX_OFFSET: usize = 1;

/// One model entry in a shell-owned provider registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelEntry {
    id: super::ModelId,
    api_kind: super::ApiKind,
    endpoint: super::EndpointIdentity,
    capabilities: super::ModelCapabilities,
}

impl ModelEntry {
    /// Builds a model entry.
    #[must_use]
    pub const fn new(
        id: super::ModelId,
        api_kind: super::ApiKind,
        endpoint: super::EndpointIdentity,
        capabilities: super::ModelCapabilities,
    ) -> Self {
        Self {
            id,
            api_kind,
            endpoint,
            capabilities,
        }
    }

    /// Returns the model id.
    #[must_use]
    pub const fn id(&self) -> &super::ModelId {
        &self.id
    }

    /// Returns the API kind.
    #[must_use]
    pub const fn api_kind(&self) -> super::ApiKind {
        self.api_kind
    }

    /// Returns the endpoint identity.
    #[must_use]
    pub const fn endpoint(&self) -> &super::EndpointIdentity {
        &self.endpoint
    }

    /// Returns model capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> &super::ModelCapabilities {
        &self.capabilities
    }
}

/// One provider entry in a shell-owned provider registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderEntry {
    id: super::ProviderId,
    credential: super::CredentialRequirement,
    models: Vec<ModelEntry>,
}

impl ProviderEntry {
    /// Builds a provider entry after validating its bounded model list.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the model list is empty, too large, or duplicated.
    pub fn new(
        id: super::ProviderId,
        credential: super::CredentialRequirement,
        models: Vec<ModelEntry>,
    ) -> Result<Self, super::RegistryError> {
        super::capability::validate_collection(&super::capability::CollectionBounds::new(
            super::MODELS_FIELD,
            models.len(),
            MAX_MODEL_COUNT,
        ))?;
        validate_distinct_models(&id, &models)?;
        Ok(Self {
            id,
            credential,
            models,
        })
    }

    /// Returns the provider id.
    #[must_use]
    pub const fn id(&self) -> &super::ProviderId {
        &self.id
    }

    /// Returns the credential requirement.
    #[must_use]
    pub const fn credential(&self) -> &super::CredentialRequirement {
        &self.credential
    }

    /// Returns provider model entries.
    #[must_use]
    pub fn models(&self) -> &[ModelEntry] {
        &self.models
    }
}

/// Shell-owned provider registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRegistry {
    providers: Vec<ProviderEntry>,
}

impl ProviderRegistry {
    /// Builds a provider registry after bounded validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when providers are empty, too many, or duplicated.
    pub fn new(providers: Vec<ProviderEntry>) -> Result<Self, super::RegistryError> {
        super::capability::validate_collection(&super::capability::CollectionBounds::new(
            super::PROVIDERS_FIELD,
            providers.len(),
            MAX_PROVIDER_COUNT,
        ))?;
        validate_distinct_entries(&providers)?;
        Ok(Self { providers })
    }

    /// Returns provider entries.
    #[must_use]
    pub fn providers(&self) -> &[ProviderEntry] {
        &self.providers
    }

    /// Selects a provider/model and resolves effective thinking metadata.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the provider/model is unavailable or unsupported.
    pub fn select_model(
        &self,
        request: &super::ModelSelection,
    ) -> Result<super::SelectedModel, super::RegistryError> {
        let provider = self.find_provider(request.provider_id()).ok_or_else(|| {
            super::RegistryError::ProviderUnavailable {
                provider: String::from(request.provider_id().as_str()),
            }
        })?;
        let model = find_model(provider, request.model_id()).ok_or_else(|| {
            super::RegistryError::ModelUnavailable {
                provider: String::from(provider.id().as_str()),
                model: String::from(request.model_id().as_str()),
            }
        })?;
        let effective_thinking = model
            .capabilities()
            .resolve_thinking(request.thinking(), request.thinking_policy())?;

        Ok(super::SelectedModel {
            provider_id: provider.id().clone(),
            model_id: model.id().clone(),
            api_kind: model.api_kind(),
            endpoint: model.endpoint().clone(),
            capabilities: model.capabilities().clone(),
            effective_thinking,
        })
    }

    /// Returns a new registry with a host-provided provider entry after validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the dynamic entry would make the registry invalid.
    pub fn with_dynamic_provider(
        &self,
        provider: ProviderEntry,
    ) -> Result<Self, super::RegistryError> {
        let mut providers = self.providers.clone();
        if let Some(existing) = providers
            .iter_mut()
            .find(|candidate| candidate.id() == provider.id())
        {
            *existing = provider;
        } else {
            providers.push(provider);
        }
        Self::new(providers)
    }

    fn find_provider(&self, provider_id: &super::ProviderId) -> Option<&ProviderEntry> {
        self.providers
            .iter()
            .find(|provider| provider.id() == provider_id)
    }
}

fn validate_distinct_entries(providers: &[ProviderEntry]) -> Result<(), super::RegistryError> {
    for (index, provider) in providers.iter().enumerate() {
        if providers
            .iter()
            .skip(index.saturating_add(NEXT_INDEX_OFFSET))
            .any(|candidate| candidate.id() == provider.id())
        {
            return Err(super::RegistryError::DuplicateProvider {
                provider: String::from(provider.id().as_str()),
                reason: super::DUPLICATE_PROVIDER_REASON,
            });
        }
    }

    Ok(())
}

fn validate_distinct_models(
    provider_id: &super::ProviderId,
    models: &[ModelEntry],
) -> Result<(), super::RegistryError> {
    for (index, model) in models.iter().enumerate() {
        if models
            .iter()
            .skip(index.saturating_add(NEXT_INDEX_OFFSET))
            .any(|candidate| candidate.id() == model.id())
        {
            return Err(super::RegistryError::DuplicateModel {
                provider: String::from(provider_id.as_str()),
                model: String::from(model.id().as_str()),
                reason: super::DUPLICATE_MODEL_REASON,
            });
        }
    }

    Ok(())
}

fn find_model<'a>(
    provider: &'a ProviderEntry,
    model_id: &super::ModelId,
) -> Option<&'a ModelEntry> {
    provider
        .models()
        .iter()
        .find(|model| model.id() == model_id)
}
