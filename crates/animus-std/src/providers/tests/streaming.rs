use super::*;

const CALL_ID: &str = "call-provider-main";
const TOOL_ID: &str = "tool-main";
const TOOL_FRAME_INDEX: usize = 3;
const THOUGHT_FRAME_INDEX: usize = 2;
const OPENAI_FRAME_COUNT: usize = 6;
const ANTHROPIC_FRAME_COUNT: usize = 5;
const CONTENT_AND_THOUGHT_FRAME_COUNT: usize = 2;
const CONTENT_THOUGHT_TERMINAL_FRAME_COUNT: usize = 3;
const THOUGHT_STREAMING_CAPABILITY: &str = "thought_streaming";
const INPUT_TOKENS: u32 = 7;
const OUTPUT_TOKENS: u32 = 11;
const EXPECTED_ANTHROPIC_OUTPUT: &str = "Hi";
const EXPECTED_OPENAI_OUTPUT: &str = "Hello";

#[test]
fn openai_provider_stream_fixture_maps_to_canonical_frames() {
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint]
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint.fixture]
    // r[verify agent_harness_runtime.provider_streaming.thought_policy]
    // r[verify agent_harness_runtime.provider_streaming.verification]
    let tool_chunk = format!(
        "event: response.function_call_arguments.delta\ndata: {TOOL_ID}|{{\"name\":\"read\"}}\n\n"
    );
    let usage_chunk = format!(
        "event: response.usage\ndata: {INPUT_TOKENS}|{OUTPUT_TOKENS}\n\nevent: response.completed\ndata: stop\n\n"
    );
    let chunks = [
        "event: response.output_text.delta\ndata: Hel\n\n",
        "event: response.output_text.delta\ndata: lo\n\nevent: response.reasoning.delta\ndata: hidden reasoning\n\n",
        tool_chunk.as_str(),
        usage_chunk.as_str(),
    ];
    let frames = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &chunks,
    })
    .expect("openai provider stream should parse");

    assert_eq!(frames.len(), OPENAI_FRAME_COUNT);
    assert_eq!(
        frames[THOUGHT_FRAME_INDEX].kind(),
        animus_adapters::ModelStreamFrameKind::ThoughtDelta
    );
    assert_eq!(
        frames[THOUGHT_FRAME_INDEX].visibility(),
        animus_adapters::ModelStreamVisibility::Redacted
    );
    assert_eq!(
        frames[TOOL_FRAME_INDEX]
            .tool_call_id()
            .expect("tool frame has id")
            .as_str(),
        TOOL_ID
    );
    let response = animus_adapters::assemble_visible_model_stream_response(frames.as_slice())
        .expect("visible output should assemble");
    assert_eq!(response.message(), EXPECTED_OPENAI_OUTPUT);
}

#[test]
fn anthropic_provider_stream_fixture_maps_to_canonical_frames() {
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint]
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint.fixture]
    // r[verify agent_harness_runtime.provider_streaming.thought_policy]
    let tool_chunk =
        format!("event: input_json_delta\ndata: {TOOL_ID}|{{\"query\":\"status\"}}\n\n");
    let usage_chunk = format!(
        "event: message_delta\ndata: {INPUT_TOKENS}|{OUTPUT_TOKENS}\n\nevent: message_stop\ndata: stop\n\n"
    );
    let chunks = [
        "event: content_block_delta\ndata: Hi\n\nevent: thinking_delta\ndata: hidden summary\n\n",
        tool_chunk.as_str(),
        usage_chunk.as_str(),
    ];
    let frames = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::AnthropicMessages,
        call_id: CALL_ID,
        chunks: &chunks,
    })
    .expect("anthropic provider stream should parse");

    assert_eq!(frames.len(), ANTHROPIC_FRAME_COUNT);
    let usage = frames[TOOL_FRAME_INDEX]
        .usage()
        .expect("usage frame follows tool frame");
    assert_eq!(usage.input_tokens(), INPUT_TOKENS);
    assert_eq!(usage.output_tokens(), OUTPUT_TOKENS);
    assert_eq!(
        animus_adapters::assemble_visible_model_stream_response(frames.as_slice())
            .expect("visible output should assemble")
            .message(),
        EXPECTED_ANTHROPIC_OUTPUT
    );
}

#[test]
fn provider_streaming_dispatch_plans_after_capability_and_shell_gates() {
    // r[verify agent_harness_runtime.provider_streaming.host_boundary]
    // r[verify agent_harness_runtime.provider_streaming.capability_gate]
    // r[verify agent_harness_runtime.provider_streaming.verification]
    let selected = registry()
        .select_model(&selection(ThinkingLevel::Low, ThinkingPolicy::Reject))
        .expect("model should select");
    let plan = plan_provider_streaming_dispatch(&ProviderStreamingDispatchInput {
        selected_model: &selected,
        request: ModelStreamingRequest::content_and_thought(),
        unsupported_policy: UnsupportedStreamPolicy::Reject,
        credential: ProviderStreamingCredentialState::Present,
        service: ProviderStreamingServiceState::Available,
    })
    .expect("provider streaming dispatch should plan");

    assert_eq!(plan.provider_id(), PROVIDER_ID);
    assert_eq!(plan.model_id(), MODEL_ID);
    assert_eq!(plan.api_kind(), ApiKind::OpenAiResponses);
    assert_eq!(plan.modes().content(), StreamRequestMode::Required);
    assert_eq!(plan.modes().thought(), StreamRequestMode::Required);
}

#[test]
fn downgrade_does_not_fabricate_thought_mode() {
    // r[verify agent_harness_runtime.provider_streaming.capability_gate]
    // r[verify agent_harness_runtime.provider_streaming.thought_policy]
    let selected = content_only_registry()
        .select_model(&selection(ThinkingLevel::Low, ThinkingPolicy::Reject))
        .expect("content-only model should select");
    let plan = plan_provider_streaming_dispatch(&ProviderStreamingDispatchInput {
        selected_model: &selected,
        request: ModelStreamingRequest::content_and_thought(),
        unsupported_policy: UnsupportedStreamPolicy::ContentOnlyDowngrade,
        credential: ProviderStreamingCredentialState::NotRequired,
        service: ProviderStreamingServiceState::Available,
    })
    .expect("content-only downgrade should plan");

    assert_eq!(plan.modes().content(), StreamRequestMode::Required);
    assert_eq!(plan.modes().thought(), StreamRequestMode::Disabled);
    assert!(plan.modes().downgraded());
}

#[test]
#[cfg_attr(animus_octet, allow(tigerstyle::numeric_units))]
fn dispatch_fails_closed_before_provider_contact() {
    // r[verify agent_harness_runtime.provider_streaming.capability_gate]
    // r[verify agent_harness_runtime.provider_streaming.verification]
    let selected = content_only_registry()
        .select_model(&selection(ThinkingLevel::Low, ThinkingPolicy::Reject))
        .expect("content-only model should select");

    let unsupported_thought_stream_error =
        plan_provider_streaming_dispatch(&ProviderStreamingDispatchInput {
            selected_model: &selected,
            request: ModelStreamingRequest::content_and_thought(),
            unsupported_policy: UnsupportedStreamPolicy::Reject,
            credential: ProviderStreamingCredentialState::NotRequired,
            service: ProviderStreamingServiceState::Available,
        })
        .expect_err("unsupported thought streaming should fail");
    assert!(matches!(
        unsupported_thought_stream_error,
        ProviderStreamingError::Capability(RegistryError::UnsupportedStreamCapability {
            capability: THOUGHT_STREAMING_CAPABILITY,
        })
    ));

    let missing_credential = plan_provider_streaming_dispatch(&ProviderStreamingDispatchInput {
        selected_model: &selected,
        request: ModelStreamingRequest::content_only(),
        unsupported_policy: UnsupportedStreamPolicy::Reject,
        credential: ProviderStreamingCredentialState::MissingRequired,
        service: ProviderStreamingServiceState::Available,
    })
    .expect_err("missing credential should fail closed");
    assert_eq!(
        missing_credential,
        ProviderStreamingError::MissingCredential
    );

    let missing_service = plan_provider_streaming_dispatch(&ProviderStreamingDispatchInput {
        selected_model: &selected,
        request: ModelStreamingRequest::content_only(),
        unsupported_policy: UnsupportedStreamPolicy::Reject,
        credential: ProviderStreamingCredentialState::NotRequired,
        service: ProviderStreamingServiceState::Missing,
    })
    .expect_err("missing service should fail closed");
    assert_eq!(missing_service, ProviderStreamingError::MissingService);
}

#[test]
fn malformed_provider_streams_fail_closed() {
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint.malformed]
    // r[verify agent_harness_runtime.provider_streaming.verification]
    let missing_finish = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &["event: response.output_text.delta\ndata: partial\n\n"],
    })
    .expect_err("missing finish should fail");
    assert!(matches!(
        missing_finish,
        ProviderStreamingError::Canonical(animus_adapters::ModelStreamError::MissingTerminalFrame)
    ));

    let terminal_not_last = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &[
            "event: response.completed\ndata: stop\n\n",
            "event: response.output_text.delta\ndata: late\n\n",
        ],
    })
    .expect_err("terminal followed by content should fail");
    assert!(matches!(
        terminal_not_last,
        ProviderStreamingError::Canonical(
            animus_adapters::ModelStreamError::TerminalFrameNotLast { .. }
        )
    ));

    let truncated = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &["event: response.output_text.delta\ndata: partial"],
    })
    .expect_err("truncated record should fail");
    assert!(matches!(
        truncated,
        ProviderStreamingError::MalformedProviderStream { .. }
    ));

    let unknown = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &["event: response.unknown_delta\ndata: surprise\n\n"],
    })
    .expect_err("unknown channel should fail");
    assert!(matches!(
        unknown,
        ProviderStreamingError::UnknownProviderChannel { .. }
    ));
}

#[test]
fn provider_error_and_cancellation_records_do_not_assemble_success() {
    // r[verify agent_harness_runtime.provider_streaming.parser_entrypoint.malformed]
    // r[verify agent_harness_runtime.provider_streaming.thought_policy]
    let error_frames = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &[
            "event: response.output_text.delta\ndata: partial\n\n",
            "event: response.error\ndata: provider_error\n\n",
        ],
    })
    .expect("provider error is represented as canonical failure");
    let error_assembly =
        animus_adapters::assemble_visible_model_stream_response(error_frames.as_slice())
            .expect_err("failure terminal cannot assemble success");
    assert!(matches!(
        error_assembly,
        animus_adapters::ModelStreamError::TerminalFrameNotFinish {
            kind: animus_adapters::ModelStreamFrameKind::Failure,
        }
    ));

    let cancelled_frames = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::AnthropicMessages,
        call_id: CALL_ID,
        chunks: &[
            "event: content_block_delta\ndata: partial\n\n",
            "event: message_cancelled\ndata: user_cancelled\n\n",
        ],
    })
    .expect("provider cancellation is represented as canonical cancellation");
    let cancellation_assembly =
        animus_adapters::assemble_visible_model_stream_response(cancelled_frames.as_slice())
            .expect_err("cancelled terminal cannot assemble success");
    assert!(matches!(
        cancellation_assembly,
        animus_adapters::ModelStreamError::TerminalFrameNotFinish {
            kind: animus_adapters::ModelStreamFrameKind::Cancelled,
        }
    ));
}

#[test]
fn unsupported_provider_family_has_no_concrete_parser_claim() {
    // r[verify agent_harness_runtime.provider_streaming.host_boundary]
    let error = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::Mock,
        call_id: CALL_ID,
        chunks: &["mock provider chunks are ignored before parser selection"],
    })
    .expect_err("mock provider has no concrete parser claim");

    assert_eq!(
        error,
        ProviderStreamingError::UnsupportedProviderFamily {
            api_kind: ApiKind::Mock,
        }
    );
}

fn content_only_registry() -> ProviderRegistry {
    let entry = ModelEntry::new(
        model_id(),
        ApiKind::OpenAiResponses,
        endpoint(),
        content_only_capabilities(),
    );
    let provider = ProviderEntry::new(provider_id(), CredentialRequirement::none(), vec![entry])
        .expect("content-only provider should validate");
    ProviderRegistry::new(vec![provider]).expect("content-only registry should validate")
}

fn content_only_capabilities() -> ModelCapabilities {
    ModelCapabilities::from_draft(ModelCapabilitiesDraft {
        input_modalities: vec![InputModality::Text],
        context_budget: CONTEXT_BUDGET,
        output_budget: OUTPUT_BUDGET,
        reasoning: ReasoningSupport::None,
        thinking_levels: vec![ThinkingLevel::None, ThinkingLevel::Low],
        streaming: ModelStreamingCapabilities::new(
            StreamSupport::Supported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
        ),
        cost: None,
    })
    .expect("content-only capabilities should validate")
}

#[test]
fn content_and_thought_parser_fixture_has_expected_count() {
    let frames = parse_provider_stream_chunks(&ProviderStreamParserInput {
        api_kind: ApiKind::OpenAiResponses,
        call_id: CALL_ID,
        chunks: &[
            "event: response.output_text.delta\ndata: hi\n\n",
            "event: response.reasoning.delta\ndata: hidden\n\n",
            "event: response.completed\ndata: stop\n\n",
        ],
    })
    .expect("fixture should parse");

    assert_eq!(frames.len(), CONTENT_THOUGHT_TERMINAL_FRAME_COUNT);
    assert_eq!(
        frames
            .iter()
            .filter(|frame| {
                matches!(
                    frame.kind(),
                    animus_adapters::ModelStreamFrameKind::ContentDelta
                        | animus_adapters::ModelStreamFrameKind::ThoughtDelta
                )
            })
            .count(),
        CONTENT_AND_THOUGHT_FRAME_COUNT
    );
    assert_eq!(
        frames
            .last()
            .expect("fixture includes terminal frame")
            .kind(),
        animus_adapters::ModelStreamFrameKind::Finish
    );
}
