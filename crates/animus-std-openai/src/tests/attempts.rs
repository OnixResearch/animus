struct FakeTransport {
    calls: std::cell::Cell<usize>,
    response:
        std::cell::RefCell<Option<Result<crate::shell::WireResponse, crate::TransportErrorClass>>>,
    request_body: std::cell::RefCell<Vec<u8>>,
}

impl FakeTransport {
    fn response(status: u16, body: Vec<u8>) -> Self {
        Self {
            calls: std::cell::Cell::new(crate::tests::EXPECTED_CALLS_ZERO),
            response: std::cell::RefCell::new(Some(Ok(crate::shell::WireResponse {
                status,
                body,
            }))),
            request_body: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn error(class: crate::TransportErrorClass) -> Self {
        Self {
            calls: std::cell::Cell::new(crate::tests::EXPECTED_CALLS_ZERO),
            response: std::cell::RefCell::new(Some(Err(class))),
            request_body: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl crate::shell::Transport for FakeTransport {
    fn send(
        &self,
        request: &crate::shell::WireRequest<'_>,
        api_key: &crate::OpenAiApiKey,
    ) -> Result<crate::shell::WireResponse, crate::TransportErrorClass> {
        assert_eq!(request.endpoint_url, crate::OFFICIAL_OPENAI_RESPONSES_URL);
        assert_eq!(request.max_response_bytes, crate::tests::MAX_RESPONSE_BYTES);
        assert_eq!(api_key.expose_value(), crate::tests::API_KEY_VALUE);
        self.calls.set(
            self.calls
                .get()
                .checked_add(1)
                .expect("fake call count fits"),
        );
        self.request_body.replace(request.body.clone());
        self.response
            .borrow_mut()
            .take()
            .expect("fake response must be configured")
    }
}

#[test]
fn shell_performs_one_explicit_attempt() {
    // r[verify agent_harness_runtime.live_openai_provider_host.transport.single_attempt]
    let service = FakeTransport::response(
        crate::tests::HTTP_OK,
        crate::tests::fixture::successful_response().into_bytes(),
    );
    let dispatch_plan = crate::tests::fixture::plan(crate::tests::RequestCase::Base);
    let key = crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL);
    let outcome = execute_fake(
        &service,
        &dispatch_plan,
        &key,
        crate::CancellationState::Active,
    )
    .expect("admitted fake transport must succeed");

    assert_eq!(service.calls.get(), crate::tests::EXPECTED_CALLS_ONE);
    assert!(!service.request_body.borrow().is_empty());
    assert_eq!(
        outcome.response().items()[crate::tests::FIRST_ITEM_INDEX].assistant_text(),
        Some(crate::tests::RESPONSE_TEXT)
    );
}

#[test]
fn cancellation_and_credential_mismatch_block_contact() {
    // r[verify agent_harness_runtime.live_openai_provider_host.transport.cancelled]
    let dispatch_plan = crate::tests::fixture::plan(crate::tests::RequestCase::Base);
    let key = crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL);
    let cancelled = FakeTransport::response(
        crate::tests::HTTP_OK,
        crate::tests::fixture::successful_response().into_bytes(),
    );
    assert_eq!(
        execute_fake(
            &cancelled,
            &dispatch_plan,
            &key,
            crate::CancellationState::Cancelled,
        ),
        Err(crate::HostError::Cancelled)
    );
    assert_eq!(cancelled.calls.get(), crate::tests::EXPECTED_CALLS_ZERO);

    let mismatch_key = crate::tests::fixture::api_key("env:OTHER_KEY");
    let mismatch = FakeTransport::response(
        crate::tests::HTTP_OK,
        crate::tests::fixture::successful_response().into_bytes(),
    );
    assert_eq!(
        execute_fake(
            &mismatch,
            &dispatch_plan,
            &mismatch_key,
            crate::CancellationState::Active,
        ),
        Err(crate::HostError::CredentialBindingMismatch)
    );
    assert_eq!(mismatch.calls.get(), crate::tests::EXPECTED_CALLS_ZERO);
}

#[test]
fn transport_status_bounds_and_malformed_responses_fail_safely() {
    // r[verify agent_harness_runtime.live_openai_provider_host.response_normalization.failure]
    let dispatch_plan = crate::tests::fixture::plan(crate::tests::RequestCase::Base);
    let key = crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL);

    let unauthorized = FakeTransport::response(
        crate::tests::HTTP_UNAUTHORIZED,
        b"private error body".to_vec(),
    );
    assert_eq!(
        execute_fake(
            &unauthorized,
            &dispatch_plan,
            &key,
            crate::CancellationState::Active,
        ),
        Err(crate::HostError::HttpStatus {
            status: crate::tests::HTTP_UNAUTHORIZED,
        })
    );

    let refused_body = FakeTransport::response(
        crate::tests::HTTP_OK,
        vec![b'x'; crate::tests::OVERSIZED_RESPONSE_BYTES],
    );
    assert_eq!(
        execute_fake(
            &refused_body,
            &dispatch_plan,
            &key,
            crate::CancellationState::Active,
        ),
        Err(crate::HostError::ResponseTooLarge {
            limit: crate::tests::MAX_RESPONSE_BYTES,
        })
    );

    let malformed = FakeTransport::response(crate::tests::HTTP_OK, b"not-json".to_vec());
    assert!(matches!(
        execute_fake(
            &malformed,
            &dispatch_plan,
            &key,
            crate::CancellationState::Active,
        ),
        Err(crate::HostError::MalformedResponse { .. })
    ));

    let failed_service = FakeTransport::error(crate::TransportErrorClass::Timeout);
    assert_eq!(
        execute_fake(
            &failed_service,
            &dispatch_plan,
            &key,
            crate::CancellationState::Active,
        ),
        Err(crate::HostError::Transport {
            class: crate::TransportErrorClass::Timeout,
        })
    );
}

fn execute_fake(
    service: &FakeTransport,
    plan: &animus_std::ProviderDispatchPlan,
    api_key: &crate::OpenAiApiKey,
    cancellation: crate::CancellationState,
) -> Result<animus_std::ProviderDispatchOutcome, crate::HostError> {
    let host_config = crate::tests::fixture::config();
    crate::shell::execute_with_transport(
        service,
        crate::shell::ExecutionRequest {
            config: &host_config,
            plan,
            api_key,
            cancellation,
        },
    )
}
