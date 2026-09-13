#[test]
fn endpoint_pairs_credentials_and_debug_output_fail_closed() {
    // r[verify agent_harness_runtime.live_openai_provider_host.boundary]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.pass]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.failure]
    let openrouter = crate::OpenAiHostConfig::new(
        crate::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
        crate::OPENROUTER_RESPONSES_URL,
        std::time::Duration::from_secs(crate::tests::TIMEOUT_SECONDS),
        crate::tests::MAX_RESPONSE_BYTES,
    )
    .expect("exact OpenRouter endpoint pair must be admitted");
    // r[verify agent_harness_runtime.local_dspark_responses.endpoint.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.transport.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.reasoning.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.reasoning.isolation]
    let local = crate::OpenAiHostConfig::new(
        crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
        crate::LOCAL_DSPARK_RESPONSES_URL,
        std::time::Duration::from_secs(crate::tests::TIMEOUT_SECONDS),
        crate::tests::MAX_RESPONSE_BYTES,
    )
    .expect("exact local-dspark endpoint pair must be admitted");
    assert!(!local.https_only());
    assert_eq!(local.projected_reasoning_effort("none"), Some("none"));
    assert!(openrouter.https_only());
    assert_eq!(openrouter.projected_reasoning_effort("none"), None);
}

#[test]
fn invalid_endpoint_pairs_and_credentials_are_rejected() {
    // r[verify agent_harness_runtime.local_dspark_responses.endpoint.failure]
    // r[verify agent_harness_runtime.local_dspark_responses.transport.failure]
    for (endpoint_identity, endpoint_url) in [
        ("other-endpoint", crate::OFFICIAL_OPENAI_RESPONSES_URL),
        (
            crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
            crate::OPENROUTER_RESPONSES_URL,
        ),
        (
            crate::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
            crate::OFFICIAL_OPENAI_RESPONSES_URL,
        ),
        (
            crate::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
            "https://example.invalid/v1/responses",
        ),
        (
            crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
            "http://192.168.1.244:8889/v1/responses",
        ),
        (
            crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
            crate::LOCAL_DSPARK_RESPONSES_URL,
        ),
    ] {
        assert!(matches!(
            crate::OpenAiHostConfig::new(
                endpoint_identity,
                endpoint_url,
                std::time::Duration::from_secs(crate::tests::TIMEOUT_SECONDS),
                crate::tests::MAX_RESPONSE_BYTES,
            ),
            Err(crate::HostError::InvalidConfiguration { .. })
        ));
    }
    assert!(matches!(
        crate::OpenAiApiKey::new(crate::tests::CREDENTIAL_LABEL, String::new()),
        Err(crate::HostError::InvalidCredential { .. })
    ));
    assert!(matches!(
        crate::OpenAiApiKey::new(crate::tests::CREDENTIAL_LABEL, String::from("unsafe key")),
        Err(crate::HostError::InvalidCredential { .. })
    ));
    let key = crate::tests::fixture::api_key(crate::tests::CREDENTIAL_LABEL);
    assert!(!format!("{key:?}").contains(crate::tests::API_KEY_VALUE));
}
