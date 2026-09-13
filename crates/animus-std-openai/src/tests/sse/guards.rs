struct InvalidReadCount;

impl std::io::Read for InvalidReadCount {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        buffer
            .len()
            .checked_add(1)
            .ok_or_else(|| std::io::Error::other("test read count overflow"))
    }
}

impl crate::shell::StreamingTransport for InvalidReadCount {
    fn open_stream(
        &self,
        _request: &crate::shell::WireRequest<'_>,
        _api_key: &crate::OpenAiApiKey,
    ) -> Result<crate::shell::StreamingWireResponse, crate::TransportErrorClass> {
        Ok(crate::shell::StreamingWireResponse {
            status: crate::tests::HTTP_OK,
            body: Box::new(Self),
        })
    }
}

#[test]
fn invalid_reader_counts_return_transport_errors() {
    let plan = super::plan_tool_request(false, None);
    let outcome = crate::shell::execute_tool_stream_with_transport(
        &InvalidReadCount,
        crate::shell::StreamingExecutionRequest {
            config: &crate::tests::fixture::config(),
            plan: &plan,
            dispatch: super::dispatch(),
            api_key: &crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL),
            cancellation: &super::FixedCancellation(false),
        },
    );
    assert!(matches!(
        outcome,
        Err(crate::HostError::Transport {
            class: crate::TransportErrorClass::ResponseRead,
        })
    ));
}

#[test]
#[ignore = "requires explicit ANIMUS_OPENAI_TOOL_LIVE_API_KEY and ANIMUS_OPENAI_TOOL_LIVE_MODEL"]
fn explicit_live_openai_tool_stream_smoke() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.authority]
    let model = std::env::var("ANIMUS_OPENAI_TOOL_LIVE_MODEL")
        .expect("explicit live tool model is required");
    assert_eq!(
        model,
        crate::tests::MODEL_ID,
        "live tool smoke model must match reviewed fixture"
    );
    let key_value = std::env::var("ANIMUS_OPENAI_TOOL_LIVE_API_KEY")
        .expect("explicit live tool API key is required");
    let host = crate::ReqwestOpenAiHost::new(crate::tests::fixture::config())
        .expect("live host must configure");
    let plan = super::plan_tool_request(false, None);
    let outcome = host
        .execute_tool_stream(crate::OpenAiToolStreamRequest {
            plan: &plan,
            dispatch: super::dispatch(),
            api_key: &crate::OpenAiApiKey::new(crate::tests::CREDENTIAL_LABEL, key_value)
                .expect("live API key must validate"),
            cancellation: &super::FixedCancellation(false),
        })
        .expect("explicit live tool stream must succeed");
    let evidence = crate::render_openai_tool_stream_evidence(&outcome);
    assert!(evidence.contains("LIVE_OPENAI_TOOL_STREAM status=completed"));
    assert!(evidence.contains("raw_response=omitted"));
    assert!(evidence.contains("tool_arguments=omitted"));
    assert!(evidence.contains("tool_output=omitted"));
    assert!(evidence.contains("credential=omitted"));
    assert!(evidence.contains("reasoning=omitted"));
    println!("{evidence}");
}

#[test]
fn malformed_reasoning_correlation_bounds_and_cancellation_fail_safely() {
    // r[verify agent_harness_runtime.tool_capable_openai_host.streaming]
    // r[verify agent_harness_runtime.tool_capable_openai_host.evidence]
    let unsafe_error = crate::parse_openai_responses_sse_chunks(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(crate::tests::MAX_RESPONSE_BYTES),
        &[super::UNSAFE_REASONING_STREAM],
    )
    .expect_err("hidden reasoning event must fail closed");
    assert_eq!(
        unsafe_error.class(),
        crate::OpenAiStreamFailureClass::UnsafeReasoning
    );
    assert!(!format!("{unsafe_error:?}").contains("private chain"));

    let stale_error = crate::parse_openai_responses_sse_chunks(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(crate::tests::MAX_RESPONSE_BYTES),
        &[super::STALE_TOOL_STREAM],
    )
    .expect_err("stale tool item identity must fail closed");
    assert_eq!(
        stale_error.class(),
        crate::OpenAiStreamFailureClass::Correlation
    );

    let bounded_error = crate::parse_openai_responses_sse_chunks(
        super::dispatch(),
        crate::OpenAiStreamBounds {
            stream_bytes: super::SMALL_STREAM_BOUND,
            event_bytes: super::SMALL_EVENT_BOUND,
            events: super::EVENT_BOUND,
            tool_argument_bytes: super::ARGUMENT_BOUND,
            tool_calls: super::TOOL_CALL_BOUND,
        },
        &[super::TOOL_STREAM],
    )
    .expect_err("aggregate stream overflow must fail closed");
    assert_eq!(
        bounded_error.class(),
        crate::OpenAiStreamFailureClass::BoundExceeded
    );
}

#[test]
fn cancellation_retains_an_empty_response_identity_and_blocks_contact() {
    let mut cancelled = crate::OpenAiResponsesSseParser::new(
        super::dispatch(),
        crate::OpenAiStreamBounds::reviewed(crate::tests::MAX_RESPONSE_BYTES),
    )
    .expect("parser bounds must validate");
    cancelled
        .cancel()
        .expect("pre-response cancellation must emit");
    let outcome = cancelled
        .finish()
        .expect("cancelled stream must finish safely");
    assert_eq!(outcome.terminal(), crate::OpenAiStreamTerminal::Cancelled);
    assert!(outcome.provider_response_id().is_empty());
    assert!(!format!("{outcome:?}").contains("resp_"));

    let service = super::FakeStreamingTransport::new([super::TOOL_STREAM.to_vec()]);
    let plan = super::plan_tool_request(false, None);
    assert!(matches!(
        crate::shell::execute_tool_stream_with_transport(
            &service,
            crate::shell::StreamingExecutionRequest {
                config: &crate::tests::fixture::config(),
                plan: &plan,
                dispatch: super::dispatch(),
                api_key: &crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL),
                cancellation: &super::FixedCancellation(true),
            },
        ),
        Err(crate::HostError::Cancelled)
    ));
    assert_eq!(service.calls.get(), super::ZERO_CALLS);
}
