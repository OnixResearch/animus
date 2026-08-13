use std::collections::VecDeque;

const TOOL_DESCRIPTION: &str = "Evaluate one bounded Steel expression.";
const TOOL_SCHEMA_JSON: &[u8] = br#"{
  "type": "object",
  "properties": {"code": {"type": "string"}},
  "required": ["code"],
  "additionalProperties": false
}"#;
const MODEL_CALL_PREFIX: &str = "live-steel-call-";
const FIRST_CALL: u64 = 1;
const CALL_INCREMENT: u64 = 1;

/// Provider stream failure projected without private provider content.
#[derive(Debug)]
pub enum ProviderStreamError {
    /// The provider service had no next deterministic response.
    Unavailable,
    /// Canonical request construction failed.
    Canonical,
    /// Provider dispatch planning failed.
    Planning,
    /// Tool cohort construction failed.
    ToolCohort,
    /// Transport or production SSE parsing failed.
    Provider,
    /// The stream did not end in an admitted completed state.
    Terminal,
}

impl core::fmt::Display for ProviderStreamError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "provider stream rejected: {self:?}")
    }
}

impl std::error::Error for ProviderStreamError {}

/// Injected boundary for one admitted provider stream.
///
/// The shell owns request planning and exact continuation. Implementations own
/// only stream acquisition. The fixture implementation uses the same public
/// production parser as the live HTTPS implementation.
pub trait ProviderStreamService {
    /// Runs one admitted provider stream.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderStreamError`] when transport, parsing, or fixture
    /// availability fails.
    fn stream(
        &mut self,
        plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, ProviderStreamError>;
}

/// Deterministic SSE-backed provider that shares the production parser.
pub struct FixtureStreamProvider {
    streams: VecDeque<Vec<u8>>,
    max_response_bytes: usize,
}

impl FixtureStreamProvider {
    /// Builds a deterministic provider from ordered complete SSE streams.
    #[must_use]
    pub fn new(streams: impl IntoIterator<Item = Vec<u8>>, max_response_bytes: usize) -> Self {
        Self {
            streams: streams.into_iter().collect(),
            max_response_bytes,
        }
    }

    /// Returns the remaining response count.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.streams.len()
    }
}

impl ProviderStreamService for FixtureStreamProvider {
    fn stream(
        &mut self,
        _plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, ProviderStreamError> {
        let stream = self
            .streams
            .pop_front()
            .ok_or(ProviderStreamError::Unavailable)?;
        animus_std_openai::parse_openai_responses_sse_chunks(
            dispatch,
            animus_std_openai::OpenAiStreamBounds::reviewed(self.max_response_bytes),
            &[stream.as_slice()],
        )
        .map_err(|_error| ProviderStreamError::Provider)
    }
}

/// Live HTTPS provider with an explicitly injected API credential.
#[cfg(feature = "live-provider")]
pub struct LiveOpenAiProvider {
    host: animus_std_openai::ReqwestOpenAiHost,
    api_key: animus_std_openai::OpenAiApiKey,
    cancellation: FixedStreamingCancellation,
}

/// Stub type when the separately opt-in live provider is not compiled.
#[cfg(not(feature = "live-provider"))]
pub struct LiveOpenAiProvider;

#[cfg(feature = "live-provider")]
impl LiveOpenAiProvider {
    /// Builds the live provider from an admitted host and injected credential.
    #[must_use]
    pub const fn new(
        host: animus_std_openai::ReqwestOpenAiHost,
        api_key: animus_std_openai::OpenAiApiKey,
    ) -> Self {
        Self {
            host,
            api_key,
            cancellation: FixedStreamingCancellation { cancelled: false },
        }
    }
}

#[cfg(feature = "live-provider")]
impl ProviderStreamService for LiveOpenAiProvider {
    fn stream(
        &mut self,
        plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, ProviderStreamError> {
        self.host
            .execute_tool_stream(animus_std_openai::OpenAiToolStreamRequest {
                plan,
                dispatch,
                api_key: &self.api_key,
                cancellation: &self.cancellation,
            })
            .map_err(|_error| ProviderStreamError::Provider)
    }
}

#[cfg(feature = "live-provider")]
struct FixedStreamingCancellation {
    cancelled: bool,
}

#[cfg(feature = "live-provider")]
impl animus_std_openai::StreamingCancellation for FixedStreamingCancellation {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

/// Shared continuation state between coding-session provider and tool adapters.
pub(crate) struct SharedProviderState {
    pending: Option<PendingContinuation>,
    next_call: u64,
}

impl Default for SharedProviderState {
    fn default() -> Self {
        Self {
            pending: None,
            next_call: FIRST_CALL,
        }
    }
}

pub(crate) struct PendingContinuation {
    binding: animus_std_openai::OpenAiContinuationBinding,
    capabilities: animus_std_openai::OpenAiToolCapabilityMatrix,
    schemas: Vec<animus_std_openai::OpenAiToolSchemaBinding>,
    result: Option<animus_std_openai::OpenAiToolResult>,
}

impl SharedProviderState {
    pub(crate) fn next_model_call_id(&mut self) -> String {
        let identity = format!("{MODEL_CALL_PREFIX}{}", self.next_call);
        self.next_call = self.next_call.saturating_add(CALL_INCREMENT);
        identity
    }

    pub(crate) fn pending(&self) -> Option<&PendingContinuation> {
        self.pending.as_ref()
    }

    pub(crate) fn pending_mut(&mut self) -> Option<&mut PendingContinuation> {
        self.pending.as_mut()
    }

    pub(crate) fn install(&mut self, pending: PendingContinuation) {
        self.pending = Some(pending);
    }

    pub(crate) fn take(&mut self) -> Option<PendingContinuation> {
        self.pending.take()
    }
}

impl PendingContinuation {
    pub(crate) fn correlation_id(&self) -> &str {
        self.binding.tool_call_id()
    }

    pub(crate) fn bind_result(&mut self, output: &str) -> Result<(), ProviderStreamError> {
        if self.result.is_some() {
            return Err(ProviderStreamError::ToolCohort);
        }
        self.result = Some(
            animus_std_openai::OpenAiToolResult::for_binding(&self.binding, output)
                .map_err(|_error| ProviderStreamError::ToolCohort)?,
        );
        Ok(())
    }

    pub(crate) fn into_dispatch(
        self,
    ) -> Result<animus_std_openai::OpenAiToolDispatch, ProviderStreamError> {
        let result = self.result.ok_or(ProviderStreamError::ToolCohort)?;
        let model_call_id = String::from(self.binding.model_call_id());
        animus_std_openai::OpenAiToolDispatch::initial(
            model_call_id.as_str(),
            self.capabilities,
            self.schemas,
            animus_std_openai::OpenAiToolPolicy::serial(
                animus_std_openai::OpenAiToolChoice::auto(),
            ),
        )
        .map_err(|_error| ProviderStreamError::ToolCohort)?
        .continue_with(self.binding, result)
        .map_err(|_error| ProviderStreamError::ToolCohort)
    }
}

pub(crate) fn initial_dispatch(
    model_id: &str,
    model_call_id: &str,
) -> Result<animus_std_openai::OpenAiToolDispatch, ProviderStreamError> {
    animus_std_openai::OpenAiToolDispatch::initial(
        model_call_id,
        tool_capabilities(model_id)?,
        tool_schemas()?,
        animus_std_openai::OpenAiToolPolicy::serial(
            animus_std_openai::OpenAiToolChoice::auto(),
        ),
    )
    .map_err(|_error| ProviderStreamError::ToolCohort)
}

pub(crate) fn pending_from_call(
    model_id: &str,
    call: &animus_std_openai::OpenAiCompletedToolCall,
) -> Result<PendingContinuation, ProviderStreamError> {
    Ok(PendingContinuation {
        binding: call.continuation_binding(),
        capabilities: tool_capabilities(model_id)?,
        schemas: tool_schemas()?,
        result: None,
    })
}

pub(crate) fn plan_request(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    continuation: bool,
    correlation_id: Option<&str>,
) -> Result<animus_std::ProviderDispatchPlan, ProviderStreamError> {
    let text = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: request.request().prompt(),
    }];
    let message = animus_adapters::CanonicalModelMessageDraft {
        role: if continuation { "tool" } else { "user" },
        content: &text,
        tool_call_id: correlation_id,
    };
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: animus_std_openai::STEEL_REPL_TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
    }];
    let canonical = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &[message],
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(
                    u32::try_from(config.max_output_tokens())
                        .map_err(|_error| ProviderStreamError::Canonical)?,
                ),
                temperature_millis: None,
                stop_sequences: &[],
                stream: true,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: true,
                effort: animus_adapters::CanonicalModelReasoningEffort::Low,
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .map_err(|_error| ProviderStreamError::Canonical)?;
    let support = animus_adapters::ProviderProjectionSupport {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses,
        developer_role: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        images: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        tools: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        json_object: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        json_schema: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        reasoning: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        streaming: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        metadata: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
    };
    let projection = animus_adapters::project_model_provider_request_fixture(&canonical, support)
        .map_err(|_error| ProviderStreamError::Planning)?;
    let expected = projection
        .literal_parts()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: config.selected_model(),
        canonical_request: &canonical,
        reviewed_projection_fixture: expected.as_slice(),
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(config.credential_reference()),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline.live-steel-agent"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancellation.live-steel-agent"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .map_err(|_error| ProviderStreamError::Planning)
}

pub(crate) fn final_text(
    outcome: &animus_std_openai::OpenAiStreamOutcome,
) -> Result<String, ProviderStreamError> {
    if outcome.terminal() != animus_std_openai::OpenAiStreamTerminal::Completed {
        return Err(ProviderStreamError::Terminal);
    }
    let text = outcome
        .frames()
        .iter()
        .filter(|frame| frame.is_default_final_visible())
        .filter_map(animus_adapters::ModelStreamFrame::body)
        .collect::<String>();
    if text.is_empty() {
        return Err(ProviderStreamError::Terminal);
    }
    Ok(text)
}

fn tool_capabilities(
    model_id: &str,
) -> Result<animus_std_openai::OpenAiToolCapabilityMatrix, ProviderStreamError> {
    animus_std_openai::OpenAiToolCapabilityMatrix::new(
        model_id,
        vec![
            animus_adapters::CanonicalModelRole::User,
            animus_adapters::CanonicalModelRole::Tool,
        ],
        vec![
            animus_std_openai::OpenAiToolCapability::new(
                animus_std_openai::STEEL_REPL_TOOL_NAME,
                animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            )
            .map_err(|_error| ProviderStreamError::ToolCohort)?,
        ],
        vec![
            animus_std_openai::OpenAiToolStreamChannel::Content,
            animus_std_openai::OpenAiToolStreamChannel::ReasoningSummary,
            animus_std_openai::OpenAiToolStreamChannel::ToolCall,
            animus_std_openai::OpenAiToolStreamChannel::Usage,
            animus_std_openai::OpenAiToolStreamChannel::Finish,
        ],
    )
    .map_err(|_error| ProviderStreamError::ToolCohort)
}

fn tool_schemas(
) -> Result<Vec<animus_std_openai::OpenAiToolSchemaBinding>, ProviderStreamError> {
    Ok(vec![
        animus_std_openai::OpenAiToolSchemaBinding::from_json(
            animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            TOOL_SCHEMA_JSON,
        )
        .map_err(|_error| ProviderStreamError::ToolCohort)?,
    ])
}
