use super::*;

mod streaming;

const PROVIDER_ID: &str = "openai";
const MODEL_ID: &str = "gpt-5-mini";
const ENDPOINT_IDENTITY: &str = "openai-responses";
const OTHER_PROVIDER_ID: &str = "local";
const OTHER_MODEL_ID: &str = "local-small";
const BAD_ID_WITH_SPACE: &str = "bad id";
const API_KEY_ENV: &str = "OPENAI_API_KEY";
const SECRET_VALUE: &str = "sk-secret-value";
const OAUTH_TOKEN_VALUE: &str = "oauth-secret-token";
const FILE_CONTENT_VALUE: &str = "credential-file-secret";
const CONTEXT_BUDGET: u32 = 128_000;
const OUTPUT_BUDGET: u32 = 8_192;
const INPUT_COST_MICRO_USD: u32 = 250;
const OUTPUT_COST_MICRO_USD: u32 = 1_000;
const INVALID_ZERO_OUTPUT_BUDGET: u32 = 0;
const FIRST_PROVIDER_INDEX: usize = 0;
const SINGLE_PROVIDER_COUNT: usize = 1;
const TWO_PROVIDER_COUNT: usize = 2;

fn provider_id() -> ProviderId {
    ProviderId::new(PROVIDER_ID).expect("provider id should validate")
}

fn model_id() -> ModelId {
    ModelId::new(MODEL_ID).expect("model id should validate")
}

fn endpoint() -> EndpointIdentity {
    EndpointIdentity::new(ENDPOINT_IDENTITY).expect("endpoint should validate")
}

fn capabilities() -> ModelCapabilities {
    ModelCapabilities::from_draft(ModelCapabilitiesDraft {
        input_modalities: vec![InputModality::Text],
        context_budget: CONTEXT_BUDGET,
        output_budget: OUTPUT_BUDGET,
        reasoning: ReasoningSupport::Optional,
        thinking_levels: vec![
            ThinkingLevel::None,
            ThinkingLevel::Low,
            ThinkingLevel::Medium,
        ],
        streaming: ModelStreamingCapabilities::new(
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
        ),
        cost: Some(TokenCost::new(INPUT_COST_MICRO_USD, OUTPUT_COST_MICRO_USD)),
    })
    .expect("capabilities should validate")
}

fn model_entry() -> ModelEntry {
    ModelEntry::new(
        model_id(),
        ApiKind::OpenAiResponses,
        endpoint(),
        capabilities(),
    )
}

fn provider_entry() -> ProviderEntry {
    let credential = CredentialRequirement::new(
        CredentialSource::environment(API_KEY_ENV).expect("credential source should validate"),
        true,
    );
    ProviderEntry::new(provider_id(), credential, vec![model_entry()])
        .expect("provider entry should validate")
}

fn registry() -> ProviderRegistry {
    ProviderRegistry::new(vec![provider_entry()]).expect("registry should validate")
}

fn selection(thinking: ThinkingLevel, policy: ThinkingPolicy) -> ModelSelection {
    ModelSelection::new(provider_id(), model_id(), thinking, policy)
}

fn other_provider_entry() -> ProviderEntry {
    let provider = ProviderId::new(OTHER_PROVIDER_ID).expect("provider id should validate");
    let model = ModelId::new(OTHER_MODEL_ID).expect("model id should validate");
    let entry = ModelEntry::new(model, ApiKind::LocalHttp, endpoint(), capabilities());
    ProviderEntry::new(provider, CredentialRequirement::none(), vec![entry])
        .expect("provider entry should validate")
}

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
