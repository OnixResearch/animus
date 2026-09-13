pub(super) mod plan;
mod prior;

/// Host-owned request admission for an initial or continuation tool call.
#[derive(Clone)]
pub struct OpenAiToolDispatch {
    model_call_id: String,
    capabilities: super::OpenAiToolCapabilityMatrix,
    schemas: Vec<super::OpenAiToolSchemaBinding>,
    policy: super::OpenAiToolPolicy,
    continuation: Option<(super::OpenAiContinuationBinding, super::OpenAiToolResult)>,
}

impl OpenAiToolDispatch {
    /// Builds an initial tool-capable dispatch admission.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when the model-call identity is invalid.
    pub fn initial(
        model_call_id: &str,
        capabilities: super::OpenAiToolCapabilityMatrix,
        schemas: Vec<super::OpenAiToolSchemaBinding>,
        policy: super::OpenAiToolPolicy,
    ) -> Result<Self, crate::HostError> {
        super::validate_label(model_call_id)?;
        Ok(Self {
            model_call_id: String::from(model_call_id),
            capabilities,
            schemas,
            policy,
            continuation: None,
        })
    }

    /// Builds one exact continuation admission from an initial cohort.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when the correlation does not match this model call.
    pub fn continue_with(
        mut self,
        binding: super::OpenAiContinuationBinding,
        result: super::OpenAiToolResult,
    ) -> Result<Self, crate::HostError> {
        if self.model_call_id != binding.model_call_id() {
            return Err(crate::HostError::Correlation {
                reason: super::CONTINUATION_MISMATCH_REASON,
            });
        }
        self.continuation = Some((binding, result));
        Ok(self)
    }

    /// Returns the Animus model-call identity.
    #[must_use]
    pub fn model_call_id(&self) -> &str {
        self.model_call_id.as_str()
    }
    /// Returns exact selected-model capability metadata.
    #[must_use]
    pub const fn capabilities(&self) -> &super::OpenAiToolCapabilityMatrix {
        &self.capabilities
    }
    /// Returns exact schema bindings.
    #[must_use]
    pub fn schemas(&self) -> &[super::OpenAiToolSchemaBinding] {
        self.schemas.as_slice()
    }
    /// Returns the exact serial tool policy.
    #[must_use]
    pub const fn policy(&self) -> &super::OpenAiToolPolicy {
        &self.policy
    }

    pub(crate) fn continuation_parts(
        &self,
    ) -> Option<(&super::OpenAiContinuationBinding, &super::OpenAiToolResult)> {
        self.continuation
            .as_ref()
            .map(|(binding, result)| (binding, result))
    }
}

impl std::fmt::Debug for OpenAiToolDispatch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiToolDispatch")
            .field("model_call_id", &self.model_call_id)
            .field("capabilities", &self.capabilities)
            .field("schemas", &self.schemas.len())
            .field("policy", &self.policy)
            .field("continuation", &self.continuation.is_some())
            .finish()
    }
}
