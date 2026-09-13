#[test]
fn literal_tool_stream_and_continuation_share_the_production_parser() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.streaming]
    // r[verify agent_harness_runtime.tool_capable_openai_host.continuation]
    let service = super::FakeStreamingTransport::new([
        super::TOOL_STREAM.to_vec(),
        super::FINAL_STREAM.to_vec(),
    ]);
    let capabilities = super::capabilities();
    let schemas = super::schemas();
    let initial_plan = super::plan_tool_request(false, None);
    let initial_dispatch = crate::OpenAiToolDispatch::initial(
        super::MODEL_CALL_ID,
        capabilities.clone(),
        schemas.clone(),
        crate::OpenAiToolPolicy::serial(crate::OpenAiToolChoice::required()),
    )
    .expect("initial dispatch must validate");
    let initial = crate::shell::execute_tool_stream_with_transport(
        &service,
        crate::shell::StreamingExecutionRequest {
            config: &crate::tests::fixture::config(),
            plan: &initial_plan,
            dispatch: initial_dispatch,
            api_key: &crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL),
            cancellation: &super::FixedCancellation(false),
        },
    )
    .expect("literal tool stream must parse");

    assert_initial_outcome(&initial);
    let binding = initial.tool_calls()[0].continuation_binding();
    let result = crate::OpenAiToolResult::for_binding(&binding, super::TOOL_RESULT)
        .expect("correlated tool result must validate");
    let continuation_plan = super::plan_tool_request(true, Some(binding.tool_call_id()));
    let continuation_dispatch = crate::OpenAiToolDispatch::initial(
        super::MODEL_CALL_ID,
        capabilities,
        schemas,
        crate::OpenAiToolPolicy::serial(crate::OpenAiToolChoice::auto()),
    )
    .expect("continuation cohort must validate")
    .continue_with(binding, result)
    .expect("continuation dispatch must validate");
    let final_outcome = crate::shell::execute_tool_stream_with_transport(
        &service,
        crate::shell::StreamingExecutionRequest {
            config: &crate::tests::fixture::config(),
            plan: &continuation_plan,
            dispatch: continuation_dispatch,
            api_key: &crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL),
            cancellation: &super::FixedCancellation(false),
        },
    )
    .expect("literal continuation stream must parse");

    assert_eq!(service.calls.get(), super::EXPECTED_LOOP_CALLS);
    assert_eq!(final_outcome.provider_response_id(), "resp_final_fixture");
    assert!(final_outcome.tool_calls().is_empty());
    assert_request_bodies(&service);
}

fn assert_request_bodies(service: &super::FakeStreamingTransport) {
    let request_bodies = service.request_bodies.borrow();
    let initial_body: serde_json::Value =
        serde_json::from_slice(&request_bodies[0]).expect("initial request JSON must parse");
    let continuation_body: serde_json::Value =
        serde_json::from_slice(&request_bodies[1]).expect("continuation request JSON must parse");
    assert_eq!(
        initial_body["tools"][0]["name"],
        crate::STEEL_REPL_TOOL_NAME
    );
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
    assert_eq!(continuation_body["input"][0]["output"], super::TOOL_RESULT);
}

#[test]
fn unsupported_tools_and_lossy_schemas_fail_before_contact() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.capabilities]
    // r[verify agent_harness_runtime.tool_capable_openai_host.projection]
    assert!(matches!(
        crate::OpenAiToolCapability::new("other_tool", crate::STEEL_REPL_TOOL_SCHEMA),
        Err(crate::HostError::UnsupportedPlan { .. })
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
        crate::OpenAiToolSchemaBinding::from_json(
            crate::STEEL_REPL_TOOL_SCHEMA,
            nested_open_schema
        ),
        Err(crate::HostError::InvalidToolSchema { .. })
    ));

    let service = super::FakeStreamingTransport::new([super::TOOL_STREAM.to_vec()]);
    let plan = super::plan_tool_request(false, None);
    let wrong_model = crate::OpenAiToolCapabilityMatrix::new(
        "other-model",
        super::roles(),
        vec![
            crate::OpenAiToolCapability::new(
                crate::STEEL_REPL_TOOL_NAME,
                crate::STEEL_REPL_TOOL_SCHEMA,
            )
            .expect("reviewed tool must validate"),
        ],
        super::stream_channels(),
    )
    .expect("bounded capability matrix must validate");
    let dispatch = crate::OpenAiToolDispatch::initial(
        super::MODEL_CALL_ID,
        wrong_model,
        super::schemas(),
        crate::OpenAiToolPolicy::serial(crate::OpenAiToolChoice::auto()),
    )
    .expect("dispatch shape must validate");
    assert!(matches!(
        crate::shell::execute_tool_stream_with_transport(
            &service,
            crate::shell::StreamingExecutionRequest {
                config: &crate::tests::fixture::config(),
                plan: &plan,
                dispatch,
                api_key: &crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL),
                cancellation: &super::FixedCancellation(false),
            },
        ),
        Err(crate::HostError::UnsupportedPlan { .. })
    ));
    assert_eq!(service.calls.get(), super::ZERO_CALLS);
}

fn assert_initial_outcome(initial: &crate::OpenAiStreamOutcome) {
    assert_eq!(initial.terminal(), crate::OpenAiStreamTerminal::Completed);
    assert_eq!(initial.provider_response_id(), "resp_tool_fixture");
    assert_eq!(initial.tool_calls().len(), super::ONE_CALL);
    assert_eq!(
        initial.tool_calls()[0].tool_name(),
        crate::STEEL_REPL_TOOL_NAME
    );
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
}
