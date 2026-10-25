//! The opt-in live route: one reviewed OpenAI-compatible Responses endpoint through
//! `animus-std-openai`, with an explicit model and an explicit credential.
//!
//! Each manager turn is one text-only, non-streaming request without tools or reasoning. The
//! route keeps no conversation. The endpoint pair is one of the host's reviewed cohorts, and no
//! request is made before the caller constructs the route with a credential.

mod selection;

/// Seconds one request may take.
const DEADLINE_SECS: u64 = 120;
/// Response bytes the host accepts.
const MAX_RESPONSE_BYTES: u64 = 262_144;
/// Output tokens one reply may use.
const OUTPUT_TOKENS: u32 = 2_048;
/// Brokered credential reference of the route.
const CREDENTIAL_LABEL: &str = "brokered:animus-campaign-manager-eval-stdin";

/// A reviewed OpenAI-compatible Responses endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endpoint {
    /// The official `OpenAI` Responses endpoint.
    OpenAi,
    /// The `OpenRouter` Responses endpoint.
    OpenRouter,
}

impl Endpoint {
    /// The provider identity.
    #[must_use]
    pub const fn provider(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::OpenRouter => "openrouter",
        }
    }

    const fn identity(self) -> &'static str {
        match self {
            Self::OpenAi => animus_std_openai::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
            Self::OpenRouter => animus_std_openai::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
        }
    }

    const fn url(self) -> &'static str {
        match self {
            Self::OpenAi => animus_std_openai::OFFICIAL_OPENAI_RESPONSES_URL,
            Self::OpenRouter => animus_std_openai::OPENROUTER_RESPONSES_URL,
        }
    }
}

/// Why the live route could not be built.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveFailure {
    /// The credential is not admissible.
    Credential,
    /// The model identity is not admissible.
    Model,
    /// The host configuration was refused.
    Host,
}

/// A live model route.
pub struct LiveRoute {
    host: animus_std_openai::ReqwestOpenAiHost,
    key: animus_std_openai::OpenAiApiKey,
    selected: animus_std::SelectedModel,
    identity: String,
}

impl LiveRoute {
    /// A route to `model` on `endpoint`, authenticated by `credential`.
    ///
    /// # Errors
    ///
    /// Returns [`LiveFailure`] when the credential, model, or host configuration is refused.
    // r[impl agent_harness_runtime.campaign_manager_eval.live]
    pub fn new(endpoint: Endpoint, model: &str, credential: String) -> Result<Self, LiveFailure> {
        let key = animus_std_openai::OpenAiApiKey::new(CREDENTIAL_LABEL, credential)
            .map_err(|_| LiveFailure::Credential)?;
        let selected = selection::selected_model(endpoint, model)?;
        let config = animus_std_openai::OpenAiHostConfig::new(
            endpoint.identity(),
            endpoint.url(),
            std::time::Duration::from_secs(DEADLINE_SECS),
            MAX_RESPONSE_BYTES,
        )
        .map_err(|_| LiveFailure::Host)?;
        let host =
            animus_std_openai::ReqwestOpenAiHost::new(config).map_err(|_| LiveFailure::Host)?;
        Ok(Self {
            host,
            key,
            selected,
            identity: route_identity(endpoint, model),
        })
    }

    /// The route's Harbor model-route identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for LiveRoute
{
    type Error = crate::RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let plan = dispatch_plan(&self.selected, request.prompt())?;
        let outcome = self
            .host
            .execute(
                &plan,
                &self.key,
                animus_std_openai::CancellationState::Active,
            )
            .map_err(|_| crate::RouteFailure::Unavailable)?;
        let text: String = outcome
            .response()
            .items()
            .iter()
            .filter_map(|item| {
                if let animus_adapters::CanonicalModelResponseItem::AssistantText { text } = item {
                    Some(text.as_str())
                } else {
                    None
                }
            })
            .collect();
        animus_adapters::ModelResponse::new(&text).map_err(|_| crate::RouteFailure::Malformed)
    }
}

/// The safe route identity of `model` on `endpoint`: characters outside `[A-Za-z0-9._-]` become
/// `:`.
fn route_identity(endpoint: Endpoint, model: &str) -> String {
    let safe_model: String = model
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || "._-".contains(character) {
                character
            } else {
                ':'
            }
        })
        .collect();
    format!("model:live:{}:{safe_model}", endpoint.provider())
}

/// The reviewed dispatch plan of one text-only prompt.
fn dispatch_plan(
    selected: &animus_std::SelectedModel,
    prompt: &str,
) -> Result<animus_std::ProviderDispatchPlan, crate::RouteFailure> {
    let canonical = canonical_request(prompt)?;
    let projection =
        animus_adapters::project_model_provider_request_fixture(&canonical, projection_rules())
            .map_err(|_| crate::RouteFailure::Rejected)?;
    let expected: Vec<&str> = projection
        .literal_parts()
        .iter()
        .map(String::as_str)
        .collect();
    animus_std::plan_provider_dispatch(&dispatch_input(selected, &canonical, &expected))
        .map_err(|_| crate::RouteFailure::Rejected)
}

/// One user message holding `prompt`, text output, no tools, no streaming, no reasoning.
fn canonical_request(
    prompt: &str,
) -> Result<animus_adapters::CanonicalModelRequest, crate::RouteFailure> {
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &[animus_adapters::CanonicalModelMessageDraft {
                role: "user",
                content: &[animus_adapters::CanonicalMessageContentPartDraft::Text {
                    text: prompt,
                }],
                tool_call_id: None,
            }],
            tools: &[],
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_TOKENS),
                temperature_millis: None,
                stop_sequences: &[],
                stream: false,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: false,
                effort: animus_adapters::CanonicalModelReasoningEffort::None,
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .map_err(|_| crate::RouteFailure::Rejected)
}

/// The `OpenAI` Responses projection rules the dispatch planner applies.
const fn projection_rules() -> animus_adapters::ProviderProjectionSupport {
    animus_adapters::ProviderProjectionSupport {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses,
        developer_role: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        images: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        tools: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        json_object: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        json_schema: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        reasoning: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        streaming: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        metadata: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
    }
}

/// The dispatch input of `canonical` for `selected`, reviewed against `expected`.
fn dispatch_input<'input>(
    selected: &'input animus_std::SelectedModel,
    canonical: &'input animus_adapters::CanonicalModelRequest,
    expected: &'input [&'input str],
) -> animus_std::ProviderDispatchInput<'input> {
    animus_std::ProviderDispatchInput {
        selected_model: selected,
        canonical_request: canonical,
        reviewed_projection_fixture: expected,
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(CREDENTIAL_LABEL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline.campaign-manager-eval"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancellation.campaign-manager-eval"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    }
}
