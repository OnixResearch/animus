#[cfg(test)]
mod tests;
pub(crate) mod wire;

const PROMPT_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("prompt");
const MESSAGE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_message");

/// Request sent from the harness to a model component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    prompt: alloc::string::String,
}

impl Request {
    /// Builds a model request from validated prompt text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the prompt is empty, too
    /// long, or contains a NUL character.
    pub fn new(prompt: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(PROMPT_FIELD, prompt)?;
        Ok(Self {
            prompt: alloc::string::String::from(prompt),
        })
    }

    /// Builds a model request from deterministic core state.
    #[must_use]
    pub fn from_state(state: &animus_core::AnimusState) -> Self {
        Self {
            prompt: state.summary(),
        }
    }

    /// Returns the prompt text.
    #[must_use]
    pub fn prompt(&self) -> &str {
        self.prompt.as_str()
    }
}

/// Response returned by a model component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    message: alloc::string::String,
}

impl Response {
    /// Builds a model response from validated message text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the message is empty, too
    /// long, or contains a NUL character.
    pub fn new(message: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(MESSAGE_FIELD, message)?;
        Ok(Self {
            message: alloc::string::String::from(message),
        })
    }

    /// Returns the model message text.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}
