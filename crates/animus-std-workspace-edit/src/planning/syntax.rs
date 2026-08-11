#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParsedFacts {
    pub(crate) mode: crate::model::LineEndingMode,
    pub(crate) final_newline: crate::model::FinalNewlineState,
}

struct LineAccumulator {
    lines: Vec<crate::model::text::SourceLine>,
    line_start: usize,
    index: usize,
    has_lf: bool,
    has_crlf: bool,
}

impl LineAccumulator {
    fn new(source_bytes: usize) -> Self {
        Self {
            lines: Vec::with_capacity(source_bytes),
            line_start: 0,
            index: 0,
            has_lf: false,
            has_crlf: false,
        }
    }

    fn consume(&mut self, bytes: &[u8]) -> Result<(), crate::model::HostIssue> {
        while self.index < bytes.len() {
            match bytes[self.index] {
                b'\r' => self.consume_crlf(bytes)?,
                b'\n' => self.consume_lf()?,
                _ => {
                    self.index = self
                        .index
                        .checked_add(1)
                        .ok_or(crate::model::HostIssue::InvalidTextSource)?;
                }
            }
        }
        Ok(())
    }

    fn consume_crlf(&mut self, bytes: &[u8]) -> Result<(), crate::model::HostIssue> {
        let next = self
            .index
            .checked_add(1)
            .ok_or(crate::model::HostIssue::InvalidTextSource)?;
        if next >= bytes.len() || bytes[next] != b'\n' {
            return Err(crate::model::HostIssue::UnsupportedLineEnding);
        }
        let terminator_end = next
            .checked_add(1)
            .ok_or(crate::model::HostIssue::InvalidTextSource)?;
        self.lines.push(crate::model::text::SourceLine {
            start: self.line_start,
            body_end: self.index,
            end: terminator_end,
        });
        self.has_crlf = true;
        self.index = terminator_end;
        self.line_start = self.index;
        Ok(())
    }

    fn consume_lf(&mut self) -> Result<(), crate::model::HostIssue> {
        let terminator_end = self
            .index
            .checked_add(1)
            .ok_or(crate::model::HostIssue::InvalidTextSource)?;
        self.lines.push(crate::model::text::SourceLine {
            start: self.line_start,
            body_end: self.index,
            end: terminator_end,
        });
        self.has_lf = true;
        self.index = terminator_end;
        self.line_start = self.index;
        Ok(())
    }

    fn finish(
        mut self,
        source_bytes: usize,
    ) -> Result<(Vec<crate::model::text::SourceLine>, ParsedFacts), crate::model::HostIssue> {
        if self.line_start < source_bytes {
            self.lines.push(crate::model::text::SourceLine {
                start: self.line_start,
                body_end: source_bytes,
                end: source_bytes,
            });
        }
        if self.lines.is_empty() {
            return Err(crate::model::HostIssue::InvalidTextSource);
        }
        let mode = match (self.has_lf, self.has_crlf) {
            (true, true) => crate::model::LineEndingMode::Mixed,
            (false, true) => crate::model::LineEndingMode::CrLf,
            (_, false) => crate::model::LineEndingMode::Lf,
        };
        let final_newline = if self.line_start == source_bytes {
            crate::model::FinalNewlineState::Present
        } else {
            crate::model::FinalNewlineState::Absent
        };
        debug_assert!(!self.lines.is_empty());
        Ok((
            self.lines,
            ParsedFacts {
                mode,
                final_newline,
            },
        ))
    }
}

pub(crate) fn parse_lines(
    bytes: &[u8],
) -> Result<(Vec<crate::model::text::SourceLine>, ParsedFacts), crate::model::HostIssue> {
    if bytes.is_empty() || bytes.len() > crate::model::MAX_SOURCE_BYTES {
        return Err(crate::model::HostIssue::InvalidTextSource);
    }
    if std::str::from_utf8(bytes).is_err() {
        return Err(crate::model::HostIssue::InvalidTextSource);
    }
    let mut accumulator = LineAccumulator::new(bytes.len());
    accumulator.consume(bytes)?;
    accumulator.finish(bytes.len())
}

pub(crate) const fn expected_normalization(
    mode: crate::model::LineEndingMode,
) -> animus_adapters::tool::hashed::TextNormalization {
    match mode {
        crate::model::LineEndingMode::Lf => animus_adapters::tool::hashed::TextNormalization::Lf,
        crate::model::LineEndingMode::CrLf => {
            animus_adapters::tool::hashed::TextNormalization::CrLf
        }
        crate::model::LineEndingMode::Mixed => {
            animus_adapters::tool::hashed::TextNormalization::MixedNormalizedLf
        }
    }
}
