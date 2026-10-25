//! The live route's construction, checked without contacting a provider.
#![cfg(feature = "live")]

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.live]
fn live_routes_admit_explicit_credentials_and_models_only() {
    let endpoint = animus_campaign_manager_eval::Endpoint::OpenRouter;
    let route = animus_campaign_manager_eval::LiveRoute::new(
        endpoint,
        "deepseek/deepseek-v4-flash-0731",
        String::from("sk-test-not-a-real-key"),
    );
    let identity = route.as_ref().map(|live| String::from(live.identity()));
    assert_eq!(
        identity.as_deref(),
        Ok("model:live:openrouter:deepseek:deepseek-v4-flash-0731")
    );
    let refusals = [
        (
            "gpt-5-mini",
            "",
            animus_campaign_manager_eval::LiveFailure::Credential,
        ),
        (
            "gpt-5-mini",
            "two words",
            animus_campaign_manager_eval::LiveFailure::Credential,
        ),
        (
            "",
            "sk-test",
            animus_campaign_manager_eval::LiveFailure::Model,
        ),
    ];
    for (model, credential, expected) in refusals {
        let refused = animus_campaign_manager_eval::LiveRoute::new(
            animus_campaign_manager_eval::Endpoint::OpenAi,
            model,
            String::from(credential),
        );
        assert_eq!(refused.err(), Some(expected), "{model:?} {credential:?}");
    }
}
