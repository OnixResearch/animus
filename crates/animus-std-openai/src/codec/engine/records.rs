impl super::OpenAiResponsesSseParser {
    pub(super) fn process_complete_events(&mut self) -> Result<(), crate::OpenAiStreamError> {
        while let Some((separator_index, separator_len)) = find_separator(self.buffer.as_slice()) {
            let event_length_bytes = crate::codec::contracts::portable_len(separator_index)?;
            if separator_index == 0 || event_length_bytes > self.bounds.event_bytes {
                return Err(crate::codec::contracts::bound_error());
            }
            let event_bytes = self
                .buffer
                .get(..separator_index)
                .ok_or_else(crate::codec::contracts::bound_error)?
                .to_vec();
            let consumed = separator_index
                .checked_add(separator_len)
                .ok_or_else(crate::codec::contracts::bound_error)?;
            if consumed > self.buffer.len() {
                return Err(crate::codec::contracts::bound_error());
            }
            debug_assert!(consumed <= self.buffer.len());
            debug_assert!(event_length_bytes <= self.bounds.event_bytes);
            self.charge_event()?;
            self.process_event(event_bytes.as_slice())?;
            self.buffer.drain(..consumed);
        }
        if crate::codec::contracts::portable_len(self.buffer.len())? > self.bounds.event_bytes {
            return Err(crate::codec::contracts::bound_error());
        }
        Ok(())
    }

    pub(super) fn charge_event(&mut self) -> Result<(), crate::OpenAiStreamError> {
        let event_count = self
            .event_count
            .checked_add(1)
            .ok_or_else(crate::codec::contracts::bound_error)?;
        if event_count > self.bounds.events {
            return Err(crate::codec::contracts::bound_error());
        }
        self.event_count = event_count;
        Ok(())
    }

    pub(super) fn process_event(&mut self, bytes: &[u8]) -> Result<(), crate::OpenAiStreamError> {
        let text = std::str::from_utf8(bytes).map_err(|_error| {
            crate::codec::contracts::malformed(crate::codec::vocabulary::MALFORMED_SSE_REASON)
        })?;
        // r[impl agent_harness_runtime.responses_sse_framing.keepalive]
        // Comment records still consume one event and their bytes.
        if text
            .lines()
            .all(|line| line.trim_end_matches('\r').starts_with(':'))
        {
            return Ok(());
        }
        let (event_name, data) = parse_sse_fields(text)?;
        // r[impl agent_harness_runtime.responses_sse_framing.termination]
        if data == crate::codec::vocabulary::STREAM_DONE {
            return self.end_marker(event_name);
        }
        if self.terminal.is_some() {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::TERMINAL_REASON,
            ));
        }
        debug_assert!(!self.stream_end_seen);
        debug_assert!(self.terminal.is_none());
        let envelope: crate::codec::fields::EventEnvelope =
            serde_json::from_str(data).map_err(|_error| {
                crate::codec::contracts::malformed(crate::codec::vocabulary::MALFORMED_SSE_REASON)
            })?;
        // r[impl agent_harness_runtime.responses_sse_framing.metadata]
        // r[impl agent_harness_runtime.responses_sse_framing.guards]
        if event_name.is_some_and(|name| name != envelope.kind) {
            return Err(crate::codec::contracts::malformed(
                crate::codec::vocabulary::EVENT_NAME_MISMATCH_REASON,
            ));
        }
        self.check_sequence(envelope.sequence_number)?;
        self.process_json_event(envelope.kind.as_str(), &envelope.value)
    }

    const fn end_marker(
        &mut self,
        event_name: Option<&str>,
    ) -> Result<(), crate::OpenAiStreamError> {
        if event_name.is_some()
            || !matches!(self.terminal, Some(crate::OpenAiStreamTerminal::Completed))
            || self.stream_end_seen
        {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::TERMINAL_REASON,
            ));
        }
        self.stream_end_seen = true;
        Ok(())
    }

    fn check_sequence(&mut self, sequence: u64) -> Result<(), crate::OpenAiStreamError> {
        if self
            .next_sequence
            .is_some_and(|expected| sequence != expected)
        {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::SEQUENCE_REASON,
            ));
        }
        self.next_sequence = Some(
            sequence
                .checked_add(crate::codec::vocabulary::ORDER_INCREMENT)
                .ok_or_else(|| {
                    crate::codec::contracts::lifecycle(crate::codec::vocabulary::SEQUENCE_REASON)
                })?,
        );
        Ok(())
    }
}

fn parse_sse_fields(record: &str) -> Result<(Option<&str>, &str), crate::OpenAiStreamError> {
    let mut event = None;
    let mut data = None;
    for line in record.lines() {
        let line = line.trim_end_matches('\r');
        if let Some(value) = line.strip_prefix(crate::codec::vocabulary::EVENT_PREFIX) {
            if event.replace(value.trim_start()).is_some() {
                return Err(crate::codec::contracts::malformed(
                    crate::codec::vocabulary::MALFORMED_SSE_REASON,
                ));
            }
        } else if let Some(value) = line.strip_prefix(crate::codec::vocabulary::DATA_PREFIX) {
            if data.replace(value.trim_start()).is_some() {
                return Err(crate::codec::contracts::malformed(
                    crate::codec::vocabulary::MALFORMED_SSE_REASON,
                ));
            }
        } else if !line.starts_with(':') {
            return Err(crate::codec::contracts::malformed(
                crate::codec::vocabulary::MALFORMED_SSE_REASON,
            ));
        }
    }
    match (event, data) {
        (event, Some(data)) if !event.is_some_and(str::is_empty) && !data.is_empty() => {
            debug_assert!(!data.is_empty());
            debug_assert!(event.is_none_or(|name| !name.is_empty()));
            Ok((event, data))
        }
        _missing => Err(crate::codec::contracts::malformed(
            crate::codec::vocabulary::MALFORMED_SSE_REASON,
        )),
    }
}

fn find_separator(bytes: &[u8]) -> Option<(usize, usize)> {
    let lf = bytes
        .windows(crate::codec::vocabulary::EVENT_SEPARATOR.len())
        .position(|window| {
            std::cmp::PartialEq::eq(window, crate::codec::vocabulary::EVENT_SEPARATOR)
        })
        .map(|index| (index, crate::codec::vocabulary::EVENT_SEPARATOR.len()));
    let crlf = bytes
        .windows(crate::codec::vocabulary::CRLF_EVENT_SEPARATOR.len())
        .position(|window| {
            std::cmp::PartialEq::eq(window, crate::codec::vocabulary::CRLF_EVENT_SEPARATOR)
        })
        .map(|index| (index, crate::codec::vocabulary::CRLF_EVENT_SEPARATOR.len()));
    Iterator::min_by_key(lf.into_iter().chain(crlf), |(index, _separator_len)| *index)
}

#[cfg(test)]
mod tests {
    #[test]
    fn separators_select_first_complete_boundary_and_reject_partial_boundary() {
        let prefix = b"x";
        let lf = crate::codec::vocabulary::EVENT_SEPARATOR;
        let crlf = crate::codec::vocabulary::CRLF_EVENT_SEPARATOR;
        let lf_first = [prefix.as_slice(), lf, prefix.as_slice(), crlf].concat();
        let crlf_first = [prefix.as_slice(), crlf, prefix.as_slice(), lf].concat();
        assert_eq!(
            super::find_separator(&lf_first),
            Some((prefix.len(), lf.len()))
        );
        assert_eq!(
            super::find_separator(&crlf_first),
            Some((prefix.len(), crlf.len()))
        );
        assert_eq!(super::find_separator(b"x\r\n\r"), None);
        assert_eq!(super::find_separator(b""), None);
    }
}
