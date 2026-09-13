const EXPECTED_TEMPERATURE: f64 = 0.25;

#[test]
fn unsupported_response_shapes_retain_rejection_meaning() {
    let plan = crate::tests::fixture::plan(crate::tests::RequestCase::Base);
    let accepted = crate::tests::fixture::successful_response();
    assert!(
        crate::normalize_openai_response(
            &plan,
            accepted.as_bytes(),
            crate::tests::MAX_RESPONSE_BYTES,
        )
        .is_ok()
    );
    for body in [
        r#"{"status":"in_progress","output":[]}"#,
        r#"{"status":"completed","output":[]}"#,
        r#"{"status":"completed","output":[{"type":"unknown"}]}"#,
        r#"{"status":"completed","output":[{"type":"message","role":"user","content":[]}]}"#,
        r#"{"status":"completed","output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":"visible"},{"type":"refusal","refusal":"denied"}]}]}"#,
        r#"{"status":"completed","output":[{"type":"message","role":"assistant","content":[{"type":"refusal","refusal":"denied"},{"type":"refusal","refusal":"again"}]}]}"#,
    ] {
        assert!(matches!(
            crate::normalize_openai_response(
                &plan,
                body.as_bytes(),
                crate::tests::MAX_RESPONSE_BYTES,
            ),
            Err(crate::HostError::MalformedResponse { .. })
        ));
    }
}

#[test]
fn pure_projection_preserves_supported_openai_fields() {
    // r[verify agent_harness_runtime.live_openai_provider_host.request_projection.text]
    let body = crate::build_openai_request_body(
        &crate::tests::fixture::config(),
        &crate::tests::fixture::plan(crate::tests::RequestCase::Base),
    )
    .expect("supported request must project");
    let value: serde_json::Value =
        serde_json::from_slice(body.as_slice()).expect("request JSON must parse");

    assert_eq!(value["model"], crate::tests::MODEL_ID);
    assert_eq!(
        value["input"][crate::tests::FIRST_ITEM_INDEX]["role"],
        "user"
    );
    assert_eq!(
        value["input"][crate::tests::FIRST_ITEM_INDEX]["content"][crate::tests::FIRST_ITEM_INDEX]["type"],
        "input_text"
    );
    assert_eq!(value["reasoning"]["effort"], "low");
    assert_eq!(value["stream"], false);
    assert_eq!(value["max_output_tokens"], crate::tests::OUTPUT_LIMIT);
    assert_eq!(value["temperature"], EXPECTED_TEMPERATURE);
    assert_eq!(
        value["metadata"][crate::tests::METADATA_KEY],
        crate::tests::METADATA_VALUE
    );
}

#[test]
fn unsupported_plan_fields_fail_before_transport() {
    // r[verify agent_harness_runtime.live_openai_provider_host.request_projection.unsupported]
    assert!(
        crate::build_openai_request_body(
            &crate::tests::fixture::config(),
            &crate::tests::fixture::plan(crate::tests::RequestCase::Base),
        )
        .is_ok()
    );
    let cases = [
        (crate::tests::RequestCase::Stream, "streaming"),
        (crate::tests::RequestCase::Tool, "tools"),
        (crate::tests::RequestCase::Image, "image"),
        (crate::tests::RequestCase::Structured, "structured_output"),
        (crate::tests::RequestCase::Stop, "stop_sequences"),
    ];
    for (request_case, expected_feature) in cases {
        assert_eq!(
            crate::build_openai_request_body(
                &crate::tests::fixture::config(),
                &crate::tests::fixture::plan(request_case)
            ),
            Err(crate::HostError::UnsupportedPlan {
                feature: expected_feature,
            })
        );
    }
}

#[test]
fn bounded_response_normalizes_to_canonical_outcome() {
    // r[verify agent_harness_runtime.live_openai_provider_host.response_normalization.text]
    let dispatch_plan = crate::tests::fixture::plan(crate::tests::RequestCase::Base);
    let outcome = crate::normalize_openai_response(
        &dispatch_plan,
        crate::tests::fixture::successful_response().as_bytes(),
        crate::tests::MAX_RESPONSE_BYTES,
    )
    .expect("supported response must normalize");

    assert_eq!(
        outcome.response().items()[crate::tests::FIRST_ITEM_INDEX].assistant_text(),
        Some(crate::tests::RESPONSE_TEXT)
    );
    assert_eq!(
        outcome
            .response()
            .usage()
            .expect("usage must be present")
            .input_tokens(),
        crate::tests::RESPONSE_INPUT_TOKENS
    );
    assert_eq!(
        outcome.receipt().status(),
        animus_std::ProviderDispatchOutcomeStatus::Success
    );
    assert!(!format!("{:?}", outcome.receipt()).contains(crate::tests::API_KEY_VALUE));

    let refusal = crate::normalize_openai_response(
        &dispatch_plan,
        crate::tests::fixture::refusal_response().as_bytes(),
        crate::tests::MAX_RESPONSE_BYTES,
    )
    .expect("supported refusal must normalize");
    assert!(matches!(
        &refusal.response().items()[crate::tests::FIRST_ITEM_INDEX],
        animus_adapters::CanonicalModelResponseItem::Refusal { .. }
    ));
}
