const MODEL_REQUEST_KIND: &str = "model_request";
const MODEL_RESPONSE_KIND: &str = "model_response";
const TOOL_REQUEST_KIND: &str = "tool_request";
const TOOL_RESPONSE_KIND: &str = "tool_response";

/// Boundary message kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Model request boundary payload.
    ModelRequest,
    /// Model response boundary payload.
    ModelResponse,
    /// Tool request boundary payload.
    ToolRequest,
    /// Tool response boundary payload.
    ToolResponse,
}

impl Kind {
    /// Returns a stable string identifier for the message kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelRequest => MODEL_REQUEST_KIND,
            Self::ModelResponse => MODEL_RESPONSE_KIND,
            Self::ToolRequest => TOOL_REQUEST_KIND,
            Self::ToolResponse => TOOL_RESPONSE_KIND,
        }
    }
}

/// Typed Preserves boundary message.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Envelope {
    /// Model request boundary payload.
    ModelRequest {
        /// Prompt sent to a model adapter.
        prompt: String,
    },
    /// Model response boundary payload.
    ModelResponse {
        /// Message returned by a model adapter.
        message: String,
    },
    /// Tool request boundary payload.
    ToolRequest {
        /// Tool name selected by the harness.
        name: String,
        /// Tool input payload.
        input: String,
    },
    /// Tool response boundary payload.
    ToolResponse {
        /// Tool output payload.
        output: String,
    },
}

impl Envelope {
    /// Returns the boundary kind for this message.
    #[must_use]
    pub const fn kind(&self) -> Kind {
        match self {
            Self::ModelRequest { .. } => Kind::ModelRequest,
            Self::ModelResponse { .. } => Kind::ModelResponse,
            Self::ToolRequest { .. } => Kind::ToolRequest,
            Self::ToolResponse { .. } => Kind::ToolResponse,
        }
    }

    /// Builds a boundary message from a validated model request.
    #[must_use]
    pub fn from_model_request(request: &animus_adapters::ModelRequest) -> Self {
        Self::ModelRequest {
            prompt: String::from(request.prompt()),
        }
    }

    /// Builds a boundary message from a validated model response.
    #[must_use]
    pub fn from_model_response(response: &animus_adapters::ModelResponse) -> Self {
        Self::ModelResponse {
            message: String::from(response.message()),
        }
    }

    /// Builds a boundary message from a validated tool request.
    #[must_use]
    pub fn from_tool_request(request: &animus_adapters::ToolRequest) -> Self {
        Self::ToolRequest {
            name: String::from(request.name()),
            input: String::from(request.input()),
        }
    }

    /// Builds a boundary message from a validated tool response.
    #[must_use]
    pub fn from_tool_response(response: &animus_adapters::ToolResponse) -> Self {
        Self::ToolResponse {
            output: String::from(response.output()),
        }
    }

    /// Converts this decoded message into a validated model request.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_model_request(
        self,
    ) -> Result<animus_adapters::ModelRequest, crate::CodecError> {
        match self {
            Self::ModelRequest { prompt } => Ok(animus_adapters::ModelRequest::new(&prompt)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ModelRequest,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated model response.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_model_response(
        self,
    ) -> Result<animus_adapters::ModelResponse, crate::CodecError> {
        match self {
            Self::ModelResponse { message } => Ok(animus_adapters::ModelResponse::new(&message)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ModelResponse,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated tool request.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_tool_request(self) -> Result<animus_adapters::ToolRequest, crate::CodecError> {
        match self {
            Self::ToolRequest { name, input } => {
                Ok(animus_adapters::ToolRequest::new(&name, &input)?)
            }
            other => Err(crate::CodecError::wrong_kind(
                Kind::ToolRequest,
                other.kind(),
            )),
        }
    }

    /// Converts this decoded message into a validated tool response.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or decoded
    /// payload text violates the adapter contract.
    pub fn try_into_tool_response(
        self,
    ) -> Result<animus_adapters::ToolResponse, crate::CodecError> {
        match self {
            Self::ToolResponse { output } => Ok(animus_adapters::ToolResponse::new(&output)?),
            other => Err(crate::CodecError::wrong_kind(
                Kind::ToolResponse,
                other.kind(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const MODEL_MESSAGE: &str = "model answer";
    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const TOOL_OUTPUT: &str = "result";
    const BLANK_TEXT: &str = "   ";

    #[test]
    fn model_request_round_trips_through_contract() {
        let request =
            animus_adapters::ModelRequest::new(MODEL_PROMPT).expect("valid request should parse");
        let message = Envelope::from_model_request(&request);
        let decoded = message
            .try_into_model_request()
            .expect("boundary message should convert");

        assert_eq!(decoded.prompt(), MODEL_PROMPT);
    }

    #[test]
    fn model_response_round_trips_through_contract() {
        let response = animus_adapters::ModelResponse::new(MODEL_MESSAGE)
            .expect("valid response should parse");
        let message = Envelope::from_model_response(&response);
        let decoded = message
            .try_into_model_response()
            .expect("boundary message should convert");

        assert_eq!(decoded.message(), MODEL_MESSAGE);
    }

    #[test]
    fn tool_request_round_trips_through_contract() {
        let request = animus_adapters::ToolRequest::new(TOOL_NAME, TOOL_INPUT)
            .expect("valid request should parse");
        let message = Envelope::from_tool_request(&request);
        let decoded = message
            .try_into_tool_request()
            .expect("boundary message should convert");

        assert_eq!(decoded.name(), TOOL_NAME);
        assert_eq!(decoded.input(), TOOL_INPUT);
    }

    #[test]
    fn tool_response_round_trips_through_contract() {
        let response =
            animus_adapters::ToolResponse::new(TOOL_OUTPUT).expect("valid response should parse");
        let message = Envelope::from_tool_response(&response);
        let decoded = message
            .try_into_tool_response()
            .expect("boundary message should convert");

        assert_eq!(decoded.output(), TOOL_OUTPUT);
    }

    #[test]
    fn wrong_kind_is_rejected() {
        let message = Envelope::ModelResponse {
            message: String::from(MODEL_MESSAGE),
        };
        let error = message
            .try_into_model_request()
            .expect_err("wrong kind should fail");

        assert!(matches!(
            error,
            crate::CodecError::WrongKind {
                expected: MODEL_REQUEST_KIND,
                actual: MODEL_RESPONSE_KIND,
            }
        ));
    }

    #[test]
    fn invalid_decoded_payload_is_revalidated() {
        let message = Envelope::ModelRequest {
            prompt: String::from(BLANK_TEXT),
        };
        let error = message
            .try_into_model_request()
            .expect_err("blank prompt should fail");

        assert!(matches!(
            error,
            crate::CodecError::AdapterContract {
                source: animus_adapters::AdapterContractError::EmptyField { .. },
            }
        ));
    }
}
