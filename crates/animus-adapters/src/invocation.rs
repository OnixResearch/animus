const FAILURE_MESSAGE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("adapter_failure_message");
const CORRELATION_ID_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("adapter_correlation_id");
const RECEIPT_REFERENCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("adapter_receipt_reference");
const FAILURE_CLASS_ABSENT: &str = "absent";
const FAILURE_CLASS_DENIED: &str = "denied";
const FAILURE_CLASS_UNAVAILABLE: &str = "unavailable";
const FAILURE_CLASS_MALFORMED: &str = "malformed_output";
const FAILURE_CLASS_CANCELLED: &str = "cancelled";
const FAILURE_CLASS_TIMED_OUT: &str = "timed_out";
const FAILURE_CLASS_EXECUTION: &str = "execution_failure";
const FALLBACK_FAILURE_MESSAGE: &str = FAILURE_CLASS_EXECUTION;
const UNSAFE_METADATA_MARKERS: &[&str] = &[
    "/home/",
    "raw_prompt:",
    "provider_payload:",
    "model_output:",
    "tool_output:",
    "begin private key",
    "api_key=",
    "aws_secret_access_key",
    "sk-",
];

// r[impl agent_harness_runtime.boundary_versions.contract_identities]
// r[impl agent_harness_runtime.adapter_outcomes.typed_envelope]
/// Stable contract identity for typed adapter-call outcome envelopes.
pub const ADAPTER_CALL_OUTCOME_CONTRACT_IDENTITY: &str = "animus.adapter-call-outcome.v1";

/// Typed fail-closed class for an adapter call that did not complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterCallFailureClass {
    /// No matching service was injected for the required capability.
    Absent,
    /// Capability or policy denied dispatch before completion.
    Denied,
    /// A matching service existed but could not accept work.
    Unavailable,
    /// The adapter returned data that failed boundary validation.
    MalformedOutput,
    /// The call was cancelled before completion.
    Cancelled,
    /// The call exceeded its caller-owned deadline.
    TimedOut,
    /// Legacy or typed adapter execution failed without a more specific class.
    ExecutionFailure,
}

impl AdapterCallFailureClass {
    /// Returns the stable fail-closed class label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Absent => FAILURE_CLASS_ABSENT,
            Self::Denied => FAILURE_CLASS_DENIED,
            Self::Unavailable => FAILURE_CLASS_UNAVAILABLE,
            Self::MalformedOutput => FAILURE_CLASS_MALFORMED,
            Self::Cancelled => FAILURE_CLASS_CANCELLED,
            Self::TimedOut => FAILURE_CLASS_TIMED_OUT,
            Self::ExecutionFailure => FAILURE_CLASS_EXECUTION,
        }
    }

    /// Maps legacy injected-service outcomes into adapter-call failure classes.
    #[must_use]
    pub const fn from_outcome_kind(outcome: crate::OutcomeKind) -> Option<Self> {
        match outcome {
            crate::OutcomeKind::Absent => Some(Self::Absent),
            crate::OutcomeKind::Unavailable => Some(Self::Unavailable),
            crate::OutcomeKind::Denied => Some(Self::Denied),
            crate::OutcomeKind::Cancelled => Some(Self::Cancelled),
            crate::OutcomeKind::TimedOut => Some(Self::TimedOut),
            crate::OutcomeKind::MalformedOutput => Some(Self::MalformedOutput),
            crate::OutcomeKind::Completed => None,
        }
    }
}

/// Borrowed safe metadata for a non-completed adapter-call outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterCallFailureMetadataDraft<'a> {
    /// Bounded safe message or reason class.
    pub message: &'a str,
    /// Optional safe request or call correlation identifier.
    pub correlation_id: Option<&'a str>,
    /// Optional safe receipt/reference metadata, such as a receipt hash label.
    pub receipt_reference: Option<&'a str>,
}

/// Validated safe metadata for a non-completed adapter-call outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterCallFailureMetadata {
    message: alloc::string::String,
    correlation_id: Option<alloc::string::String>,
    receipt_reference: Option<alloc::string::String>,
}

impl AdapterCallFailureMetadata {
    /// Validates safe adapter-call failure metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when metadata is empty, too long,
    /// contains a NUL byte, or carries known raw prompt, provider, credential,
    /// path, model-output, or tool-output leak markers.
    pub fn new(
        draft: AdapterCallFailureMetadataDraft<'_>,
    ) -> Result<Self, crate::AdapterContractError> {
        let message = safe_required_text(FAILURE_MESSAGE_FIELD, draft.message)?;
        let correlation_id = safe_optional_text(CORRELATION_ID_FIELD, draft.correlation_id)?;
        let receipt_reference =
            safe_optional_text(RECEIPT_REFERENCE_FIELD, draft.receipt_reference)?;
        Ok(Self {
            message,
            correlation_id,
            receipt_reference,
        })
    }

    /// Returns the bounded safe failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns the optional safe correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Returns the optional safe receipt/reference metadata.
    #[must_use]
    pub fn receipt_reference(&self) -> Option<&str> {
        self.receipt_reference.as_deref()
    }

    fn fallback() -> Self {
        Self {
            message: alloc::string::String::from(FALLBACK_FAILURE_MESSAGE),
            correlation_id: None,
            receipt_reference: None,
        }
    }
}

/// Typed adapter-call envelope preserving completed values and fail-closed classes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterCallOutcome<Response> {
    /// Adapter call completed and returned a validated response DTO.
    Completed {
        /// Validated response.
        response: Response,
    },
    /// Adapter call failed closed before a completed response existed.
    Failed {
        /// Stable fail-closed class.
        class: AdapterCallFailureClass,
        /// Bounded safe failure metadata.
        metadata: AdapterCallFailureMetadata,
    },
}

impl<Response> AdapterCallOutcome<Response> {
    /// Builds a completed adapter-call envelope.
    #[must_use]
    pub const fn completed(response: Response) -> Self {
        Self::Completed { response }
    }

    /// Builds a non-completed adapter-call envelope after metadata validation.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when failure metadata is unsafe.
    pub fn failed(
        class: AdapterCallFailureClass,
        metadata: AdapterCallFailureMetadataDraft<'_>,
    ) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::Failed {
            class,
            metadata: AdapterCallFailureMetadata::new(metadata)?,
        })
    }

    /// Builds a fallback fail-closed envelope with a fixed safe failure message.
    #[must_use]
    pub fn fallback_failed(class: AdapterCallFailureClass) -> Self {
        Self::Failed {
            class,
            metadata: AdapterCallFailureMetadata::fallback(),
        }
    }

    /// Returns the stable outcome contract identity.
    #[must_use]
    pub const fn contract_identity() -> &'static str {
        ADAPTER_CALL_OUTCOME_CONTRACT_IDENTITY
    }

    /// Returns true only when the adapter call completed.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    /// Returns the fail-closed class for non-completed outcomes.
    #[must_use]
    pub const fn failure_class(&self) -> Option<AdapterCallFailureClass> {
        match self {
            Self::Completed { .. } => None,
            Self::Failed { class, .. } => Some(*class),
        }
    }
}

// r[impl agent_harness_runtime.adapter_outcomes.compatibility]
/// Opt-in adapter port that returns a typed outcome envelope instead of prose-only errors.
pub trait ComponentOutcomeAdapter<Request, Response>:
    crate::ComponentAdapter<Request, Response>
{
    /// Calls the component and returns a typed completed or fail-closed outcome.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] for transport-level failures before a typed
    /// adapter outcome can be produced.
    fn call_with_outcome(
        &mut self,
        request: &Request,
    ) -> Result<AdapterCallOutcome<Response>, Self::Error>;
}

/// Opt-in model adapter contract with typed adapter-call outcomes.
pub trait ModelOutcomeAdapter:
    ComponentOutcomeAdapter<crate::ModelRequest, crate::ModelResponse>
{
}

impl<T> ModelOutcomeAdapter for T where
    T: ComponentOutcomeAdapter<crate::ModelRequest, crate::ModelResponse>
{
}

/// Opt-in tool adapter contract with typed adapter-call outcomes.
pub trait ToolOutcomeAdapter:
    ComponentOutcomeAdapter<crate::ToolRequest, crate::ToolResponse>
{
}

impl<T> ToolOutcomeAdapter for T where
    T: ComponentOutcomeAdapter<crate::ToolRequest, crate::ToolResponse>
{
}

fn safe_required_text(
    field: crate::validation::FieldName,
    value: &str,
) -> Result<alloc::string::String, crate::AdapterContractError> {
    let normalized = crate::validation::normalized_adapter_field(field, value)?;
    reject_unsafe_metadata(field, normalized.as_str())?;
    Ok(normalized)
}

fn safe_optional_text(
    field: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<alloc::string::String>, crate::AdapterContractError> {
    value
        .map(|text| safe_required_text(field, text))
        .transpose()
}

fn reject_unsafe_metadata(
    field: crate::validation::FieldName,
    value: &str,
) -> Result<(), crate::AdapterContractError> {
    let lower = value.to_ascii_lowercase();
    for marker in UNSAFE_METADATA_MARKERS {
        if lower.contains(marker) {
            return Err(crate::AdapterContractError::UnsupportedStatusState {
                field: field.as_str(),
                value: alloc::string::String::from(*marker),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
