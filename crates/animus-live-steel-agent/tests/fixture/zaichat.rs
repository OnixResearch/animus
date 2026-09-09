//! Real Steel execution with offline Coding Plan request and SSE adapters.

const MODEL: &str = "glm-5.3-flash";
const PROMPT: &str = "Use Steel to calculate; retain this original instruction.";
const THOUGHT: &str = "synthetic private reasoning to preserve";
const TOOL_ID: &str = "coding-plan-steel-call";
const EXPECTED_REQUESTS: usize = 2;
const EXPECTED_CONTINUATION_MESSAGES: usize = 3;
const ASSISTANT_INDEX: usize = 1;
const TOOL_RESULT_INDEX: usize = 2;
type Captures = std::sync::Arc<std::sync::Mutex<Vec<serde_json::Value>>>;
type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

struct OfflineChat {
    streams: std::collections::VecDeque<Vec<u8>>,
    host: animus_std_openai::OpenAiHostConfig,
    captures: Captures,
}

impl animus_live_steel_agent::ProviderStreamService for OfflineChat {
    fn stream(
        &mut self,
        plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, animus_live_steel_agent::ProviderStreamError>
    {
        let stream = self
            .streams
            .pop_front()
            .ok_or(animus_live_steel_agent::ProviderStreamError::Unavailable)?;
        let body =
            animus_std_openai::build_zai_chat_completions_request_body(&self.host, plan, &dispatch)
                .map_err(|error| {
                    eprintln!("offline request rejected: {error}");
                    animus_live_steel_agent::ProviderStreamError::Planning
                })?;
        assert_eq!(plan.api_kind(), animus_std::ApiKind::OpenAiChat);
        assert_eq!(
            plan.endpoint(),
            animus_std_openai::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY
        );
        self.captures
            .lock()
            .expect("capture lock")
            .push(serde_json::from_slice(&body).expect("request JSON"));
        let bound =
            usize::try_from(self.host.max_response_bytes()).expect("bounded response bytes");
        let mut parser = animus_std_openai::OpenAiChatCompletionsSseParser::for_request(
            &self.host,
            plan,
            dispatch,
            animus_std_openai::OpenAiStreamBounds::reviewed(bound),
        )
        .map_err(|_error| animus_live_steel_agent::ProviderStreamError::Planning)?;
        parser
            .push_chunk(&stream)
            .map_err(|_error| animus_live_steel_agent::ProviderStreamError::Provider)?;
        parser
            .finish()
            .map_err(|_error| animus_live_steel_agent::ProviderStreamError::Provider)
    }
}

fn run(
    streams: impl IntoIterator<Item = Vec<u8>>,
    captures: Captures,
) -> TestResult<animus_live_steel_agent::LiveSteelAgentRunOutput> {
    let mut value: serde_json::Value = serde_json::from_slice(super::CONFIG)?;
    value["provider"]["provider_id"] = serde_json::json!("zai");
    value["provider"]["model_id"] = serde_json::json!(MODEL);
    value["provider"]["endpoint_identity"] =
        serde_json::json!(animus_std_openai::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY);
    value["provider"]["endpoint_url"] =
        serde_json::json!(animus_std_openai::ZAI_CODING_PLAN_COMPLETIONS_URL);
    value["provider"]["credential_reference"] = serde_json::json!("brokered:injected-zai-api-key");
    let config = super::parse_live_steel_agent_config(&serde_json::to_vec(&value)?)?;
    let host = animus_std_openai::OpenAiHostConfig::new(
        animus_std_openai::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY,
        animus_std_openai::ZAI_CODING_PLAN_COMPLETIONS_URL,
        config.provider_timeout(),
        config.max_provider_response_bytes(),
    )?;
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
        .map_err(|_error| "fixture kernel")?;
    let composition =
        super::LiveSteelAgentComposition::new(super::LiveSteelAgentCompositionInput {
            config,
            provider: OfflineChat {
                streams: streams.into_iter().collect(),
                host,
                captures,
            },
            session_persistence: super::MemoryPersistence::default(),
            snapshot_persistence: super::MemorySnapshotPersistence::default(),
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        })?;
    Ok(composition.run_prompt(PROMPT)?)
}

fn stream(delta: &serde_json::Value, finish: &str, done: bool) -> Vec<u8> {
    let value = serde_json::json!({"id": "synthetic-chat-response", "model": MODEL, "object": "chat.completion.chunk",
        "choices": [{"index": 0, "delta": delta, "finish_reason": finish}]});
    let suffix = if done { "data: [DONE]\n\n" } else { "" };
    format!("data: {value}\n\n{suffix}").into_bytes()
}

fn tool(done: bool) -> Vec<u8> {
    stream(
        &serde_json::json!({"reasoning_content": THOUGHT, "tool_calls": [{"index": 0, "id": TOOL_ID,
        "type": "function", "function": {"name": "steel_repl", "arguments": "{\"code\":\"(+ 1 2)\"}"}}]}),
        "tool_calls",
        done,
    )
}

#[test]
fn coding_plan_continuation_uses_the_real_steel_result_and_original_prompt() -> TestResult {
    let captures = Captures::default();
    let result = run(
        [
            tool(true),
            stream(&serde_json::json!({"content": "3"}), "stop", true),
        ],
        Captures::clone(&captures),
    );
    assert!(
        result.is_ok(),
        "offline run failed after {} request(s): {:?}",
        captures.lock().expect("capture lock").len(),
        result.as_ref().err()
    );
    let output = result?;
    assert_eq!(output.final_text(), "3");
    assert_eq!(
        output.receipt().tool_dispatch_count(),
        super::EXPECTED_TOOL_DISPATCHES
    );
    assert_eq!(
        output.receipt().host_observation_count(),
        super::EXPECTED_ZERO_HOST_EFFECTS
    );
    let requests = captures.lock().expect("capture lock");
    assert_eq!(requests.len(), EXPECTED_REQUESTS);
    let messages = requests.last().expect("continuation")["messages"]
        .as_array()
        .expect("messages");
    assert_eq!(messages.len(), EXPECTED_CONTINUATION_MESSAGES);
    assert_eq!(messages[0], requests[0]["messages"][0]);
    assert_eq!(messages[0]["content"], format!("user: {PROMPT}"));
    assert_eq!(messages[ASSISTANT_INDEX]["reasoning_content"], THOUGHT);
    assert_eq!(messages[TOOL_RESULT_INDEX]["tool_call_id"], TOOL_ID);
    assert_eq!(messages[TOOL_RESULT_INDEX]["content"], "3");
    for private in [PROMPT, THOUGHT, TOOL_ID, "(+ 1 2)"] {
        assert!(!output.receipt().render().contains(private));
    }
    Ok(())
}

#[test]
fn missing_done_or_exhausted_provider_never_produces_an_accepted_answer() {
    for done in [false, true] {
        let captures = Captures::default();
        assert!(run([tool(done)], Captures::clone(&captures)).is_err());
        assert_eq!(captures.lock().expect("capture lock").len(), 1);
    }
}
