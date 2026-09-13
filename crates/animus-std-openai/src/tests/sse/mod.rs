mod exchange;
mod framing;
mod guards;
const MODEL_CALL_ID: &str = "model-call-fixture";
const INITIAL_PROMPT: &str = "Use Steel to calculate.";
const TOOL_RESULT: &str = "3";
const STREAM_TOOL_DESCRIPTION: &str = "Evaluate one bounded Steel expression.";
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
const TOOL_STREAM: &[u8] = include_bytes!("../../../tests/fixtures/openai-responses-tool-call.sse");
const FINAL_STREAM: &[u8] = include_bytes!("../../../tests/fixtures/openai-responses-final.sse");
const UNSAFE_REASONING_STREAM: &[u8] =
    include_bytes!("../../../tests/fixtures/openai-responses-unknown-reasoning.sse");
const STALE_TOOL_STREAM: &[u8] =
    include_bytes!("../../../tests/fixtures/openai-responses-stale-tool.sse");
const EXPECTED_LOOP_CALLS: usize = 2;
const ONE_CALL: usize = 1;
const ZERO_CALLS: usize = 0;
const SMALL_STREAM_BOUND: u64 = 64;
const SMALL_EVENT_BOUND: u64 = 64;
const EVENT_BOUND: u64 = 128;
const ARGUMENT_BOUND: u64 = 32;
const TOOL_CALL_BOUND: u64 = 1;

struct FakeStreamingTransport {
    calls: std::cell::Cell<usize>,
    responses: std::cell::RefCell<std::collections::VecDeque<Vec<u8>>>,
    request_bodies: std::cell::RefCell<Vec<Vec<u8>>>,
}

impl FakeStreamingTransport {
    fn new(responses: impl IntoIterator<Item = Vec<u8>>) -> Self {
        Self {
            calls: std::cell::Cell::new(ZERO_CALLS),
            responses: std::cell::RefCell::new(responses.into_iter().collect()),
            request_bodies: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl crate::shell::StreamingTransport for FakeStreamingTransport {
    fn open_stream(
        &self,
        request: &crate::shell::WireRequest<'_>,
        api_key: &crate::OpenAiApiKey,
    ) -> Result<crate::shell::StreamingWireResponse, crate::TransportErrorClass> {
        assert_eq!(request.endpoint_url, crate::OFFICIAL_OPENAI_RESPONSES_URL);
        assert_eq!(api_key.expose_value(), crate::tests::API_KEY_VALUE);
        self.calls.set(
            self.calls
                .get()
                .checked_add(ONE_CALL)
                .expect("fake call count fits"),
        );
        self.request_bodies.borrow_mut().push(request.body.clone());
        let body = self
            .responses
            .borrow_mut()
            .pop_front()
            .expect("one fake stream must be configured for each contact");
        Ok(crate::shell::StreamingWireResponse {
            status: crate::tests::HTTP_OK,
            body: Box::new(std::io::Cursor::new(body)),
        })
    }
}

struct FixedCancellation(bool);

impl crate::StreamingCancellation for FixedCancellation {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

fn dispatch() -> crate::OpenAiToolDispatch {
    crate::OpenAiToolDispatch::initial(
        MODEL_CALL_ID,
        capabilities(),
        schemas(),
        crate::OpenAiToolPolicy::serial(crate::OpenAiToolChoice::auto()),
    )
    .expect("reviewed dispatch must validate")
}

fn capabilities() -> crate::OpenAiToolCapabilityMatrix {
    crate::OpenAiToolCapabilityMatrix::new(
        crate::tests::MODEL_ID,
        roles(),
        vec![
            crate::OpenAiToolCapability::new(
                crate::STEEL_REPL_TOOL_NAME,
                crate::STEEL_REPL_TOOL_SCHEMA,
            )
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

fn stream_channels() -> Vec<crate::OpenAiToolStreamChannel> {
    vec![
        crate::OpenAiToolStreamChannel::Content,
        crate::OpenAiToolStreamChannel::ReasoningSummary,
        crate::OpenAiToolStreamChannel::ToolCall,
        crate::OpenAiToolStreamChannel::Usage,
        crate::OpenAiToolStreamChannel::Finish,
    ]
}

fn schemas() -> Vec<crate::OpenAiToolSchemaBinding> {
    vec![
        crate::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, TOOL_SCHEMA_JSON)
            .expect("reviewed schema must validate"),
    ]
}

fn plan_tool_request(
    continuation: bool,
    tool_call_id: Option<&str>,
) -> animus_std::ProviderDispatchPlan {
    let canonical = canonical_tool_request(continuation, tool_call_id);
    let plan = animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &crate::tests::fixture::selected_model(),
        canonical_request: &canonical,
        reviewed_projection_fixture: if continuation {
            CONTINUATION_PROJECTION
        } else {
            INITIAL_PROJECTION
        },
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(crate::tests::CREDENTIAL_LABEL),
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
    .expect("tool dispatch plan must validate");
    assert!(plan.request().stream());
    assert_eq!(
        plan.credential_source_label(),
        crate::tests::CREDENTIAL_LABEL
    );
    plan
}

fn canonical_tool_request(
    continuation: bool,
    tool_call_id: Option<&str>,
) -> animus_adapters::CanonicalModelRequest {
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
        name: crate::STEEL_REPL_TOOL_NAME,
        description: STREAM_TOOL_DESCRIPTION,
        input_schema_label: crate::STEEL_REPL_TOOL_SCHEMA,
    }];
    let canonical = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(crate::tests::OUTPUT_LIMIT),
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
    assert_eq!(canonical.messages().len(), messages.len());
    assert_eq!(canonical.tools().len(), tools.len());
    canonical
}
