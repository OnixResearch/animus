/// Unvalidated model capability metadata supplied by a shell config boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCapabilitiesDraft {
    /// Input modalities declared by the provider config.
    pub input_modalities: Vec<super::basic::InputModality>,
    /// Raw context-token budget.
    pub context_budget: u32,
    /// Raw output-token budget.
    pub output_budget: u32,
    /// Declared reasoning support.
    pub reasoning: super::super::ReasoningSupport,
    /// Raw supported thinking levels.
    pub thinking_levels: Vec<super::super::ThinkingLevel>,
    /// Explicit streaming capability metadata.
    pub streaming: super::flow::ModelStreamingCapabilities,
    /// Optional integer cost metadata.
    pub cost: Option<super::basic::TokenCost>,
}

/// Validated model capability metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCapabilities {
    input_modalities: Vec<super::basic::InputModality>,
    context_budget: core::num::NonZeroU32,
    output_budget: core::num::NonZeroU32,
    reasoning: super::super::ReasoningSupport,
    thinking_levels: Vec<super::super::ThinkingLevel>,
    streaming: super::flow::ModelStreamingCapabilities,
    cost: Option<super::basic::TokenCost>,
}

impl ModelCapabilities {
    /// Builds model capability metadata after boundary validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::super::RegistryError`] when budgets are zero or bounded lists are invalid.
    pub fn from_draft(draft: ModelCapabilitiesDraft) -> Result<Self, super::super::RegistryError> {
        super::validate_collection(&super::CollectionBounds::new(
            super::super::MODALITIES_FIELD,
            draft.input_modalities.len(),
            super::MAX_MODALITY_COUNT,
        ))?;
        super::validate_collection(&super::CollectionBounds::new(
            super::super::THINKING_LEVELS_FIELD,
            draft.thinking_levels.len(),
            super::MAX_THINKING_LEVEL_COUNT,
        ))?;
        let context_budget =
            super::non_zero_budget(super::super::CONTEXT_BUDGET_FIELD, draft.context_budget)?;
        let output_budget =
            super::non_zero_budget(super::super::OUTPUT_BUDGET_FIELD, draft.output_budget)?;
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
    pub fn input_modalities(&self) -> &[super::basic::InputModality] {
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
    pub const fn reasoning(&self) -> super::super::ReasoningSupport {
        self.reasoning
    }

    /// Returns supported thinking levels.
    #[must_use]
    pub fn thinking_levels(&self) -> &[super::super::ThinkingLevel] {
        &self.thinking_levels
    }

    /// Returns explicit streaming capability metadata.
    #[must_use]
    pub const fn streaming(&self) -> super::flow::ModelStreamingCapabilities {
        self.streaming
    }

    /// Returns optional integer cost metadata.
    #[must_use]
    pub const fn cost(&self) -> Option<super::basic::TokenCost> {
        self.cost
    }

    /// Resolves a requested thinking level according to the supplied policy.
    ///
    /// # Errors
    ///
    /// Returns [`super::super::RegistryError`] when `policy` rejects an unsupported level.
    pub fn resolve_thinking(
        &self,
        requested: super::super::ThinkingLevel,
        policy: super::super::ThinkingPolicy,
    ) -> Result<super::super::ThinkingLevel, super::super::RegistryError> {
        if self.thinking_levels.contains(&requested) {
            return Ok(requested);
        }

        match policy {
            super::super::ThinkingPolicy::Clamp => Ok(self.clamp_thinking(requested)),
            super::super::ThinkingPolicy::Reject => {
                Err(super::super::RegistryError::UnsupportedThinkingLevel {
                    requested: requested.as_str(),
                })
            }
        }
    }

    /// Resolves requested stream modes against declared model capabilities.
    ///
    /// # Errors
    ///
    /// Returns [`super::super::RegistryError`] when an unsupported requested mode is
    /// rejected by policy or content streaming itself is unsupported.
    pub fn resolve_streaming(
        &self,
        requested: super::flow::ModelStreamingRequest,
        policy: super::flow::UnsupportedStreamPolicy,
    ) -> Result<super::flow::ResolvedStreamingModes, super::super::RegistryError> {
        let (content, content_downgraded) = self.resolve_stream_feature(
            super::flow::StreamFeature::Content,
            requested.mode_for(super::flow::StreamFeature::Content),
            policy,
        )?;
        let (thought, thought_downgraded) = self.resolve_stream_feature(
            super::flow::StreamFeature::Thought,
            requested.mode_for(super::flow::StreamFeature::Thought),
            policy,
        )?;
        let (tool_call_delta, tool_downgraded) = self.resolve_stream_feature(
            super::flow::StreamFeature::ToolCallDelta,
            requested.mode_for(super::flow::StreamFeature::ToolCallDelta),
            policy,
        )?;
        let (usage, usage_downgraded) = self.resolve_stream_feature(
            super::flow::StreamFeature::Usage,
            requested.mode_for(super::flow::StreamFeature::Usage),
            policy,
        )?;

        Ok(super::flow::ResolvedStreamingModes::new(
            content,
            thought,
            tool_call_delta,
            usage,
            content_downgraded || thought_downgraded || tool_downgraded || usage_downgraded,
        ))
    }

    fn resolve_stream_feature(
        &self,
        feature: super::flow::StreamFeature,
        requested: super::flow::StreamRequestMode,
        policy: super::flow::UnsupportedStreamPolicy,
    ) -> Result<(super::flow::StreamRequestMode, bool), super::super::RegistryError> {
        if !requested.is_required() {
            return Ok((super::flow::StreamRequestMode::Disabled, false));
        }

        if self.streaming.support_for(feature).is_supported() {
            return Ok((super::flow::StreamRequestMode::Required, false));
        }

        if feature == super::flow::StreamFeature::Content
            || policy == super::flow::UnsupportedStreamPolicy::Reject
        {
            return Err(super::super::RegistryError::UnsupportedStreamCapability {
                capability: feature.as_str(),
            });
        }

        Ok((super::flow::StreamRequestMode::Disabled, true))
    }

    fn clamp_thinking(
        &self,
        requested: super::super::ThinkingLevel,
    ) -> super::super::ThinkingLevel {
        self.thinking_levels
            .iter()
            .copied()
            .filter(|level| level.rank() <= requested.rank())
            .max_by_key(|level| level.rank())
            .unwrap_or(super::DEFAULT_CLAMP_FALLBACK)
    }
}
