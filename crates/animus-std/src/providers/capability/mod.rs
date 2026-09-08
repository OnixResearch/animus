/// Provider API family used by a shell-owned provider entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiKind {
    /// OpenAI-compatible responses API.
    OpenAiResponses,
    /// OpenAI-compatible chat-completions API.
    OpenAiChat,
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
            Self::OpenAiChat => "openai_chat",
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

/// Model stream feature named in negotiation diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamFeature {
    /// Assistant-visible content token or delta streaming.
    Content,
    /// Provider-exposed thought or reasoning streaming.
    Thought,
    /// Tool-call delta streaming.
    ToolCallDelta,
    /// Usage update streaming.
    Usage,
}

impl StreamFeature {
    /// Returns the stable stream-feature label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Content => "content_streaming",
            Self::Thought => "thought_streaming",
            Self::ToolCallDelta => "tool_call_delta_streaming",
            Self::Usage => "usage_streaming",
        }
    }
}

/// Whether one stream feature is supported by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamSupport {
    /// The feature is unsupported.
    Unsupported,
    /// The feature is supported.
    Supported,
}

impl StreamSupport {
    /// Returns true when the feature is supported.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        match self {
            Self::Supported => true,
            Self::Unsupported => false,
        }
    }
}

/// Effective streaming capability metadata declared by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelStreamingCapabilities {
    content: StreamSupport,
    thought: StreamSupport,
    tool_call_delta: StreamSupport,
    usage: StreamSupport,
}

impl ModelStreamingCapabilities {
    /// Builds explicit streaming capability metadata.
    #[must_use]
    pub const fn new(
        content: StreamSupport,
        thought: StreamSupport,
        tool_call_delta: StreamSupport,
        usage: StreamSupport,
    ) -> Self {
        Self {
            content,
            thought,
            tool_call_delta,
            usage,
        }
    }

    /// Returns metadata for a model with no streaming support.
    #[must_use]
    pub const fn none() -> Self {
        Self::new(
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
        )
    }

    /// Returns content-stream support.
    #[must_use]
    pub const fn content(self) -> StreamSupport {
        self.content
    }

    /// Returns thought-stream support.
    #[must_use]
    pub const fn thought(self) -> StreamSupport {
        self.thought
    }

    /// Returns tool-call delta stream support.
    #[must_use]
    pub const fn tool_call_delta(self) -> StreamSupport {
        self.tool_call_delta
    }

    /// Returns usage stream support.
    #[must_use]
    pub const fn usage(self) -> StreamSupport {
        self.usage
    }

    pub(super) const fn support_for(self, feature: StreamFeature) -> StreamSupport {
        match feature {
            StreamFeature::Content => self.content,
            StreamFeature::Thought => self.thought,
            StreamFeature::ToolCallDelta => self.tool_call_delta,
            StreamFeature::Usage => self.usage,
        }
    }
}

/// Requested state for one stream feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamRequestMode {
    /// The host did not request this stream feature.
    Disabled,
    /// The host requested this stream feature.
    Required,
}

impl StreamRequestMode {
    /// Returns true when a host requested the feature.
    #[must_use]
    pub const fn is_required(self) -> bool {
        match self {
            Self::Required => true,
            Self::Disabled => false,
        }
    }
}

/// Host request for model stream modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelStreamingRequest {
    content: StreamRequestMode,
    thought: StreamRequestMode,
    tool_call_delta: StreamRequestMode,
    usage: StreamRequestMode,
}

impl ModelStreamingRequest {
    /// Builds an explicit stream-mode request.
    #[must_use]
    pub const fn new(
        content: StreamRequestMode,
        thought: StreamRequestMode,
        tool_call_delta: StreamRequestMode,
        usage: StreamRequestMode,
    ) -> Self {
        Self {
            content,
            thought,
            tool_call_delta,
            usage,
        }
    }

    /// Returns a content-only streaming request.
    #[must_use]
    pub const fn content_only() -> Self {
        Self::new(
            StreamRequestMode::Required,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
        )
    }

    /// Returns a request for content and thought streaming.
    #[must_use]
    pub const fn content_and_thought() -> Self {
        Self::new(
            StreamRequestMode::Required,
            StreamRequestMode::Required,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
        )
    }

    pub(super) const fn mode_for(self, feature: StreamFeature) -> StreamRequestMode {
        match feature {
            StreamFeature::Content => self.content,
            StreamFeature::Thought => self.thought,
            StreamFeature::ToolCallDelta => self.tool_call_delta,
            StreamFeature::Usage => self.usage,
        }
    }
}

/// Host policy for unsupported stream requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedStreamPolicy {
    /// Reject unsupported requested stream features.
    Reject,
    /// Disable unsupported non-content stream features while preserving content streaming.
    ContentOnlyDowngrade,
}

/// Bundle of the four per-feature resolved stream modes used to build [`ResolvedStreamingModes`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StreamModeSet {
    /// Resolved content stream mode.
    pub(super) content: StreamRequestMode,
    /// Resolved thought stream mode.
    pub(super) thought: StreamRequestMode,
    /// Resolved tool-call delta stream mode.
    pub(super) tool_call_delta: StreamRequestMode,
    /// Resolved usage stream mode.
    pub(super) usage: StreamRequestMode,
}

/// Effective stream modes after capability negotiation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedStreamingModes {
    content: StreamRequestMode,
    thought: StreamRequestMode,
    tool_call_delta: StreamRequestMode,
    usage: StreamRequestMode,
    downgraded: bool,
}

impl ResolvedStreamingModes {
    pub(super) const fn new(modes: StreamModeSet, downgraded: bool) -> Self {
        Self {
            content: modes.content,
            thought: modes.thought,
            tool_call_delta: modes.tool_call_delta,
            usage: modes.usage,
            downgraded,
        }
    }

    /// Returns resolved content stream mode.
    #[must_use]
    pub const fn content(self) -> StreamRequestMode {
        self.content
    }

    /// Returns resolved thought stream mode.
    #[must_use]
    pub const fn thought(self) -> StreamRequestMode {
        self.thought
    }

    /// Returns resolved tool-call delta stream mode.
    #[must_use]
    pub const fn tool_call_delta(self) -> StreamRequestMode {
        self.tool_call_delta
    }

    /// Returns resolved usage stream mode.
    #[must_use]
    pub const fn usage(self) -> StreamRequestMode {
        self.usage
    }

    /// Returns true when at least one unsupported non-content mode was disabled.
    #[must_use]
    pub const fn downgraded(self) -> bool {
        self.downgraded
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
    /// Explicit streaming capability metadata.
    pub streaming: ModelStreamingCapabilities,
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
    streaming: ModelStreamingCapabilities,
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
            streaming: draft.streaming,
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

    /// Returns explicit streaming capability metadata.
    #[must_use]
    pub const fn streaming(&self) -> ModelStreamingCapabilities {
        self.streaming
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

    /// Resolves requested stream modes against declared model capabilities.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when an unsupported requested mode is
    /// rejected by policy or content streaming itself is unsupported.
    pub fn resolve_streaming(
        &self,
        requested: ModelStreamingRequest,
        policy: UnsupportedStreamPolicy,
    ) -> Result<ResolvedStreamingModes, super::RegistryError> {
        let (content, content_downgraded) = self.resolve_stream_feature(
            StreamFeature::Content,
            requested.mode_for(StreamFeature::Content),
            policy,
        )?;
        let (thought, thought_downgraded) = self.resolve_stream_feature(
            StreamFeature::Thought,
            requested.mode_for(StreamFeature::Thought),
            policy,
        )?;
        let (tool_call_delta, tool_downgraded) = self.resolve_stream_feature(
            StreamFeature::ToolCallDelta,
            requested.mode_for(StreamFeature::ToolCallDelta),
            policy,
        )?;
        let (usage, usage_downgraded) = self.resolve_stream_feature(
            StreamFeature::Usage,
            requested.mode_for(StreamFeature::Usage),
            policy,
        )?;

        Ok(ResolvedStreamingModes::new(
            StreamModeSet {
                content,
                thought,
                tool_call_delta,
                usage,
            },
            content_downgraded || thought_downgraded || tool_downgraded || usage_downgraded,
        ))
    }

    fn resolve_stream_feature(
        &self,
        feature: StreamFeature,
        requested: StreamRequestMode,
        policy: UnsupportedStreamPolicy,
    ) -> Result<(StreamRequestMode, bool), super::RegistryError> {
        if !requested.is_required() {
            return Ok((StreamRequestMode::Disabled, false));
        }

        if self.streaming.support_for(feature).is_supported() {
            return Ok((StreamRequestMode::Required, false));
        }

        if feature == StreamFeature::Content || policy == UnsupportedStreamPolicy::Reject {
            return Err(super::RegistryError::UnsupportedStreamCapability {
                capability: feature.as_str(),
            });
        }

        Ok((StreamRequestMode::Disabled, true))
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

const MAX_ENDPOINT_BYTES: usize = 256;
const MAX_MODALITY_COUNT: usize = 4;
const MAX_THINKING_LEVEL_COUNT: usize = 4;
const DEFAULT_CLAMP_FALLBACK: super::ThinkingLevel = super::ThinkingLevel::None;

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

pub(super) fn non_zero_budget(
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
