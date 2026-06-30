const MAX_ENDPOINT_BYTES: usize = 256;
const MAX_MODALITY_COUNT: usize = 4;
const MAX_THINKING_LEVEL_COUNT: usize = 4;
const DEFAULT_CLAMP_FALLBACK: super::ThinkingLevel = super::ThinkingLevel::None;

/// Provider API family used by a shell-owned provider entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiKind {
    /// OpenAI-compatible responses API.
    OpenAiResponses,
    /// Anthropic-compatible messages API.
    AnthropicMessages,
    /// Local HTTP endpoint controlled by the host.
    LocalHttp,
    /// Deterministic mock provider for tests and fixtures.
    Mock,
}

impl ApiKind {
    /// Returns the stable API-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAiResponses => "openai_responses",
            Self::AnthropicMessages => "anthropic_messages",
            Self::LocalHttp => "local_http",
            Self::Mock => "mock",
        }
    }
}

/// Safe endpoint identity for diagnostics and selection metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointIdentity(String);

impl EndpointIdentity {
    /// Builds an endpoint identity after bounded shell validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the endpoint label is empty or too long.
    pub fn new(raw: &str) -> Result<Self, super::RegistryError> {
        if raw.is_empty() {
            return Err(super::RegistryError::InvalidEndpoint {
                field: super::ENDPOINT_FIELD,
                reason: super::INVALID_ENDPOINT_EMPTY_REASON,
            });
        }

        if raw.len() > MAX_ENDPOINT_BYTES {
            return Err(super::RegistryError::InvalidEndpoint {
                field: super::ENDPOINT_FIELD,
                reason: super::INVALID_ENDPOINT_LENGTH_REASON,
            });
        }

        Ok(Self(String::from(raw)))
    }

    /// Returns the endpoint identity text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Input modality declared by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputModality {
    /// UTF-8 text input.
    Text,
    /// Image input supplied by a host shell.
    Image,
}

/// Integer token cost in micro-USD per one million tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenCost {
    input_micro_usd_per_million: u32,
    output_micro_usd_per_million: u32,
}

impl TokenCost {
    /// Builds integer token cost metadata.
    #[must_use]
    pub const fn new(input_micro_usd_per_million: u32, output_micro_usd_per_million: u32) -> Self {
        Self {
            input_micro_usd_per_million,
            output_micro_usd_per_million,
        }
    }

    /// Returns input cost in micro-USD per one million tokens.
    #[must_use]
    pub const fn input_micro_usd_per_million(self) -> u32 {
        self.input_micro_usd_per_million
    }

    /// Returns output cost in micro-USD per one million tokens.
    #[must_use]
    pub const fn output_micro_usd_per_million(self) -> u32 {
        self.output_micro_usd_per_million
    }
}

/// Unvalidated model capability metadata supplied by a shell config boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCapabilitiesDraft {
    /// Input modalities declared by the provider config.
    pub input_modalities: Vec<InputModality>,
    /// Raw context-token budget.
    pub context_budget: u32,
    /// Raw output-token budget.
    pub output_budget: u32,
    /// Declared reasoning support.
    pub reasoning: super::ReasoningSupport,
    /// Raw supported thinking levels.
    pub thinking_levels: Vec<super::ThinkingLevel>,
    /// Optional integer cost metadata.
    pub cost: Option<TokenCost>,
}

/// Validated model capability metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCapabilities {
    input_modalities: Vec<InputModality>,
    context_budget: core::num::NonZeroU32,
    output_budget: core::num::NonZeroU32,
    reasoning: super::ReasoningSupport,
    thinking_levels: Vec<super::ThinkingLevel>,
    cost: Option<TokenCost>,
}

impl ModelCapabilities {
    /// Builds model capability metadata after boundary validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when budgets are zero or bounded lists are invalid.
    pub fn from_draft(draft: ModelCapabilitiesDraft) -> Result<Self, super::RegistryError> {
        validate_collection(&CollectionBounds::new(
            super::MODALITIES_FIELD,
            draft.input_modalities.len(),
            MAX_MODALITY_COUNT,
        ))?;
        validate_collection(&CollectionBounds::new(
            super::THINKING_LEVELS_FIELD,
            draft.thinking_levels.len(),
            MAX_THINKING_LEVEL_COUNT,
        ))?;
        let context_budget = non_zero_budget(super::CONTEXT_BUDGET_FIELD, draft.context_budget)?;
        let output_budget = non_zero_budget(super::OUTPUT_BUDGET_FIELD, draft.output_budget)?;
        Ok(Self {
            input_modalities: draft.input_modalities,
            context_budget,
            output_budget,
            reasoning: draft.reasoning,
            thinking_levels: draft.thinking_levels,
            cost: draft.cost,
        })
    }

    /// Returns declared input modalities.
    #[must_use]
    pub fn input_modalities(&self) -> &[InputModality] {
        &self.input_modalities
    }

    /// Returns the context-token budget.
    #[must_use]
    pub const fn context_budget(&self) -> core::num::NonZeroU32 {
        self.context_budget
    }

    /// Returns the output-token budget.
    #[must_use]
    pub const fn output_budget(&self) -> core::num::NonZeroU32 {
        self.output_budget
    }

    /// Returns reasoning support metadata.
    #[must_use]
    pub const fn reasoning(&self) -> super::ReasoningSupport {
        self.reasoning
    }

    /// Returns supported thinking levels.
    #[must_use]
    pub fn thinking_levels(&self) -> &[super::ThinkingLevel] {
        &self.thinking_levels
    }

    /// Returns optional integer cost metadata.
    #[must_use]
    pub const fn cost(&self) -> Option<TokenCost> {
        self.cost
    }

    /// Resolves a requested thinking level according to the supplied policy.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when `policy` rejects an unsupported level.
    pub fn resolve_thinking(
        &self,
        requested: super::ThinkingLevel,
        policy: super::ThinkingPolicy,
    ) -> Result<super::ThinkingLevel, super::RegistryError> {
        if self.thinking_levels.contains(&requested) {
            return Ok(requested);
        }

        match policy {
            super::ThinkingPolicy::Clamp => Ok(self.clamp_thinking(requested)),
            super::ThinkingPolicy::Reject => Err(super::RegistryError::UnsupportedThinkingLevel {
                requested: requested.as_str(),
            }),
        }
    }

    fn clamp_thinking(&self, requested: super::ThinkingLevel) -> super::ThinkingLevel {
        self.thinking_levels
            .iter()
            .copied()
            .filter(|level| level.rank() <= requested.rank())
            .max_by_key(|level| level.rank())
            .unwrap_or(DEFAULT_CLAMP_FALLBACK)
    }
}

pub(super) struct CollectionBounds {
    field: &'static str,
    actual_len: usize,
    max_len: usize,
}

impl CollectionBounds {
    pub(super) const fn new(field: &'static str, actual_len: usize, max_len: usize) -> Self {
        Self {
            field,
            actual_len,
            max_len,
        }
    }
}

fn non_zero_budget(
    field: &'static str,
    budget: u32,
) -> Result<core::num::NonZeroU32, super::RegistryError> {
    core::num::NonZeroU32::new(budget).ok_or(super::RegistryError::InvalidBudget {
        field,
        reason: super::INVALID_BUDGET_REASON,
    })
}

pub(super) fn validate_collection(bounds: &CollectionBounds) -> Result<(), super::RegistryError> {
    if bounds.actual_len == 0 {
        return Err(super::RegistryError::EmptyCollection {
            field: bounds.field,
            reason: super::EMPTY_COLLECTION_REASON,
        });
    }

    if bounds.actual_len > bounds.max_len {
        return Err(super::RegistryError::CollectionLimit {
            field: bounds.field,
            reason: super::COLLECTION_LIMIT_REASON,
        });
    }

    Ok(())
}
