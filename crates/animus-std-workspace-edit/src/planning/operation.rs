#[derive(Clone, Copy)]
pub(crate) struct LineSpan {
    pub(crate) first: u32,
    pub(crate) last: u32,
}

#[derive(Clone, Copy)]
struct ReplacementFacts<'a> {
    kind: &'a str,
    lines: LineSpan,
    bytes: &'a [u8],
}

#[derive(Clone, Copy)]
pub(crate) struct SpanInput<'a> {
    pub(crate) snapshot: &'a crate::model::AdmittedSnapshot,
    pub(crate) start_tag: &'a str,
    pub(crate) end_tag: &'a str,
    pub(crate) replacement_text: Option<&'a str>,
}

fn tag_line(
    snapshot: &crate::model::AdmittedSnapshot,
    tag: &str,
) -> Result<u32, crate::model::HostIssue> {
    snapshot
        .tags
        .iter()
        .find_map(|(candidate, line)| (candidate == tag).then_some(*line))
        .ok_or(crate::model::HostIssue::UnknownTag)
}

fn range_is_visible(snapshot: &crate::model::AdmittedSnapshot, span: LineSpan) -> bool {
    snapshot
        .visible_ranges
        .iter()
        .any(|range| range.lines.start <= span.first && span.last < range.lines.end)
}

fn boundary_is_visible(
    snapshot: &crate::model::AdmittedSnapshot,
    line: u32,
    side: crate::model::BoundarySide,
) -> bool {
    snapshot
        .visible_boundaries
        .iter()
        .any(|boundary| boundary.line == line && boundary.side == side)
}

fn line_index(line: u32) -> Result<usize, crate::model::HostIssue> {
    let zero_based = line
        .checked_sub(1)
        .ok_or(crate::model::HostIssue::SnapshotShapeMismatch)?;
    usize::try_from(zero_based).map_err(|_| crate::model::HostIssue::SnapshotShapeMismatch)
}

fn logical_replacement(text: &str) -> Result<&str, crate::model::HostIssue> {
    if text.is_empty() || text.len() > crate::model::MAX_REPLACEMENT_BYTES {
        return Err(crate::model::HostIssue::InvalidReplacement);
    }
    if text.contains('\r') || text.ends_with('\n') {
        return Err(crate::model::HostIssue::InvalidReplacement);
    }
    Ok(text)
}

fn encode_logical(
    text: &str,
    separator: &[u8],
    tail: &[u8],
) -> Result<Vec<u8>, crate::model::HostIssue> {
    let text = logical_replacement(text)?;
    let capacity_bytes = text.len().saturating_add(tail.len());
    let mut output = Vec::with_capacity(capacity_bytes);
    for (index, part) in text.split('\n').enumerate() {
        if index > 0 {
            output.extend_from_slice(separator);
        }
        output.extend_from_slice(part.as_bytes());
    }
    output.extend_from_slice(tail);
    if output.len() > crate::model::MAX_REPLACEMENT_BYTES {
        return Err(crate::model::HostIssue::InvalidReplacement);
    }
    Ok(output)
}

fn replacement_identity(facts: ReplacementFacts<'_>) -> String {
    crate::planning::common::hash_parts(
        crate::planning::common::FRAME_DOMAIN_REPLACEMENT,
        &[
            facts.kind.as_bytes(),
            &facts.lines.first.to_le_bytes(),
            &facts.lines.last.to_le_bytes(),
            facts.bytes,
        ],
    )
}

pub(crate) fn span(
    input: SpanInput<'_>,
) -> Result<crate::model::mutation::AppliedOperation, crate::model::HostIssue> {
    let lines = LineSpan {
        first: tag_line(input.snapshot, input.start_tag)?,
        last: tag_line(input.snapshot, input.end_tag)?,
    };
    if lines.first > lines.last {
        return Err(crate::model::HostIssue::OverlappingOperation);
    }
    if !range_is_visible(input.snapshot, lines) {
        return Err(crate::model::HostIssue::HiddenSpan);
    }
    let first = &input.snapshot.lines[line_index(lines.first)?];
    let last = &input.snapshot.lines[line_index(lines.last)?];
    let (kind, replacement) = if let Some(text) = input.replacement_text {
        let tail = &input.snapshot.source_bytes[last.body_end..last.end];
        (
            crate::planning::common::KIND_REPLACE,
            encode_logical(text, input.snapshot.replacement_ending.bytes(), tail)?,
        )
    } else {
        (crate::planning::common::KIND_DELETE, Vec::new())
    };
    let identity = replacement_identity(ReplacementFacts {
        kind,
        lines,
        bytes: replacement.as_slice(),
    });
    debug_assert!(first.start <= last.end);
    Ok(crate::model::mutation::AppliedOperation {
        start: first.start,
        end: last.end,
        is_insertion: false,
        replacement,
        kind,
        first_line: lines.first,
        last_line: lines.last,
        replacement_identity: identity,
    })
}

pub(crate) fn insertion(
    snapshot: &crate::model::AdmittedSnapshot,
    line_tag: &str,
    side: crate::model::BoundarySide,
    text: &str,
) -> Result<crate::model::mutation::AppliedOperation, crate::model::HostIssue> {
    let line_number = tag_line(snapshot, line_tag)?;
    if !boundary_is_visible(snapshot, line_number, side) {
        return Err(crate::model::HostIssue::HiddenBoundary);
    }
    let line = &snapshot.lines[line_index(line_number)?];
    let separator = snapshot.replacement_ending.bytes();
    let (offset, replacement) = match side {
        crate::model::BoundarySide::Before => {
            (line.start, encode_logical(text, separator, separator)?)
        }
        crate::model::BoundarySide::After if line.end > line.body_end => {
            (line.end, encode_logical(text, separator, separator)?)
        }
        crate::model::BoundarySide::After => {
            let mut bytes = separator.to_vec();
            bytes.extend_from_slice(&encode_logical(text, separator, &[])?);
            (line.end, bytes)
        }
    };
    let lines = LineSpan {
        first: line_number,
        last: line_number,
    };
    let identity = replacement_identity(ReplacementFacts {
        kind: crate::planning::common::KIND_INSERT,
        lines,
        bytes: replacement.as_slice(),
    });
    debug_assert!(offset <= snapshot.source_bytes.len());
    Ok(crate::model::mutation::AppliedOperation {
        start: offset,
        end: offset,
        is_insertion: true,
        replacement,
        kind: crate::planning::common::KIND_INSERT,
        first_line: line_number,
        last_line: line_number,
        replacement_identity: identity,
    })
}

pub(crate) fn end_insertion(
    snapshot: &crate::model::AdmittedSnapshot,
    text: &str,
) -> Result<crate::model::mutation::AppliedOperation, crate::model::HostIssue> {
    if !snapshot.end_boundary_visible {
        return Err(crate::model::HostIssue::HiddenBoundary);
    }
    let last_index = snapshot
        .lines
        .len()
        .checked_sub(1)
        .ok_or(crate::model::HostIssue::InvalidTextSource)?;
    let one_based = last_index
        .checked_add(1)
        .ok_or(crate::model::HostIssue::SnapshotShapeMismatch)?;
    let line_number =
        u32::try_from(one_based).map_err(|_| crate::model::HostIssue::SnapshotShapeMismatch)?;
    let last = &snapshot.lines[last_index];
    let separator = snapshot.replacement_ending.bytes();
    let replacement = if last.end > last.body_end {
        encode_logical(text, separator, separator)?
    } else {
        let mut bytes = separator.to_vec();
        bytes.extend_from_slice(&encode_logical(text, separator, &[])?);
        bytes
    };
    let lines = LineSpan {
        first: line_number,
        last: line_number,
    };
    let identity = replacement_identity(ReplacementFacts {
        kind: crate::planning::common::KIND_INSERT_END,
        lines,
        bytes: replacement.as_slice(),
    });
    debug_assert_eq!(last_index.checked_add(1), Some(one_based));
    Ok(crate::model::mutation::AppliedOperation {
        start: snapshot.source_bytes.len(),
        end: snapshot.source_bytes.len(),
        is_insertion: true,
        replacement,
        kind: crate::planning::common::KIND_INSERT_END,
        first_line: line_number,
        last_line: line_number,
        replacement_identity: identity,
    })
}
