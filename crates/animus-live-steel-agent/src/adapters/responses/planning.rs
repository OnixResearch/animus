pub(crate) struct PlannedProviderRequest {
    pub(crate) plan: animus_std::ProviderDispatchPlan,
    pub(crate) canonical: animus_adapters::CanonicalModelRequest,
}

pub(crate) fn plan_request(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    continuation: bool,
    correlation_id: Option<&str>,
) -> Result<PlannedProviderRequest, super::ProviderStreamError> {
    let canonical = build_canonical(config, request, continuation, correlation_id)?;
    let capabilities = animus_adapters::ProviderProjectionSupport {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses,
        developer_role: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        images: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        tools: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        json_object: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        json_schema: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
        reasoning: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        streaming: animus_adapters::ProviderProjectionFeatureSupport::Preserved,
        metadata: animus_adapters::ProviderProjectionFeatureSupport::Unsupported,
    };
    let projection =
        animus_adapters::project_model_provider_request_fixture(&canonical, capabilities)
            .map_err(|_error| super::ProviderStreamError::Planning)?;
    let expected = projection
        .literal_parts()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let plan = animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: config.selected_model(),
        canonical_request: &canonical,
        reviewed_projection_fixture: expected.as_slice(),
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(config.credential_reference()),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline.live-steel-agent"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancellation.live-steel-agent"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .map_err(|_error| super::ProviderStreamError::Planning)?;
    debug_assert_eq!(plan.provider_id(), config.provider_id());
    debug_assert_eq!(plan.model_id(), config.model_id());
    Ok(PlannedProviderRequest { plan, canonical })
}

pub(super) fn project_reasoning_request(
    effort: &str,
) -> Result<animus_adapters::CanonicalModelReasoningRequestDraft<'static>, super::ProviderStreamError>
{
    // r[impl agent_harness_runtime.local_dspark_reasoning_planning.policy]
    if effort == super::DISABLED_REASONING_EFFORT {
        Ok(animus_adapters::CanonicalModelReasoningRequestDraft {
            enabled: false,
            effort: animus_adapters::CanonicalModelReasoningEffort::None,
            summary_label: None,
        })
    } else if effort == super::LOW_REASONING_EFFORT {
        Ok(animus_adapters::CanonicalModelReasoningRequestDraft {
            enabled: true,
            effort: animus_adapters::CanonicalModelReasoningEffort::Low,
            summary_label: None,
        })
    } else {
        Err(super::ProviderStreamError::Canonical)
    }
}

pub(crate) fn final_text(
    outcome: &animus_std_openai::OpenAiStreamOutcome,
) -> Result<String, super::ProviderStreamError> {
    if outcome.terminal() != animus_std_openai::OpenAiStreamTerminal::Completed {
        return Err(super::ProviderStreamError::Terminal);
    }
    let text = outcome
        .frames()
        .iter()
        .filter(|frame| frame.is_default_final_visible())
        .filter_map(animus_adapters::ModelStreamFrame::body)
        .collect::<String>();
    if text.is_empty() {
        return Err(super::ProviderStreamError::Terminal);
    }
    Ok(text)
}

pub(super) fn tool_capabilities(
    model_id: &str,
) -> Result<animus_std_openai::OpenAiToolCapabilityMatrix, super::ProviderStreamError> {
    animus_std_openai::OpenAiToolCapabilityMatrix::new(
        model_id,
        vec![
            animus_adapters::CanonicalModelRole::User,
            animus_adapters::CanonicalModelRole::Tool,
        ],
        vec![
            animus_std_openai::OpenAiToolCapability::new(
                animus_std_openai::STEEL_REPL_TOOL_NAME,
                animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            )
            .map_err(|_error| super::ProviderStreamError::ToolCohort)?,
        ],
        vec![
            animus_std_openai::OpenAiToolStreamChannel::Content,
            animus_std_openai::OpenAiToolStreamChannel::ReasoningSummary,
            animus_std_openai::OpenAiToolStreamChannel::ToolCall,
            animus_std_openai::OpenAiToolStreamChannel::Usage,
            animus_std_openai::OpenAiToolStreamChannel::Finish,
        ],
    )
    .map_err(|_error| super::ProviderStreamError::ToolCohort)
}

pub(super) fn tool_schemas()
-> Result<Vec<animus_std_openai::OpenAiToolSchemaBinding>, super::ProviderStreamError> {
    Ok(vec![
        animus_std_openai::OpenAiToolSchemaBinding::from_json(
            animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            super::TOOL_SCHEMA_JSON,
        )
        .map_err(|_error| super::ProviderStreamError::ToolCohort)?,
    ])
}

fn build_canonical(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    continuation: bool,
    correlation_id: Option<&str>,
) -> Result<animus_adapters::CanonicalModelRequest, super::ProviderStreamError> {
    let text = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: request.request().prompt(),
    }];
    let message = animus_adapters::CanonicalModelMessageDraft {
        role: if continuation { "tool" } else { "user" },
        content: &text,
        tool_call_id: correlation_id,
    };
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: animus_std_openai::STEEL_REPL_TOOL_NAME,
        description: super::TOOL_DESCRIPTION,
        input_schema_label: animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
    }];
    let canonical = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &[message],
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(
                    u32::try_from(config.max_output_tokens())
                        .map_err(|_error| super::ProviderStreamError::Canonical)?,
                ),
                temperature_millis: None,
                stop_sequences: &[],
                stream: true,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: project_reasoning_request(config.reasoning_effort())?,
            provenance: &[],
        },
    )
    .map_err(|_error| super::ProviderStreamError::Canonical)?;
    debug_assert_eq!(tools.len(), 1);
    debug_assert_eq!(text.len(), 1);
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    #[test]
    fn reasoning_projection_preserves_disabled_and_low_policy() {
        // r[verify agent_harness_runtime.local_dspark_reasoning_planning.policy.none]
        // r[verify agent_harness_runtime.local_dspark_reasoning_planning.policy.low]
        let disabled = super::project_reasoning_request(super::super::DISABLED_REASONING_EFFORT)
            .expect("disabled reasoning must project");
        assert!(!disabled.enabled);
        assert_eq!(
            disabled.effort,
            animus_adapters::CanonicalModelReasoningEffort::None
        );

        let low = super::project_reasoning_request(super::super::LOW_REASONING_EFFORT)
            .expect("low reasoning must project");
        assert!(low.enabled);
        assert_eq!(
            low.effort,
            animus_adapters::CanonicalModelReasoningEffort::Low
        );
    }

    #[test]
    fn unknown_reasoning_projection_fails_closed() {
        // r[verify agent_harness_runtime.local_dspark_reasoning_planning.policy.failure]
        assert!(matches!(
            super::project_reasoning_request("medium"),
            Err(super::super::ProviderStreamError::Canonical)
        ));
    }
}
