//! Tests for canonical model message contracts.
#![allow(unknown_lints, function_length, numeric_units)]

use crate::model::*;

const USER_PROMPT: &str = "Summarize the safe plan.";
const SYSTEM_PROMPT: &str = "Stay inside adapter contracts.";
const TOOL_RESULT: &str = "tool result";
const TOOL_CALL_ID: &str = "call-lookup";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Returns bounded facts.";
const TOOL_SCHEMA: &str = "schema.lookup.v1";
const IMAGE_DESCRIPTOR: &str = "image-descriptor-1";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "b3-safe-image-hash";
const JSON_SCHEMA: &str = "schema.answer.v1";
const METADATA_KEY: &str = "trace";
const METADATA_VALUE: &str = "safe-trace";
const STOP_SEQUENCE: &str = "<END>";
const RESPONSE_TEXT: &str = "done";
const STRUCTURED_PAYLOAD: &str = "payload-hash";
const REFUSAL_REASON: &str = "policy_refusal";
const SAFETY_CATEGORY: &str = "safety_boundary";
const MALFORMED_DIAGNOSTIC: &str = "missing_required_field";
const RAW_PAYLOAD_LABEL: &str = "raw_provider_payload:{secret}";
const HIDDEN_REASONING_LABEL: &str = "hidden_reasoning: private chain";
const UNSUPPORTED_ROLE: &str = "robot";
const UNSUPPORTED_CONTENT_KIND: &str = "audio-bytes";
const WRONG_CONTRACT_IDENTITY: &str = "animus.model-message-request.v0";
const OVERSIZED_METADATA_COUNT: usize = 33;
const INPUT_TOKENS: u32 = 10;
const OUTPUT_TOKENS: u32 = 6;
const CACHED_TOKENS: u32 = 4;
const TOO_MANY_CACHED_TOKENS: u32 = 11;
const TEMPERATURE_MILLIS: u16 = 4;

#[test]
fn canonical_request_accepts_structured_conversation() {
    // r[verify agent_harness_runtime.model_compat.message_request.validation]
    let system_content = [CanonicalContentPartDraft::Text {
        text: SYSTEM_PROMPT,
    }];
    let user_content = [
        CanonicalContentPartDraft::Text { text: USER_PROMPT },
        CanonicalContentPartDraft::ImageDescriptor {
            descriptor_id: IMAGE_DESCRIPTOR,
            media_type: IMAGE_MEDIA_TYPE,
            content_hash: IMAGE_HASH,
        },
    ];
    let tool_content = [CanonicalContentPartDraft::Text { text: TOOL_RESULT }];
    let messages = [
        CanonicalModelMessageDraft {
            role: CanonicalModelRole::System.as_str(),
            content: &system_content,
            tool_call_id: None,
        },
        CanonicalModelMessageDraft {
            role: CanonicalModelRole::User.as_str(),
            content: &user_content,
            tool_call_id: None,
        },
        CanonicalModelMessageDraft {
            role: CanonicalModelRole::Tool.as_str(),
            content: &tool_content,
            tool_call_id: Some(TOOL_CALL_ID),
        },
    ];
    let tools = [CanonicalToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let stop_sequences = [STOP_SEQUENCE];
    let provenance = [CanonicalSafeMetadataDraft {
        key: METADATA_KEY,
        value: METADATA_VALUE,
    }];

    let request = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &messages,
        tools: &tools,
        output: CanonicalOutputControlsDraft {
            max_output_tokens: Some(OUTPUT_TOKENS),
            temperature_millis: Some(TEMPERATURE_MILLIS),
            stop_sequences: &stop_sequences,
            stream: true,
        },
        response_format: CanonicalResponseFormatIntentDraft::JsonSchema {
            schema_label: JSON_SCHEMA,
        },
        reasoning: CanonicalReasoningRequestDraft {
            enabled: true,
            effort: CanonicalReasoningEffort::Medium,
            summary_label: Some("safe-summary"),
        },
        provenance: &provenance,
    })
    .expect("structured canonical request should validate");

    assert_eq!(request.messages().len(), messages.len());
    assert_eq!(request.tools()[0].name(), TOOL_NAME);
    assert!(request.output().stream());
    assert_eq!(
        request.reasoning().effort(),
        CanonicalReasoningEffort::Medium
    );
    assert_eq!(
        CanonicalModelRequest::contract_identity(),
        CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY
    );
}

#[test]
fn canonical_request_rejects_malformed_roles_and_empty_messages() {
    // r[verify agent_harness_runtime.model_compat.message_request.validation]
    let content = [CanonicalContentPartDraft::Text { text: USER_PROMPT }];
    let bad_role = [CanonicalModelMessageDraft {
        role: UNSUPPORTED_ROLE,
        content: &content,
        tool_call_id: None,
    }];
    let error = request_with_messages(&bad_role).expect_err("unsupported role should fail");
    assert!(matches!(
        error,
        CanonicalModelContractError::UnsupportedRole { .. }
    ));

    let empty_messages: [CanonicalModelMessageDraft<'_>; 0] = [];
    let error = request_with_messages(&empty_messages).expect_err("empty messages should fail");
    assert_eq!(
        error,
        CanonicalModelContractError::EmptyCollection {
            field: "model_role",
        }
    );
}

#[test]
fn canonical_request_rejects_unsupported_content_and_missing_required_fields() {
    // r[verify agent_harness_runtime.model_compat.message_request.validation]
    let unsupported_content = [CanonicalContentPartDraft::Unsupported {
        kind: UNSUPPORTED_CONTENT_KIND,
    }];
    let unsupported_message = [CanonicalModelMessageDraft {
        role: CanonicalModelRole::User.as_str(),
        content: &unsupported_content,
        tool_call_id: None,
    }];
    let error =
        request_with_messages(&unsupported_message).expect_err("unsupported part should fail");
    assert!(matches!(
        error,
        CanonicalModelContractError::UnsupportedContentPart { .. }
    ));

    let text_content = [CanonicalContentPartDraft::Text { text: USER_PROMPT }];
    let message = [CanonicalModelMessageDraft {
        role: CanonicalModelRole::User.as_str(),
        content: &text_content,
        tool_call_id: None,
    }];
    let error = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: WRONG_CONTRACT_IDENTITY,
        messages: &message,
        tools: &[],
        output: default_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: default_reasoning(),
        provenance: &[],
    })
    .expect_err("wrong contract identity should fail");
    assert!(matches!(
        error,
        CanonicalModelContractError::InvalidContractIdentity { .. }
    ));
}

#[test]
fn canonical_request_rejects_oversized_metadata_and_leaks() {
    // r[verify agent_harness_runtime.model_compat.message_request.validation]
    let content = [CanonicalContentPartDraft::Text { text: USER_PROMPT }];
    let message = [CanonicalModelMessageDraft {
        role: CanonicalModelRole::User.as_str(),
        content: &content,
        tool_call_id: None,
    }];
    let oversized = alloc::vec![
        CanonicalSafeMetadataDraft {
            key: METADATA_KEY,
            value: METADATA_VALUE,
        };
        OVERSIZED_METADATA_COUNT
    ];
    let error = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &message,
        tools: &[],
        output: default_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: default_reasoning(),
        provenance: &oversized,
    })
    .expect_err("oversized metadata should fail");
    assert!(matches!(
        error,
        CanonicalModelContractError::CollectionTooLarge { .. }
    ));

    let leaky_content = [CanonicalContentPartDraft::RawProviderPayload {
        label: RAW_PAYLOAD_LABEL,
    }];
    let leaky_message = [CanonicalModelMessageDraft {
        role: CanonicalModelRole::User.as_str(),
        content: &leaky_content,
        tool_call_id: None,
    }];
    let error =
        request_with_messages(&leaky_message).expect_err("raw provider payload should fail");
    assert_eq!(
        error,
        CanonicalModelContractError::UnsafeText {
            field: "model_raw_provider_payload",
            reason: CanonicalUnsafeTextReason::RawProviderPayload,
        }
    );
}

#[test]
fn canonical_response_accepts_text_structured_tool_and_refusal_cases() {
    // r[verify agent_harness_runtime.model_compat.response_envelope.validation]
    let text = [CanonicalModelResponseItemDraft::AssistantText {
        text: RESPONSE_TEXT,
    }];
    let text_response = response_with_items(&text, Some(CanonicalFinishReason::Stop.as_str()))
        .expect("text response should validate");
    assert_eq!(
        text_response.items()[0].assistant_text(),
        Some(RESPONSE_TEXT)
    );
    assert_eq!(
        CanonicalModelResponseEnvelope::contract_identity(),
        CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY
    );

    let structured = [CanonicalModelResponseItemDraft::StructuredPayload {
        schema_label: JSON_SCHEMA,
        payload_label: STRUCTURED_PAYLOAD,
    }];
    response_with_items(&structured, Some(CanonicalFinishReason::Stop.as_str()))
        .expect("structured response should validate");

    let tool = [CanonicalModelResponseItemDraft::ToolCall {
        call_id: TOOL_CALL_ID,
        tool_name: TOOL_NAME,
        arguments_label: TOOL_SCHEMA,
    }];
    response_with_items(&tool, Some(CanonicalFinishReason::ToolCall.as_str()))
        .expect("tool-call response should validate");

    let refusal = [CanonicalModelResponseItemDraft::Refusal {
        reason: REFUSAL_REASON,
    }];
    response_with_items(&refusal, Some(CanonicalFinishReason::Refusal.as_str()))
        .expect("refusal response should validate");

    let malformed = [CanonicalModelResponseItemDraft::MalformedOutput {
        diagnostic: MALFORMED_DIAGNOSTIC,
    }];
    response_with_items(&malformed, Some(CanonicalFinishReason::Malformed.as_str()))
        .expect("malformed-output diagnostic response should validate");
}

#[test]
fn canonical_response_rejects_missing_finish_and_malformed_usage() {
    // r[verify agent_harness_runtime.model_compat.response_envelope.validation]
    let items = [CanonicalModelResponseItemDraft::AssistantText {
        text: RESPONSE_TEXT,
    }];
    let missing_finish = response_with_items(&items, None).expect_err("finish reason is required");
    assert_eq!(
        missing_finish,
        CanonicalModelContractError::MissingFinishReason {
            field: "model_finish_reason",
        }
    );

    let malformed_usage =
        CanonicalModelResponseEnvelope::from_draft(CanonicalModelResponseEnvelopeDraft {
            contract_identity: CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY,
            items: &items,
            usage: Some(CanonicalUsageDraft {
                input: INPUT_TOKENS,
                output: OUTPUT_TOKENS,
                cached_input: Some(TOO_MANY_CACHED_TOKENS),
            }),
            finish_reason: Some(CanonicalFinishReason::Stop.as_str()),
            diagnostics: &[],
            provenance: &[],
        })
        .expect_err("cached usage greater than input should fail");
    assert!(matches!(
        malformed_usage,
        CanonicalModelContractError::MalformedUsage { .. }
    ));
}

#[test]
fn canonical_response_rejects_impossible_combinations_and_leaks() {
    // r[verify agent_harness_runtime.model_compat.response_envelope.validation]
    let impossible = [
        CanonicalModelResponseItemDraft::AssistantText {
            text: RESPONSE_TEXT,
        },
        CanonicalModelResponseItemDraft::Safety {
            category: SAFETY_CATEGORY,
        },
    ];
    let error = response_with_items(&impossible, Some(CanonicalFinishReason::Safety.as_str()))
        .expect_err("success and safety should not combine");
    assert!(matches!(
        error,
        CanonicalModelContractError::ImpossibleResponseCombination { .. }
    ));

    let raw = [CanonicalModelResponseItemDraft::RawProviderPayload {
        label: RAW_PAYLOAD_LABEL,
    }];
    let error = response_with_items(&raw, Some(CanonicalFinishReason::Stop.as_str()))
        .expect_err("raw provider payload should fail");
    assert_eq!(
        error,
        CanonicalModelContractError::UnsafeText {
            field: "model_raw_provider_payload",
            reason: CanonicalUnsafeTextReason::RawProviderPayload,
        }
    );

    let hidden = [CanonicalModelResponseItemDraft::HiddenReasoning {
        label: HIDDEN_REASONING_LABEL,
    }];
    let error = response_with_items(&hidden, Some(CanonicalFinishReason::Stop.as_str()))
        .expect_err("hidden reasoning should fail");
    assert_eq!(
        error,
        CanonicalModelContractError::UnsafeText {
            field: "model_hidden_reasoning",
            reason: CanonicalUnsafeTextReason::HiddenReasoning,
        }
    );
}

#[test]
fn legacy_prompt_message_round_trips_through_canonical_defaults() {
    // r[verify agent_harness_runtime.model_compat.legacy_bridge.equivalent]
    let legacy_request = Request::new(USER_PROMPT).expect("legacy request should validate");
    let canonical_request = CanonicalModelRequest::from_legacy_request(&legacy_request)
        .expect("legacy request should project to canonical");
    let projected_request = canonical_request
        .to_legacy_request()
        .expect("canonical compatibility request should project back");
    assert_eq!(projected_request, legacy_request);

    let legacy_response = Response::new(RESPONSE_TEXT).expect("legacy response should validate");
    let canonical_response = CanonicalModelResponseEnvelope::from_legacy_response(&legacy_response)
        .expect("legacy response should project to canonical");
    let projected_response = canonical_response
        .to_legacy_response()
        .expect("canonical text response should project back");
    assert_eq!(projected_response, legacy_response);
}

#[test]
fn legacy_projection_fails_closed_for_lossy_canonical_responses() {
    // r[verify agent_harness_runtime.model_compat.legacy_bridge.equivalent]
    let structured = [CanonicalModelResponseItemDraft::StructuredPayload {
        schema_label: JSON_SCHEMA,
        payload_label: STRUCTURED_PAYLOAD,
    }];
    let canonical_response =
        response_with_items(&structured, Some(CanonicalFinishReason::Stop.as_str()))
            .expect("structured response should validate");
    let error = canonical_response
        .to_legacy_response()
        .expect_err("structured response should not collapse into legacy text");
    assert_eq!(
        error,
        CanonicalModelContractError::LossyLegacyProjection {
            reason: "canonical response carries non-text semantics",
        }
    );

    let text_content = [CanonicalContentPartDraft::Text { text: USER_PROMPT }];
    let message = [CanonicalModelMessageDraft {
        role: CanonicalModelRole::User.as_str(),
        content: &text_content,
        tool_call_id: None,
    }];
    let tools = [CanonicalToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let canonical_request = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &message,
        tools: &tools,
        output: default_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: default_reasoning(),
        provenance: &[],
    })
    .expect("tool-enabled request should validate");
    let error = canonical_request
        .to_legacy_request()
        .expect_err("tool-enabled request should not collapse into legacy prompt");
    assert_eq!(
        error,
        CanonicalModelContractError::LossyLegacyProjection {
            reason: "canonical request cannot collapse into one legacy prompt",
        }
    );
}

fn request_with_messages(
    messages: &[CanonicalModelMessageDraft<'_>],
) -> Result<CanonicalModelRequest, CanonicalModelContractError> {
    CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages,
        tools: &[],
        output: default_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: default_reasoning(),
        provenance: &[],
    })
}

fn response_with_items(
    items: &[CanonicalModelResponseItemDraft<'_>],
    finish_reason: Option<&str>,
) -> Result<CanonicalModelResponseEnvelope, CanonicalModelContractError> {
    CanonicalModelResponseEnvelope::from_draft(CanonicalModelResponseEnvelopeDraft {
        contract_identity: CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY,
        items,
        usage: Some(CanonicalUsageDraft {
            input: INPUT_TOKENS,
            output: OUTPUT_TOKENS,
            cached_input: Some(CACHED_TOKENS),
        }),
        finish_reason,
        diagnostics: &[],
        provenance: &[],
    })
}

const fn default_output<'a>() -> CanonicalOutputControlsDraft<'a> {
    CanonicalOutputControlsDraft {
        max_output_tokens: None,
        temperature_millis: None,
        stop_sequences: &[],
        stream: false,
    }
}

const fn default_reasoning<'a>() -> CanonicalReasoningRequestDraft<'a> {
    CanonicalReasoningRequestDraft {
        enabled: false,
        effort: CanonicalReasoningEffort::None,
        summary_label: None,
    }
}
