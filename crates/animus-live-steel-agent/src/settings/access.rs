impl super::ValidatedLiveSteelAgentConfig {
    /// Returns the application label.
    #[must_use]
    pub fn application_id(&self) -> &str {
        self.raw.application_id.as_str()
    }

    /// Returns the exact session BLAKE3 identity.
    #[must_use]
    pub fn session_identity(&self) -> &str {
        self.raw.session_identity.as_str()
    }

    /// Returns the branch label.
    #[must_use]
    pub fn branch_id(&self) -> &str {
        self.raw.branch_id.as_str()
    }

    /// Returns the deterministic BLAKE3 identity of the validated JSON input.
    #[must_use]
    pub fn config_identity(&self) -> &str {
        self.config_identity.as_str()
    }

    /// Returns a fresh copy with explicit coding-session recovery enabled.
    #[must_use]
    pub fn with_session_resume_required(&self, resume_required: bool) -> Self {
        let mut config = self.clone();
        config.raw.session_resume_required = resume_required;
        config
    }

    /// Returns whether coding-session recovery must run before new work.
    #[must_use]
    pub const fn session_resume_required(&self) -> bool {
        self.raw.session_resume_required
    }

    /// Returns the exact admitted provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.raw.provider.provider_id.as_str()
    }

    /// Returns the exact credential reference. The value is never in config.
    #[must_use]
    pub fn credential_reference(&self) -> &str {
        self.raw.provider.credential_reference.as_str()
    }

    /// Returns the validated provider reasoning effort.
    #[must_use]
    pub fn reasoning_effort(&self) -> &str {
        self.raw.provider.reasoning_effort.as_str()
    }

    /// Returns the exact selected model.
    #[must_use]
    pub const fn selected_model(&self) -> &animus_std::SelectedModel {
        &self.selected_model
    }

    /// Returns the selected model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.raw.provider.model_id.as_str()
    }

    /// Returns the endpoint identity.
    #[must_use]
    pub fn endpoint_identity(&self) -> &str {
        self.raw.provider.endpoint_identity.as_str()
    }

    /// Returns the endpoint URL.
    #[must_use]
    pub fn endpoint_url(&self) -> &str {
        self.raw.provider.endpoint_url.as_str()
    }

    /// Builds the exact admitted OpenAI-compatible host configuration.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentConfigError::ProviderMismatch`] if the lower-level
    /// host rejects the endpoint pair or transport bounds.
    pub fn provider_host_config(
        &self,
    ) -> Result<animus_std_openai::OpenAiHostConfig, super::LiveSteelAgentConfigError> {
        animus_std_openai::OpenAiHostConfig::new(
            self.endpoint_identity(),
            self.endpoint_url(),
            self.provider_timeout(),
            self.max_provider_response_bytes(),
        )
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)
    }

    /// Returns the provider deadline.
    #[must_use]
    pub const fn provider_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.raw.bounds.provider_timeout_ms)
    }

    /// Returns the aggregate provider response byte bound.
    #[must_use]
    pub const fn max_provider_response_bytes(&self) -> u64 {
        self.raw.bounds.max_provider_response_bytes
    }

    /// Returns the canonical output token bound.
    #[must_use]
    pub const fn max_output_tokens(&self) -> u64 {
        self.raw.bounds.max_output_tokens
    }

    /// Returns the model-visible tool result bound.
    #[must_use]
    pub const fn max_tool_result_chars(&self) -> u32 {
        self.raw.bounds.max_tool_result_chars
    }

    /// Returns the queue item bound.
    #[must_use]
    pub const fn max_queue_items(&self) -> u32 {
        self.raw.bounds.max_queue_items
    }

    /// Returns the validated RLM kernel policy.
    #[must_use]
    pub const fn rlm_policy(&self) -> &animus_adapter_rlm_kernel::RlmKernelPolicy {
        &self.rlm_policy
    }

    /// Returns the explicitly selected skill descriptors in deterministic order.
    #[must_use]
    pub fn skill_modules(&self) -> &[animus_adapter_rlm_kernel::SteelSkillModuleDescriptor] {
        self.skill_modules.as_slice()
    }

    /// Returns exact operation labels.
    #[must_use]
    pub fn admitted_operations(&self) -> &[String] {
        self.raw.admitted_operations.as_slice()
    }

    /// Returns exact child profile labels.
    #[must_use]
    pub fn admitted_child_profiles(&self) -> &[String] {
        self.raw.admitted_child_profiles.as_slice()
    }

    /// Returns the durable store capability label.
    #[must_use]
    pub fn store_capability_label(&self) -> &str {
        self.raw.store.capability_label.as_str()
    }

    /// Returns the session journal leaf name.
    #[must_use]
    pub fn session_journal(&self) -> &str {
        self.raw.store.session_journal.as_str()
    }

    /// Returns the session lock leaf name.
    #[must_use]
    pub fn session_lock(&self) -> &str {
        self.raw.store.session_lock.as_str()
    }

    /// Returns the kernel journal leaf name.
    #[must_use]
    pub fn kernel_journal(&self) -> &str {
        self.raw.store.kernel_journal.as_str()
    }

    /// Returns the kernel lock leaf name.
    #[must_use]
    pub fn kernel_lock(&self) -> &str {
        self.raw.store.kernel_lock.as_str()
    }

    /// Returns the configured authority generation.
    #[must_use]
    pub const fn authority_generation(&self) -> u64 {
        self.raw.authority.generation
    }
}
