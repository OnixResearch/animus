/// Explicit portable byte and event bounds for one production SSE parse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenAiStreamBounds {
    /// Aggregate stream byte limit.
    pub stream_bytes: u64,
    /// One SSE event byte limit.
    pub event_bytes: u64,
    /// Parsed SSE event count limit.
    pub events: u64,
    /// Aggregate function arguments limit for one call.
    pub tool_argument_bytes: u64,
    /// Function-call count limit for one response.
    pub tool_calls: u64,
}

impl OpenAiStreamBounds {
    /// Builds bounded defaults for the reviewed cohort.
    #[must_use]
    pub const fn reviewed(max_stream_bytes: u64) -> Self {
        Self {
            stream_bytes: max_stream_bytes,
            event_bytes: if max_stream_bytes < super::vocabulary::MAX_EVENT_BYTES {
                max_stream_bytes
            } else {
                super::vocabulary::MAX_EVENT_BYTES
            },
            events: super::vocabulary::MAX_STREAM_EVENTS,
            tool_argument_bytes: super::vocabulary::MAX_TOOL_ARGUMENT_BYTES,
            tool_calls: super::vocabulary::MAX_TOOL_CALLS,
        }
    }

    pub(super) fn validate(self) -> Result<(), OpenAiStreamError> {
        let has_valid_bytes =
            self.stream_bytes > 0 && self.event_bytes > 0 && self.event_bytes <= self.stream_bytes;
        let has_valid_events =
            self.events > 0 && self.events <= super::vocabulary::MAX_STREAM_EVENTS;
        let has_valid_arguments = self.tool_argument_bytes > 0
            && self.tool_argument_bytes <= super::vocabulary::MAX_TOOL_ARGUMENT_BYTES;
        let has_valid_calls =
            self.tool_calls > 0 && self.tool_calls <= super::vocabulary::MAX_TOOL_CALLS;
        if !has_valid_bytes || !has_valid_events {
            return Err(bound_error());
        }
        if !has_valid_arguments || !has_valid_calls {
            return Err(bound_error());
        }
        debug_assert!(self.event_bytes <= self.stream_bytes);
        debug_assert!(self.events <= super::vocabulary::MAX_STREAM_EVENTS);
        Ok(())
    }
}

/// Safe production parser failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiStreamFailureClass {
    /// SSE framing or JSON syntax was malformed.
    Malformed,
    /// A configured body, event, count, or argument bound was exceeded.
    BoundExceeded,
    /// Provider lifecycle or sequence was invalid.
    Lifecycle,
    /// Response, output-item, or function-call correlation failed.
    Correlation,
    /// The provider emitted an event outside the reviewed cohort.
    UnsupportedEvent,
    /// Reasoning data did not use an admitted exposed event.
    UnsafeReasoning,
    /// The provider ended with a failed or incomplete response.
    ProviderFailure,
    /// Explicit cancellation ended parsing.
    Cancelled,
    /// Canonical stream-frame validation failed.
    Canonical,
}

/// Safe production parser failure without provider payloads.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiStreamError {
    class: OpenAiStreamFailureClass,
    reason: &'static str,
}

impl OpenAiStreamError {
    pub(super) const fn new(class: OpenAiStreamFailureClass, reason: &'static str) -> Self {
        Self { class, reason }
    }

    /// Returns the safe parser failure class.
    #[must_use]
    pub const fn class(&self) -> OpenAiStreamFailureClass {
        self.class
    }

    /// Returns a stable safe reason.
    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

impl std::fmt::Debug for OpenAiStreamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiStreamError")
            .field("class", &self.class)
            .field("reason", &self.reason)
            .finish()
    }
}

impl std::fmt::Display for OpenAiStreamError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.reason)
    }
}

impl std::error::Error for OpenAiStreamError {}

pub(super) const fn malformed(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Malformed, reason)
}

pub(super) const fn lifecycle(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Lifecycle, reason)
}

pub(super) const fn correlation(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Correlation, reason)
}

pub(super) const fn bound_error() -> OpenAiStreamError {
    OpenAiStreamError::new(
        OpenAiStreamFailureClass::BoundExceeded,
        super::vocabulary::STREAM_BOUND_REASON,
    )
}

pub(super) const fn bound_tool_argument() -> OpenAiStreamError {
    OpenAiStreamError::new(
        OpenAiStreamFailureClass::BoundExceeded,
        super::vocabulary::TOOL_ARGUMENT_BOUND_REASON,
    )
}

pub(super) fn portable_len(length: usize) -> Result<u64, OpenAiStreamError> {
    u64::try_from(length).map_err(|_error| bound_error())
}
