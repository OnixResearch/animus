//! Explicit chat plans and finite transport responses for continuation controls.

pub(super) const MODEL: &str = "glm-5.3-flash";
pub(super) const CREDENTIAL: &str = "env:ZAI_API_KEY";
pub(super) const PROMPT: &str = "Use Steel and preserve this original instruction.";
pub(super) const RESULT: &str = "3";
pub(super) const REASONING: &str = "private reasoning: first then second";
const HTTP_OK: u16 = 200;
const RESPONSE_LIMIT: u64 = 65_536;
const REQUEST_TIMEOUT_SECONDS: u64 = 30;
const OUTPUT_TOKENS: u32 = 1_024;

pub(super) fn config() -> crate::OpenAiHostConfig {
    crate::OpenAiHostConfig::new(
        crate::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY,
        crate::ZAI_CODING_PLAN_COMPLETIONS_URL,
        std::time::Duration::from_secs(REQUEST_TIMEOUT_SECONDS),
        RESPONSE_LIMIT,
    )
    .expect("chat config")
}

pub(super) struct Cancel(pub(super) bool);
impl crate::StreamingCancellation for Cancel {
    fn is_cancelled(&self) -> bool {
        self.0
    }
}

pub(super) struct Transport {
    responses: std::cell::RefCell<std::collections::VecDeque<Vec<u8>>>,
    pub(super) bodies: std::cell::RefCell<Vec<serde_json::Value>>,
}
impl Transport {
    pub(super) fn new(streams: impl IntoIterator<Item = Vec<u8>>) -> Self {
        Self {
            responses: std::cell::RefCell::new(streams.into_iter().collect()),
            bodies: std::cell::RefCell::new(Vec::new()),
        }
    }
}
impl crate::shell::StreamingTransport for Transport {
    fn open_stream(
        &self,
        request: &crate::shell::WireRequest<'_>,
        key: &crate::OpenAiApiKey,
    ) -> Result<crate::shell::StreamingWireResponse, crate::TransportErrorClass> {
        assert_eq!(request.endpoint_url, crate::ZAI_CODING_PLAN_COMPLETIONS_URL);
        assert_eq!(key.source_label(), CREDENTIAL);
        self.bodies
            .borrow_mut()
            .push(serde_json::from_slice(&request.body).expect("request JSON"));
        let stream = self
            .responses
            .borrow_mut()
            .pop_front()
            .expect("finite fixture stream");
        Ok(crate::shell::StreamingWireResponse {
            status: HTTP_OK,
            body: Box::new(std::io::Cursor::new(stream)),
        })
    }
}

pub(super) fn run(
    transport: &Transport,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: crate::OpenAiToolDispatch,
) -> Result<crate::OpenAiStreamOutcome, crate::HostError> {
    let key = crate::OpenAiApiKey::new(CREDENTIAL, String::from("synthetic-never-for-network"))
        .expect("synthetic key");
    crate::shell::execute_chat_stream_with_transport(
        transport,
        crate::shell::StreamingExecutionRequest {
            config: &config(),
            plan,
            dispatch,
            api_key: &key,
            cancellation: &Cancel(false),
        },
    )
}

pub(super) fn tool_stream(id: &str) -> Vec<u8> {
    let (prefix, suffix) = REASONING
        .split_once(" then ")
        .expect("two reasoning fragments");
    let thought = super::chunk(
        &serde_json::json!({"reasoning_content": format!("{prefix} then ")}),
        None,
    ) + &super::chunk(&serde_json::json!({"reasoning_content": suffix}), None);
    let call = super::chunk(
        &serde_json::json!({"tool_calls": [{"index": 0, "id": id, "type": "function",
        "function": {"name": crate::STEEL_REPL_TOOL_NAME, "arguments": "{\"code\":\"(+ 1 2)\"}"}}]}),
        Some("tool_calls"),
    );
    (thought + &call + "data: [DONE]\n\n").into_bytes()
}

pub(super) fn plan(text: &str, call_id: Option<&str>) -> animus_std::ProviderDispatchPlan {
    plan_reviewed(text, call_id, "family:openai_chat").expect("chat plan")
}

pub(super) fn plan_reviewed(
    text: &str,
    call_id: Option<&str>,
    family: &str,
) -> Result<animus_std::ProviderDispatchPlan, animus_std::ProviderDispatchError> {
    let role = if call_id.is_some() { "tool" } else { "user" };
    let content = [animus_adapters::CanonicalMessageContentPartDraft::Text { text }];
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role,
        content: &content,
        tool_call_id: call_id,
    }];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: crate::STEEL_REPL_TOOL_NAME,
        description: "Evaluate Steel.",
        input_schema_label: crate::STEEL_REPL_TOOL_SCHEMA,
    }];
    let canonical = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_TOKENS),
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
    .expect("canonical chat request");
    let role_projection = format!("role:{role}");
    let text_projection = format!("text:{text}");
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected(),
        canonical_request: &canonical,
        reviewed_projection_fixture: &[
            family,
            &role_projection,
            &text_projection,
            "tool:steel_repl",
            "response:text",
            "reasoning:low",
            "stream:true",
        ],
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(CREDENTIAL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("chat-fixture-deadline"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("chat-fixture-cancel"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
}

fn selected() -> animus_std::SelectedModel {
    let provider_id = animus_std::ProviderId::new("zai").expect("provider");
    let model_id = animus_std::ModelId::new(MODEL).expect("model");
    let model = animus_std::ModelEntry::new(
        model_id.clone(),
        animus_std::ApiKind::OpenAiChat,
        animus_std::EndpointIdentity::new(crate::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY)
            .expect("endpoint"),
        super::super::selected_model().capabilities().clone(),
    );
    let credential = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::environment("ZAI_API_KEY").expect("source"),
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id.clone(), credential, vec![model])
        .expect("provider entry");
    animus_std::ProviderRegistry::new(vec![provider])
        .expect("registry")
        .select_model(&animus_std::ModelSelection::new(
            provider_id,
            model_id,
            animus_std::ThinkingLevel::Low,
            animus_std::ThinkingPolicy::Reject,
        ))
        .expect("selected model")
}
