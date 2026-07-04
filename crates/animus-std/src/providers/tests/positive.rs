use super::*;

#[test]
fn selects_supported_model_with_effective_metadata() {
    let selected = registry()
        .select_model(&selection(ThinkingLevel::Low, ThinkingPolicy::Reject))
        .expect("model should select");

    assert_eq!(selected.provider_id().as_str(), PROVIDER_ID);
    assert_eq!(selected.model_id().as_str(), MODEL_ID);
    assert_eq!(selected.api_kind(), ApiKind::OpenAiResponses);
    assert_eq!(selected.endpoint().as_str(), ENDPOINT_IDENTITY);
    assert_eq!(selected.effective_thinking(), ThinkingLevel::Low);
    assert_eq!(
        selected.capabilities().context_budget().get(),
        CONTEXT_BUDGET
    );
}

#[test]
fn clamps_unsupported_thinking_level_when_policy_allows() {
    let selected = registry()
        .select_model(&selection(ThinkingLevel::High, ThinkingPolicy::Clamp))
        .expect("model should select with clamped thinking");

    assert_eq!(selected.effective_thinking(), ThinkingLevel::Medium);
}

#[test]
fn resolves_supported_streaming_modes() {
    let resolved = capabilities()
        .resolve_streaming(
            ModelStreamingRequest::content_and_thought(),
            UnsupportedStreamPolicy::Reject,
        )
        .expect("content and thought streaming should resolve");

    assert_eq!(resolved.content(), StreamRequestMode::Required);
    assert_eq!(resolved.thought(), StreamRequestMode::Required);
    assert!(!resolved.downgraded());
}

#[test]
fn downgrades_unsupported_thought_streaming_without_fabricating_thought_mode() {
    let resolved = content_only_capabilities()
        .resolve_streaming(
            ModelStreamingRequest::content_and_thought(),
            UnsupportedStreamPolicy::ContentOnlyDowngrade,
        )
        .expect("unsupported thought streaming should downgrade");

    assert_eq!(resolved.content(), StreamRequestMode::Required);
    assert_eq!(resolved.thought(), StreamRequestMode::Disabled);
    assert!(resolved.downgraded());
}

#[test]
fn credential_handles_do_not_expose_secret_values() {
    let requirement = CredentialRequirement::new(
        CredentialSource::environment(API_KEY_ENV).expect("credential source should validate"),
        true,
    );
    let handle = resolve_credential_from_env_pairs(&requirement, [(API_KEY_ENV, SECRET_VALUE)])
        .expect("credential should resolve")
        .expect("required credential should return handle");

    assert_eq!(handle.source_label(), "env:OPENAI_API_KEY");
    assert!(!format!("{handle:?}").contains(SECRET_VALUE));
}

#[test]
fn dynamic_provider_validation_accepts_valid_entries() {
    let active = registry();
    let updated = active
        .with_dynamic_provider(other_provider_entry())
        .expect("valid dynamic provider should apply");

    assert_eq!(updated.providers().len(), TWO_PROVIDER_COUNT);
    assert_eq!(active.providers().len(), SINGLE_PROVIDER_COUNT);
}

#[test]
fn runtime_crate_has_no_provider_registry_dependency() {
    const RUNTIME_MANIFEST: &str = include_str!("../../../../animus-runtime/Cargo.toml");
    const RUNTIME_LIB: &str = include_str!("../../../../animus-runtime/src/lib.rs");

    assert!(!RUNTIME_MANIFEST.contains("animus-std"));
    assert!(!RUNTIME_LIB.contains("ProviderRegistry"));
    assert!(!RUNTIME_LIB.contains("CredentialSource"));
}
