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

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PROMPT: &str = "intent: build reliable systems";
    const BLANK_TEXT: &str = "   ";
    const NUL_TEXT: &str = "bad\0text";
    const NUL_BYTE_INDEX: usize = 3;

    struct EchoAdapter;

    impl crate::ComponentAdapter<Request, Response> for EchoAdapter {
        type Error = crate::AdapterContractError;

        fn call(&mut self, request: &Request) -> Result<Response, Self::Error> {
            Response::new(request.prompt())
        }
    }

    #[test]
    fn adapter_contract_accepts_valid_request_response() {
        let request = Request::new(VALID_PROMPT).expect("valid prompt should parse");
        let mut adapter = EchoAdapter;
        let response = <EchoAdapter as crate::ComponentAdapter<Request, Response>>::call(
            &mut adapter,
            &request,
        )
        .expect("valid echo response should parse");

        assert_eq!(request.prompt(), VALID_PROMPT);
        assert_eq!(response.message(), VALID_PROMPT);
    }

    #[test]
    fn request_from_state_uses_core_summary() {
        let intent =
            animus_core::Intent::new("build reliable systems").expect("valid intent should parse");
        let state = animus_core::AnimusState::new(intent);
        let request = Request::from_state(&state);

        assert_eq!(request.prompt(), VALID_PROMPT);
    }

    #[test]
    fn response_accepts_multiline_text() {
        let response =
            Response::new("first line\nsecond line").expect("multiline response should parse");

        assert_eq!(response.message(), "first line\nsecond line");
    }

    #[test]
    fn response_rejects_empty_text() {
        let error = Response::new(BLANK_TEXT).expect_err("blank response should fail");

        assert_eq!(
            error,
            crate::AdapterContractError::EmptyField {
                field: MESSAGE_FIELD.as_str(),
            }
        );
    }

    #[test]
    fn request_rejects_overlong_prompt() {
        let too_long_prompt_chars =
            crate::MAX_ADAPTER_FIELD_CHARS.saturating_add(crate::MIN_ADAPTER_FIELD_CHARS);
        let raw = "a".repeat(too_long_prompt_chars);
        let error = Request::new(&raw).expect_err("overlong prompt should fail");

        assert_eq!(
            error,
            crate::AdapterContractError::FieldTooLong {
                field: PROMPT_FIELD.as_str(),
                max_chars: crate::MAX_ADAPTER_FIELD_CHARS,
                actual_chars: too_long_prompt_chars,
            }
        );
    }

    #[test]
    fn request_rejects_nul_character() {
        let error = Request::new(NUL_TEXT).expect_err("NUL prompt should fail");

        assert_eq!(
            error,
            crate::AdapterContractError::NulCharacter {
                field: PROMPT_FIELD.as_str(),
                byte_index: NUL_BYTE_INDEX,
            }
        );
    }
}
