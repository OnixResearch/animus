use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::io::Cursor;

use super::*;
use crate::shell::{
    StreamingExecutionRequest, StreamingTransport, StreamingWireResponse,
    execute_tool_stream_with_transport,
};

const MODEL_CALL_ID: &str = "model-call-fixture";
const INITIAL_PROMPT: &str = "Use Steel to calculate.";
const TOOL_RESULT: &str = "3";
const TOOL_DESCRIPTION: &str = "Evaluate one bounded Steel expression.";
const TOOL_SCHEMA_JSON: &[u8] = br#"{
  "type": "object",
  "properties": {
    "code": {"type": "string"}
  },
  "required": ["code"],
  "additionalProperties": false
}"#;
const INITIAL_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Use Steel to calculate.",
    "tool:steel_repl",
    "response:text",
    "reasoning:low",
    "stream:true",
];
const CONTINUATION_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:tool",
    "text:3",
    "tool:steel_repl",
    "response:text",
    "reasoning:low",
    "stream:true",
];
const TOOL_STREAM: &[u8] = include_bytes!("../../tests/fixtures/openai-responses-tool-call.sse");
const FINAL_STREAM: &[u8] = include_bytes!("../../tests/fixtures/openai-responses-final.sse");
const UNSAFE_REASONING_STREAM: &[u8] =
    include_bytes!("../../tests/fixtures/openai-responses-unknown-reasoning.sse");
const STALE_TOOL_STREAM: &[u8] =
    include_bytes!("../../tests/fixtures/openai-responses-stale-tool.sse");
const HTTP_OK: u16 = 200;
const EXPECTED_LOOP_CALLS: usize = 2;
const ONE_CALL: usize = 1;
const ZERO_CALLS: usize = 0;
const SMALL_STREAM_BOUND: usize = 64;
const SMALL_EVENT_BOUND: usize = 64;
const EVENT_BOUND: usize = 128;
const ARGUMENT_BOUND: usize = 32;
const TOOL_CALL_BOUND: usize = 1;

struct FakeStreamingTransport {
    calls: Cell<usize>,
    responses: RefCell<VecDeque<Vec<u8>>>,
    request_bodies: RefCell<Vec<Vec<u8>>>,
}

impl FakeStreamingTransport {
    fn new(responses: impl IntoIterator<Item = Vec<u8>>) -> Self {
        Self {
            calls: Cell::new(ZERO_CALLS),
            responses: RefCell::new(responses.into_iter().collect()),
            request_bodies: RefCell::new(Vec::new()),
        }
    }
}

impl StreamingTransport for FakeStreamingTransport {
    fn open_stream(
        &self,
        request: &crate::shell::WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<StreamingWireResponse, TransportErrorClass> {
        assert_eq!(request.endpoint_url, OFFICIAL_OPENAI_RESPONSES_URL);
        assert_eq!(api_key.expose_value(), API_KEY_VALUE);
        self.calls.set(self.calls.get().saturating_add(ONE_CALL));
        self.request_bodies.borrow_mut().push(request.body.clone());
        let body = self
            .responses
            .borrow_mut()
            .pop_front()
            .expect("one fake stream must be configured for each contact");
        Ok(StreamingWireResponse {
            status: HTTP_OK,
            body: Box::new(Cursor::new(body)),
        })
    }
}

struct FixedCancellation(bool);

impl StreamingCancellation for FixedCancellation {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

#[test]
#[ignore = "requires explicit ANIMUS_OPENAI_TOOL_LIVE_API_KEY and ANIMUS_OPENAI_TOOL_LIVE_MODEL"]
fn explicit_live_openai_tool_stream_smoke() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.authority]
    let model = std::env::var("ANIMUS_OPENAI_TOOL_LIVE_MODEL")
        .expect("explicit live tool model is required");
    assert_eq!(
        model, MODEL_ID,
        "live tool smoke model must match reviewed fixture"
    );
    let key_value = std::env::var("ANIMUS_OPENAI_TOOL_LIVE_API_KEY")
        .expect("explicit live tool API key is required");
    let host = ReqwestOpenAiHost::new(config()).expect("live host must configure");
    let plan = plan_tool_request(false, None);
    let outcome = host
        .execute_tool_stream(OpenAiToolStreamRequest {
            plan: &plan,
            dispatch: dispatch(),
            api_key: &OpenAiApiKey::new(CREDENTIAL_LABEL, key_value)
                .expect("live API key must validate"),
            cancellation: &FixedCancellation(false),
        })
        .expect("explicit live tool stream must succeed");
    let evidence = render_openai_tool_stream_evidence(&outcome);
    assert!(evidence.contains("LIVE_OPENAI_TOOL_STREAM status=completed"));
    assert!(evidence.contains("raw_response=omitted"));
    assert!(evidence.contains("tool_arguments=omitted"));
    assert!(evidence.contains("tool_output=omitted"));
    assert!(evidence.contains("credential=omitted"));
    assert!(evidence.contains("reasoning=omitted"));
    println!("{evidence}");
}

#[test]
fn literal_tool_stream_and_continuation_share_the_production_parser() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.streaming]
    // r[verify agent_harness_runtime.tool_capable_openai_host.continuation]
    let transport = FakeStreamingTransport::new([TOOL_STREAM.to_vec(), FINAL_STREAM.to_vec()]);
    let capabilities = capabilities();
    let schemas = schemas();
    let initial_plan = plan_tool_request(false, None);
    let initial_dispatch = OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        capabilities.clone(),
        schemas.clone(),
        OpenAiToolPolicy::serial(OpenAiToolChoice::required()),
    )
    .expect("initial dispatch must validate");
    let initial = execute_tool_stream_with_transport(
        &transport,
        StreamingExecutionRequest {
            config: &config(),
            plan: &initial_plan,
            dispatch: initial_dispatch,
            api_key: &api_key(CREDENTIAL_LABEL),
            cancellation: &FixedCancellation(false),
        },
    )
    .expect("literal tool stream must parse");

    assert_eq!(initial.terminal(), OpenAiStreamTerminal::Completed);
    assert_eq!(initial.provider_response_id(), "resp_tool_fixture");
    assert_eq!(initial.tool_calls().len(), ONE_CALL);
    assert_eq!(initial.tool_calls()[0].tool_name(), STEEL_REPL_TOOL_NAME);
    assert_eq!(
        initial.tool_calls()[0].arguments_json(),
        r#"{"code":"(+ 1 2)"}"#
    );
    assert_eq!(
        initial.retry_stream_state(),
        animus_runtime::provider_retry::ProviderRetryStreamState::Committed(
            animus_runtime::provider_retry::ProviderRetrySemanticFrame::Reasoning,
        )
    );
    assert!(initial.frames().iter().any(|frame| {
        frame.kind() == animus_adapters::ModelStreamFrameKind::ThoughtDelta
            && frame.visibility() == animus_adapters::ModelStreamVisibility::Redacted
    }));

    let binding = initial.tool_calls()[0].continuation_binding();
    let result = OpenAiToolResult::for_binding(&binding, TOOL_RESULT)
        .expect("correlated tool result must validate");
    let continuation_plan = plan_tool_request(true, Some(binding.tool_call_id()));
    let continuation_dispatch = OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        capabilities,
        schemas,
        OpenAiToolPolicy::serial(OpenAiToolChoice::auto()),
    )
    .expect("continuation cohort must validate")
    .continue_with(binding, result)
    .expect("continuation dispatch must validate");
    let final_outcome = execute_tool_stream_with_transport(
        &transport,
        StreamingExecutionRequest {
            config: &config(),
            plan: &continuation_plan,
            dispatch: continuation_dispatch,
            api_key: &api_key(CREDENTIAL_LABEL),
            cancellation: &FixedCancellation(false),
        },
    )
    .expect("literal continuation stream must parse");

    assert_eq!(transport.calls.get(), EXPECTED_LOOP_CALLS);
    assert_eq!(final_outcome.provider_response_id(), "resp_final_fixture");
    assert!(final_outcome.tool_calls().is_empty());
    let request_bodies = transport.request_bodies.borrow();
    let initial_body: serde_json::Value =
        serde_json::from_slice(&request_bodies[0]).expect("initial request JSON must parse");
    let continuation_body: serde_json::Value =
        serde_json::from_slice(&request_bodies[1]).expect("continuation request JSON must parse");
    assert_eq!(initial_body["tools"][0]["name"], STEEL_REPL_TOOL_NAME);
    assert_eq!(initial_body["tools"][0]["strict"], true);
    assert_eq!(initial_body["parallel_tool_calls"], false);
    assert_eq!(
        continuation_body["previous_response_id"],
        "resp_tool_fixture"
    );
    assert_eq!(
        continuation_body["input"][0]["type"],
        "function_call_output"
    );
    assert_eq!(continuation_body["input"][0]["call_id"], "call_fixture");
    assert_eq!(continuation_body["input"][0]["output"], TOOL_RESULT);
}

#[test]
fn unsupported_tools_and_lossy_schemas_fail_before_contact() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.capabilities]
    // r[verify agent_harness_runtime.tool_capable_openai_host.projection]
    assert!(matches!(
        OpenAiToolCapability::new("other_tool", STEEL_REPL_TOOL_SCHEMA),
        Err(HostError::UnsupportedPlan { .. })
    ));
    let nested_open_schema = br#"{
      "type": "object",
      "properties": {
        "request": {
          "type": "object",
          "properties": {"code": {"type": "string"}},
          "required": ["code"]
        }
      },
      "required": ["request"],
      "additionalProperties": false
    }"#;
    assert!(matches!(
        OpenAiToolSchemaBinding::from_json(STEEL_REPL_TOOL_SCHEMA, nested_open_schema),
        Err(HostError::InvalidToolSchema { .. })
    ));

    let transport = FakeStreamingTransport::new([TOOL_STREAM.to_vec()]);
    let plan = plan_tool_request(false, None);
    let wrong_model = OpenAiToolCapabilityMatrix::new(
        "other-model",
        roles(),
        vec![
            OpenAiToolCapability::new(STEEL_REPL_TOOL_NAME, STEEL_REPL_TOOL_SCHEMA)
                .expect("reviewed tool must validate"),
        ],
        stream_channels(),
    )
    .expect("bounded capability matrix must validate");
    let dispatch = OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        wrong_model,
        schemas(),
        OpenAiToolPolicy::serial(OpenAiToolChoice::auto()),
    )
    .expect("dispatch shape must validate");
    assert!(matches!(
        execute_tool_stream_with_transport(
            &transport,
            StreamingExecutionRequest {
                config: &config(),
                plan: &plan,
                dispatch,
                api_key: &api_key(CREDENTIAL_LABEL),
                cancellation: &FixedCancellation(false),
            },
        ),
        Err(HostError::UnsupportedPlan { .. })
    ));
    assert_eq!(transport.calls.get(), ZERO_CALLS);
}

#[test]
fn malformed_reasoning_correlation_bounds_and_cancellation_fail_safely() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.streaming]
    // r[verify agent_harness_runtime.tool_capable_openai_host.evidence]
    let unsafe_error = parse_openai_responses_sse_chunks(
        dispatch(),
        OpenAiStreamBounds::reviewed(
            usize::try_from(MAX_RESPONSE_BYTES).expect("test bound must fit usize"),
        ),
        &[UNSAFE_REASONING_STREAM],
    )
    .expect_err("hidden reasoning event must fail closed");
    assert_eq!(
        unsafe_error.class(),
        OpenAiStreamFailureClass::UnsafeReasoning
    );
    assert!(!format!("{unsafe_error:?}").contains("private chain"));

    let stale_error = parse_openai_responses_sse_chunks(
        dispatch(),
        OpenAiStreamBounds::reviewed(
            usize::try_from(MAX_RESPONSE_BYTES).expect("test bound must fit usize"),
        ),
        &[STALE_TOOL_STREAM],
    )
    .expect_err("stale tool item identity must fail closed");
    assert_eq!(stale_error.class(), OpenAiStreamFailureClass::Correlation);

    let bounded_error = parse_openai_responses_sse_chunks(
        dispatch(),
        OpenAiStreamBounds {
            stream_bytes: SMALL_STREAM_BOUND,
            event_bytes: SMALL_EVENT_BOUND,
            events: EVENT_BOUND,
            tool_argument_bytes: ARGUMENT_BOUND,
            tool_calls: TOOL_CALL_BOUND,
        },
        &[TOOL_STREAM],
    )
    .expect_err("aggregate stream overflow must fail closed");
    assert_eq!(
        bounded_error.class(),
        OpenAiStreamFailureClass::BoundExceeded
    );

    let mut cancelled = OpenAiResponsesSseParser::new(
        dispatch(),
        OpenAiStreamBounds::reviewed(
            usize::try_from(MAX_RESPONSE_BYTES).expect("test bound must fit usize"),
        ),
    )
    .expect("parser bounds must validate");
    cancelled
        .cancel()
        .expect("pre-response cancellation must emit");
    let outcome = cancelled
        .finish()
        .expect("cancelled stream must finish safely");
    assert_eq!(outcome.terminal(), OpenAiStreamTerminal::Cancelled);
    assert!(outcome.provider_response_id().is_empty());
    assert!(!format!("{outcome:?}").contains("resp_"));

    let transport = FakeStreamingTransport::new([TOOL_STREAM.to_vec()]);
    let plan = plan_tool_request(false, None);
    assert!(matches!(
        execute_tool_stream_with_transport(
            &transport,
            StreamingExecutionRequest {
                config: &config(),
                plan: &plan,
                dispatch: dispatch(),
                api_key: &api_key(CREDENTIAL_LABEL),
                cancellation: &FixedCancellation(true),
            },
        ),
        Err(HostError::Cancelled)
    ));
    assert_eq!(transport.calls.get(), ZERO_CALLS);
}

fn dispatch() -> OpenAiToolDispatch {
    OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        capabilities(),
        schemas(),
        OpenAiToolPolicy::serial(OpenAiToolChoice::auto()),
    )
    .expect("reviewed dispatch must validate")
}

fn capabilities() -> OpenAiToolCapabilityMatrix {
    OpenAiToolCapabilityMatrix::new(
        MODEL_ID,
        roles(),
        vec![
            OpenAiToolCapability::new(STEEL_REPL_TOOL_NAME, STEEL_REPL_TOOL_SCHEMA)
                .expect("reviewed tool must validate"),
        ],
        stream_channels(),
    )
    .expect("reviewed capability matrix must validate")
}

fn roles() -> Vec<animus_adapters::CanonicalModelRole> {
    vec![
        animus_adapters::CanonicalModelRole::User,
        animus_adapters::CanonicalModelRole::Tool,
    ]
}

fn stream_channels() -> Vec<OpenAiToolStreamChannel> {
    vec![
        OpenAiToolStreamChannel::Content,
        OpenAiToolStreamChannel::ReasoningSummary,
        OpenAiToolStreamChannel::ToolCall,
        OpenAiToolStreamChannel::Usage,
        OpenAiToolStreamChannel::Finish,
    ]
}

fn schemas() -> Vec<OpenAiToolSchemaBinding> {
    vec![
        OpenAiToolSchemaBinding::from_json(STEEL_REPL_TOOL_SCHEMA, TOOL_SCHEMA_JSON)
            .expect("reviewed schema must validate"),
    ]
}

fn plan_tool_request(
    continuation: bool,
    tool_call_id: Option<&str>,
) -> animus_std::ProviderDispatchPlan {
    let text = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: if continuation {
            TOOL_RESULT
        } else {
            INITIAL_PROMPT
        },
    }];
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: if continuation { "tool" } else { "user" },
        content: &text,
        tool_call_id,
    }];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: STEEL_REPL_TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: STEEL_REPL_TOOL_SCHEMA,
    }];
    let canonical = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_LIMIT),
                temperature_millis: None,
                stop_sequences: &[],
                stream: true,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: true,
                effort: animus_adapters::CanonicalModelReasoningEffort::Low,
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .expect("canonical tool request must validate");
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected_model(),
        canonical_request: &canonical,
        reviewed_projection_fixture: if continuation {
            CONTINUATION_PROJECTION
        } else {
            INITIAL_PROJECTION
        },
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(CREDENTIAL_LABEL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline-tool-stream"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancel-tool-stream"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .expect("tool dispatch plan must validate")
}
