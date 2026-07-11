const MODEL_REQUEST_KIND: &str = "model_request";
const MODEL_RESPONSE_KIND: &str = "model_response";
const TOOL_REQUEST_KIND: &str = "tool_request";
const TOOL_RESPONSE_KIND: &str = "tool_response";
const COMPLETE_STATUS: &str = "complete";
const TRUNCATED_STATUS: &str = "truncated";
const SAFE_REDACTION: &str = "safe";
const REDACTED_REDACTION: &str = "redacted";
const SECRET_ADJACENT_REDACTION: &str = "secret_adjacent";
const MODEL_VALIDATION_REASON: &str = "owning model DTO rejected decoded fields";
const TOOL_VALIDATION_REASON: &str = "owning tool DTO rejected decoded fields";
const PAYLOAD_KIND_REASON: &str = "requested DTO does not match the encoded payload kind";
const TOOL_STATUS_REASON: &str = "unsupported bounded-output status";
const TOOL_REDACTION_REASON: &str = "unsupported output redaction class";

/// Stable adapter payload kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterKind {
    /// Model request.
    ModelRequest,
    /// Model response.
    ModelResponse,
    /// Tool request.
    ToolRequest,
    /// Tool response.
    ToolResponse,
}

impl AdapterKind {
    /// Returns the stable kind label.
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

/// One validated tool argument field in a Preserves payload.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolArgumentRecord {
    name: String,
    value: String,
}

/// Current-shape tool request payload without runtime dependencies.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolRequestRecord {
    name: String,
    schema_identity: String,
    arguments: Vec<ToolArgumentRecord>,
    correlation_id: String,
    visible_char_budget: u32,
    mutation_targets: Vec<String>,
    required_capability_labels: Vec<String>,
    approval_posture: String,
}

impl ToolRequestRecord {
    fn from_request(request: &animus_adapters::ToolRequest) -> Self {
        Self {
            name: String::from(request.name()),
            schema_identity: String::from(request.schema_identity()),
            arguments: request
                .arguments()
                .iter()
                .map(|argument| ToolArgumentRecord {
                    name: String::from(argument.name()),
                    value: String::from(argument.value()),
                })
                .collect(),
            correlation_id: String::from(request.correlation_id()),
            visible_char_budget: request.output_policy().visible_char_budget,
            mutation_targets: request.mutation_targets().to_vec(),
            required_capability_labels: request.required_capability_labels().to_vec(),
            approval_posture: String::from(request.approval_posture().as_str()),
        }
    }

    fn try_into_request(self) -> Result<animus_adapters::ToolRequest, crate::Error> {
        let argument_names: Vec<&str> = self
            .arguments
            .iter()
            .map(|argument| argument.name.as_str())
            .collect();
        let argument_fields: Vec<animus_adapters::tool::ArgumentFieldDraft<'_>> = self
            .arguments
            .iter()
            .map(|argument| animus_adapters::tool::ArgumentFieldDraft {
                name: Some(argument.name.as_str()),
                value: Some(argument.value.as_str()),
            })
            .collect();
        let prepared = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: self.schema_identity.as_str(),
                allowed_field_names: argument_names.as_slice(),
                compatibility_rules: &[],
                fields: argument_fields.as_slice(),
            },
        )
        .map_err(|_| invalid_tool_payload(TOOL_REQUEST_KIND, TOOL_VALIDATION_REASON))?;
        let mutation_targets: Vec<&str> =
            self.mutation_targets.iter().map(String::as_str).collect();
        let capability_labels: Vec<&str> = self
            .required_capability_labels
            .iter()
            .map(String::as_str)
            .collect();
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(self.name.as_str()),
            arguments: Some(&prepared),
            correlation_id: Some(self.correlation_id.as_str()),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: self.visible_char_budget,
            },
            mutation_targets: mutation_targets.as_slice(),
            required_capability_labels: capability_labels.as_slice(),
            approval_posture: self.approval_posture.as_str(),
        })
        .map_err(|_| invalid_tool_payload(TOOL_REQUEST_KIND, TOOL_VALIDATION_REASON))
    }
}

/// Bounded tool response payload without a raw full-output body.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolResponseRecord {
    visible_text: String,
    status: String,
    byte_count: u64,
    char_count: u64,
    line_count: u64,
    content_hash: Option<String>,
    full_output_reference: Option<String>,
    redaction_class: String,
}

impl ToolResponseRecord {
    fn from_response(response: &animus_adapters::ToolResponse) -> Self {
        Self {
            visible_text: String::from(response.model_visible_text()),
            status: String::from(output_status_label(response.status())),
            byte_count: response.byte_count(),
            char_count: response.char_count(),
            line_count: response.line_count(),
            content_hash: response.content_hash().map(String::from),
            full_output_reference: response.full_output_reference().map(String::from),
            redaction_class: String::from(response.redaction_class().as_str()),
        }
    }

    fn try_into_response(self) -> Result<animus_adapters::ToolResponse, crate::Error> {
        let status = parse_output_status(&self.status)?;
        let redaction_class = parse_redaction_class(&self.redaction_class)?;
        let bounded = animus_adapters::tool::BoundedOutput::from_metadata(
            &animus_adapters::tool::BoundedOutputMetadataDraft {
                visible_text: self.visible_text.as_str(),
                status,
                byte_count: self.byte_count,
                char_count: self.char_count,
                line_count: self.line_count,
                content_hash: self.content_hash.as_deref(),
                full_output_reference: self.full_output_reference.as_deref(),
            },
        )
        .map_err(|_| invalid_tool_payload(TOOL_RESPONSE_KIND, TOOL_VALIDATION_REASON))?;
        Ok(animus_adapters::ToolResponse::from_bounded(
            bounded,
            redaction_class,
        ))
    }
}

/// Typed adapter-only Preserves payload.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AdapterMessage {
    /// Model request payload.
    ModelRequest {
        /// Validated model prompt.
        prompt: String,
    },
    /// Model response payload.
    ModelResponse {
        /// Validated model response message.
        message: String,
    },
    /// Tool request payload.
    ToolRequest {
        /// Current-shape request record.
        request: ToolRequestRecord,
    },
    /// Tool response payload.
    ToolResponse {
        /// Bounded response record.
        response: ToolResponseRecord,
    },
}

impl AdapterMessage {
    /// Builds a model request payload from an owning adapter DTO.
    #[must_use]
    pub fn from_model_request(request: &animus_adapters::ModelRequest) -> Self {
        Self::ModelRequest {
            prompt: String::from(request.prompt()),
        }
    }

    /// Builds a model response payload from an owning adapter DTO.
    #[must_use]
    pub fn from_model_response(response: &animus_adapters::ModelResponse) -> Self {
        Self::ModelResponse {
            message: String::from(response.message()),
        }
    }

    /// Builds a tool request payload from an owning adapter DTO.
    #[must_use]
    pub fn from_tool_request(request: &animus_adapters::ToolRequest) -> Self {
        Self::ToolRequest {
            request: ToolRequestRecord::from_request(request),
        }
    }

    /// Builds a bounded tool response payload from an owning adapter DTO.
    #[must_use]
    pub fn from_tool_response(response: &animus_adapters::ToolResponse) -> Self {
        Self::ToolResponse {
            response: ToolResponseRecord::from_response(response),
        }
    }

    /// Returns the stable payload kind.
    #[must_use]
    pub const fn kind(&self) -> AdapterKind {
        match self {
            Self::ModelRequest { .. } => AdapterKind::ModelRequest,
            Self::ModelResponse { .. } => AdapterKind::ModelResponse,
            Self::ToolRequest { .. } => AdapterKind::ToolRequest,
            Self::ToolResponse { .. } => AdapterKind::ToolResponse,
        }
    }

    /// Reconstructs and revalidates an owning model request DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the kind or payload is invalid.
    pub fn try_into_model_request(self) -> Result<animus_adapters::ModelRequest, crate::Error> {
        let Self::ModelRequest { prompt } = self else {
            return Err(invalid_kind(MODEL_REQUEST_KIND));
        };
        animus_adapters::ModelRequest::new(&prompt)
            .map_err(|_| invalid_model_payload(MODEL_REQUEST_KIND))
    }

    /// Reconstructs and revalidates an owning model response DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the kind or payload is invalid.
    pub fn try_into_model_response(self) -> Result<animus_adapters::ModelResponse, crate::Error> {
        let Self::ModelResponse { message } = self else {
            return Err(invalid_kind(MODEL_RESPONSE_KIND));
        };
        animus_adapters::ModelResponse::new(&message)
            .map_err(|_| invalid_model_payload(MODEL_RESPONSE_KIND))
    }

    /// Reconstructs and revalidates an owning tool request DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the kind or payload is invalid.
    pub fn try_into_tool_request(self) -> Result<animus_adapters::ToolRequest, crate::Error> {
        let Self::ToolRequest { request } = self else {
            return Err(invalid_kind(TOOL_REQUEST_KIND));
        };
        request.try_into_request()
    }

    /// Reconstructs and revalidates an owning tool response DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the kind or payload is invalid.
    pub fn try_into_tool_response(self) -> Result<animus_adapters::ToolResponse, crate::Error> {
        let Self::ToolResponse { response } = self else {
            return Err(invalid_kind(TOOL_RESPONSE_KIND));
        };
        response.try_into_response()
    }

    pub(crate) fn validate(&self) -> Result<(), crate::Error> {
        match self.clone() {
            Self::ModelRequest { .. } => self.clone().try_into_model_request().map(|_| ()),
            Self::ModelResponse { .. } => self.clone().try_into_model_response().map(|_| ()),
            Self::ToolRequest { request } => request.try_into_request().map(|_| ()),
            Self::ToolResponse { response } => response.try_into_response().map(|_| ()),
        }
    }
}

fn output_status_label(status: animus_adapters::tool::OutputStatus) -> &'static str {
    match status {
        animus_adapters::tool::OutputStatus::Complete => COMPLETE_STATUS,
        animus_adapters::tool::OutputStatus::Truncated => TRUNCATED_STATUS,
    }
}

fn parse_output_status(status: &str) -> Result<animus_adapters::tool::OutputStatus, crate::Error> {
    match status {
        COMPLETE_STATUS => Ok(animus_adapters::tool::OutputStatus::Complete),
        TRUNCATED_STATUS => Ok(animus_adapters::tool::OutputStatus::Truncated),
        _ => Err(invalid_tool_payload(TOOL_RESPONSE_KIND, TOOL_STATUS_REASON)),
    }
}

fn parse_redaction_class(
    redaction: &str,
) -> Result<animus_adapters::tool::OutputRedactionClass, crate::Error> {
    match redaction {
        SAFE_REDACTION => Ok(animus_adapters::tool::OutputRedactionClass::Safe),
        REDACTED_REDACTION => Ok(animus_adapters::tool::OutputRedactionClass::Redacted),
        SECRET_ADJACENT_REDACTION => {
            Ok(animus_adapters::tool::OutputRedactionClass::SecretAdjacent)
        }
        _ => Err(invalid_tool_payload(
            TOOL_RESPONSE_KIND,
            TOOL_REDACTION_REASON,
        )),
    }
}

const fn invalid_model_payload(kind: &'static str) -> crate::Error {
    crate::Error::InvalidAdapterPayload {
        kind,
        reason: MODEL_VALIDATION_REASON,
    }
}

const fn invalid_tool_payload(kind: &'static str, reason: &'static str) -> crate::Error {
    crate::Error::InvalidAdapterPayload { kind, reason }
}

const fn invalid_kind(kind: &'static str) -> crate::Error {
    crate::Error::InvalidAdapterPayload {
        kind,
        reason: PAYLOAD_KIND_REASON,
    }
}
