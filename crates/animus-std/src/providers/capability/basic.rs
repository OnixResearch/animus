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
    /// Returns [`super::super::RegistryError`] when the endpoint label is empty or too long.
    pub fn new(raw: &str) -> Result<Self, super::super::RegistryError> {
        if raw.is_empty() {
            return Err(super::super::RegistryError::InvalidEndpoint {
                field: super::super::ENDPOINT_FIELD,
                reason: super::super::INVALID_ENDPOINT_EMPTY_REASON,
            });
        }

        if raw.len() > super::MAX_ENDPOINT_BYTES {
            return Err(super::super::RegistryError::InvalidEndpoint {
                field: super::super::ENDPOINT_FIELD,
                reason: super::super::INVALID_ENDPOINT_LENGTH_REASON,
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
