/// Model selection request resolved by the shell registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelSelection {
    provider_id: super::ProviderId,
    model_id: super::ModelId,
    thinking: super::ThinkingLevel,
    thinking_policy: super::ThinkingPolicy,
}

impl ModelSelection {
    /// Builds a model selection request.
    #[must_use]
    pub const fn new(
        provider_id: super::ProviderId,
        model_id: super::ModelId,
        thinking: super::ThinkingLevel,
        thinking_policy: super::ThinkingPolicy,
    ) -> Self {
        Self {
            provider_id,
            model_id,
            thinking,
            thinking_policy,
        }
    }

    /// Returns selected provider id.
    #[must_use]
    pub const fn provider_id(&self) -> &super::ProviderId {
        &self.provider_id
    }

    /// Returns selected model id.
    #[must_use]
    pub const fn model_id(&self) -> &super::ModelId {
        &self.model_id
    }

    /// Returns requested thinking level.
    #[must_use]
    pub const fn thinking(&self) -> super::ThinkingLevel {
        self.thinking
    }

    /// Returns thinking policy.
    #[must_use]
    pub const fn thinking_policy(&self) -> super::ThinkingPolicy {
        self.thinking_policy
    }
}

/// Selected model metadata handed to a shell-injected model service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedModel {
    pub(super) provider_id: super::ProviderId,
    pub(super) model_id: super::ModelId,
    pub(super) api_kind: super::ApiKind,
    pub(super) endpoint: super::EndpointIdentity,
    pub(super) capabilities: super::ModelCapabilities,
    pub(super) effective_thinking: super::ThinkingLevel,
}

impl SelectedModel {
    /// Returns selected provider id.
    #[must_use]
    pub const fn provider_id(&self) -> &super::ProviderId {
        &self.provider_id
    }

    /// Returns selected model id.
    #[must_use]
    pub const fn model_id(&self) -> &super::ModelId {
        &self.model_id
    }

    /// Returns API kind metadata.
    #[must_use]
    pub const fn api_kind(&self) -> super::ApiKind {
        self.api_kind
    }

    /// Returns endpoint identity metadata.
    #[must_use]
    pub const fn endpoint(&self) -> &super::EndpointIdentity {
        &self.endpoint
    }

    /// Returns selected model capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> &super::ModelCapabilities {
        &self.capabilities
    }

    /// Returns effective thinking level.
    #[must_use]
    pub const fn effective_thinking(&self) -> super::ThinkingLevel {
        self.effective_thinking
    }
}
