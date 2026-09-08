//! Host-owned concrete provider streaming parser and dispatch seams.
//!
//! This module is intentionally shell-owned. It accepts already-admitted raw
//! provider stream chunks, converts them through deterministic parser cores into
//! canonical `animus-adapters` stream frames, and plans dispatch only after model
//! capability, credential, and injected-service readiness checks pass. It does
//! not create provider clients, read credentials, open networks, retry calls, or
//! move provider-native streams into reusable runtime crates.

const EVENT_PREFIX: &str = "event: ";
const DATA_PREFIX: &str = "data: ";
const RECORD_SEPARATOR: &str = "\n\n";
const FIELD_SEPARATOR: char = '|';
const FIRST_ORDER: u64 = 1;
const ORDER_INCREMENT: u64 = 1;
const EVENT_LINE_COUNT_MIN: usize = 2;
const TOOL_PART_COUNT: usize = 2;
const USAGE_PART_COUNT: usize = 2;
const INPUT_USAGE_INDEX: usize = 0;
const OUTPUT_USAGE_INDEX: usize = 1;
const TOOL_ID_INDEX: usize = 0;
const TOOL_BODY_INDEX: usize = 1;
const EMPTY_RECORD_CAPACITY: usize = 0;

const OPENAI_CONTENT_EVENT: &str = "response.output_text.delta";
const OPENAI_THOUGHT_EVENT: &str = "response.reasoning.delta";
const OPENAI_TOOL_EVENT: &str = "response.function_call_arguments.delta";
const OPENAI_USAGE_EVENT: &str = "response.usage";
const OPENAI_FINISH_EVENT: &str = "response.completed";
const OPENAI_CANCELLED_EVENT: &str = "response.cancelled";
const OPENAI_ERROR_EVENT: &str = "response.error";

const ANTHROPIC_CONTENT_EVENT: &str = "content_block_delta";
const ANTHROPIC_THOUGHT_EVENT: &str = "thinking_delta";
const ANTHROPIC_TOOL_EVENT: &str = "input_json_delta";
const ANTHROPIC_USAGE_EVENT: &str = "message_delta";
const ANTHROPIC_FINISH_EVENT: &str = "message_stop";
const ANTHROPIC_CANCELLED_EVENT: &str = "message_cancelled";
const ANTHROPIC_ERROR_EVENT: &str = "error";

const MALFORMED_EMPTY_CHUNKS: &str = "provider stream chunks must be non-empty";
const MALFORMED_TRUNCATED_RECORD: &str = "provider stream record is truncated";
const MALFORMED_MISSING_EVENT: &str = "provider stream record is missing an event line";
const MALFORMED_MISSING_DATA: &str = "provider stream record is missing a data line";
const MALFORMED_TOOL_DELTA: &str = "tool-call delta must contain tool id and delta body";
const MALFORMED_USAGE_DELTA: &str = "usage delta must contain input and output token counts";
const MALFORMED_USAGE_NUMBER: &str = "usage token count is not a number";
const MAX_PROVIDER_STREAM_CHUNKS: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;

/// Provider streaming dispatch or parser error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderStreamingError {
    /// A selected model cannot satisfy the requested streaming mode.
    Capability(super::RegistryError),
    /// A credential required by the selected provider was missing before dispatch.
    MissingCredential,
    /// No host streaming service was injected.
    MissingService,
    /// A host streaming service exists but is currently unavailable.
    ServiceUnavailable,
    /// The selected provider family has no concrete parser entrypoint here.
    UnsupportedProviderFamily {
        /// Provider API family.
        api_kind: super::ApiKind,
    },
    /// Raw provider chunks were malformed before canonical frame validation.
    MalformedProviderStream {
        /// Safe diagnostic reason.
        reason: &'static str,
    },
    /// Raw provider chunks named an unsupported channel.
    UnknownProviderChannel {
        /// Safe channel label from the stream record.
        channel: String,
    },
    /// Canonical model stream validation failed.
    Canonical(animus_adapters::ModelStreamError),
}

impl From<animus_adapters::ModelStreamError> for ProviderStreamingError {
    fn from(error: animus_adapters::ModelStreamError) -> Self {
        Self::Canonical(error)
    }
}

impl From<super::RegistryError> for ProviderStreamingError {
    fn from(error: super::RegistryError) -> Self {
        Self::Capability(error)
    }
}

impl std::fmt::Display for ProviderStreamingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capability(error) => write!(formatter, "stream capability denied: {error}"),
            Self::MissingCredential => formatter.write_str("provider streaming credential missing"),
            Self::MissingService => formatter.write_str("provider streaming service missing"),
            Self::ServiceUnavailable => {
                formatter.write_str("provider streaming service unavailable")
            }
            Self::UnsupportedProviderFamily { api_kind } => {
                write!(
                    formatter,
                    "unsupported provider streaming family {}",
                    api_kind.as_str()
                )
            }
            Self::MalformedProviderStream { reason } => {
                write!(formatter, "malformed provider stream: {reason}")
            }
            Self::UnknownProviderChannel { channel } => {
                write!(formatter, "unknown provider stream channel {channel}")
            }
            Self::Canonical(error) => write!(formatter, "canonical model stream error: {error}"),
        }
    }
}

impl std::error::Error for ProviderStreamingError {}

/// Readiness of the host-owned credential before provider streaming dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderStreamingCredentialState {
    /// The provider/model does not require credentials for this dispatch.
    NotRequired,
    /// A shell-owned credential handle is present; its secret value stays outside this module.
    Present,
    /// A required credential was absent.
    MissingRequired,
}

/// Readiness of the host-injected streaming service before dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderStreamingServiceState {
    /// A host-owned service is admitted.
    Available,
    /// No streaming service was injected.
    Missing,
    /// A service was injected but cannot currently dispatch.
    Unavailable,
}

/// Borrowed input for planning concrete provider streaming dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderStreamingDispatchInput<'a> {
    /// Selected model metadata from the shell registry.
    pub selected_model: &'a super::SelectedModel,
    /// Requested streaming modes.
    pub request: super::ModelStreamingRequest,
    /// Policy for unsupported stream modes.
    pub unsupported_policy: super::UnsupportedStreamPolicy,
    /// Credential readiness at the shell boundary.
    pub credential: ProviderStreamingCredentialState,
    /// Injected service readiness at the shell boundary.
    pub service: ProviderStreamingServiceState,
}

/// Safe dispatch plan handed to a host-owned streaming service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderStreamingDispatchPlan {
    provider_id: String,
    model_id: String,
    api_kind: super::ApiKind,
    endpoint: String,
    modes: super::ResolvedStreamingModes,
}

impl ProviderStreamingDispatchPlan {
    /// Returns the selected provider id.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    /// Returns the selected model id.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    /// Returns the provider API family.
    #[must_use]
    pub const fn api_kind(&self) -> super::ApiKind {
        self.api_kind
    }

    /// Returns the safe endpoint identity.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    /// Returns resolved streaming modes.
    #[must_use]
    pub const fn modes(&self) -> super::ResolvedStreamingModes {
        self.modes
    }
}

/// Plans provider streaming dispatch without constructing provider clients.
///
/// # Errors
///
/// Returns [`ProviderStreamingError`] when capability negotiation, credential
/// readiness, or service readiness fails closed.
///
/// # Panics
///
/// Panics if selected provider/model/endpoint metadata is empty after registry validation.
pub fn plan_provider_streaming_dispatch(
    input: &ProviderStreamingDispatchInput<'_>,
) -> Result<ProviderStreamingDispatchPlan, ProviderStreamingError> {
    assert!(
        !input.selected_model.provider_id().as_str().is_empty(),
        "selected provider id is validated before streaming dispatch"
    );
    assert!(
        !input.selected_model.model_id().as_str().is_empty(),
        "selected model id is validated before streaming dispatch"
    );
    assert!(
        !input.selected_model.endpoint().as_str().is_empty(),
        "selected endpoint identity is validated before streaming dispatch"
    );

    match input.credential {
        ProviderStreamingCredentialState::MissingRequired => {
            return Err(ProviderStreamingError::MissingCredential);
        }
        ProviderStreamingCredentialState::NotRequired
        | ProviderStreamingCredentialState::Present => {}
    }

    match input.service {
        ProviderStreamingServiceState::Missing => {
            return Err(ProviderStreamingError::MissingService);
        }
        ProviderStreamingServiceState::Unavailable => {
            return Err(ProviderStreamingError::ServiceUnavailable);
        }
        ProviderStreamingServiceState::Available => {}
    }

    let modes = input
        .selected_model
        .capabilities()
        .resolve_streaming(input.request, input.unsupported_policy)?;

    Ok(ProviderStreamingDispatchPlan {
        provider_id: String::from(input.selected_model.provider_id().as_str()),
        model_id: String::from(input.selected_model.model_id().as_str()),
        api_kind: input.selected_model.api_kind(),
        endpoint: String::from(input.selected_model.endpoint().as_str()),
        modes,
    })
}

/// Borrowed raw provider stream parser input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderStreamParserInput<'a> {
    /// Provider family whose production parser entrypoint is being exercised.
    pub api_kind: super::ApiKind,
    /// Stable model-call correlation id assigned by the shell.
    pub call_id: &'a str,
    /// Admitted raw provider stream chunks.
    pub chunks: &'a [&'a str],
}

/// Parses concrete provider stream chunks into canonical model frames.
///
/// # Errors
///
/// Returns [`ProviderStreamingError`] when chunks are unsupported, malformed, or
/// canonical stream validation fails.
pub fn parse_provider_stream_chunks(
    input: &ProviderStreamParserInput<'_>,
) -> Result<Vec<animus_adapters::ModelStreamFrame>, ProviderStreamingError> {
    match input.api_kind {
        super::ApiKind::OpenAiResponses => {
            parse_events(input.call_id, input.chunks, openai_event_kind)
        }
        super::ApiKind::AnthropicMessages => {
            parse_events(input.call_id, input.chunks, anthropic_event_kind)
        }
        super::ApiKind::OpenAiChat | super::ApiKind::LocalHttp | super::ApiKind::Mock => {
            Err(ProviderStreamingError::UnsupportedProviderFamily {
                api_kind: input.api_kind,
            })
        }
    }
}

type EventMapper = fn(&str) -> Result<ProviderRecordKind, ProviderStreamingError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProviderRecordKind {
    Content,
    Thought,
    Tool,
    Usage,
    Finish,
    Cancelled,
    Failure,
}

fn parse_events(
    call_id: &str,
    chunks: &[&str],
    mapper: EventMapper,
) -> Result<Vec<animus_adapters::ModelStreamFrame>, ProviderStreamingError> {
    validate_chunks(chunks)?;
    let buffer = join_chunks(chunks);
    validate_complete(buffer.as_str())?;
    let mut frames = Vec::with_capacity(record_count(buffer.as_str()));
    let mut order = FIRST_ORDER;
    for record in buffer.split(RECORD_SEPARATOR) {
        if record.is_empty() {
            continue;
        }
        frames.push(parse_record(call_id, order, record, mapper)?);
        order = order.saturating_add(ORDER_INCREMENT);
    }
    animus_adapters::validate_model_stream_frames(frames.as_slice())?;
    Ok(frames)
}

fn validate_chunks(chunks: &[&str]) -> Result<(), ProviderStreamingError> {
    assert!(
        chunks.len() <= MAX_PROVIDER_STREAM_CHUNKS,
        "provider stream chunk count fits adapter diagnostic bounds"
    );
    if chunks.is_empty() {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_EMPTY_CHUNKS,
        });
    }
    Ok(())
}

fn join_chunks(chunks: &[&str]) -> String {
    assert!(!chunks.is_empty(), "non-empty chunks checked by caller");
    let mut buffer = String::new();
    for chunk in chunks {
        buffer.push_str(chunk);
    }
    buffer
}

fn validate_complete(buffer: &str) -> Result<(), ProviderStreamingError> {
    if buffer.ends_with(RECORD_SEPARATOR) {
        return Ok(());
    }
    Err(ProviderStreamingError::MalformedProviderStream {
        reason: MALFORMED_TRUNCATED_RECORD,
    })
}

fn record_count(buffer: &str) -> usize {
    if buffer.is_empty() {
        return EMPTY_RECORD_CAPACITY;
    }
    buffer
        .split(RECORD_SEPARATOR)
        .filter(|record| !record.is_empty())
        .count()
}

fn parse_record(
    call_id: &str,
    order: u64,
    record: &str,
    mapper: EventMapper,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    assert!(!record.is_empty(), "provider stream record is non-empty");
    let (event, data) = event_and_data(record)?;
    let kind = mapper(event)?;
    frame_from_record_kind(call_id, order, kind, data)
}

fn event_and_data(record: &str) -> Result<(&str, &str), ProviderStreamingError> {
    assert!(
        !record.is_empty(),
        "event/data parser receives non-empty record"
    );
    let mut event = None;
    let mut data = None;
    let mut line_count = EMPTY_RECORD_CAPACITY;
    for line in record.lines() {
        line_count = line_count.saturating_add(1);
        if let Some(raw_event) = line.strip_prefix(EVENT_PREFIX) {
            event = Some(raw_event);
        }
        if let Some(raw_data) = line.strip_prefix(DATA_PREFIX) {
            data = Some(raw_data);
        }
    }
    if line_count < EVENT_LINE_COUNT_MIN {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_MISSING_DATA,
        });
    }
    let Some(event) = event else {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_MISSING_EVENT,
        });
    };
    let Some(data) = data else {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_MISSING_DATA,
        });
    };
    assert!(
        line_count >= EVENT_LINE_COUNT_MIN,
        "event/data parser validates minimum record shape"
    );
    Ok((event, data))
}

fn frame_from_record_kind(
    call_id: &str,
    order: u64,
    kind: ProviderRecordKind,
    data: &str,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    match kind {
        ProviderRecordKind::Content => text_frame(&TextFrameInput {
            call_id,
            order,
            kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
            body: data,
            visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
        }),
        ProviderRecordKind::Thought => text_frame(&TextFrameInput {
            call_id,
            order,
            kind: animus_adapters::ModelStreamFrameKind::ThoughtDelta,
            body: data,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        }),
        ProviderRecordKind::Tool => tool_frame(call_id, order, data),
        ProviderRecordKind::Usage => usage_frame(call_id, order, data),
        ProviderRecordKind::Finish => terminal_frame(
            call_id,
            order,
            animus_adapters::ModelStreamFrameKind::Finish,
            None,
        ),
        ProviderRecordKind::Cancelled => terminal_frame(
            call_id,
            order,
            animus_adapters::ModelStreamFrameKind::Cancelled,
            Some(data),
        ),
        ProviderRecordKind::Failure => terminal_frame(
            call_id,
            order,
            animus_adapters::ModelStreamFrameKind::Failure,
            Some(data),
        ),
    }
}

struct TextFrameInput<'a> {
    call_id: &'a str,
    order: u64,
    kind: animus_adapters::ModelStreamFrameKind,
    body: &'a str,
    visibility: animus_adapters::ModelStreamVisibility,
}

fn text_frame(
    input: &TextFrameInput<'_>,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    Ok(animus_adapters::ModelStreamFrame::from_draft(
        animus_adapters::ModelStreamFrameDraft {
            call_id: input.call_id,
            order: input.order,
            kind: input.kind,
            body: Some(input.body),
            tool_call_id: None,
            usage: None,
            visibility: input.visibility,
        },
    )?)
}

fn tool_frame(
    call_id: &str,
    order: u64,
    data: &str,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    let parts: Vec<&str> = data.splitn(TOOL_PART_COUNT, FIELD_SEPARATOR).collect();
    if parts.len() != TOOL_PART_COUNT {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_TOOL_DELTA,
        });
    }
    Ok(animus_adapters::ModelStreamFrame::from_draft(
        animus_adapters::ModelStreamFrameDraft {
            call_id,
            order,
            kind: animus_adapters::ModelStreamFrameKind::ToolCallDelta,
            body: Some(parts[TOOL_BODY_INDEX]),
            tool_call_id: Some(parts[TOOL_ID_INDEX]),
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        },
    )?)
}

fn usage_frame(
    call_id: &str,
    order: u64,
    data: &str,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    let parts: Vec<&str> = data.split(FIELD_SEPARATOR).collect();
    if parts.len() != USAGE_PART_COUNT {
        return Err(ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_USAGE_DELTA,
        });
    }
    assert_eq!(
        parts.len(),
        USAGE_PART_COUNT,
        "usage parser validates input and output token parts"
    );
    let input_tokens = usage_count(parts[INPUT_USAGE_INDEX])?;
    let output_tokens = usage_count(parts[OUTPUT_USAGE_INDEX])?;
    let usage = animus_adapters::ModelStreamUsage::new(input_tokens, output_tokens);
    assert_eq!(
        usage.input_tokens(),
        input_tokens,
        "usage frame preserves provider input-token count"
    );
    assert_eq!(
        usage.output_tokens(),
        output_tokens,
        "usage frame preserves provider output-token count"
    );
    Ok(animus_adapters::ModelStreamFrame::from_draft(
        animus_adapters::ModelStreamFrameDraft {
            call_id,
            order,
            kind: animus_adapters::ModelStreamFrameKind::Usage,
            body: None,
            tool_call_id: None,
            usage: Some(usage),
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        },
    )?)
}

fn terminal_frame(
    call_id: &str,
    order: u64,
    kind: animus_adapters::ModelStreamFrameKind,
    body: Option<&str>,
) -> Result<animus_adapters::ModelStreamFrame, ProviderStreamingError> {
    Ok(animus_adapters::ModelStreamFrame::from_draft(
        animus_adapters::ModelStreamFrameDraft {
            call_id,
            order,
            kind,
            body,
            tool_call_id: None,
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        },
    )?)
}

fn usage_count(raw: &str) -> Result<u32, ProviderStreamingError> {
    raw.parse::<u32>()
        .map_err(|_error| ProviderStreamingError::MalformedProviderStream {
            reason: MALFORMED_USAGE_NUMBER,
        })
}

fn openai_event_kind(event: &str) -> Result<ProviderRecordKind, ProviderStreamingError> {
    match event {
        OPENAI_CONTENT_EVENT => Ok(ProviderRecordKind::Content),
        OPENAI_THOUGHT_EVENT => Ok(ProviderRecordKind::Thought),
        OPENAI_TOOL_EVENT => Ok(ProviderRecordKind::Tool),
        OPENAI_USAGE_EVENT => Ok(ProviderRecordKind::Usage),
        OPENAI_FINISH_EVENT => Ok(ProviderRecordKind::Finish),
        OPENAI_CANCELLED_EVENT => Ok(ProviderRecordKind::Cancelled),
        OPENAI_ERROR_EVENT => Ok(ProviderRecordKind::Failure),
        _unknown => Err(ProviderStreamingError::UnknownProviderChannel {
            channel: String::from(event),
        }),
    }
}

fn anthropic_event_kind(event: &str) -> Result<ProviderRecordKind, ProviderStreamingError> {
    match event {
        ANTHROPIC_CONTENT_EVENT => Ok(ProviderRecordKind::Content),
        ANTHROPIC_THOUGHT_EVENT => Ok(ProviderRecordKind::Thought),
        ANTHROPIC_TOOL_EVENT => Ok(ProviderRecordKind::Tool),
        ANTHROPIC_USAGE_EVENT => Ok(ProviderRecordKind::Usage),
        ANTHROPIC_FINISH_EVENT => Ok(ProviderRecordKind::Finish),
        ANTHROPIC_CANCELLED_EVENT => Ok(ProviderRecordKind::Cancelled),
        ANTHROPIC_ERROR_EVENT => Ok(ProviderRecordKind::Failure),
        _unknown => Err(ProviderStreamingError::UnknownProviderChannel {
            channel: String::from(event),
        }),
    }
}
