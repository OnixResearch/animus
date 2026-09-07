//! Data-only framing shares the named stream's semantic validation.

const MAX_RESPONSE_BYTES: usize = 262_144;
const FRAGMENT_BYTES: usize = 1;
const DONE_RECORD: &str = "data: [DONE]\n\n";
const CREATED_HEADER: &str = "event: response.created\n";
const COMPLETED_HEADER: &str = "event: response.completed\n";
const SEQUENCE_ONE: &str = "\"sequence_number\":1";
const SEQUENCE_ZERO: &str = "\"sequence_number\":0";

fn unnamed(bytes: &[u8]) -> String {
    let text = std::str::from_utf8(bytes).expect("fixture is UTF8");
    let mut result = text
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("event:"))
        .collect::<String>();
    if !result.ends_with("\n\n") {
        result.push('\n');
    }
    assert!(!result.contains("event:"));
    assert!(result.contains("data:"));
    result
}

fn parse(bytes: &[u8]) -> Result<crate::OpenAiStreamOutcome, crate::OpenAiStreamError> {
    let mut parser = crate::OpenAiResponsesSseParser::new(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(MAX_RESPONSE_BYTES),
    )?;
    for part in bytes.chunks(FRAGMENT_BYTES) {
        parser.push_chunk(part)?;
    }
    parser.finish()
}

#[test]
fn named_unnamed_mixed_and_crlf_streams_preserve_semantic_frames() {
    // r[verify agent_harness_runtime.responses_sse_framing.metadata]
    // r[verify agent_harness_runtime.responses_sse_framing.guards]
    let baseline = parse(super::FINAL_STREAM).expect("named fixture succeeds");
    let text = format!(
        "{}\n\n",
        std::str::from_utf8(super::FINAL_STREAM)
            .expect("fixture UTF8")
            .trim_end()
    );
    for candidate in [
        unnamed(super::FINAL_STREAM),
        format!("{}{DONE_RECORD}", unnamed(super::FINAL_STREAM)),
        format!("{text}{DONE_RECORD}"),
        text.replacen(CREATED_HEADER, "", 1),
        unnamed(super::FINAL_STREAM).replace('\n', "\r\n"),
    ] {
        let actual = parse(candidate.as_bytes()).expect("reviewed framing succeeds");
        assert_eq!(actual.terminal(), baseline.terminal());
        assert_eq!(
            actual.provider_response_id(),
            baseline.provider_response_id()
        );
        assert_eq!(actual.frames(), baseline.frames());
        assert_eq!(actual.tool_calls().len(), baseline.tool_calls().len());
    }
}

#[test]
fn terminal_marker_never_replaces_completion_or_allows_later_data() {
    // r[verify agent_harness_runtime.responses_sse_framing.termination]
    let valid = unnamed(super::FINAL_STREAM);
    assert_eq!(
        parse(format!("{valid}{DONE_RECORD}").as_bytes())
            .expect("valid marker")
            .terminal(),
        crate::OpenAiStreamTerminal::Completed
    );
    let duplicate = format!("{valid}{DONE_RECORD}{DONE_RECORD}");
    let after_marker = format!("{valid}{DONE_RECORD}{valid}");
    let before_completion = format!("{DONE_RECORD}{valid}");
    let named_marker = format!("{valid}{COMPLETED_HEADER}{DONE_RECORD}");
    for invalid in [
        DONE_RECORD,
        &duplicate,
        &after_marker,
        &before_completion,
        &named_marker,
    ] {
        assert!(
            parse(invalid.as_bytes()).is_err(),
            "invalid marker placement must fail"
        );
    }
    let without_terminator = format!("{valid}data: [DONE]");
    assert!(parse(without_terminator.as_bytes()).is_ok());
    let failed = concat!(
        "data: {\"type\":\"response.created\",\"sequence_number\":0,\"response\":{\"id\":\"resp_failed\",\"status\":\"in_progress\"}}\n\n",
        "data: {\"type\":\"response.failed\",\"sequence_number\":1,\"response\":{\"id\":\"resp_failed\",\"status\":\"failed\"}}\n\n",
    );
    assert_eq!(
        parse(failed.as_bytes())
            .expect("failed terminal is observable")
            .terminal(),
        crate::OpenAiStreamTerminal::Failed
    );
    assert!(parse(format!("{failed}{DONE_RECORD}").as_bytes()).is_err());
}

#[test]
fn keepalive_comment_records_are_ignored_without_masking_violations() {
    // r[verify agent_harness_runtime.responses_sse_framing.keepalive]
    // r[verify agent_harness_runtime.responses_sse_framing.keepalive.guards]
    let baseline = parse(super::FINAL_STREAM).expect("baseline succeeds");
    let valid = unnamed(super::FINAL_STREAM);
    let keepalive = ": \n\n";
    let crlf = ": keep-alive\r\n\r\n";
    let mut parts = valid.split_inclusive("\n\n");
    let first = parts.next().expect("first record");
    let rest = parts.collect::<String>();
    let leading = format!("{keepalive}{keepalive}{valid}{DONE_RECORD}");
    let between = format!("{first}: ping\n\n{rest}{DONE_RECORD}");
    let inside = format!(
        "{}{rest}{DONE_RECORD}",
        first.replacen("data:", ": ping\ndata:", 1)
    );
    let crlf_variant = format!("{crlf}{valid}{DONE_RECORD}");
    for candidate in [leading, between, inside, crlf_variant] {
        let actual = parse(candidate.as_bytes()).expect("keep-alive framing succeeds");
        assert_eq!(actual.terminal(), baseline.terminal());
        assert_eq!(actual.frames(), baseline.frames());
    }
    let sequence = unnamed(super::FINAL_STREAM).replacen(SEQUENCE_ONE, SEQUENCE_ZERO, 1);
    assert_ne!(sequence, valid);
    let masked_sequence = format!("{keepalive}{sequence}{DONE_RECORD}");
    let premature_marker = format!("{keepalive}{DONE_RECORD}{valid}");
    let unknown_line = ": ping\njunk\n\n";
    for invalid in [keepalive, &masked_sequence, &premature_marker, unknown_line] {
        assert!(
            parse(invalid.as_bytes()).is_err(),
            "comments must not admit incomplete or invalid streams"
        );
    }
}

#[test]
fn optional_headers_preserve_metadata_sequence_and_reasoning_rejections() {
    // r[verify agent_harness_runtime.responses_sse_framing.metadata]
    // r[verify agent_harness_runtime.responses_sse_framing.guards]
    let named = std::str::from_utf8(super::FINAL_STREAM).expect("fixture UTF8");
    let data = unnamed(super::FINAL_STREAM);
    let mismatch = named.replacen(CREATED_HEADER, COMPLETED_HEADER, 1);
    let empty = named.replacen(CREATED_HEADER, "event:\n", 1);
    let duplicate = named.replacen(
        CREATED_HEADER,
        &format!("{CREATED_HEADER}{CREATED_HEADER}"),
        1,
    );
    let sequence = data.replacen(SEQUENCE_ONE, SEQUENCE_ZERO, 1);
    assert_ne!(sequence, data);
    let missing_type = data.replacen(
        "\"type\":\"response.created\"",
        "\"missing\":\"response.created\"",
        1,
    );
    assert_ne!(missing_type, data);
    for invalid in [
        mismatch,
        empty,
        duplicate,
        sequence,
        missing_type,
        "data: {}\n\n".to_owned(),
        "data: {}\ndata: {}\n\n".to_owned(),
        "id: unsupported\ndata: {}\n\n".to_owned(),
        unnamed(super::UNSAFE_REASONING_STREAM),
        unnamed(super::STALE_TOOL_STREAM),
    ] {
        assert!(parse(invalid.as_bytes()).is_err());
    }
}

#[test]
fn marker_consumes_event_and_byte_bounds() {
    // r[verify agent_harness_runtime.responses_sse_framing.guards]
    let base = unnamed(super::FINAL_STREAM);
    let value = format!("{base}{DONE_RECORD}");
    let mut bounds = crate::OpenAiStreamBounds::reviewed(MAX_RESPONSE_BYTES);
    bounds.events = base
        .lines()
        .filter(|line| line.starts_with("data:"))
        .count();
    let mut parser =
        crate::OpenAiResponsesSseParser::new(super::dispatch(), bounds).expect("bounds valid");
    assert!(parser.push_chunk(base.as_bytes()).is_ok());
    assert!(parser.push_chunk(DONE_RECORD.as_bytes()).is_err());
    let mut parser = crate::OpenAiResponsesSseParser::new(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(base.len()),
    )
    .expect("byte bounds valid");
    assert!(parser.push_chunk(value.as_bytes()).is_err());
    assert!(parse(value.as_bytes()).is_ok());
}

#[test]
fn unnamed_calls_retain_their_bound_arguments() {
    // r[verify agent_harness_runtime.responses_sse_framing.guards]
    let named = parse(super::TOOL_STREAM).expect("named tools parse");
    let data = format!("{}{DONE_RECORD}", unnamed(super::TOOL_STREAM));
    let actual = parse(data.as_bytes()).expect("unnamed tools parse");
    assert_eq!(actual.frames(), named.frames());
    assert_eq!(actual.tool_calls().len(), named.tool_calls().len());
    assert_eq!(
        actual.tool_calls()[0].arguments_json(),
        named.tool_calls()[0].arguments_json()
    );
    assert!(parse(unnamed(super::STALE_TOOL_STREAM).as_bytes()).is_err());
}
