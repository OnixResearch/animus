/// Shared continuation state between coding-session provider and tool adapters.
pub(crate) struct SharedProviderState {
    pending: Option<PendingCall>,
    next_call: u64,
}

impl Default for SharedProviderState {
    fn default() -> Self {
        Self {
            pending: None,
            next_call: super::FIRST_CALL,
        }
    }
}

pub(crate) struct PendingCall {
    binding: animus_std_openai::OpenAiContinuationBinding,
    capabilities: animus_std_openai::OpenAiToolCapabilityMatrix,
    schemas: Vec<animus_std_openai::OpenAiToolSchemaBinding>,
    result: Option<animus_std_openai::OpenAiToolResult>,
}

impl SharedProviderState {
    pub(crate) fn next_model_call_id(&mut self) -> Result<String, super::ProviderStreamError> {
        let next_call = self
            .next_call
            .checked_add(super::CALL_INCREMENT)
            .ok_or(super::ProviderStreamError::Planning)?;
        let identity = format!("{}{}", super::MODEL_CALL_PREFIX, self.next_call);
        self.next_call = next_call;
        Ok(identity)
    }

    pub(crate) fn pending(&self) -> Option<&PendingCall> {
        self.pending.as_ref()
    }

    pub(crate) fn pending_mut(&mut self) -> Option<&mut PendingCall> {
        self.pending.as_mut()
    }

    pub(crate) fn install(&mut self, pending: PendingCall) {
        self.pending = Some(pending);
    }

    pub(crate) fn take(&mut self) -> Option<PendingCall> {
        self.pending.take()
    }
}

impl PendingCall {
    pub(crate) fn correlation_id(&self) -> &str {
        self.binding.tool_call_id()
    }

    pub(crate) fn bind_result(&mut self, output: &str) -> Result<(), super::ProviderStreamError> {
        if self.result.is_some() {
            return Err(super::ProviderStreamError::ToolCohort);
        }
        self.result = Some(
            animus_std_openai::OpenAiToolResult::for_binding(&self.binding, output)
                .map_err(|_error| super::ProviderStreamError::ToolCohort)?,
        );
        Ok(())
    }

    pub(crate) fn into_dispatch(
        self,
    ) -> Result<animus_std_openai::OpenAiToolDispatch, super::ProviderStreamError> {
        let result = self.result.ok_or(super::ProviderStreamError::ToolCohort)?;
        let model_call_id = String::from(self.binding.model_call_id());
        animus_std_openai::OpenAiToolDispatch::initial(
            model_call_id.as_str(),
            self.capabilities,
            self.schemas,
            animus_std_openai::OpenAiToolPolicy::serial(
                animus_std_openai::OpenAiToolChoice::auto(),
            ),
        )
        .map_err(|_error| super::ProviderStreamError::ToolCohort)?
        .continue_with(self.binding, result)
        .map_err(|_error| super::ProviderStreamError::ToolCohort)
    }
}

pub(crate) fn initial_dispatch(
    call: InitialCall<'_>,
) -> Result<animus_std_openai::OpenAiToolDispatch, super::ProviderStreamError> {
    animus_std_openai::OpenAiToolDispatch::initial(
        call.model_call_id,
        super::planning::tool_capabilities(call.model_id)?,
        super::planning::tool_schemas()?,
        animus_std_openai::OpenAiToolPolicy::serial(animus_std_openai::OpenAiToolChoice::auto()),
    )
    .map_err(|_error| super::ProviderStreamError::ToolCohort)
}

pub(crate) fn pending_from_call(
    model_id: &str,
    call: &animus_std_openai::OpenAiCompletedToolCall,
) -> Result<PendingCall, super::ProviderStreamError> {
    Ok(PendingCall {
        binding: call.continuation_binding(),
        capabilities: super::planning::tool_capabilities(model_id)?,
        schemas: super::planning::tool_schemas()?,
        result: None,
    })
}

#[derive(Clone, Copy)]
pub(crate) struct InitialCall<'a> {
    pub(crate) model_id: &'a str,
    pub(crate) model_call_id: &'a str,
}

#[cfg(test)]
mod tests {
    #[test]
    fn exhausted_model_call_sequence_fails_without_state_change() {
        let mut state = super::SharedProviderState {
            pending: None,
            next_call: u64::MAX,
        };
        assert!(matches!(
            state.next_model_call_id(),
            Err(super::super::ProviderStreamError::Planning)
        ));
        assert_eq!(state.next_call, u64::MAX);
        assert!(state.pending().is_none());
    }
}
