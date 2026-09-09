//! Accepted and rejected lifecycle controls for the chat stream.

use super::{ANSWER, RESPONSE_ID, TOOL_CALL_ID, chunk, fresh_parser, record};
use crate::{OpenAiStreamTerminal, STEEL_REPL_TOOL_NAME};
use animus_adapters::{ModelStreamFrame, ModelStreamFrameKind};
use serde_json::json;

const DONE: &[u8] = b"data: [DONE]\n\n";
const VALID_ARGUMENTS: &str = r#"{"code":"(+ 1 2)"}"#;
const OTHER_CHOICE_INDEX: u64 = 1;

#[test]
fn final_content_delta_survives_every_byte_boundary() {
    let stream = chunk(&json!({"content": ANSWER}), Some("stop")) + "data: [DONE]\n\n";
    for split in 1..stream.len() {
        let mut parser = fresh_parser();
        parser
            .push_chunk(&stream.as_bytes()[..split])
            .expect("first fragment");
        parser
            .push_chunk(&stream.as_bytes()[split..])
            .expect("second fragment");
        let outcome = parser.finish().expect("complete stream");
        let bodies: Vec<&str> = outcome
            .frames()
            .iter()
            .filter(|frame| frame.kind() == ModelStreamFrameKind::ContentDelta)
            .filter_map(ModelStreamFrame::body)
            .collect();
        assert_eq!(bodies, vec![ANSWER]);
    }
}

#[test]
fn final_tool_delta_is_not_lost() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(
            chunk(
                &json!({"tool_calls": [{"index": 0, "id": TOOL_CALL_ID,
        "type": "function", "function": {"name": STEEL_REPL_TOOL_NAME,
        "arguments": VALID_ARGUMENTS}}]}),
                Some("tool_calls"),
            )
            .as_bytes(),
        )
        .expect("final call delta");
    parser.push_chunk(DONE).expect("done");
    let outcome = parser.finish().expect("complete call");
    assert_eq!(outcome.tool_calls().len(), 1);
    assert_eq!(outcome.tool_calls()[0].arguments_json(), VALID_ARGUMENTS);
}

#[test]
fn local_dspark_reasoning_delta_is_accepted_as_thought_and_combined_deltas_keep_content() {
    // The private local-dspark vLLM emits reasoning under `reasoning`, not `reasoning_content`.
    let mut parser = fresh_parser();
    parser
        .push_chunk(chunk(&json!({"reasoning": "think then"}), None).as_bytes())
        .expect("reasoning delta");
    parser
        .push_chunk(chunk(&json!({"content": ANSWER, "reasoning": "."}), None).as_bytes())
        .expect("combined delta");
    parser
        .push_chunk(chunk(&json!({}), Some("stop")).as_bytes())
        .expect("finish");
    parser.push_chunk(DONE).expect("done");
    let outcome = parser.finish().expect("complete stream");
    let thoughts: Vec<&str> = outcome
        .frames()
        .iter()
        .filter(|frame| frame.kind() == ModelStreamFrameKind::ThoughtDelta)
        .filter_map(ModelStreamFrame::body)
        .collect();
    assert_eq!(thoughts, vec!["think then", "."]);
    let content: Vec<&str> = outcome
        .frames()
        .iter()
        .filter(|frame| frame.kind() == ModelStreamFrameKind::ContentDelta)
        .filter_map(ModelStreamFrame::body)
        .collect();
    assert_eq!(content, vec![ANSWER]);
}

#[test]
fn rejected_record_permanently_blocks_success() {
    let mut parser = fresh_parser();
    parser
        .push_chunk(chunk(&json!({}), Some("stop")).as_bytes())
        .expect("finish reason");
    assert!(parser.push_chunk(b"data: invalid-json\n\n").is_err());
    assert!(parser.push_chunk(DONE).is_err());
    assert!(parser.cancel().is_err());
    assert!(parser.finish().is_err());
}

#[test]
fn done_rejects_usage_comments_and_content() {
    let usage = record(
        &json!({"id": RESPONSE_ID, "choices": [],
        "usage": {"prompt_tokens": 1, "completion_tokens": 1}})
        .to_string(),
    );
    for suffix in [
        usage,
        String::from(": keepalive\n\n"),
        chunk(&json!({"content": ANSWER}), None),
    ] {
        let mut parser = fresh_parser();
        parser
            .push_chunk(chunk(&json!({}), Some("stop")).as_bytes())
            .expect("finish reason");
        parser.push_chunk(DONE).expect("done");
        assert!(parser.push_chunk(suffix.as_bytes()).is_err());
        assert!(parser.finish().is_err());
    }
}

#[test]
fn cancellation_before_response_identity_is_valid_and_final() {
    let mut parser = fresh_parser();
    parser.cancel().expect("pre-response cancellation");
    let outcome = parser.finish().expect("cancelled outcome");
    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Cancelled);
    assert!(outcome.provider_response_id().is_empty());

    let mut parser = fresh_parser();
    parser.cancel().expect("cancel");
    assert!(
        parser
            .push_chunk(chunk(&json!({}), None).as_bytes())
            .is_err()
    );
    assert!(parser.finish().is_err());
}

#[test]
fn malformed_completed_arguments_never_produce_a_tool_call() {
    for arguments in ["{", "", "[]", "null", "1", "{} trailing"] {
        let mut parser = fresh_parser();
        parser
            .push_chunk(
                chunk(
                    &json!({"tool_calls": [{"index": 0, "id": TOOL_CALL_ID,
            "type": "function", "function": {"name": STEEL_REPL_TOOL_NAME,
            "arguments": arguments}}]}),
                    None,
                )
                .as_bytes(),
            )
            .expect("argument fragment");
        assert!(
            parser
                .push_chunk(chunk(&json!({}), Some("tool_calls")).as_bytes())
                .is_err()
        );
        assert!(parser.finish().is_err());
    }
}

#[test]
fn nonzero_choice_and_malformed_deltas_are_rejected() {
    let other = record(
        &json!({"id": RESPONSE_ID, "choices": [{"index": OTHER_CHOICE_INDEX,
        "delta": {"content": ANSWER}, "finish_reason": null}]})
        .to_string(),
    );
    let mut parser = fresh_parser();
    assert!(parser.push_chunk(other.as_bytes()).is_err());
    for delta in [
        json!({"content": 1}),
        json!({"reasoning_content": []}),
        json!({"tool_calls": {}}),
        json!({"role": "user"}),
        json!({"function_call": {}}),
    ] {
        let mut parser = fresh_parser();
        assert!(parser.push_chunk(chunk(&delta, None).as_bytes()).is_err());
    }
}

#[test]
fn wrong_model_and_tool_identity_are_rejected() {
    let mut parser = fresh_parser();
    let wrong_model = chunk(&json!({}), None).replace("glm-5.3-flash", "another-model");
    assert!(parser.push_chunk(wrong_model.as_bytes()).is_err());

    let mut parser = fresh_parser();
    let calls = json!({"tool_calls": [
        {"index": 0, "id": TOOL_CALL_ID, "type": "function", "function": {
            "name": STEEL_REPL_TOOL_NAME, "arguments": VALID_ARGUMENTS}},
        {"index": 1, "id": TOOL_CALL_ID, "type": "function", "function": {
            "name": STEEL_REPL_TOOL_NAME, "arguments": VALID_ARGUMENTS}}]});
    assert!(parser.push_chunk(chunk(&calls, None).as_bytes()).is_err());
}
