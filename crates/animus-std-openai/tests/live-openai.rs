//! Ignored opt-in live `OpenAI` smoke and offline planning controls.

#[cfg(test)]
mod tests {
    const API_KEY_ENV: &str = "ANIMUS_OPENAI_LIVE_API_KEY";
    const MODEL_ENV: &str = "ANIMUS_OPENAI_LIVE_MODEL";
    const REVIEWED_MODEL: &str = "reviewed-model";

    #[test]
    #[ignore = "requires explicit ANIMUS_OPENAI_LIVE_API_KEY and ANIMUS_OPENAI_LIVE_MODEL"]
    fn explicit_live_openai_smoke() {
        let api_key = std::env::var(API_KEY_ENV).expect("explicit live API key is required");
        let model = std::env::var(MODEL_ENV).expect("explicit live model is required");
        let evidence = animus_std_openai::run_packaged_live_openai_smoke(model.as_str(), api_key)
            .expect("explicit live OpenAI request must succeed");

        assert!(evidence.contains("LIVE_OPENAI_SMOKE status=passed"));
        assert!(evidence.contains("provider=openai"));
        assert!(evidence.contains("raw_response=omitted"));
        assert!(evidence.contains("model_output=omitted"));
        assert!(evidence.contains("credential=omitted"));
    }

    #[test]
    fn valid_model_has_an_offline_plan() {
        assert!(animus_std_openai::plan_live_openai_smoke(REVIEWED_MODEL).is_ok());
    }

    #[test]
    fn invalid_model_is_rejected_without_live_contact() {
        assert_eq!(
            animus_std_openai::plan_live_openai_smoke("invalid model"),
            Err(animus_std_openai::LiveOpenAiSmokeError::InvalidModel)
        );
        assert!(animus_std_openai::plan_live_openai_smoke("").is_err());
    }
}
