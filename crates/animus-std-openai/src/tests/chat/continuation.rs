//! Conversation-preserving chat continuation and pre-contact denial controls.

use super::fixture;

const NEXT_TOOL_ID: &str = "next-steel-call";
const EXPECTED_CONTACTS: usize = 3;
const EXPECTED_FINAL_MESSAGES: usize = 5;
const FIRST_ASSISTANT: usize = 1;
const FIRST_RESULT: usize = 2;
const SECOND_ASSISTANT: usize = 3;
const SECOND_RESULT: usize = 4;

fn continuation(call: &crate::OpenAiCompletedToolCall) -> crate::OpenAiToolDispatch {
    let binding = call.continuation_binding();
    let result =
        crate::OpenAiToolResult::for_binding(&binding, fixture::RESULT).expect("tool result");
    super::dispatch()
        .continue_with(binding, result)
        .expect("correlated continuation")
}

#[test]
fn serial_continuations_preserve_original_messages_and_reasoning() {
    let final_stream = (super::chunk(
        &serde_json::json!({"content": "verified answer"}),
        Some("stop"),
    ) + "data: [DONE]\n\n")
        .into_bytes();
    let transport = fixture::Transport::new([
        fixture::tool_stream(super::TOOL_CALL_ID),
        fixture::tool_stream(NEXT_TOOL_ID),
        final_stream,
    ]);
    let first = fixture::run(
        &transport,
        &fixture::plan(fixture::PROMPT, None),
        super::dispatch(),
    )
    .expect("first call");
    let second = fixture::run(
        &transport,
        &fixture::plan(fixture::RESULT, Some(super::TOOL_CALL_ID)),
        continuation(&first.tool_calls()[0]),
    )
    .expect("second call");
    let final_outcome = fixture::run(
        &transport,
        &fixture::plan(fixture::RESULT, Some(NEXT_TOOL_ID)),
        continuation(&second.tool_calls()[0]),
    )
    .expect("final call");
    assert_eq!(
        final_outcome.terminal(),
        crate::OpenAiStreamTerminal::Completed
    );
    assert!(final_outcome.tool_calls().is_empty());
    let bodies = transport.bodies.borrow();
    assert_eq!(bodies.len(), EXPECTED_CONTACTS);
    let last = bodies.last().expect("last body");
    let messages = last["messages"].as_array().expect("messages");
    assert_eq!(messages.len(), EXPECTED_FINAL_MESSAGES);
    assert_eq!(messages[0]["content"], fixture::PROMPT);
    for index in [FIRST_ASSISTANT, SECOND_ASSISTANT] {
        assert_eq!(messages[index]["role"], "assistant");
        assert_eq!(messages[index]["reasoning_content"], fixture::REASONING);
        assert_eq!(
            messages[index]["tool_calls"][0]["function"]["arguments"],
            "{\"code\":\"(+ 1 2)\"}"
        );
    }
    for index in [FIRST_RESULT, SECOND_RESULT] {
        assert_eq!(messages[index]["role"], "tool");
        assert_eq!(messages[index]["content"], fixture::RESULT);
    }
    let binding = first.tool_calls()[0].continuation_binding();
    let redacted = format!("{binding:?} {first:?}");
    assert!(!redacted.contains(fixture::PROMPT));
    assert!(!redacted.contains(fixture::REASONING));
}

#[test]
fn chat_planner_rejects_a_responses_projection_fixture() {
    assert_eq!(
        fixture::plan(fixture::PROMPT, None).api_kind(),
        animus_std::ApiKind::OpenAiChat
    );
    assert!(matches!(
        fixture::plan_reviewed(fixture::PROMPT, None, "family:openai_responses"),
        Err(animus_std::ProviderDispatchError::RequestProjection { .. })
    ));
}

#[test]
fn repeated_tool_identity_is_not_accepted_across_turns() {
    let transport = fixture::Transport::new([
        fixture::tool_stream(super::TOOL_CALL_ID),
        fixture::tool_stream(super::TOOL_CALL_ID),
    ]);
    let first = fixture::run(
        &transport,
        &fixture::plan(fixture::PROMPT, None),
        super::dispatch(),
    )
    .expect("first call");
    let result = fixture::run(
        &transport,
        &fixture::plan(fixture::RESULT, Some(super::TOOL_CALL_ID)),
        continuation(&first.tool_calls()[0]),
    );
    assert!(matches!(result, Err(crate::HostError::Stream { source })
        if source.class() == crate::OpenAiStreamFailureClass::Correlation));
}

#[test]
fn response_binding_cannot_resume_chat_even_with_retained_history() {
    let transport = fixture::Transport::new([fixture::tool_stream(super::TOOL_CALL_ID)]);
    let first = fixture::run(
        &transport,
        &fixture::plan(fixture::PROMPT, None),
        super::dispatch(),
    )
    .expect("first call");
    let mut binding = first.tool_calls()[0].continuation_binding();
    binding.api_kind = animus_std::ApiKind::OpenAiResponses;
    let output = crate::OpenAiToolResult::for_binding(&binding, fixture::RESULT).expect("result");
    let dispatch = super::dispatch()
        .continue_with(binding, output)
        .expect("correlated result");
    let denied = fixture::Transport::new([]);
    assert!(
        fixture::run(
            &denied,
            &fixture::plan(fixture::RESULT, Some(super::TOOL_CALL_ID)),
            dispatch
        )
        .is_err()
    );
    assert!(denied.bodies.borrow().is_empty());
}

#[test]
fn request_uses_explicit_zai_thinking_and_omits_unset_temperature() {
    let body = crate::chat::build_zai_chat_completions_request_body(
        &fixture::config(),
        &fixture::plan(fixture::PROMPT, None),
        &super::dispatch(),
    )
    .expect("body");
    let body: serde_json::Value = serde_json::from_slice(&body).expect("JSON");
    assert_eq!(body["thinking"]["type"], "enabled");
    assert_eq!(body["thinking"]["clear_thinking"], false);
    assert_eq!(body["reasoning_effort"], "low");
    assert!(body.get("temperature").is_none());
}

#[test]
fn unbound_parser_call_cannot_invent_missing_history() {
    let mut parser = super::fresh_parser();
    parser
        .push_chunk(&fixture::tool_stream(super::TOOL_CALL_ID))
        .expect("tool stream");
    let outcome = parser.finish().expect("parsed call");
    let transport = fixture::Transport::new([]);
    assert!(
        fixture::run(
            &transport,
            &fixture::plan(fixture::RESULT, Some(super::TOOL_CALL_ID)),
            continuation(&outcome.tool_calls()[0])
        )
        .is_err()
    );
    assert!(transport.bodies.borrow().is_empty());
}

#[test]
fn credential_and_cancellation_denials_never_contact_transport() {
    let transport = fixture::Transport::new([]);
    let plan = fixture::plan(fixture::PROMPT, None);
    for (label, cancelled) in [("env:OTHER_KEY", false), (fixture::CREDENTIAL, true)] {
        let key = crate::OpenAiApiKey::new(label, String::from("synthetic")).expect("key");
        let result = crate::shell::execute_chat_stream_with_transport(
            &transport,
            crate::shell::StreamingExecutionRequest {
                config: &fixture::config(),
                plan: &plan,
                dispatch: super::dispatch(),
                api_key: &key,
                cancellation: &fixture::Cancel(cancelled),
            },
        );
        assert!(result.is_err());
        assert!(transport.bodies.borrow().is_empty());
    }
}
