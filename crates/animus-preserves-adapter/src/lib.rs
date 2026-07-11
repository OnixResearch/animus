//! Minimal Preserves boundary for Animus adapter DTOs.
//!
//! The crate depends on adapter contracts and shared schema primitives, but not
//! `animus-runtime`. Decoding always re-runs the owning adapter DTO validators.

mod codec;
mod error;
mod message;
mod metadata;
mod record;

pub use animus_preserves_schema::{CoverageTier, SchemaFamily, SchemaIdentity, WireStatus};
pub use codec::{
    from_bytes, from_record_bytes, from_record_text, from_text, to_bytes, to_record_bytes,
    to_record_text, to_text,
};
pub use error::Error;
pub use message::{
    AdapterKind, AdapterMessage, ToolArgumentRecord, ToolRequestRecord, ToolResponseRecord,
};
pub use metadata::CompatibilityField;
pub use record::AdapterRecord;

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const MODEL_RESPONSE: &str = "bounded response";
    const TOOL_NAME: &str = "inspect";
    const TOOL_SCHEMA: &str = "animus.tool.inspect.v1";
    const TOOL_ARGUMENT_NAME: &str = "target";
    const TOOL_ARGUMENT_VALUE: &str = "workspace";
    const TOOL_CALL_ID: &str = "call-7";
    const TOOL_MUTATION_TARGET: &str = "workspace.read";
    const TOOL_CAPABILITY: &str = "tool.inspect";
    const TOOL_OUTPUT: &str = "ok";
    const SAFE_METADATA_NAME: &str = "trace_label";
    const SAFE_METADATA_VALUE: &str = "adapter-roundtrip";
    const PRIVATE_TOKEN: &str = "super-private-token";
    const PRIVATE_PATH: &str = "/home/example/private.txt";
    const PROVIDER_JSON: &str = "{\"secret\":true}";
    const MALFORMED_TEXT: &str = "<AdapterRecord";
    const MISSING_PAYLOAD_TEXT: &str = "<AdapterRecord <SchemaIdentity <adapter_message> 1> []>";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;
    const TOOL_VISIBLE_BUDGET: u32 = 128;
    const TOO_MANY_METADATA_FIELDS: usize = 9;

    fn model_request() -> animus_adapters::ModelRequest {
        animus_adapters::ModelRequest::new(MODEL_PROMPT).expect("model request should validate")
    }

    fn model_response() -> animus_adapters::ModelResponse {
        animus_adapters::ModelResponse::new(MODEL_RESPONSE).expect("model response should validate")
    }

    fn tool_request() -> animus_adapters::ToolRequest {
        let fields = [animus_adapters::tool::ArgumentFieldDraft {
            name: Some(TOOL_ARGUMENT_NAME),
            value: Some(TOOL_ARGUMENT_VALUE),
        }];
        let arguments = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TOOL_SCHEMA,
                allowed_field_names: &[TOOL_ARGUMENT_NAME],
                compatibility_rules: &[],
                fields: &fields,
            },
        )
        .expect("tool arguments should validate");
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(TOOL_NAME),
            arguments: Some(&arguments),
            correlation_id: Some(TOOL_CALL_ID),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_VISIBLE_BUDGET,
            },
            mutation_targets: &[TOOL_MUTATION_TARGET],
            required_capability_labels: &[TOOL_CAPABILITY],
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
        })
        .expect("tool request should validate")
    }

    fn tool_response() -> animus_adapters::ToolResponse {
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: TOOL_OUTPUT,
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            animus_adapters::tool::OutputPolicy {
                visible_char_budget: TOOL_VISIBLE_BUDGET,
            },
        )
        .expect("tool output should validate");
        animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        )
    }

    fn stable_messages() -> Vec<AdapterMessage> {
        vec![
            AdapterMessage::from_model_request(&model_request()),
            AdapterMessage::from_model_response(&model_response()),
            AdapterMessage::from_tool_request(&tool_request()),
            AdapterMessage::from_tool_response(&tool_response()),
        ]
    }

    fn encode_unchecked_text(record: &AdapterRecord) -> String {
        let mut buffer = Vec::new();
        let mut writer = preserves::value::TextWriter::new(&mut buffer);
        preserves::ser::to_writer(&mut writer, record).expect("fixture should serialize");
        String::from_utf8(buffer).expect("Preserves text must be UTF-8")
    }

    #[test]
    fn stable_adapter_families_round_trip_through_text_and_binary() {
        for message in stable_messages() {
            let text = to_text(&message).expect("adapter message should encode as text");
            let bytes = to_bytes(&message).expect("adapter message should encode as binary");

            assert_eq!(from_text(&text).expect("text should decode"), message);
            assert_eq!(from_bytes(&bytes).expect("binary should decode"), message);
        }
    }

    #[test]
    fn decoded_payloads_reconstruct_owning_adapter_dtos() {
        let decoded_model = from_text(
            &to_text(&AdapterMessage::from_model_request(&model_request()))
                .expect("model request should encode"),
        )
        .expect("model request should decode")
        .try_into_model_request()
        .expect("model request should revalidate");
        let decoded_tool = from_bytes(
            &to_bytes(&AdapterMessage::from_tool_request(&tool_request()))
                .expect("tool request should encode"),
        )
        .expect("tool request should decode")
        .try_into_tool_request()
        .expect("tool request should revalidate");

        assert_eq!(decoded_model, model_request());
        assert_eq!(decoded_tool, tool_request());
    }

    #[test]
    fn safe_compatibility_metadata_survives_round_trip() {
        let metadata = CompatibilityField::new(SAFE_METADATA_NAME, SAFE_METADATA_VALUE)
            .expect("safe metadata should validate");
        let record = AdapterRecord::with_metadata(
            AdapterMessage::from_model_response(&model_response()),
            vec![metadata.clone()],
        )
        .expect("record should validate");
        let decoded =
            from_record_text(&to_record_text(&record).expect("record should encode with metadata"))
                .expect("record should decode with metadata");

        assert_eq!(decoded.metadata(), &[metadata]);
    }

    #[test]
    fn malformed_missing_unsupported_wrong_family_and_invalid_dto_fail_closed() {
        let unsupported = AdapterRecord::unchecked(
            SchemaIdentity::new(SchemaFamily::AdapterMessage, UNSUPPORTED_SCHEMA_VERSION),
            Vec::new(),
            AdapterMessage::from_model_request(&model_request()),
        );
        let wrong_family = AdapterRecord::unchecked(
            SchemaIdentity::current(SchemaFamily::RuntimeEvent),
            Vec::new(),
            AdapterMessage::from_model_request(&model_request()),
        );
        let invalid_dto = AdapterRecord::unchecked(
            SchemaIdentity::current(SchemaFamily::AdapterMessage),
            Vec::new(),
            AdapterMessage::ModelRequest {
                prompt: String::new(),
            },
        );

        assert_eq!(from_record_text(MALFORMED_TEXT), Err(Error::MalformedText));
        assert_eq!(
            from_record_text(MISSING_PAYLOAD_TEXT),
            Err(Error::MalformedText)
        );
        assert!(matches!(
            from_record_text(&encode_unchecked_text(&unsupported)),
            Err(Error::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            from_record_text(&encode_unchecked_text(&wrong_family)),
            Err(Error::WrongSchemaFamily {
                actual: "runtime_event"
            })
        ));
        assert!(matches!(
            from_record_text(&encode_unchecked_text(&invalid_dto)),
            Err(Error::InvalidAdapterPayload {
                kind: "model_request",
                ..
            })
        ));
    }

    #[test]
    fn unsafe_unbounded_and_leaking_metadata_fail_without_echoing_values() {
        let raw_prompt = CompatibilityField::new("raw_prompt", PRIVATE_TOKEN)
            .expect_err("raw prompt metadata must fail");
        let provider = CompatibilityField::new("provider_payload", PROVIDER_JSON)
            .expect_err("provider JSON metadata must fail");
        let path = CompatibilityField::new("source", PRIVATE_PATH)
            .expect_err("private path metadata must fail");
        let fields: Vec<CompatibilityField> = (0..TOO_MANY_METADATA_FIELDS)
            .map(|index| {
                CompatibilityField::new(&format!("safe_{index}"), "ok")
                    .expect("bounded metadata should validate")
            })
            .collect();
        let too_many = AdapterRecord::with_metadata(
            AdapterMessage::from_model_request(&model_request()),
            fields,
        )
        .expect_err("unbounded metadata collection must fail");

        for error in [raw_prompt, provider, path, too_many] {
            let diagnostic = error.to_string();
            assert!(!diagnostic.contains(PRIVATE_TOKEN));
            assert!(!diagnostic.contains(PRIVATE_PATH));
            assert!(!diagnostic.contains(PROVIDER_JSON));
        }
    }
}
