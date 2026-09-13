const OPENAI_MODEL_ID: &str = "gpt-5-mini";
const SUBSTITUTED_OPENROUTER_MODEL_ID: &str = "openai/gpt-5.5";
const RESPONSE_TEXT: &str = "READY";
const API_KEY_SENTINEL: &str = "secret-live-key";
const FIRST_ITEM_INDEX: usize = 0;

#[test]
fn positive_plans_and_evidence_are_bounded() {
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.contract.plan]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.contract.evidence]
    let cases = [
        (
            super::plan_live_openai_smoke(OPENAI_MODEL_ID).expect("OpenAI model must produce plan"),
            super::OPENAI_STATUS,
            super::OPENAI_PROVIDER_ID,
            crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
            OPENAI_MODEL_ID,
            super::render_live_openai_smoke_evidence
                as fn(&animus_std::ProviderDispatchOutcome) -> String,
        ),
        (
            super::plan_live_local_dspark_smoke(super::LOCAL_DSPARK_SMOKE_MODEL_ID)
                .expect("reviewed local-dspark model must produce plan"),
            super::LOCAL_DSPARK_STATUS,
            super::LOCAL_DSPARK_PROVIDER_ID,
            crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
            super::LOCAL_DSPARK_SMOKE_MODEL_ID,
            super::render_live_local_dspark_smoke_evidence
                as fn(&animus_std::ProviderDispatchOutcome) -> String,
        ),
        (
            super::plan_live_openrouter_smoke(super::OPENROUTER_SMOKE_MODEL_ID)
                .expect("reviewed OpenRouter model must produce plan"),
            super::OPENROUTER_STATUS,
            super::OPENROUTER_PROVIDER_ID,
            crate::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
            super::OPENROUTER_SMOKE_MODEL_ID,
            super::render_live_openrouter_smoke_evidence
                as fn(&animus_std::ProviderDispatchOutcome) -> String,
        ),
    ];
    for (plan, status, provider, endpoint, model, render) in cases {
        let response = serde_json::json!({
            "status": "completed",
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{"type": "output_text", "text": RESPONSE_TEXT}]
            }]
        })
        .to_string();
        let outcome =
            crate::normalize_openai_response(&plan, response.as_bytes(), super::MAX_RESPONSE_BYTES)
                .expect("supported response must normalize");
        let evidence = render(&outcome);

        assert_eq!(plan.provider_id(), provider);
        assert_eq!(plan.endpoint(), endpoint);
        assert_eq!(
            outcome.response().items()[FIRST_ITEM_INDEX].assistant_text(),
            Some(RESPONSE_TEXT)
        );
        assert!(evidence.contains(status));
        assert!(evidence.contains("raw_response=omitted"));
        assert!(evidence.contains("credential=omitted"));
        assert!(!evidence.contains(super::PROMPT));
        assert!(!evidence.contains(RESPONSE_TEXT));
        assert!(!evidence.contains(API_KEY_SENTINEL));
        assert!(!evidence.contains(model));
    }
}

#[test]
fn local_disabled_reasoning_is_explicit_in_text_projection() {
    // r[verify agent_harness_runtime.local_dspark_responses.reasoning.pass]
    let plan = super::plan_live_local_dspark_smoke(super::LOCAL_DSPARK_SMOKE_MODEL_ID)
        .expect("reviewed local model must produce plan");
    let config = crate::OpenAiHostConfig::new(
        crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
        crate::LOCAL_DSPARK_RESPONSES_URL,
        std::time::Duration::from_secs(super::TIMEOUT_SECONDS),
        super::MAX_RESPONSE_BYTES,
    )
    .expect("exact local endpoint must be admitted");
    let body =
        crate::build_openai_request_body(&config, &plan).expect("local request must project");
    let value: serde_json::Value =
        serde_json::from_slice(body.as_slice()).expect("projected JSON must parse");
    assert_eq!(value["reasoning"]["effort"], "none");
}

#[test]
fn negative_invalid_model_fails_before_host_execution() {
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.binary.failure]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
    let invalid_model = "invalid model with spaces";
    assert_eq!(
        super::plan_live_openai_smoke(invalid_model),
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    );
    assert_eq!(
        super::plan_live_openrouter_smoke(invalid_model),
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    );
    assert_eq!(
        super::plan_live_local_dspark_smoke(invalid_model),
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    );
    assert_eq!(
        super::plan_live_local_dspark_smoke(SUBSTITUTED_OPENROUTER_MODEL_ID),
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    );
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.failure]
    assert_eq!(
        super::plan_live_openrouter_smoke(SUBSTITUTED_OPENROUTER_MODEL_ID),
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    );
}

struct RefusingWriter;

impl std::fmt::Write for RefusingWriter {
    fn write_str(&mut self, _text: &str) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

#[test]
fn smoke_error_display_preserves_text_and_writer_failure() {
    for (error, expected) in [
        (
            super::LiveOpenAiSmokeError::InvalidModel,
            "live Responses smoke model is invalid",
        ),
        (
            super::LiveOpenAiSmokeError::InvalidPlan,
            "live Responses smoke plan is invalid",
        ),
        (
            super::LiveOpenAiSmokeError::EmptyOutcome,
            "live Responses smoke response has no output",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
        assert_eq!(
            std::fmt::write(&mut RefusingWriter, format_args!("{error}")),
            Err(std::fmt::Error),
        );
    }
}
