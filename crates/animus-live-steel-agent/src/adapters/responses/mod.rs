pub(crate) mod continuation;
#[cfg(test)]
mod controls;
pub(crate) mod planning;

const TOOL_DESCRIPTION: &str = "Evaluate one bounded Steel expression.";
const TOOL_SCHEMA_JSON: &[u8] = br#"{
  "type": "object",
  "properties": {"code": {"type": "string"}},
  "required": ["code"],
  "additionalProperties": false
}"#;
const DISABLED_REASONING_EFFORT: &str = "none";
const LOW_REASONING_EFFORT: &str = "low";
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
    streams: std::collections::VecDeque<Vec<u8>>,
    max_response_bytes: u64,
}

impl FixtureStreamProvider {
    /// Builds a deterministic provider from ordered complete SSE streams.
    #[must_use]
    pub fn new(streams: impl IntoIterator<Item = Vec<u8>>, max_response_bytes: u64) -> Self {
        Self {
            streams: streams.into_iter().collect(),
            max_response_bytes,
        }
    }

    /// Returns the portable remaining response count.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderStreamError::Unavailable`] if the count exceeds `u64`.
    pub fn remaining(&self) -> Result<u64, ProviderStreamError> {
        u64::try_from(self.streams.len()).map_err(|_error| ProviderStreamError::Unavailable)
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
pub(super) struct FixedStreamingCancellation {
    cancelled: bool,
}

#[cfg(feature = "live-provider")]
impl animus_std_openai::StreamingCancellation for FixedStreamingCancellation {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}
