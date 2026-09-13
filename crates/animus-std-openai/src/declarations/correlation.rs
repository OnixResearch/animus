/// Host-local identity binding for one committed provider function call.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiContinuationBinding {
    model_call_id: String,
    provider_response_id: String,
    output_item_id: String,
    tool_call_id: String,
    tool_name: String,
    arguments_identity: String,
}

impl OpenAiContinuationBinding {
    pub(crate) fn from_completed_call(call: &OpenAiCompletedToolCall) -> Self {
        Self {
            model_call_id: call.model_call_id.clone(),
            provider_response_id: call.provider_response_id.clone(),
            output_item_id: call.output_item_id.clone(),
            tool_call_id: call.tool_call_id.clone(),
            tool_name: call.tool_name.clone(),
            arguments_identity: super::blake3_identity(call.arguments_json.as_bytes()),
        }
    }

    /// Returns the Animus model-call identity.
    #[must_use]
    pub fn model_call_id(&self) -> &str {
        self.model_call_id.as_str()
    }
    /// Returns the provider response identity needed for `previous_response_id`.
    #[must_use]
    pub fn provider_response_id(&self) -> &str {
        self.provider_response_id.as_str()
    }
    /// Returns the provider output-item identity.
    #[must_use]
    pub fn output_item_id(&self) -> &str {
        self.output_item_id.as_str()
    }
    /// Returns the provider function-call identity.
    #[must_use]
    pub fn tool_call_id(&self) -> &str {
        self.tool_call_id.as_str()
    }
    /// Returns the admitted tool name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }
    /// Returns the BLAKE3 identity of the completed JSON arguments.
    #[must_use]
    pub fn arguments_identity(&self) -> &str {
        self.arguments_identity.as_str()
    }
}

impl std::fmt::Debug for OpenAiContinuationBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiContinuationBinding")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("tool_name", &self.tool_name)
            .field("arguments_identity", &self.arguments_identity)
            .finish()
    }
}

/// One bounded provider function call completed by the production parser.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiCompletedToolCall {
    pub(crate) model_call_id: String,
    pub(crate) provider_response_id: String,
    pub(crate) output_item_id: String,
    pub(crate) tool_call_id: String,
    pub(crate) tool_name: String,
    pub(crate) arguments_json: String,
}

impl OpenAiCompletedToolCall {
    /// Returns the exact function name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }
    /// Returns the completed JSON arguments for tool admission outside this adapter.
    #[must_use]
    pub fn arguments_json(&self) -> &str {
        self.arguments_json.as_str()
    }
    /// Returns a host-local continuation binding for this committed call.
    #[must_use]
    pub fn continuation_binding(&self) -> OpenAiContinuationBinding {
        OpenAiContinuationBinding::from_completed_call(self)
    }
}

impl std::fmt::Debug for OpenAiCompletedToolCall {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiCompletedToolCall")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("tool_name", &self.tool_name)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

/// Explicit tool result and correlation facts for one continuation request.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiToolResult {
    model_call_id: String,
    provider_response_id: String,
    output_item_id: String,
    tool_call_id: String,
    output: String,
}

impl OpenAiToolResult {
    /// Builds a result that matches one parser-issued binding.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when an identity is invalid or output exceeds the bound.
    pub fn for_binding(
        binding: &OpenAiContinuationBinding,
        output: &str,
    ) -> Result<Self, crate::HostError> {
        for label in [
            binding.model_call_id(),
            binding.provider_response_id(),
            binding.output_item_id(),
            binding.tool_call_id(),
        ] {
            super::validate_label(label)?;
        }
        if output.len() > super::MAX_TOOL_RESULT_BYTES {
            return Err(crate::HostError::InvalidToolCohort {
                reason: super::RESULT_BOUND_REASON,
            });
        }
        Ok(Self {
            model_call_id: String::from(binding.model_call_id()),
            provider_response_id: String::from(binding.provider_response_id()),
            output_item_id: String::from(binding.output_item_id()),
            tool_call_id: String::from(binding.tool_call_id()),
            output: String::from(output),
        })
    }

    /// Returns the private tool output for request projection.
    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_str()
    }

    pub(super) fn matches(&self, binding: &OpenAiContinuationBinding) -> bool {
        self.model_call_id == binding.model_call_id
            && self.provider_response_id == binding.provider_response_id
            && self.output_item_id == binding.output_item_id
            && self.tool_call_id == binding.tool_call_id
    }
}

impl std::fmt::Debug for OpenAiToolResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiToolResult")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("output", &"[REDACTED]")
            .finish()
    }
}
