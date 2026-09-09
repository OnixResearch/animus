//! Chat-completions parser controls over literal Z.ai-shaped fixture records.

use crate::{
    OpenAiChatCompletionsSseParser, OpenAiStreamBounds, OpenAiStreamTerminal, OpenAiToolCapability,
    OpenAiToolCapabilityMatrix, OpenAiToolChoice, OpenAiToolDispatch, OpenAiToolPolicy,
    OpenAiToolSchemaBinding, OpenAiToolStreamChannel, STEEL_REPL_TOOL_NAME, STEEL_REPL_TOOL_SCHEMA,
};
use animus_adapters::ModelStreamFrameKind;

const MODEL_CALL_ID: &str = "chat-model-call-fixture";
const TOOL_SCHEMA_JSON: &[u8] = br#"{"type":"object","properties":{"code":{"type":"string"}},"required":["code"],"additionalProperties":false}"#;
const RESPONSE_ID: &str = "chatcmpl-fixture-1";
const TOOL_CALL_ID: &str = "call_fixture_1";
const ANSWER: &str = "3";

mod lifecycle;
mod continuation;
mod fixture;

fn fresh_parser() -> OpenAiChatCompletionsSseParser {
    OpenAiChatCompletionsSseParser::new(dispatch(), OpenAiStreamBounds::reviewed(4_096))
        .expect("reviewed bounds must validate")
}

fn dispatch() -> OpenAiToolDispatch {
    OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        OpenAiToolCapabilityMatrix::new(
            "glm-5.3-flash",
            vec![
                animus_adapters::CanonicalModelRole::User,
                animus_adapters::CanonicalModelRole::Tool,
            ],
            vec![
                OpenAiToolCapability::new(STEEL_REPL_TOOL_NAME, STEEL_REPL_TOOL_SCHEMA)
                    .expect("reviewed tool must validate"),
            ],
            vec![
                OpenAiToolStreamChannel::Content,
                OpenAiToolStreamChannel::ReasoningSummary,
                OpenAiToolStreamChannel::ToolCall,
                OpenAiToolStreamChannel::Usage,
                OpenAiToolStreamChannel::Finish,
            ],
        )
        .expect("reviewed capability matrix must validate"),
        vec![
            OpenAiToolSchemaBinding::from_json(STEEL_REPL_TOOL_SCHEMA, TOOL_SCHEMA_JSON)
                .expect("reviewed schema must validate"),
        ],
        OpenAiToolPolicy::serial(OpenAiToolChoice::required()),
    )
    .expect("dispatch must validate")
}

fn record(body: &str) -> String {
    format!("data: {body}\n\n")
}

fn chunk(delta: &serde_json::Value, finish: Option<&str>) -> String {
    let finish_value = finish.map_or(serde_json::Value::Null, serde_json::Value::from);
    record(
        &serde_json::json!({"id": RESPONSE_ID, "object": "chat.completion.chunk",
            "model": "glm-5.3-flash",
            "choices": [{"index": 0, "delta": delta, "finish_reason": finish_value}]})
        .to_string(),
    )
}

#[test]
fn full_tool_stream_parses_to_canonical_frames() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(
            chunk(
                &serde_json::json!({"role": "assistant", "content": ANSWER}),
                None,
            )
            .as_bytes(),
        )
        .expect("content chunk must parse");
    parser
        .push_chunk(chunk(&serde_json::json!({"reasoning_content": "thinking"}), None).as_bytes())
        .expect("reasoning chunk must parse");
    parser
        .push_chunk(
            record(
                &serde_json::json!({"id": RESPONSE_ID, "choices": [{"index": 0,
                "delta": {"tool_calls": [{"index": 0, "id": TOOL_CALL_ID, "type": "function",
                "function": {"name": STEEL_REPL_TOOL_NAME, "arguments": "{\"code\":\"("}}]}}]})
                .to_string(),
            )
            .as_bytes(),
        )
        .expect("first tool fragment must parse");
    parser
        .push_chunk(
            record(
                &serde_json::json!({"id": RESPONSE_ID, "choices": [{"index": 0,
                "delta": {"tool_calls": [{"index": 0, "function": {"arguments": "+ 1 2)\"}"}}]}}]})
                .to_string(),
            )
            .as_bytes(),
        )
        .expect("second tool fragment must parse");
    parser
        .push_chunk(
            record(
                &serde_json::json!({"id": RESPONSE_ID, "choices": [{"index": 0, "delta": {},
                "finish_reason": "tool_calls"}],
                "usage": {"prompt_tokens": 9, "completion_tokens": 5}})
                .to_string(),
            )
            .as_bytes(),
        )
        .expect("finish chunk must parse");
    parser
        .push_chunk(b"data: [DONE]\n\n")
        .expect("done marker must parse");
    let outcome = parser.finish().expect("completed stream must finish");

    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Completed);
    assert_eq!(outcome.provider_response_id(), RESPONSE_ID);
    assert_eq!(outcome.tool_calls().len(), 1);
    assert_eq!(outcome.tool_calls()[0].tool_name(), STEEL_REPL_TOOL_NAME);
    assert_eq!(
        outcome.tool_calls()[0].arguments_json(),
        "{\"code\":\"(+ 1 2)\"}"
    );
    let kinds: Vec<ModelStreamFrameKind> = outcome
        .frames()
        .iter()
        .map(<animus_adapters::ModelStreamFrame>::kind)
        .collect();
    assert_eq!(
        kinds,
        vec![
            ModelStreamFrameKind::ContentDelta,
            ModelStreamFrameKind::ThoughtDelta,
            ModelStreamFrameKind::ToolCallDelta,
            ModelStreamFrameKind::ToolCallDelta,
            ModelStreamFrameKind::Usage,
            ModelStreamFrameKind::Finish,
        ]
    );
    let usage = outcome
        .frames()
        .iter()
        .find(|frame| frame.kind() == ModelStreamFrameKind::Usage)
        .and_then(<animus_adapters::ModelStreamFrame>::usage)
        .expect("usage frame must carry usage");
    assert_eq!(usage.input_tokens(), 9);
    assert_eq!(usage.output_tokens(), 5);
}

#[test]
fn content_only_stream_completes_with_separate_usage_record() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(chunk(&serde_json::json!({"content": ANSWER}), None).as_bytes())
        .expect("content chunk must parse");
    parser
        .push_chunk(chunk(&serde_json::json!({}), Some("stop")).as_bytes())
        .expect("finish must parse");
    parser
        .push_chunk(
            record(
                &serde_json::json!({"id": RESPONSE_ID, "choices": [],
                "usage": {"prompt_tokens": 3, "completion_tokens": 1}})
                .to_string(),
            )
            .as_bytes(),
        )
        .expect("usage-only record must parse");
    parser
        .push_chunk(b"data: [DONE]\n\n")
        .expect("done must parse");
    let outcome = parser.finish().expect("stream must finish");
    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Completed);
    assert!(outcome.tool_calls().is_empty());
    let bodies: Vec<&str> = outcome
        .frames()
        .iter()
        .filter(|frame| frame.kind() == ModelStreamFrameKind::ContentDelta)
        .filter_map(|frame| frame.body())
        .collect();
    assert_eq!(bodies, vec![ANSWER]);
}

#[test]
fn unsupported_finish_reason_fails_the_stream_but_accepts_done() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(chunk(&serde_json::json!({"content": "partial"}), Some("length")).as_bytes())
        .expect("finish with length must parse");
    parser
        .push_chunk(b"data: [DONE]\n\n")
        .expect("done after failure must parse");
    let outcome = parser.finish().expect("failed stream still finishes");
    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Failed);
}

#[test]
fn premature_done_trailing_bytes_and_double_done_fail() {
    let mut parser = fresh_parser();
    assert!(parser.push_chunk(b"data: [DONE]\n\n").is_err());

    let mut parser2 = fresh_parser();
    parser2
        .push_chunk(chunk(&serde_json::json!({}), Some("stop")).as_bytes())
        .expect("finish must parse");
    parser2
        .push_chunk(b"data: [DONE]\n\n")
        .expect("first done must parse");
    assert!(parser2.push_chunk(b"data: [DONE]\n\n").is_err());
    assert!(parser2.finish().is_err());

    let mut parser3 = fresh_parser();
    parser3
        .push_chunk(chunk(&serde_json::json!({}), Some("stop")).as_bytes())
        .expect("finish must parse");
    assert!(parser3.finish().is_err());

    let mut parser4 = fresh_parser();
    parser4
        .push_chunk(chunk(&serde_json::json!({}), Some("stop")).as_bytes())
        .expect("finish must parse");
    parser4
        .push_chunk(b"data: [DONE]\n\n")
        .expect("done must parse");
    assert!(parser4.finish().is_ok());
}

#[test]
fn bounds_identity_and_unadmitted_tools_fail_closed() {
    let oversized_chunk = vec![b'x'; 64];
    let mut bounded =
        OpenAiChatCompletionsSseParser::new(dispatch(), OpenAiStreamBounds::reviewed(16))
            .expect("bounds must validate");
    assert!(bounded.push_chunk(&oversized_chunk).is_err());

    let mut drift_parser = fresh_parser();
    let drifted = record(
        &serde_json::json!({"id": "chatcmpl-other", "choices": [{"index": 0,
        "delta": {"content": "x"}, "finish_reason": null}]})
        .to_string(),
    );
    let mut bytes = chunk(&serde_json::json!({"content": ""}), None).into_bytes();
    bytes.extend_from_slice(drifted.as_bytes());
    assert!(drift_parser.push_chunk(bytes.as_slice()).is_err());

    let mut unadmitted_parser = fresh_parser();
    let unadmitted = record(
        &serde_json::json!({"id": RESPONSE_ID, "choices": [{"index": 0,
        "delta": {"tool_calls": [{"index": 0, "id": "call_other", "type": "function",
        "function": {"name": "other_tool", "arguments": "{}"}}]}}]})
        .to_string(),
    );
    assert!(unadmitted_parser.push_chunk(unadmitted.as_bytes()).is_err());
}

#[test]
fn keepalive_comments_and_cancellation_stay_bounded() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(b": keepalive\n\n")
        .expect("comment record must parse");
    parser
        .push_chunk(chunk(&serde_json::json!({"content": ANSWER}), None).as_bytes())
        .expect("content chunk must parse");
    parser.cancel().expect("cancellation must emit");
    let outcome = parser.finish().expect("cancelled stream must finish");
    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Cancelled);
}
