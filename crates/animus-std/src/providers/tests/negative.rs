use super::*;

#[test]
fn rejects_unsupported_thinking_level_when_policy_requires() {
    let error = registry()
        .select_model(&selection(ThinkingLevel::High, ThinkingPolicy::Reject))
        .expect_err("unsupported thinking should fail");

    assert!(matches!(
        error,
        RegistryError::UnsupportedThinkingLevel { requested: "high" }
    ));
}

#[test]
fn rejects_unsupported_thought_streaming_when_policy_requires() {
    let error = content_only_capabilities()
        .resolve_streaming(
            ModelStreamingRequest::content_and_thought(),
            UnsupportedStreamPolicy::Reject,
        )
        .expect_err("unsupported thought streaming should fail");

    assert!(matches!(
        error,
        RegistryError::UnsupportedStreamCapability {
            capability: "thought_streaming",
        }
    ));
}

#[test]
fn rejects_malformed_provider_identifiers() {
    let error = ProviderId::new(BAD_ID_WITH_SPACE).expect_err("bad id should fail");

    assert!(matches!(
        error,
        RegistryError::InvalidIdentifier {
            field: PROVIDER_ID_FIELD,
            reason: INVALID_ID_CHAR_REASON,
        }
    ));
}

#[test]
fn rejects_non_positive_budgets() {
    let error = ModelCapabilities::from_draft(ModelCapabilitiesDraft {
        input_modalities: vec![InputModality::Text],
        context_budget: CONTEXT_BUDGET,
        output_budget: INVALID_ZERO_OUTPUT_BUDGET,
        reasoning: ReasoningSupport::None,
        thinking_levels: vec![ThinkingLevel::None],
        streaming: ModelStreamingCapabilities::none(),
        cost: None,
    })
    .expect_err("zero output budget should fail");

    assert!(matches!(
        error,
        RegistryError::InvalidBudget {
            field: OUTPUT_BUDGET_FIELD,
            reason: INVALID_BUDGET_REASON,
        }
    ));
}

#[test]
fn rejects_missing_endpoint_identity() {
    let error = EndpointIdentity::new("").expect_err("empty endpoint should fail");

    assert!(matches!(
        error,
        RegistryError::InvalidEndpoint {
            field: ENDPOINT_FIELD,
            reason: INVALID_ENDPOINT_EMPTY_REASON,
        }
    ));
}

#[test]
fn reports_missing_credentials_with_redacted_diagnostic() {
    let requirement = CredentialRequirement::new(
        CredentialSource::environment(API_KEY_ENV).expect("credential source should validate"),
        true,
    );
    let error = resolve_credential_from_env_pairs(
        &requirement,
        [(API_KEY_ENV, ""), ("OTHER_TOKEN", SECRET_VALUE)],
    )
    .expect_err("missing credential should fail");
    let diagnostic = error.to_string();

    assert!(matches!(error, RegistryError::MissingCredential { .. }));
    assert!(diagnostic.contains("env:OPENAI_API_KEY"));
    assert!(!diagnostic.contains(SECRET_VALUE));
}

#[test]
fn non_env_credential_failures_are_redacted() {
    let oauth = CredentialRequirement::new(
        CredentialSource::oauth("openai").expect("oauth label should validate"),
        true,
    );
    let file = CredentialRequirement::new(
        CredentialSource::file("openai-key-file").expect("file label should validate"),
        true,
    );
    let oauth_error = resolve_credential_from_env_pairs(&oauth, [("TOKEN", OAUTH_TOKEN_VALUE)])
        .expect_err("oauth shell unavailable should fail");
    let file_error = resolve_credential_from_env_pairs(&file, [("TOKEN", FILE_CONTENT_VALUE)])
        .expect_err("file shell unavailable should fail");

    assert!(!oauth_error.to_string().contains(OAUTH_TOKEN_VALUE));
    assert!(!file_error.to_string().contains(FILE_CONTENT_VALUE));
}

#[test]
fn dynamic_provider_validation_rejects_malformed_entries_without_mutating_active_registry() {
    let active = registry();
    let invalid_provider = ProviderEntry::new(provider_id(), CredentialRequirement::none(), vec![])
        .expect_err("empty model list should fail");

    assert!(matches!(
        invalid_provider,
        RegistryError::EmptyCollection {
            field: MODELS_FIELD,
            reason: EMPTY_COLLECTION_REASON,
        }
    ));
    assert_eq!(active.providers().len(), SINGLE_PROVIDER_COUNT);
    assert_eq!(
        active.providers()[FIRST_PROVIDER_INDEX].id().as_str(),
        PROVIDER_ID
    );
}
