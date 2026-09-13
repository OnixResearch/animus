mod calls;
mod events;
mod records;
mod terminal;
#[cfg(test)]
mod tests;

/// Deterministic incremental production parser for `OpenAI` Responses SSE bytes.
#[derive(Clone)]
pub struct OpenAiResponsesSseParser {
    model_call_id: String,
    dispatch: crate::OpenAiToolDispatch,
    bounds: super::OpenAiStreamBounds,
    buffer: Vec<u8>,
    total_bytes: u64,
    event_count: u64,
    next_sequence: Option<u64>,
    next_order: u64,
    response_id: Option<String>,
    lifecycle_started: bool,
    terminal: Option<super::OpenAiStreamTerminal>,
    stream_end_seen: bool,
    frames: Vec<animus_adapters::ModelStreamFrame>,
    output_items: std::collections::BTreeMap<String, super::fields::OutputItemKind>,
    completed_output_items: std::collections::BTreeSet<String>,
    tool_states: std::collections::BTreeMap<String, calls::ToolState>,
    tool_calls: Vec<crate::OpenAiCompletedToolCall>,
    usage_emitted: bool,
    retry_progress: super::OpenAiRetryStreamProgress,
}

impl OpenAiResponsesSseParser {
    /// Builds one parser from explicit model-call admission and bounds.
    ///
    /// # Errors
    /// Returns [`super::OpenAiStreamError`] when parser bounds are invalid.
    pub fn new(
        dispatch: crate::OpenAiToolDispatch,
        bounds: super::OpenAiStreamBounds,
    ) -> Result<Self, super::OpenAiStreamError> {
        bounds.validate()?;
        debug_assert!(bounds.event_bytes <= bounds.stream_bytes);
        debug_assert!(bounds.tool_calls > 0);
        Ok(Self {
            model_call_id: String::from(dispatch.model_call_id()),
            dispatch,
            bounds,
            buffer: Vec::new(),
            total_bytes: 0,
            event_count: 0,
            next_sequence: None,
            next_order: super::vocabulary::FIRST_ORDER,
            response_id: None,
            lifecycle_started: false,
            terminal: None,
            stream_end_seen: false,
            frames: Vec::new(),
            output_items: std::collections::BTreeMap::new(),
            completed_output_items: std::collections::BTreeSet::new(),
            tool_states: std::collections::BTreeMap::new(),
            tool_calls: Vec::new(),
            usage_emitted: false,
            retry_progress: super::OpenAiRetryStreamProgress::TransportStarted,
        })
    }

    /// Consumes one admitted provider byte chunk atomically.
    ///
    /// # Errors
    /// Returns a safe error for refused input. Refusal leaves this parser unchanged.
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), super::OpenAiStreamError> {
        // r[impl agent_harness_runtime.tool_capable_openai_host.streaming]
        if chunk.is_empty() {
            return Err(super::contracts::malformed(
                super::vocabulary::MALFORMED_SSE_REASON,
            ));
        }
        let total_bytes = self
            .total_bytes
            .checked_add(super::contracts::portable_len(chunk.len())?)
            .ok_or_else(super::contracts::bound_error)?;
        if total_bytes > self.bounds.stream_bytes {
            return Err(super::contracts::bound_error());
        }
        let mut candidate = self.clone();
        candidate.total_bytes = total_bytes;
        candidate.buffer.extend_from_slice(chunk);
        candidate.process_complete_events()?;
        debug_assert_eq!(candidate.total_bytes, total_bytes);
        debug_assert!(candidate.event_count <= candidate.bounds.events);
        *self = candidate;
        Ok(())
    }

    /// Emits a canonical cancelled terminal frame.
    ///
    /// # Errors
    /// Returns a safe error if the stream already ended. Refusal leaves the parser unchanged.
    pub fn cancel(&mut self) -> Result<(), super::OpenAiStreamError> {
        if self.terminal.is_some() {
            return Err(super::contracts::lifecycle(
                super::vocabulary::TERMINAL_REASON,
            ));
        }
        let frame = self.terminal_frame(
            animus_adapters::ModelStreamFrameKind::Cancelled,
            Some(super::vocabulary::CANCELLED_REASON),
        )?;
        self.frames.push(frame);
        self.terminal = Some(super::OpenAiStreamTerminal::Cancelled);
        Ok(())
    }

    /// Completes parsing and validates the canonical frame stream.
    ///
    /// # Errors
    /// Returns a safe error for trailing bytes, absent terminal state, unfinished calls, or invalid frames.
    pub fn finish(mut self) -> Result<super::OpenAiStreamOutcome, super::OpenAiStreamError> {
        self.process_complete_events()?;
        if !self.buffer.is_empty() {
            if super::contracts::portable_len(self.buffer.len())? > self.bounds.event_bytes {
                return Err(super::contracts::bound_error());
            }
            let event_bytes = std::mem::take(&mut self.buffer);
            self.charge_event()?;
            self.process_event(event_bytes.as_slice())?;
        }
        let terminal = self
            .terminal
            .ok_or_else(|| super::contracts::lifecycle(super::vocabulary::TERMINAL_REASON))?;
        self.validate_completion(terminal)?;
        animus_adapters::validate_model_stream_frames(self.frames.as_slice()).map_err(
            |_error| {
                super::OpenAiStreamError::new(
                    super::OpenAiStreamFailureClass::Canonical,
                    super::vocabulary::MALFORMED_SSE_REASON,
                )
            },
        )?;
        let provider_response_id = match terminal {
            super::OpenAiStreamTerminal::Cancelled if self.response_id.is_none() => String::new(),
            super::OpenAiStreamTerminal::Completed
            | super::OpenAiStreamTerminal::Failed
            | super::OpenAiStreamTerminal::Cancelled => self.response_id.ok_or_else(|| {
                super::contracts::correlation(super::vocabulary::RESPONSE_ID_REASON)
            })?,
        };
        debug_assert!(self.event_count <= self.bounds.events);
        debug_assert!(self.total_bytes <= self.bounds.stream_bytes);
        Ok(super::OpenAiStreamOutcome {
            provider_response_id,
            frames: self.frames,
            tool_calls: self.tool_calls,
            terminal,
            retry_progress: self.retry_progress,
        })
    }

    fn validate_completion(
        &self,
        terminal: super::OpenAiStreamTerminal,
    ) -> Result<(), super::OpenAiStreamError> {
        if std::cmp::PartialEq::eq(&terminal, &super::OpenAiStreamTerminal::Completed) {
            let has_unfinished_calls = self.tool_states.values().any(|state| !state.done);
            let has_unfinished_items = std::cmp::PartialEq::ne(
                &self.output_items.len(),
                &self.completed_output_items.len(),
            );
            if has_unfinished_calls || has_unfinished_items {
                return Err(super::contracts::correlation(
                    super::vocabulary::TOOL_CORRELATION_REASON,
                ));
            }
        }
        Ok(())
    }

    fn require_started(&self) -> Result<(), super::OpenAiStreamError> {
        if self.lifecycle_started {
            Ok(())
        } else {
            Err(super::contracts::lifecycle(
                super::vocabulary::RESPONSE_LIFECYCLE_REASON,
            ))
        }
    }

    fn check_response_id(&self, actual: &str) -> Result<(), super::OpenAiStreamError> {
        if std::cmp::PartialEq::eq(&self.response_id.as_deref(), &Some(actual)) {
            Ok(())
        } else {
            Err(super::contracts::correlation(
                super::vocabulary::RESPONSE_ID_REASON,
            ))
        }
    }

    fn require_output_item(
        &self,
        item_id: &str,
        expected: super::fields::OutputItemKind,
    ) -> Result<(), super::OpenAiStreamError> {
        if std::cmp::PartialEq::eq(&self.output_items.get(item_id), &Some(&expected))
            && !self.completed_output_items.contains(item_id)
        {
            Ok(())
        } else {
            Err(super::contracts::correlation(
                super::vocabulary::TOOL_CORRELATION_REASON,
            ))
        }
    }
}

/// Parses admitted chunks through the production parser.
///
/// # Errors
/// Returns a safe error for malformed, unsupported, over-limit, stale, or incomplete streams.
pub fn parse_openai_responses_sse_chunks(
    dispatch: crate::OpenAiToolDispatch,
    bounds: super::OpenAiStreamBounds,
    chunks: &[&[u8]],
) -> Result<super::OpenAiStreamOutcome, super::OpenAiStreamError> {
    if chunks.is_empty()
        || super::contracts::portable_len(chunks.len())? > super::vocabulary::MAX_STREAM_CHUNKS
    {
        return Err(super::contracts::bound_error());
    }
    let mut parser = OpenAiResponsesSseParser::new(dispatch, bounds)?;
    for chunk in chunks {
        parser.push_chunk(chunk)?;
    }
    parser.finish()
}
