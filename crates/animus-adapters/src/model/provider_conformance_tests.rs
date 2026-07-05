use super::*;

const DEVELOPER_TEXT: &str = "Preserve provider-neutral semantics.";
const USER_TEXT: &str = "Answer with safe structure.";
const IMAGE_DESCRIPTOR: &str = "image-one";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "b3-image";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Bounded lookup.";
const TOOL_SCHEMA: &str = "schema.lookup";
const RESPONSE_SCHEMA: &str = "schema.answer";
const RESPONSE_TEXT: &str = "ok";
const PAYLOAD_LABEL: &str = "payload-hash";
const TOOL_CALL_ID: &str = "call-one";
const ARGUMENTS_LABEL: &str = "arguments-hash";
const REFUSAL_REASON: &str = "policy_refusal";
const SAFETY_CATEGORY: &str = "self_harm";
const MALFORMED_DIAGNOSTIC: &str = "malformed-json";
const CREDENTIAL_SENTINEL: &str = "credential:redacted-token";
const RAW_PAYLOAD: &str = "raw_provider_payload:{secret}";
const RAW_REQUEST_PAYLOAD_LABEL: &str = "provider_payload:request";
const HIDDEN_REASONING: &str = "hidden_reasoning: private";
const INPUT_TOKENS: u32 = 12;
const OUTPUT_TOKENS: u32 = 8;
const CACHED_INPUT_TOKENS: u32 = 4;
const EXPECTED_LITERAL_COUNT: usize = 11;
const STREAM_CALL_ID: &str = "call-1";
const STREAM_TOOL_ID: &str = "tool-1";
const INDEX_FEATURE: &str = "json_schema";
const INDEX_EVIDENCE: &str = "fixture-json-schema";
const INDEX_REASON: &str = "provider family has no image input";
const PRESERVED_FEATURE: ProviderProjectionFeatureSupport =
    ProviderProjectionFeatureSupport::Preserved;
const UNSUPPORTED_FEATURE: ProviderProjectionFeatureSupport =
    ProviderProjectionFeatureSupport::Unsupported;

#[test]
fn request_projection_matches_literal_fixtures_for_claimed_families() {
    // r[verify agent_harness_runtime.provider_projection_conformance.literal_fixtures]
    let request = canonical_projection_request();
    for family in [
        ProviderProjectionFamily::OpenAiResponses,
        ProviderProjectionFamily::OpenAiChat,
        ProviderProjectionFamily::AnthropicMessages,
        ProviderProjectionFamily::Mock,
    ] {
        let expected = expected_literals(family);
        let expected_refs: alloc::vec::Vec<_> =
            expected.iter().map(alloc::string::String::as_str).collect();
        let projection =
            assert_provider_request_fixture(&request, full_support(family), &expected_refs)
                .expect("literal provider projection should match reviewed fixture");
        assert_eq!(projection.family(), family);
        assert_eq!(projection.literal_parts().len(), EXPECTED_LITERAL_COUNT);
    }
}

#[test]
fn request_projection_fails_closed_for_lossy_provider_support() {
    // r[verify agent_harness_runtime.provider_projection_conformance.lossy_projection]
    let request = canonical_projection_request();
    for (support, feature) in [
        (
            ProviderProjectionSupport {
                developer_role: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "role",
        ),
        (
            ProviderProjectionSupport {
                images: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "content_part",
        ),
        (
            ProviderProjectionSupport {
                tools: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "tool",
        ),
        (
            ProviderProjectionSupport {
                json_schema: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "structured_output",
        ),
        (
            ProviderProjectionSupport {
                reasoning: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "reasoning",
        ),
        (
            ProviderProjectionSupport {
                streaming: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "streaming",
        ),
        (
            ProviderProjectionSupport {
                metadata: UNSUPPORTED_FEATURE,
                ..full_support(ProviderProjectionFamily::OpenAiResponses)
            },
            "metadata",
        ),
    ] {
        let error = project_provider_request_fixture(&request, support)
            .expect_err("lossy support should fail closed");
        assert_eq!(error, ProviderProjectionError::LossyProjection { feature });
    }
}

#[test]
fn request_leaks_and_malformed_metadata_fail_before_provider_projection() {
    // r[verify agent_harness_runtime.provider_projection_conformance.lossy_projection]
    let raw_content = [CanonicalContentPartDraft::RawProviderPayload {
        label: RAW_REQUEST_PAYLOAD_LABEL,
    }];
    let raw_message = [CanonicalModelMessageDraft {
        role: "user",
        content: &raw_content,
        tool_call_id: None,
    }];
    let raw_payload_error = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &raw_message,
        tools: &[],
        output: compatibility_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: compatibility_reasoning(),
        provenance: &[],
    })
    .expect_err("raw provider payload in request should fail before projection");
    assert!(matches!(
        raw_payload_error,
        CanonicalModelContractError::UnsafeText { .. }
    ));

    let text_content = [CanonicalContentPartDraft::Text { text: USER_TEXT }];
    let text_message = [CanonicalModelMessageDraft {
        role: "user",
        content: &text_content,
        tool_call_id: None,
    }];
    let unsafe_metadata = [CanonicalSafeMetadataDraft {
        key: "trace_label",
        value: CREDENTIAL_SENTINEL,
    }];
    let metadata_error = CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &text_message,
        tools: &[],
        output: compatibility_output(),
        response_format: CanonicalResponseFormatIntentDraft::Text,
        reasoning: compatibility_reasoning(),
        provenance: &unsafe_metadata,
    })
    .expect_err("unsafe metadata should fail before projection");
    assert!(matches!(
        metadata_error,
        CanonicalModelContractError::UnsafeText { .. }
    ));
}

#[test]
fn response_normalization_preserves_supported_semantics() {
    // r[verify agent_harness_runtime.provider_projection_conformance.response_normalization]
    let text = normalize_provider_response_fixture(text_response_fixture(
        RESPONSE_TEXT,
        Some(CanonicalFinishReason::Stop.as_str()),
    ))
    .expect("text fixture should normalize");
    assert_eq!(text.items()[0].assistant_text(), Some(RESPONSE_TEXT));
    assert_eq!(
        text.usage().expect("usage should exist").input_tokens(),
        INPUT_TOKENS
    );

    let structured = normalize_provider_response_fixture(structured_response_fixture(
        PAYLOAD_LABEL,
        Some(CanonicalFinishReason::Stop.as_str()),
    ))
    .expect("structured fixture should normalize");
    assert!(matches!(
        structured.items()[0],
        CanonicalModelResponseItem::StructuredPayload { .. }
    ));

    let tool = normalize_provider_response_fixture(tool_response_fixture(
        TOOL_CALL_ID,
        TOOL_NAME,
        ARGUMENTS_LABEL,
        Some(CanonicalFinishReason::ToolCall.as_str()),
    ))
    .expect("tool-call fixture should normalize");
    assert!(matches!(
        tool.items()[0],
        CanonicalModelResponseItem::ToolCall { .. }
    ));

    let refusal = normalize_provider_response_fixture(refusal_response_fixture(
        REFUSAL_REASON,
        Some(CanonicalFinishReason::Refusal.as_str()),
    ))
    .expect("refusal fixture should normalize");
    assert!(matches!(
        refusal.items()[0],
        CanonicalModelResponseItem::Refusal { .. }
    ));

    let safety = normalize_provider_response_fixture(ProviderResponseFixtureDraft {
        safety: Some(SAFETY_CATEGORY),
        ..base_response_fixture(Some(CanonicalFinishReason::Safety.as_str()))
    })
    .expect("safety fixture should normalize");
    assert!(matches!(
        safety.items()[0],
        CanonicalModelResponseItem::Safety { .. }
    ));

    let malformed = normalize_provider_response_fixture(ProviderResponseFixtureDraft {
        malformed_output: Some(MALFORMED_DIAGNOSTIC),
        ..base_response_fixture(Some(CanonicalFinishReason::Malformed.as_str()))
    })
    .expect("malformed fixture should normalize");
    assert!(matches!(
        malformed.items()[0],
        CanonicalModelResponseItem::MalformedOutput { .. }
    ));
}

#[test]
fn response_normalization_fails_closed_for_malformed_and_leaky_fixtures() {
    // r[verify agent_harness_runtime.provider_projection_conformance.response_fail_closed]
    let missing_finish =
        normalize_provider_response_fixture(text_response_fixture(RESPONSE_TEXT, None))
            .expect_err("missing finish should fail");
    assert!(matches!(
        missing_finish,
        ProviderProjectionError::CanonicalContract { .. }
    ));

    let malformed_usage = normalize_provider_response_fixture(ProviderResponseFixtureDraft {
        usage: Some(CanonicalUsageDraft {
            input: 0,
            output: 0,
            cached_input: None,
        }),
        ..text_response_fixture(RESPONSE_TEXT, Some(CanonicalFinishReason::Stop.as_str()))
    })
    .expect_err("malformed usage should fail");
    assert!(matches!(
        malformed_usage,
        ProviderProjectionError::CanonicalContract { .. }
    ));

    let impossible = normalize_provider_response_fixture(ProviderResponseFixtureDraft {
        text: Some(RESPONSE_TEXT),
        refusal: Some(REFUSAL_REASON),
        ..base_response_fixture(Some(CanonicalFinishReason::Refusal.as_str()))
    })
    .expect_err("text plus refusal should fail");
    assert!(matches!(
        impossible,
        ProviderProjectionError::CanonicalContract { .. }
    ));

    for draft in [
        ProviderResponseFixtureDraft {
            raw_provider_payload: Some(RAW_PAYLOAD),
            ..base_response_fixture(Some(CanonicalFinishReason::Stop.as_str()))
        },
        ProviderResponseFixtureDraft {
            hidden_reasoning: Some(HIDDEN_REASONING),
            ..base_response_fixture(Some(CanonicalFinishReason::Stop.as_str()))
        },
        text_response_fixture(
            CREDENTIAL_SENTINEL,
            Some(CanonicalFinishReason::Stop.as_str()),
        ),
    ] {
        let error = normalize_provider_response_fixture(draft).expect_err("leak should fail");
        assert!(matches!(
            error,
            ProviderProjectionError::CanonicalContract { .. }
        ));
    }
}

#[test]
fn streaming_fixtures_use_the_production_parser_entrypoint() {
    // r[verify agent_harness_runtime.provider_projection_conformance.streaming_entrypoint]
    let chunks = [
        "content|call-1|1|he",
        "llo\n",
        "tool|call-1|2|tool-1|arguments-hash\n",
        "usage|call-1|3|12|8\n",
        "finish|call-1|4\n",
    ];
    let expected = [
        wire::parts::Kind::ContentDelta,
        wire::parts::Kind::ToolCallDelta,
        wire::parts::Kind::Usage,
        wire::parts::Kind::Finish,
    ];
    let frames = assert_provider_stream_fixture(&chunks, &expected)
        .expect("stream fixture should parse through production entrypoint");
    assert_eq!(frames.len(), expected.len());
    assert_eq!(frames[1].call_id().as_str(), STREAM_CALL_ID);
    assert_eq!(
        frames[1]
            .tool_call_id()
            .expect("tool id should exist")
            .as_str(),
        STREAM_TOOL_ID
    );

    let cancelled = assert_provider_stream_fixture(
        &[
            "content|call-1|1|partial\n",
            "cancelled|call-1|2|user_cancelled\n",
        ],
        &[
            wire::parts::Kind::ContentDelta,
            wire::parts::Kind::Cancelled,
        ],
    )
    .expect("cancelled stream fixture should parse");
    assert_eq!(cancelled[1].kind(), wire::parts::Kind::Cancelled);

    let failed = assert_provider_stream_fixture(
        &["failure|call-1|1|provider_error\n"],
        &[wire::parts::Kind::Failure],
    )
    .expect("failure stream fixture should parse");
    assert_eq!(failed[0].kind(), wire::parts::Kind::Failure);
}

#[test]
fn streaming_fixtures_reject_malformed_or_unexpected_order() {
    // r[verify agent_harness_runtime.provider_projection_conformance.streaming_fail_closed]
    let malformed = assert_provider_stream_fixture(
        &["tool|only-two-fields\n"],
        &[wire::parts::Kind::ToolCallDelta],
    )
    .expect_err("malformed stream chunk should fail");
    assert!(matches!(
        malformed,
        ProviderProjectionError::StreamFixture { .. }
    ));

    let truncated = assert_provider_stream_fixture(
        &["content|call-1|1|partial"],
        &[wire::parts::Kind::ContentDelta],
    )
    .expect_err("truncated stream chunk should fail");
    assert!(matches!(
        truncated,
        ProviderProjectionError::StreamFixture { .. }
    ));

    let missing_terminal = assert_provider_stream_fixture(
        &["content|call-1|1|partial\n"],
        &[wire::parts::Kind::ContentDelta],
    )
    .expect_err("stream without terminal frame should fail");
    assert!(matches!(
        missing_terminal,
        ProviderProjectionError::StreamFixture { .. }
    ));

    let unexpected_order = assert_provider_stream_fixture(
        &["finish|call-1|1\n", "content|call-1|2|late\n"],
        &[wire::parts::Kind::Finish, wire::parts::Kind::ContentDelta],
    )
    .expect_err("late content after finish should fail");
    assert!(matches!(
        unexpected_order,
        ProviderProjectionError::StreamFixture { .. }
    ));
}

#[test]
fn conformance_index_requires_evidence_or_reason_by_status() {
    // r[verify agent_harness_runtime.provider_projection_conformance.index]
    let entries = validate_provider_projection_index(&[
        ProviderProjectionIndexEntryDraft {
            family: ProviderProjectionFamily::OpenAiResponses.as_str(),
            feature: INDEX_FEATURE,
            status: ProviderProjectionIndexStatus::Covered,
            evidence: Some(INDEX_EVIDENCE),
            reason: None,
        },
        ProviderProjectionIndexEntryDraft {
            family: ProviderProjectionFamily::AnthropicMessages.as_str(),
            feature: "image",
            status: ProviderProjectionIndexStatus::Unsupported,
            evidence: None,
            reason: Some(INDEX_REASON),
        },
        ProviderProjectionIndexEntryDraft {
            family: ProviderProjectionFamily::Mock.as_str(),
            feature: "native billing hooks",
            status: ProviderProjectionIndexStatus::NotApplicable,
            evidence: None,
            reason: Some("mock provider does not bill"),
        },
    ])
    .expect("complete index entries should validate");
    assert_eq!(entries[0].feature(), INDEX_FEATURE);
    assert_eq!(
        entries[1].status(),
        ProviderProjectionIndexStatus::Unsupported
    );

    let missing_evidence =
        validate_provider_projection_index(&[ProviderProjectionIndexEntryDraft {
            family: ProviderProjectionFamily::OpenAiResponses.as_str(),
            feature: INDEX_FEATURE,
            status: ProviderProjectionIndexStatus::Covered,
            evidence: None,
            reason: None,
        }])
        .expect_err("covered case without evidence should fail");
    assert!(matches!(
        missing_evidence,
        ProviderProjectionError::MissingIndexField { .. }
    ));

    let missing_reason = validate_provider_projection_index(&[ProviderProjectionIndexEntryDraft {
        family: ProviderProjectionFamily::OpenAiChat.as_str(),
        feature: "image",
        status: ProviderProjectionIndexStatus::Unsupported,
        evidence: None,
        reason: None,
    }])
    .expect_err("unsupported case without reason should fail");
    assert!(matches!(
        missing_reason,
        ProviderProjectionError::MissingIndexField { .. }
    ));
}

fn canonical_projection_request() -> CanonicalModelRequest {
    let developer_content = [CanonicalContentPartDraft::Text {
        text: DEVELOPER_TEXT,
    }];
    let user_content = [
        CanonicalContentPartDraft::Text { text: USER_TEXT },
        CanonicalContentPartDraft::ImageDescriptor {
            descriptor_id: IMAGE_DESCRIPTOR,
            media_type: IMAGE_MEDIA_TYPE,
            content_hash: IMAGE_HASH,
        },
    ];
    let messages = [
        CanonicalModelMessageDraft {
            role: "developer",
            content: &developer_content,
            tool_call_id: None,
        },
        CanonicalModelMessageDraft {
            role: "user",
            content: &user_content,
            tool_call_id: None,
        },
    ];
    let tools = [CanonicalToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let provenance = [CanonicalSafeMetadataDraft {
        key: "trace_label",
        value: "projection-request",
    }];
    CanonicalModelRequest::from_draft(CanonicalModelRequestDraft {
        contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
        messages: &messages,
        tools: &tools,
        output: CanonicalOutputControlsDraft {
            max_output_tokens: None,
            temperature_millis: None,
            stop_sequences: &[],
            stream: true,
        },
        response_format: CanonicalResponseFormatIntentDraft::JsonSchema {
            schema_label: RESPONSE_SCHEMA,
        },
        reasoning: CanonicalReasoningRequestDraft {
            enabled: true,
            effort: CanonicalReasoningEffort::High,
            summary_label: Some("summary-label"),
        },
        provenance: &provenance,
    })
    .expect("canonical request fixture should validate")
}

fn compatibility_output() -> CanonicalOutputControlsDraft<'static> {
    CanonicalOutputControlsDraft {
        max_output_tokens: None,
        temperature_millis: None,
        stop_sequences: &[],
        stream: false,
    }
}

fn compatibility_reasoning() -> CanonicalReasoningRequestDraft<'static> {
    CanonicalReasoningRequestDraft {
        enabled: false,
        effort: CanonicalReasoningEffort::None,
        summary_label: None,
    }
}

fn full_support(family: ProviderProjectionFamily) -> ProviderProjectionSupport {
    ProviderProjectionSupport {
        family,
        developer_role: PRESERVED_FEATURE,
        images: PRESERVED_FEATURE,
        tools: PRESERVED_FEATURE,
        json_object: PRESERVED_FEATURE,
        json_schema: PRESERVED_FEATURE,
        reasoning: PRESERVED_FEATURE,
        streaming: PRESERVED_FEATURE,
        metadata: PRESERVED_FEATURE,
    }
}

fn expected_literals(
    family: ProviderProjectionFamily,
) -> [alloc::string::String; EXPECTED_LITERAL_COUNT] {
    [
        alloc::format!("family:{}", family.as_str()),
        alloc::string::String::from("role:developer"),
        alloc::format!("text:{DEVELOPER_TEXT}"),
        alloc::string::String::from("role:user"),
        alloc::format!("text:{USER_TEXT}"),
        alloc::string::String::from("image:descriptor"),
        alloc::format!("tool:{TOOL_NAME}"),
        alloc::format!("response:{RESPONSE_SCHEMA}"),
        alloc::string::String::from("reasoning:high"),
        alloc::string::String::from("stream:true"),
        alloc::string::String::from("metadata:trace_label=projection-request"),
    ]
}

fn base_response_fixture(finish_reason: Option<&str>) -> ProviderResponseFixtureDraft<'_> {
    ProviderResponseFixtureDraft {
        family: ProviderProjectionFamily::OpenAiResponses.as_str(),
        text: None,
        structured_payload: None,
        tool_call_id: None,
        tool_name: None,
        tool_arguments: None,
        refusal: None,
        safety: None,
        malformed_output: None,
        usage: Some(CanonicalUsageDraft {
            input: INPUT_TOKENS,
            output: OUTPUT_TOKENS,
            cached_input: Some(CACHED_INPUT_TOKENS),
        }),
        finish_reason,
        raw_provider_payload: None,
        hidden_reasoning: None,
    }
}

fn text_response_fixture(
    text: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        text: Some(text),
        ..base_response_fixture(finish_reason)
    }
}

fn structured_response_fixture(
    payload_label: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        structured_payload: Some(payload_label),
        ..base_response_fixture(finish_reason)
    }
}

fn tool_response_fixture(
    call_id: &'static str,
    tool_name: &'static str,
    arguments_label: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        tool_call_id: Some(call_id),
        tool_name: Some(tool_name),
        tool_arguments: Some(arguments_label),
        ..base_response_fixture(finish_reason)
    }
}

fn refusal_response_fixture(
    reason: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        refusal: Some(reason),
        ..base_response_fixture(finish_reason)
    }
}
