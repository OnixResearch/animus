//! Bounded projection of caller-fetched Models.dev provider snapshots.
//!
//! This module parses an explicit `api.json` snapshot and projects only the
//! caller-approved provider and model bindings. It performs no network access,
//! credential lookup, endpoint discovery, availability checks, or refresh work.

use std::collections::BTreeMap;

use serde::Deserialize;

/// Canonical Models.dev provider catalog endpoint.
pub const MODELS_DEV_API_URL: &str = "https://models.dev/api.json";
/// Safe source label for Models.dev provider catalog snapshots.
pub const MODELS_DEV_SOURCE_LABEL: &str = "models.dev/api.json";
/// Maximum accepted Models.dev snapshot size.
pub const MAX_MODELS_DEV_SNAPSHOT_BYTES: usize = 8_388_608;

const MICRO_USD_PER_USD: u64 = 1_000_000;
const MAX_USD_FRACTION_DIGITS: usize = 6;
const DECIMAL_RADIX: u64 = 10;
const UNSUPPORTED_MODALITY_REASON: &str = "model has no supported input modality";
const INVALID_CONTEXT_REASON: &str = "model context limit is unsupported";
const INVALID_OUTPUT_REASON: &str = "model output limit is unsupported";
const EMPTY_MODEL_BINDING_REASON: &str = "provider binding must select at least one model";
const MODEL_BINDING_LIMIT_REASON: &str = "provider binding exceeds the model selection limit";

/// Stable JSON error class for Models.dev snapshot ingestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelsDevJsonErrorKind {
    /// The input is not valid JSON syntax.
    Syntax,
    /// The input JSON does not match the required Models.dev projection shape.
    Data,
    /// The input ended before one complete JSON value was available.
    EndOfInput,
}

/// Typed Models.dev snapshot projection failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelsDevIngestError {
    /// The caller supplied more bytes than the bounded parser accepts.
    SnapshotTooLarge {
        /// Actual byte count.
        actual_bytes: usize,
        /// Maximum accepted byte count.
        max_bytes: usize,
    },
    /// The snapshot could not be decoded.
    Json(ModelsDevJsonErrorKind),
    /// A provider binding selected no models.
    EmptyModelBinding {
        /// Safe provider identifier.
        provider: String,
        /// Stable reason.
        reason: &'static str,
    },
    /// A provider binding selected too many models.
    ModelBindingLimit {
        /// Safe provider identifier.
        provider: String,
        /// Stable reason.
        reason: &'static str,
    },
    /// A selected provider was absent from the snapshot.
    ProviderMissing {
        /// Safe provider identifier.
        provider: String,
    },
    /// A provider object did not repeat its map key as its identifier.
    ProviderIdentityMismatch {
        /// Safe expected provider identifier.
        provider: String,
    },
    /// A selected model was absent from the selected provider.
    ModelMissing {
        /// Safe provider identifier.
        provider: String,
        /// Safe model identifier.
        model: String,
    },
    /// A model object did not repeat its map key as its identifier.
    ModelIdentityMismatch {
        /// Safe provider identifier.
        provider: String,
        /// Safe expected model identifier.
        model: String,
    },
    /// A selected model cannot fit the current Animus registry contract.
    UnsupportedModel {
        /// Safe provider identifier.
        provider: String,
        /// Safe model identifier.
        model: String,
        /// Stable rejection reason.
        reason: &'static str,
    },
    /// The projected registry failed normal Animus registry validation.
    Registry(super::RegistryError),
}

impl std::fmt::Display for ModelsDevIngestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnapshotTooLarge {
                actual_bytes,
                max_bytes,
            } => write!(
                formatter,
                "Models.dev snapshot has {actual_bytes} bytes; maximum is {max_bytes} bytes"
            ),
            Self::Json(kind) => write!(formatter, "Models.dev snapshot JSON is invalid: {kind:?}"),
            Self::EmptyModelBinding { provider, reason }
            | Self::ModelBindingLimit { provider, reason } => {
                write!(
                    formatter,
                    "Models.dev provider binding {provider} is invalid: {reason}"
                )
            }
            Self::ProviderMissing { provider } => {
                write!(
                    formatter,
                    "Models.dev snapshot does not contain provider {provider}"
                )
            }
            Self::ProviderIdentityMismatch { provider } => write!(
                formatter,
                "Models.dev provider {provider} has inconsistent identity metadata"
            ),
            Self::ModelMissing { provider, model } => write!(
                formatter,
                "Models.dev provider {provider} does not contain model {model}"
            ),
            Self::ModelIdentityMismatch { provider, model } => write!(
                formatter,
                "Models.dev model {model} for provider {provider} has inconsistent identity metadata"
            ),
            Self::UnsupportedModel {
                provider,
                model,
                reason,
            } => write!(
                formatter,
                "Models.dev model {model} for provider {provider} is unsupported: {reason}"
            ),
            Self::Registry(error) => write!(
                formatter,
                "projected Models.dev registry is invalid: {error}"
            ),
        }
    }
}

impl std::error::Error for ModelsDevIngestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            Self::SnapshotTooLarge { .. }
            | Self::Json(_)
            | Self::EmptyModelBinding { .. }
            | Self::ModelBindingLimit { .. }
            | Self::ProviderMissing { .. }
            | Self::ProviderIdentityMismatch { .. }
            | Self::ModelMissing { .. }
            | Self::ModelIdentityMismatch { .. }
            | Self::UnsupportedModel { .. } => None,
        }
    }
}

impl From<super::RegistryError> for ModelsDevIngestError {
    fn from(error: super::RegistryError) -> Self {
        Self::Registry(error)
    }
}

/// Unvalidated caller-owned binding for one Models.dev provider cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelsDevProviderBindingDraft {
    /// Selected Models.dev provider identifier.
    pub provider_id: super::ProviderId,
    /// Explicit Animus API family.
    pub api_kind: super::ApiKind,
    /// Explicit safe endpoint identity.
    pub endpoint: super::EndpointIdentity,
    /// Explicit credential requirement.
    pub credential: super::CredentialRequirement,
    /// Exact selected model cohort in caller order.
    pub model_ids: Vec<super::ModelId>,
}

/// Caller-owned binding for one Models.dev provider cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelsDevProviderBinding {
    provider_id: super::ProviderId,
    api_kind: super::ApiKind,
    endpoint: super::EndpointIdentity,
    credential: super::CredentialRequirement,
    model_ids: Vec<super::ModelId>,
}

impl ModelsDevProviderBinding {
    /// Builds an explicit provider and model binding.
    ///
    /// The caller supplies dispatch semantics because Models.dev metadata does
    /// not prove which Animus API adapter or credential flow is safe to use.
    ///
    /// # Errors
    ///
    /// Returns [`ModelsDevIngestError`] when the model selection is empty or
    /// exceeds the normal registry model bound.
    pub fn from_draft(draft: ModelsDevProviderBindingDraft) -> Result<Self, ModelsDevIngestError> {
        if draft.model_ids.is_empty() {
            return Err(ModelsDevIngestError::EmptyModelBinding {
                provider: String::from(draft.provider_id.as_str()),
                reason: EMPTY_MODEL_BINDING_REASON,
            });
        }
        if draft.model_ids.len() > super::catalog::MAX_MODEL_COUNT {
            return Err(ModelsDevIngestError::ModelBindingLimit {
                provider: String::from(draft.provider_id.as_str()),
                reason: MODEL_BINDING_LIMIT_REASON,
            });
        }
        Ok(Self {
            provider_id: draft.provider_id,
            api_kind: draft.api_kind,
            endpoint: draft.endpoint,
            credential: draft.credential,
            model_ids: draft.model_ids,
        })
    }

    /// Returns the selected Models.dev provider identifier.
    #[must_use]
    pub const fn provider_id(&self) -> &super::ProviderId {
        &self.provider_id
    }

    /// Returns the explicit Animus API family.
    #[must_use]
    pub const fn api_kind(&self) -> super::ApiKind {
        self.api_kind
    }

    /// Returns the explicit safe endpoint identity.
    #[must_use]
    pub const fn endpoint(&self) -> &super::EndpointIdentity {
        &self.endpoint
    }

    /// Returns the explicit credential requirement.
    #[must_use]
    pub const fn credential(&self) -> &super::CredentialRequirement {
        &self.credential
    }

    /// Returns the exact selected model cohort in caller order.
    #[must_use]
    pub fn model_ids(&self) -> &[super::ModelId] {
        &self.model_ids
    }
}

/// Safe result of one Models.dev snapshot projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelsDevIngestReport {
    registry: super::ProviderRegistry,
    snapshot_blake3: String,
    provider_count: usize,
    model_count: usize,
    omitted_cost_count: usize,
}

impl ModelsDevIngestReport {
    /// Returns the validated provider registry projection.
    #[must_use]
    pub const fn registry(&self) -> &super::ProviderRegistry {
        &self.registry
    }

    /// Consumes the report and returns the validated registry.
    #[must_use]
    pub fn into_registry(self) -> super::ProviderRegistry {
        self.registry
    }

    /// Returns the BLAKE3 identity of the exact caller-supplied bytes.
    #[must_use]
    pub fn snapshot_blake3(&self) -> &str {
        &self.snapshot_blake3
    }

    /// Returns the projected provider count.
    #[must_use]
    pub const fn provider_count(&self) -> usize {
        self.provider_count
    }

    /// Returns the projected model count.
    #[must_use]
    pub const fn model_count(&self) -> usize {
        self.model_count
    }

    /// Returns the count of selected models whose price could not be represented exactly.
    #[must_use]
    pub const fn omitted_cost_count(&self) -> usize {
        self.omitted_cost_count
    }
}

/// Projects an already-fetched Models.dev `api.json` snapshot into a validated registry.
///
/// Provider and model order follows `bindings`. Unknown source fields are
/// ignored so additive Models.dev schema changes do not grant new behavior.
///
/// # Errors
///
/// Returns [`ModelsDevIngestError`] for oversized or malformed snapshots,
/// absent bound entries, unsupported selected model metadata, or normal
/// Animus registry validation failures.
pub fn ingest_models_dev_api_json(
    input: &[u8],
    bindings: &[ModelsDevProviderBinding],
) -> Result<ModelsDevIngestReport, ModelsDevIngestError> {
    if input.len() > MAX_MODELS_DEV_SNAPSHOT_BYTES {
        return Err(ModelsDevIngestError::SnapshotTooLarge {
            actual_bytes: input.len(),
            max_bytes: MAX_MODELS_DEV_SNAPSHOT_BYTES,
        });
    }

    let snapshot: BTreeMap<String, serde_json::Value> =
        serde_json::from_slice(input).map_err(|error| classify_json_error(&error))?;
    let mut providers = Vec::with_capacity(bindings.len());
    let mut model_count = 0_usize;
    let mut omitted_cost_count = 0_usize;

    for binding in bindings {
        let provider_label = binding.provider_id().as_str();
        let raw_provider_value =
            snapshot
                .get(provider_label)
                .ok_or_else(|| ModelsDevIngestError::ProviderMissing {
                    provider: String::from(provider_label),
                })?;
        let raw_provider: RawProvider = serde_json::from_value(raw_provider_value.clone())
            .map_err(|error| classify_json_error(&error))?;
        if raw_provider.id != provider_label {
            return Err(ModelsDevIngestError::ProviderIdentityMismatch {
                provider: String::from(provider_label),
            });
        }

        let mut models = Vec::with_capacity(binding.model_ids().len());
        for model_id in binding.model_ids() {
            let model_label = model_id.as_str();
            let raw_model_value = raw_provider.models.get(model_label).ok_or_else(|| {
                ModelsDevIngestError::ModelMissing {
                    provider: String::from(provider_label),
                    model: String::from(model_label),
                }
            })?;
            let raw_model: RawModel = serde_json::from_value(raw_model_value.clone())
                .map_err(|error| classify_json_error(&error))?;
            if raw_model.id != model_label {
                return Err(ModelsDevIngestError::ModelIdentityMismatch {
                    provider: String::from(provider_label),
                    model: String::from(model_label),
                });
            }

            let (capabilities, cost_omitted) =
                project_capabilities(provider_label, model_label, &raw_model)?;
            omitted_cost_count = omitted_cost_count.saturating_add(usize::from(cost_omitted));
            model_count = model_count.saturating_add(1);
            models.push(super::ModelEntry::new(
                model_id.clone(),
                binding.api_kind(),
                binding.endpoint().clone(),
                capabilities,
            ));
        }

        providers.push(super::ProviderEntry::new(
            binding.provider_id().clone(),
            binding.credential().clone(),
            models,
        )?);
    }

    let provider_count = providers.len();
    let registry = super::ProviderRegistry::new(providers)?;
    Ok(ModelsDevIngestReport {
        registry,
        snapshot_blake3: blake3::hash(input).to_hex().to_string(),
        provider_count,
        model_count,
        omitted_cost_count,
    })
}

fn project_capabilities(
    provider: &str,
    model: &str,
    raw: &RawModel,
) -> Result<(super::ModelCapabilities, bool), ModelsDevIngestError> {
    let input_modalities = project_input_modalities(&raw.modalities.input);
    if input_modalities.is_empty() {
        return Err(unsupported_model(
            provider,
            model,
            UNSUPPORTED_MODALITY_REASON,
        ));
    }

    let context_budget = u32::try_from(raw.limit.context)
        .ok()
        .filter(|budget| *budget != 0)
        .ok_or_else(|| unsupported_model(provider, model, INVALID_CONTEXT_REASON))?;
    let output_budget = u32::try_from(raw.limit.output)
        .ok()
        .filter(|budget| *budget != 0)
        .ok_or_else(|| unsupported_model(provider, model, INVALID_OUTPUT_REASON))?;
    let reasoning = if raw.reasoning {
        super::ReasoningSupport::Optional
    } else {
        super::ReasoningSupport::None
    };
    let thinking_levels = project_thinking_levels(raw);
    let projected_cost = raw.cost.as_ref().and_then(project_cost);
    let cost_omitted = raw.cost.is_some() && projected_cost.is_none();
    let capabilities = super::ModelCapabilities::from_draft(super::ModelCapabilitiesDraft {
        input_modalities,
        context_budget,
        output_budget,
        reasoning,
        thinking_levels,
        streaming: super::ModelStreamingCapabilities::none(),
        cost: projected_cost,
    })?;
    Ok((capabilities, cost_omitted))
}

fn project_input_modalities(values: &[String]) -> Vec<super::InputModality> {
    let mut projected = Vec::new();
    for value in values {
        let modality = match value.as_str() {
            "text" => Some(super::InputModality::Text),
            "image" => Some(super::InputModality::Image),
            _ => None,
        };
        if let Some(modality) = modality {
            if !projected.contains(&modality) {
                projected.push(modality);
            }
        }
    }
    projected
}

fn project_thinking_levels(raw: &RawModel) -> Vec<super::ThinkingLevel> {
    let mut levels = vec![super::ThinkingLevel::None];
    if !raw.reasoning {
        return levels;
    }
    for option in &raw.reasoning_options {
        for value in option.values.as_deref().unwrap_or_default() {
            let level = match value.as_str() {
                "low" => Some(super::ThinkingLevel::Low),
                "medium" => Some(super::ThinkingLevel::Medium),
                "high" => Some(super::ThinkingLevel::High),
                _ => None,
            };
            if let Some(level) = level {
                if !levels.contains(&level) {
                    levels.push(level);
                }
            }
        }
    }
    levels
}

fn project_cost(raw: &RawCost) -> Option<super::TokenCost> {
    let input = parse_micro_usd(&raw.input)?;
    let output = parse_micro_usd(&raw.output)?;
    Some(super::TokenCost::new(input, output))
}

fn parse_micro_usd(number: &serde_json::Number) -> Option<u32> {
    let text = number.to_string();
    if text.starts_with('-') || text.contains(['e', 'E']) {
        return None;
    }
    let (whole_text, fraction_text) = text.split_once('.').unwrap_or((text.as_str(), ""));
    if fraction_text.len() > MAX_USD_FRACTION_DIGITS {
        return None;
    }
    let whole = whole_text.parse::<u64>().ok()?;
    let fraction = if fraction_text.is_empty() {
        0_u64
    } else {
        fraction_text.parse::<u64>().ok()?
    };
    let padding = MAX_USD_FRACTION_DIGITS.checked_sub(fraction_text.len())?;
    let exponent = u32::try_from(padding).ok()?;
    let fraction_scale = DECIMAL_RADIX.checked_pow(exponent)?;
    let whole_micro = whole.checked_mul(MICRO_USD_PER_USD)?;
    let fraction_micro = fraction.checked_mul(fraction_scale)?;
    u32::try_from(whole_micro.checked_add(fraction_micro)?).ok()
}

fn unsupported_model(provider: &str, model: &str, reason: &'static str) -> ModelsDevIngestError {
    ModelsDevIngestError::UnsupportedModel {
        provider: String::from(provider),
        model: String::from(model),
        reason,
    }
}

fn classify_json_error(error: &serde_json::Error) -> ModelsDevIngestError {
    let kind = match error.classify() {
        serde_json::error::Category::Syntax => ModelsDevJsonErrorKind::Syntax,
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            ModelsDevJsonErrorKind::Data
        }
        serde_json::error::Category::Eof => ModelsDevJsonErrorKind::EndOfInput,
    };
    ModelsDevIngestError::Json(kind)
}

#[derive(Deserialize)]
struct RawProvider {
    id: String,
    models: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct RawModel {
    id: String,
    reasoning: bool,
    #[serde(default)]
    reasoning_options: Vec<RawReasoningOption>,
    modalities: RawModalities,
    limit: RawLimit,
    cost: Option<RawCost>,
}

#[derive(Deserialize)]
struct RawReasoningOption {
    #[serde(default)]
    values: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct RawModalities {
    input: Vec<String>,
}

#[derive(Deserialize)]
struct RawLimit {
    context: u64,
    output: u64,
}

#[derive(Deserialize)]
struct RawCost {
    input: serde_json::Number,
    output: serde_json::Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDER_ID: &str = "openai";
    const MODEL_ID: &str = "gpt-5-mini";
    const MISSING_MODEL_ID: &str = "missing-model";
    const ENDPOINT_IDENTITY: &str = "models-dev-openai-responses";
    const API_KEY_ENV: &str = "OPENAI_API_KEY";
    const EXPECTED_PROVIDER_COUNT: usize = 1;
    const EXPECTED_MODEL_COUNT: usize = 1;
    const EXPECTED_MODALITY_COUNT: usize = 2;
    const EXPECTED_INPUT_COST_MICRO_USD: u32 = 250_000;
    const EXPECTED_OUTPUT_COST_MICRO_USD: u32 = 2_000_000;
    const EXPECTED_HASH_HEX_CHARS: usize = 64;
    const SNAPSHOT: &str = r#"{
      "openai": {
        "id": "openai",
        "env": ["OPENAI_API_KEY"],
        "npm": "@ai-sdk/openai",
        "models": {
          "gpt-5-mini": {
            "id": "gpt-5-mini",
            "name": "GPT-5 Mini",
            "reasoning": true,
            "reasoning_options": [
              {"type": "effort", "values": ["low", "high", "max"]}
            ],
            "tool_call": true,
            "modalities": {
              "input": ["text", "image", "pdf"],
              "output": ["text"]
            },
            "limit": {"context": 400000, "output": 128000},
            "cost": {"input": 0.25, "output": 2}
          }
        }
      },
      "unselected": {
        "id": "unselected",
        "models": {"schema-drift": null}
      }
    }"#;
    const AUDIO_ONLY_SNAPSHOT: &str = r#"{
      "openai": {
        "id": "openai",
        "models": {
          "gpt-5-mini": {
            "id": "gpt-5-mini",
            "reasoning": false,
            "modalities": {"input": ["audio"], "output": ["text"]},
            "limit": {"context": 400000, "output": 128000},
            "cost": null
          }
        }
      }
    }"#;

    fn binding(model: &str) -> ModelsDevProviderBinding {
        ModelsDevProviderBinding::from_draft(ModelsDevProviderBindingDraft {
            provider_id: super::super::ProviderId::new(PROVIDER_ID)
                .expect("provider id must validate"),
            api_kind: super::super::ApiKind::OpenAiResponses,
            endpoint: super::super::EndpointIdentity::new(ENDPOINT_IDENTITY)
                .expect("endpoint identity must validate"),
            credential: super::super::CredentialRequirement::new(
                super::super::CredentialSource::environment(API_KEY_ENV)
                    .expect("credential source must validate"),
                true,
            ),
            model_ids: vec![super::super::ModelId::new(model).expect("model id must validate")],
        })
        .expect("binding must validate")
    }

    #[test]
    fn projects_explicit_provider_and_model_cohort() {
        // r[verify agent_harness_runtime.pi_provider_registry.dynamic_validation]
        let report = ingest_models_dev_api_json(SNAPSHOT.as_bytes(), &[binding(MODEL_ID)])
            .expect("selected Models.dev cohort must project");
        let selected = report
            .registry()
            .select_model(&super::super::ModelSelection::new(
                super::super::ProviderId::new(PROVIDER_ID).expect("provider id must validate"),
                super::super::ModelId::new(MODEL_ID).expect("model id must validate"),
                super::super::ThinkingLevel::High,
                super::super::ThinkingPolicy::Reject,
            ))
            .expect("projected model must select");
        let cost = selected
            .capabilities()
            .cost()
            .expect("exact source price must project");

        assert_eq!(report.provider_count(), EXPECTED_PROVIDER_COUNT);
        assert_eq!(report.model_count(), EXPECTED_MODEL_COUNT);
        assert_eq!(report.omitted_cost_count(), 0);
        assert_eq!(report.snapshot_blake3().len(), EXPECTED_HASH_HEX_CHARS);
        assert_eq!(selected.endpoint().as_str(), ENDPOINT_IDENTITY);
        assert_eq!(
            selected.capabilities().input_modalities().len(),
            EXPECTED_MODALITY_COUNT
        );
        assert_eq!(
            selected.effective_thinking(),
            super::super::ThinkingLevel::High
        );
        assert_eq!(
            cost.input_micro_usd_per_million(),
            EXPECTED_INPUT_COST_MICRO_USD
        );
        assert_eq!(
            cost.output_micro_usd_per_million(),
            EXPECTED_OUTPUT_COST_MICRO_USD
        );
    }

    #[test]
    fn rejects_missing_bound_model_without_partial_registry() {
        // r[verify agent_harness_runtime.pi_provider_registry.dynamic_validation.reject]
        let error = ingest_models_dev_api_json(SNAPSHOT.as_bytes(), &[binding(MISSING_MODEL_ID)])
            .expect_err("missing selected model must fail");

        assert!(matches!(
            error,
            ModelsDevIngestError::ModelMissing { provider, model }
                if provider == PROVIDER_ID && model == MISSING_MODEL_ID
        ));
    }

    #[test]
    fn rejects_selected_model_without_supported_input_modality() {
        let error =
            ingest_models_dev_api_json(AUDIO_ONLY_SNAPSHOT.as_bytes(), &[binding(MODEL_ID)])
                .expect_err("unsupported selected model must fail");

        assert!(matches!(
            error,
            ModelsDevIngestError::UnsupportedModel { reason, .. }
                if reason == UNSUPPORTED_MODALITY_REASON
        ));
    }

    #[test]
    fn rejects_malformed_snapshot_with_typed_error() {
        const MALFORMED_SNAPSHOT: &str = "{";
        let error = ingest_models_dev_api_json(MALFORMED_SNAPSHOT.as_bytes(), &[binding(MODEL_ID)])
            .expect_err("malformed snapshot must fail");

        assert!(matches!(
            error,
            ModelsDevIngestError::Json(ModelsDevJsonErrorKind::EndOfInput)
        ));
    }
}
