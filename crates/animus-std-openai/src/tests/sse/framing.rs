//! Data-only framing shares the named stream's semantic validation.

const MAX_RESPONSE_BYTES: u64 = 262_144;
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
fn reasoning_text_items_parse_as_redacted_thoughts() {
    // r[verify agent_harness_runtime.responses_sse_framing.reasoning_text]
    // r[verify agent_harness_runtime.responses_sse_framing.reasoning_text.parts]
    let reasoning_stream = concat!(
        ": keep-alive\n\n",
        "data: {\"type\":\"response.created\",\"sequence_number\":0,\"response\":{\"id\":\"resp_reasoning\",\"status\":\"in_progress\"}}\n\n",
        "data: {\"type\":\"response.in_progress\",\"sequence_number\":1,\"response\":{\"id\":\"resp_reasoning\",\"status\":\"in_progress\"}}\n\n",
        "data: {\"type\":\"response.output_item.added\",\"sequence_number\":2,\"output_index\":0,\"item\":{\"id\":\"rs_item\",\"type\":\"reasoning\",\"status\":\"in_progress\",\"summary\":[]}}\n\n",
        "data: {\"type\":\"response.content_part.added\",\"sequence_number\":3,\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"reasoning_text\",\"text\":\"\"}}\n\n",
        "data: {\"type\":\"response.reasoning_text.delta\",\"sequence_number\":4,\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"delta\":\"thin\"}\n\n",
        "data: {\"type\":\"response.reasoning_text.delta\",\"sequence_number\":5,\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"delta\":\"king\"}\n\n",
        "data: {\"type\":\"response.reasoning_text.done\",\"sequence_number\":6,\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"text\":\"thinking\"}\n\n",
        "data: {\"type\":\"response.content_part.done\",\"sequence_number\":7,\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"reasoning_text\",\"text\":\"thinking\"}}\n\n",
        "data: {\"type\":\"response.output_item.done\",\"sequence_number\":8,\"output_index\":0,\"item\":{\"id\":\"rs_item\",\"type\":\"reasoning\",\"status\":\"complete\",\"summary\":[]}}\n\n",
        "data: {\"type\":\"response.output_item.added\",\"sequence_number\":9,\"output_index\":1,\"item\":{\"id\":\"msg_item\",\"type\":\"message\",\"role\":\"assistant\",\"content\":[]}}\n\n",
        "data: {\"type\":\"response.content_part.added\",\"sequence_number\":10,\"item_id\":\"msg_item\",\"output_index\":1,\"content_index\":0,\"part\":{\"type\":\"output_text\",\"text\":\"\",\"annotations\":[]}}\n\n",
        "data: {\"type\":\"response.output_text.delta\",\"sequence_number\":11,\"item_id\":\"msg_item\",\"output_index\":1,\"content_index\":0,\"delta\":\"keep\"}\n\n",
        "data: {\"type\":\"response.output_text.done\",\"sequence_number\":12,\"item_id\":\"msg_item\",\"output_index\":1,\"content_index\":0,\"text\":\"keep\"}\n\n",
        "data: {\"type\":\"response.content_part.done\",\"sequence_number\":13,\"item_id\":\"msg_item\",\"output_index\":1,\"content_index\":0,\"part\":{\"type\":\"output_text\",\"text\":\"keep\",\"annotations\":[]}}\n\n",
        "data: {\"type\":\"response.output_item.done\",\"sequence_number\":14,\"output_index\":1,\"item\":{\"id\":\"msg_item\",\"type\":\"message\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"keep\",\"annotations\":[]}]}}\n\n",
        "data: {\"type\":\"response.completed\",\"sequence_number\":15,\"response\":{\"id\":\"resp_reasoning\",\"status\":\"completed\",\"usage\":{\"input_tokens\":16,\"output_tokens\":2}}}\n\n",
        "data: [DONE]\n\n",
    );
    let outcome = parse(reasoning_stream.as_bytes()).expect("reasoning text stream parses");
    assert_eq!(outcome.terminal(), crate::OpenAiStreamTerminal::Completed);
    let visible: String = outcome
        .frames()
        .iter()
        .filter(|frame| frame.is_default_final_visible())
        .filter_map(animus_adapters::ModelStreamFrame::body)
        .collect();
    assert_eq!(visible, "keep");
    let uncorrelated = reasoning_stream.replacen(
        "\"content_index\":0,\"delta\":\"thin\"",
        "\"delta\":\"thin\"",
        1,
    );
    assert_ne!(uncorrelated, reasoning_stream);
    assert!(
        parse(uncorrelated.as_bytes()).is_err(),
        "uncorrelated reasoning delta must fail"
    );
    let unknown_item = reasoning_stream.replacen(
        "\"item_id\":\"rs_item\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"reasoning_text\"",
        "\"item_id\":\"rs_unknown\",\"output_index\":0,\"content_index\":0,\"part\":{\"type\":\"reasoning_text\"",
        1,
    );
    assert_ne!(unknown_item, reasoning_stream);
    assert!(
        parse(unknown_item.as_bytes()).is_err(),
        "unknown reasoning item part must fail"
    );
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
    bounds.events = u64::try_from(
        base.lines()
            .filter(|line| line.starts_with("data:"))
            .count(),
    )
    .expect("fixture event count fits portable bound");
    let mut parser =
        crate::OpenAiResponsesSseParser::new(super::dispatch(), bounds).expect("bounds valid");
    assert!(parser.push_chunk(base.as_bytes()).is_ok());
    assert!(parser.push_chunk(DONE_RECORD.as_bytes()).is_err());
    let mut parser = crate::OpenAiResponsesSseParser::new(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(
            u64::try_from(base.len()).expect("fixture byte count fits portable bound"),
        ),
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
