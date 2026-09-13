pub(super) fn config() -> crate::OpenAiHostConfig {
    crate::OpenAiHostConfig::new(
        crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
        crate::OFFICIAL_OPENAI_RESPONSES_URL,
        std::time::Duration::from_secs(crate::tests::TIMEOUT_SECONDS),
        crate::tests::MAX_RESPONSE_BYTES,
    )
    .expect("test configuration must be admitted")
}

pub(super) fn api_key(source_label: &str) -> crate::OpenAiApiKey {
    crate::OpenAiApiKey::new(source_label, String::from(crate::tests::API_KEY_VALUE))
        .expect("test key must be admitted")
}

pub(super) fn plan(request_case: crate::tests::RequestCase) -> animus_std::ProviderDispatchPlan {
    let selected = selected_model();
    let canonical = canonical_request(request_case);
    let plan = animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected,
        canonical_request: &canonical,
        reviewed_projection_fixture: projection(request_case),
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(crate::tests::CREDENTIAL_LABEL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline-live-openai"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancel-live-openai"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .expect("test dispatch plan must be admitted");
    assert_eq!(
        plan.endpoint(),
        crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY
    );
    assert_eq!(
        plan.credential_source_label(),
        crate::tests::CREDENTIAL_LABEL
    );
    plan
}

fn projection(request_case: crate::tests::RequestCase) -> &'static [&'static str] {
    match request_case {
        crate::tests::RequestCase::Base | crate::tests::RequestCase::Stop => {
            crate::tests::BASE_PROJECTION
        }
        crate::tests::RequestCase::Stream => crate::tests::STREAM_PROJECTION,
        crate::tests::RequestCase::Tool => crate::tests::TOOL_PROJECTION,
        crate::tests::RequestCase::Image => crate::tests::IMAGE_PROJECTION,
        crate::tests::RequestCase::Structured => crate::tests::STRUCTURED_PROJECTION,
    }
}

fn canonical_request(
    request_case: crate::tests::RequestCase,
) -> animus_adapters::CanonicalModelRequest {
    let text = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: crate::tests::USER_TEXT,
    }];
    let image = [
        animus_adapters::CanonicalMessageContentPartDraft::Text {
            text: crate::tests::USER_TEXT,
        },
        animus_adapters::CanonicalMessageContentPartDraft::ImageDescriptor {
            descriptor_id: crate::tests::IMAGE_DESCRIPTOR,
            media_type: crate::tests::IMAGE_MEDIA_TYPE,
            content_hash: crate::tests::IMAGE_HASH,
        },
    ];
    let content = if matches!(request_case, crate::tests::RequestCase::Image) {
        image.as_slice()
    } else {
        text.as_slice()
    };
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: "user",
        content,
        tool_call_id: None,
    }];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: crate::tests::TOOL_NAME,
        description: crate::tests::TOOL_DESCRIPTION,
        input_schema_label: crate::tests::TOOL_SCHEMA,
    }];
    let metadata = [animus_adapters::CanonicalModelSafeMetadataDraft {
        key: crate::tests::METADATA_KEY,
        value: crate::tests::METADATA_VALUE,
    }];
    let request = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: if matches!(request_case, crate::tests::RequestCase::Tool) {
                &tools
            } else {
                &[]
            },
            output: output_controls(request_case),
            response_format: if matches!(request_case, crate::tests::RequestCase::Structured) {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::JsonSchema {
                    schema_label: crate::tests::RESPONSE_SCHEMA,
                }
            } else {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::Text
            },
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: true,
                effort: animus_adapters::CanonicalModelReasoningEffort::Low,
                summary_label: None,
            },
            provenance: &metadata,
        },
    )
    .expect("test canonical request must validate");
    assert_eq!(request.messages().len(), messages.len());
    assert_eq!(
        request.tools().is_empty(),
        !matches!(request_case, crate::tests::RequestCase::Tool)
    );
    request
}

pub(super) fn selected_model() -> animus_std::SelectedModel {
    let provider_id =
        animus_std::ProviderId::new(crate::tests::PROVIDER_ID).expect("provider ID must validate");
    let model_id =
        animus_std::ModelId::new(crate::tests::MODEL_ID).expect("model ID must validate");
    let endpoint =
        animus_std::EndpointIdentity::new(crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY)
            .expect("endpoint identity must validate");
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![
                animus_std::InputModality::Text,
                animus_std::InputModality::Image,
            ],
            context_budget: crate::tests::CONTEXT_BUDGET,
            output_budget: crate::tests::MODEL_OUTPUT_BUDGET,
            reasoning: animus_std::ReasoningSupport::Optional,
            thinking_levels: vec![
                animus_std::ThinkingLevel::None,
                animus_std::ThinkingLevel::Low,
            ],
            streaming: animus_std::ModelStreamingCapabilities::new(
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
            ),
            cost: None,
        })
        .expect("capabilities must validate");
    let model = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let credential = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::environment("ANIMUS_OPENAI_LIVE_API_KEY")
            .expect("credential source must validate"),
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, credential, vec![model])
        .expect("provider entry must validate");
    let selected = animus_std::ProviderRegistry::new(vec![provider])
        .expect("registry must validate")
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(crate::tests::PROVIDER_ID)
                .expect("provider ID must validate"),
            animus_std::ModelId::new(crate::tests::MODEL_ID).expect("model ID must validate"),
            animus_std::ThinkingLevel::Low,
            animus_std::ThinkingPolicy::Reject,
        ))
        .expect("model must select");
    assert_eq!(selected.provider_id().as_str(), crate::tests::PROVIDER_ID);
    assert_eq!(selected.model_id().as_str(), crate::tests::MODEL_ID);
    selected
}

pub(super) fn refusal_response() -> String {
    serde_json::json!({
        "id": "resp_refusal_fixture",
        "object": "response",
        "status": "completed",
        "output": [
            {
                "type": "message",
                "role": "assistant",
                "content": [
                    {"type": "refusal", "refusal": crate::tests::REFUSAL_REASON}
                ]
            }
        ]
    })
    .to_string()
}

pub(super) fn successful_response() -> String {
    serde_json::json!({
        "id": "resp_fixture",
        "object": "response",
        "status": "completed",
        "output": [
            {"type": "reasoning", "summary": []},
            {
                "type": "message",
                "role": "assistant",
                "content": [
                    {"type": "output_text", "text": crate::tests::RESPONSE_TEXT, "annotations": []}
                ]
            }
        ],
        "usage": {
            "input_tokens": crate::tests::RESPONSE_INPUT_TOKENS,
            "output_tokens": crate::tests::RESPONSE_OUTPUT_TOKENS,
            "input_tokens_details": {"cached_tokens": crate::tests::RESPONSE_CACHED_TOKENS}
        }
    })
    .to_string()
}

fn output_controls(
    request_case: crate::tests::RequestCase,
) -> animus_adapters::CanonicalModelOutputControlsDraft<'static> {
    animus_adapters::CanonicalModelOutputControlsDraft {
        max_output_tokens: Some(crate::tests::OUTPUT_LIMIT),
        temperature_millis: Some(crate::tests::TEMPERATURE_MILLIS),
        stop_sequences: if matches!(request_case, crate::tests::RequestCase::Stop) {
            &[crate::tests::STOP_SEQUENCE]
        } else {
            &[]
        },
        stream: matches!(request_case, crate::tests::RequestCase::Stream),
    }
}
