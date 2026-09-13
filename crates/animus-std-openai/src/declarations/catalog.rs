/// Exact provider stream channel admitted for a tool-capable Responses model.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenAiToolStreamChannel {
    /// Assistant-visible text deltas.
    Content,
    /// Provider-exposed reasoning-summary deltas.
    ReasoningSummary,
    /// Function-call argument deltas.
    ToolCall,
    /// Token usage at terminal completion.
    Usage,
    /// One explicit terminal event.
    Finish,
}

/// Exact function name and schema label admitted for one selected model.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OpenAiToolCapability {
    name: String,
    schema_label: String,
}

impl OpenAiToolCapability {
    /// Builds one exact tool capability.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when a label is invalid or outside the reviewed cohort.
    pub fn new(name: &str, schema_label: &str) -> Result<Self, crate::HostError> {
        super::validate_label(name)?;
        super::validate_label(schema_label)?;
        if name != super::STEEL_REPL_TOOL_NAME || schema_label != super::STEEL_REPL_TOOL_SCHEMA {
            return super::unsupported("steel_repl_tool_cohort");
        }
        Ok(Self {
            name: String::from(name),
            schema_label: String::from(schema_label),
        })
    }

    /// Returns the exact function name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the exact canonical schema label.
    #[must_use]
    pub fn schema_label(&self) -> &str {
        self.schema_label.as_str()
    }
}

/// Exact selected-model capability metadata for the reviewed tool cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolCapabilityMatrix {
    model_id: String,
    roles: Vec<animus_adapters::CanonicalModelRole>,
    tools: Vec<OpenAiToolCapability>,
    stream_channels: Vec<OpenAiToolStreamChannel>,
}

impl OpenAiToolCapabilityMatrix {
    /// Builds exact selected-model metadata after bounded validation.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] for empty, duplicate, over-limit, or unsafe metadata.
    pub fn new(
        model_id: &str,
        roles: Vec<animus_adapters::CanonicalModelRole>,
        tools: Vec<OpenAiToolCapability>,
        stream_channels: Vec<OpenAiToolStreamChannel>,
    ) -> Result<Self, crate::HostError> {
        super::validate_label(model_id)?;
        let has_invalid_roles = roles.is_empty() || roles.len() > super::MAX_TOOL_CAPABILITIES;
        let has_invalid_channels =
            stream_channels.is_empty() || stream_channels.len() > super::MAX_TOOL_CAPABILITIES;
        if has_invalid_roles || has_invalid_channels || tools.len() != 1 {
            return Err(crate::HostError::InvalidToolCohort {
                reason: super::CAPABILITY_BOUND_REASON,
            });
        }
        super::validate_unique_by(roles.as_slice(), |role| role.as_str())?;
        super::validate_unique(tools.as_slice())?;
        super::validate_unique(stream_channels.as_slice())?;
        debug_assert_eq!(tools.len(), 1);
        debug_assert!(!roles.is_empty());
        Ok(Self {
            model_id: String::from(model_id),
            roles,
            tools,
            stream_channels,
        })
    }

    /// Returns the exact selected model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    /// Returns the admitted canonical roles.
    #[must_use]
    pub fn roles(&self) -> &[animus_adapters::CanonicalModelRole] {
        self.roles.as_slice()
    }

    /// Returns exact tool and schema capabilities.
    #[must_use]
    pub fn tools(&self) -> &[OpenAiToolCapability] {
        self.tools.as_slice()
    }

    /// Returns exact stream channels.
    #[must_use]
    pub fn stream_channels(&self) -> &[OpenAiToolStreamChannel] {
        self.stream_channels.as_slice()
    }
}
