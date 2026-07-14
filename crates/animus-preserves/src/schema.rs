pub use animus_preserves_schema::{
    CoverageTier, SchemaDescriptor, SchemaFamily, SchemaIdentity, WireStatus, schema_catalog,
};

#[cfg(test)]
const INITIAL_SCHEMA_VERSION: u16 = animus_preserves_schema::CURRENT_SCHEMA_VERSION;
const MAX_COMPATIBILITY_FIELDS: usize = 8;
const MAX_COMPATIBILITY_FIELD_NAME_BYTES: usize = 64;
const MAX_COMPATIBILITY_FIELD_VALUE_BYTES: usize = 256;
const EMPTY_COMPATIBILITY_FIELD_REASON: &str =
    "compatibility metadata field name must be non-empty";
const INVALID_COMPATIBILITY_FIELD_NAME_REASON: &str =
    "compatibility metadata field name must use safe identifier characters";
const UNSAFE_COMPATIBILITY_FIELD_REASON: &str =
    "compatibility metadata field is not safe to preserve across boundaries";
const UNBOUNDED_COMPATIBILITY_FIELD_REASON: &str =
    "compatibility metadata exceeds the bounded preservation limit";
const PRIVATE_PATH_COMPATIBILITY_FIELD_REASON: &str =
    "compatibility metadata must not preserve full private paths";
const PROVIDER_JSON_COMPATIBILITY_FIELD_REASON: &str =
    "compatibility metadata must not preserve provider payload JSON";
#[cfg(test)]
const SCHEMA_CATALOG_LEN: usize = 9;
const RAW_PROMPT_FIELD: &str = "raw_prompt";
const PROMPT_FIELD: &str = "prompt";
const CREDENTIAL_FIELD: &str = "credential";
const PROVIDER_JSON_FIELD: &str = "provider_json";
const FULL_PATH_FIELD: &str = "full_path";
const FILE_BODY_FIELD: &str = "file_body";
const TOOL_INPUT_FIELD: &str = "tool_input";
const TOOL_OUTPUT_FIELD: &str = "tool_output";
const METADATA_FIELD_LABEL: &str = "metadata";
const PRIVATE_HOME_PATH_MARKER: &str = "/home/";
const PRIVATE_USERS_PATH_MARKER: &str = "/Users/";
const WINDOWS_DRIVE_PATH_MARKER: &str = ":\\";
const JSON_OBJECT_START: char = '{';
const JSON_ARRAY_START: char = '[';
const ASCII_LOWERCASE_A: char = 'a';
const ASCII_LOWERCASE_Z: char = 'z';
const ASCII_UPPERCASE_A: char = 'A';
const ASCII_UPPERCASE_Z: char = 'Z';
const ASCII_DIGIT_ZERO: char = '0';
const ASCII_DIGIT_NINE: char = '9';
const SAFE_FIELD_DASH: char = '-';
const SAFE_FIELD_UNDERSCORE: char = '_';
const SAFE_FIELD_DOT: char = '.';
const UNSAFE_FAMILY_VALUE_REASON: &str = "family payload must not preserve raw prompts, provider payloads, credentials, file bodies, tool bodies, or unbounded output";
const UNSAFE_FAMILY_VALUE_MARKERS: &[&str] = &[
    "raw_prompt",
    "provider_payload",
    "credential",
    "file_body",
    "tool_input",
    "tool_output",
    "model_output",
    "unbounded_output",
    "unbounded_log",
    "begin private key",
    "api_key=",
];

/// Safe bounded compatibility metadata preserved with a versioned boundary record.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityField {
    name: String,
    value: String,
}

impl CompatibilityField {
    /// Builds a compatibility metadata field after validating safe bounded content.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the field name or value is unsafe to preserve.
    pub fn new(name: &str, value: &str) -> Result<Self, crate::CodecError> {
        let field = Self {
            name: String::from(name),
            value: String::from(value),
        };
        field.validate()?;
        Ok(field)
    }

    /// Returns the metadata field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the metadata field value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(&self) -> Result<(), crate::CodecError> {
        validate_field_name(&self.name)?;
        validate_field_value(&self.value)
    }
}

/// Versioned Preserves record for stable external boundary messages.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryRecord {
    schema: SchemaIdentity,
    metadata: Vec<CompatibilityField>,
    payload: crate::BoundaryMessage,
}

impl BoundaryRecord {
    /// Builds a versioned record with no compatibility metadata.
    #[must_use]
    pub fn from_message(payload: &crate::BoundaryMessage) -> Self {
        Self {
            schema: schema_identity_for_message(payload),
            metadata: Vec::new(),
            payload: payload.clone(),
        }
    }

    /// Builds a versioned record with validated compatibility metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the schema or metadata is unsafe.
    pub fn with_metadata(
        payload: crate::BoundaryMessage,
        metadata: Vec<CompatibilityField>,
    ) -> Result<Self, crate::CodecError> {
        let record = Self {
            schema: schema_identity_for_message(&payload),
            metadata,
            payload,
        };
        record.validate()?;
        Ok(record)
    }

    /// Returns the declared schema identity.
    #[must_use]
    pub const fn schema(&self) -> SchemaIdentity {
        self.schema
    }

    /// Returns preserved safe compatibility metadata.
    #[must_use]
    pub fn metadata(&self) -> &[CompatibilityField] {
        &self.metadata
    }

    /// Returns the typed boundary payload.
    #[must_use]
    pub const fn payload(&self) -> &crate::BoundaryMessage {
        &self.payload
    }

    /// Consumes the record and returns the typed boundary payload.
    #[must_use]
    pub fn into_payload(self) -> crate::BoundaryMessage {
        self.payload
    }

    /// Validates schema version, schema family, and compatibility metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the record is unsupported or unsafe.
    pub fn validate(&self) -> Result<(), crate::CodecError> {
        validate_schema_identity(self.schema)?;
        validate_schema_family(self.schema.family(), &self.payload)?;
        validate_metadata(&self.metadata)
    }

    pub(crate) fn from_decoded(record: Self) -> Result<Self, crate::CodecError> {
        record.validate()?;
        Ok(record)
    }
}

pub(crate) fn validate_schema_identity(schema: SchemaIdentity) -> Result<(), crate::CodecError> {
    schema
        .validate_supported()
        .map_err(|error| crate::CodecError::UnsupportedSchemaVersion {
            family: error.family().as_str(),
            version: error.version(),
            supported: error.supported(),
        })
}

/// Returns the current schema identity for a typed boundary message.
#[must_use]
pub fn schema_identity_for_message(message: &crate::BoundaryMessage) -> SchemaIdentity {
    SchemaIdentity::current(schema_family_for_kind(message.kind()))
}

fn schema_family_for_kind(kind: crate::BoundaryKind) -> SchemaFamily {
    match kind {
        crate::BoundaryKind::ModelRequest
        | crate::BoundaryKind::ModelResponse
        | crate::BoundaryKind::ToolRequest
        | crate::BoundaryKind::ToolResponse => SchemaFamily::AdapterMessage,
        crate::BoundaryKind::RuntimeEvent => SchemaFamily::RuntimeEvent,
        crate::BoundaryKind::RuntimeReport => SchemaFamily::RuntimeReport,
    }
}

fn validate_schema_family(
    actual: SchemaFamily,
    payload: &crate::BoundaryMessage,
) -> Result<(), crate::CodecError> {
    let expected = schema_identity_for_message(payload).family();
    if actual == expected {
        return Ok(());
    }

    Err(crate::CodecError::WrongSchemaFamily {
        expected: expected.as_str(),
        actual: actual.as_str(),
    })
}

pub(crate) fn validate_metadata(metadata: &[CompatibilityField]) -> Result<(), crate::CodecError> {
    if metadata.len() > MAX_COMPATIBILITY_FIELDS {
        return Err(crate::CodecError::TooManyCompatibilityFields {
            limit: MAX_COMPATIBILITY_FIELDS,
        });
    }

    for field in metadata {
        field.validate()?;
    }

    Ok(())
}

fn validate_field_name(name: &str) -> Result<(), crate::CodecError> {
    if name.trim().is_empty() {
        return Err(compatibility_error(EMPTY_COMPATIBILITY_FIELD_REASON));
    }

    if name.len() > MAX_COMPATIBILITY_FIELD_NAME_BYTES {
        return Err(compatibility_error(UNBOUNDED_COMPATIBILITY_FIELD_REASON));
    }

    if !name.chars().all(is_safe_field_name_char) {
        return Err(compatibility_error(INVALID_COMPATIBILITY_FIELD_NAME_REASON));
    }

    if is_unsafe_field_name(name) {
        return Err(compatibility_error(UNSAFE_COMPATIBILITY_FIELD_REASON));
    }

    Ok(())
}

pub(crate) fn validate_family_value(
    field: &'static str,
    value: &str,
) -> Result<(), crate::CodecError> {
    validate_field_value(value)?;
    let normalized = value.to_ascii_lowercase();
    if UNSAFE_FAMILY_VALUE_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        return Err(crate::CodecError::UnsafeCompatibilityField {
            field,
            reason: UNSAFE_FAMILY_VALUE_REASON,
        });
    }
    Ok(())
}

fn validate_field_value(value: &str) -> Result<(), crate::CodecError> {
    if value.len() > MAX_COMPATIBILITY_FIELD_VALUE_BYTES {
        return Err(compatibility_error(UNBOUNDED_COMPATIBILITY_FIELD_REASON));
    }

    if contains_private_path(value) {
        return Err(compatibility_error(PRIVATE_PATH_COMPATIBILITY_FIELD_REASON));
    }

    if looks_like_provider_json(value) {
        return Err(compatibility_error(
            PROVIDER_JSON_COMPATIBILITY_FIELD_REASON,
        ));
    }

    Ok(())
}

fn is_safe_field_name_char(character: char) -> bool {
    matches!(
        character,
        ASCII_LOWERCASE_A..=ASCII_LOWERCASE_Z
            | ASCII_UPPERCASE_A..=ASCII_UPPERCASE_Z
            | ASCII_DIGIT_ZERO..=ASCII_DIGIT_NINE
            | SAFE_FIELD_DASH
            | SAFE_FIELD_UNDERSCORE
            | SAFE_FIELD_DOT
    )
}

fn is_unsafe_field_name(name: &str) -> bool {
    matches!(
        name,
        RAW_PROMPT_FIELD
            | PROMPT_FIELD
            | CREDENTIAL_FIELD
            | PROVIDER_JSON_FIELD
            | FULL_PATH_FIELD
            | FILE_BODY_FIELD
            | TOOL_INPUT_FIELD
            | TOOL_OUTPUT_FIELD
    )
}

fn contains_private_path(value: &str) -> bool {
    value.contains(PRIVATE_HOME_PATH_MARKER)
        || value.contains(PRIVATE_USERS_PATH_MARKER)
        || value.contains(WINDOWS_DRIVE_PATH_MARKER)
}

fn looks_like_provider_json(value: &str) -> bool {
    let trimmed = value.trim_start();
    trimmed.starts_with(JSON_OBJECT_START) || trimmed.starts_with(JSON_ARRAY_START)
}

fn compatibility_error(reason: &'static str) -> crate::CodecError {
    crate::CodecError::UnsafeCompatibilityField {
        field: METADATA_FIELD_LABEL,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAFE_METADATA_NAME: &str = "future.trace";
    const SAFE_METADATA_VALUE: &str = "bounded-safe-metadata";
    const UNSAFE_RAW_PROMPT_VALUE: &str = "raw prompt body";
    const UNSAFE_PROVIDER_JSON_VALUE: &str = "{\"raw\":true}";
    const UNSAFE_PRIVATE_PATH_VALUE: &str = "/home/example/private.txt";
    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const TOOL_ARGUMENT_NAME: &str = "query";
    const TOOL_SCHEMA: &str = "schema.search.v1";
    const TOOL_CALL_ID: &str = "tool-call-1";
    const TOOL_VISIBLE_BUDGET: u32 = 64;
    const TOOL_MUTATION_TARGET: &str = "resource:search";
    const TOOL_CAPABILITY: &str = "tool.search";
    const TOOL_OUTPUT: &str = "result";
    const RUNTIME_SUMMARY: &str = "runtime started";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = INITIAL_SCHEMA_VERSION + 1;
    const TOO_MANY_METADATA_FIELDS: usize = MAX_COMPATIBILITY_FIELDS + 1;
    const OVERSIZED_METADATA_VALUE_BYTES: usize = MAX_COMPATIBILITY_FIELD_VALUE_BYTES + 1;
    const OVERSIZED_METADATA_FILL: char = 'x';

    fn stable_boundary_messages() -> Vec<crate::BoundaryMessage> {
        let report = animus_runtime::RunReport::new(
            vec![animus_runtime::RuntimeEvent::RuntimeStarted {
                summary: String::from(RUNTIME_SUMMARY),
            }],
            animus_runtime::RuntimeStatus::Stopped,
        );

        vec![
            crate::BoundaryMessage::ModelRequest {
                prompt: String::from(MODEL_PROMPT),
            },
            crate::BoundaryMessage::from_tool_response(&tool_response()),
            crate::BoundaryMessage::from_runtime_event(
                &animus_runtime::RuntimeEvent::ToolRequested {
                    request: tool_request(),
                },
            ),
            crate::BoundaryMessage::from_runtime_report(&report),
        ]
    }

    fn tool_request() -> animus_adapters::ToolRequest {
        let fields = [animus_adapters::tool::ArgumentFieldDraft {
            name: Some(TOOL_ARGUMENT_NAME),
            value: Some(TOOL_INPUT),
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

    fn runtime_event_record_with_schema(schema: SchemaIdentity) -> BoundaryRecord {
        BoundaryRecord {
            schema,
            metadata: Vec::new(),
            payload: crate::BoundaryMessage::RuntimeStopped,
        }
    }

    #[test]
    fn catalog_declares_boundary_coverage_tiers() {
        let catalog = schema_catalog();

        assert_eq!(catalog.len(), SCHEMA_CATALOG_LEN);
        assert_eq!(
            catalog[SchemaFamily::AdapterMessage as usize].coverage_tier(),
            CoverageTier::StableExternalWireContract
        );
        assert!(
            SchemaFamily::RuntimeEvent
                .coverage_tier()
                .requires_stable_fixtures()
        );
        assert!(
            !SchemaFamily::DatasetExportRow
                .coverage_tier()
                .requires_stable_fixtures()
        );
    }

    #[test]
    fn record_binds_payload_to_current_schema_identity() {
        for message in stable_boundary_messages() {
            let record = BoundaryRecord::from_message(&message);

            assert_eq!(record.schema(), schema_identity_for_message(&message));
            assert_eq!(record.schema().version(), INITIAL_SCHEMA_VERSION);
            record.validate().expect("schema record should validate");
        }
    }

    #[test]
    fn stable_records_round_trip_text_and_binary() {
        for message in stable_boundary_messages() {
            let record = BoundaryRecord::from_message(&message);
            let text = crate::to_record_text(&record).expect("record should encode as text");
            let decoded_text = crate::from_record_text(&text).expect("record should decode text");
            let bytes = crate::to_record_bytes(&record).expect("record should encode as bytes");
            let decoded_bytes =
                crate::from_record_bytes(&bytes).expect("record should decode bytes");

            assert_eq!(decoded_text, record);
            assert_eq!(decoded_bytes, record);
        }
    }

    #[test]
    fn safe_future_metadata_is_preserved() {
        let metadata = CompatibilityField::new(SAFE_METADATA_NAME, SAFE_METADATA_VALUE)
            .expect("safe metadata should validate");
        let record = BoundaryRecord::with_metadata(
            crate::BoundaryMessage::RuntimeStopped,
            vec![metadata.clone()],
        )
        .expect("safe metadata record should validate");
        let text = crate::to_record_text(&record).expect("record should encode as text");
        let decoded = crate::from_record_text(&text).expect("record should decode text");

        assert_eq!(decoded.metadata(), &[metadata]);
    }

    #[test]
    fn unsupported_schema_version_fails_closed() {
        let record = runtime_event_record_with_schema(SchemaIdentity::new(
            SchemaFamily::RuntimeEvent,
            UNSUPPORTED_SCHEMA_VERSION,
        ));
        let error = record
            .validate()
            .expect_err("unsupported version should fail");

        assert!(matches!(
            error,
            crate::CodecError::UnsupportedSchemaVersion {
                family: "runtime_event",
                version: UNSUPPORTED_SCHEMA_VERSION,
                supported: INITIAL_SCHEMA_VERSION,
            }
        ));
    }

    #[test]
    fn wrong_schema_family_fails_closed() {
        let record =
            runtime_event_record_with_schema(SchemaIdentity::current(SchemaFamily::AdapterMessage));
        let error = record
            .validate()
            .expect_err("wrong schema family should fail");

        assert!(matches!(
            error,
            crate::CodecError::WrongSchemaFamily {
                expected: "runtime_event",
                actual: "adapter_message",
            }
        ));
    }

    #[test]
    fn unsafe_unknown_field_name_is_rejected() {
        let error = CompatibilityField::new(RAW_PROMPT_FIELD, UNSAFE_RAW_PROMPT_VALUE)
            .expect_err("raw prompt metadata should fail");

        assert!(matches!(
            error,
            crate::CodecError::UnsafeCompatibilityField {
                field: METADATA_FIELD_LABEL,
                reason: UNSAFE_COMPATIBILITY_FIELD_REASON,
            }
        ));
    }

    #[test]
    fn unbounded_unknown_field_value_is_rejected() {
        let value: String =
            std::iter::repeat_n(OVERSIZED_METADATA_FILL, OVERSIZED_METADATA_VALUE_BYTES).collect();
        let error = CompatibilityField::new(SAFE_METADATA_NAME, &value)
            .expect_err("oversized metadata should fail");

        assert!(matches!(
            error,
            crate::CodecError::UnsafeCompatibilityField {
                field: METADATA_FIELD_LABEL,
                reason: UNBOUNDED_COMPATIBILITY_FIELD_REASON,
            }
        ));
    }

    #[test]
    fn too_many_unknown_fields_are_rejected() {
        let metadata = (0..TOO_MANY_METADATA_FIELDS)
            .map(|index| CompatibilityField {
                name: format!("future.{index}"),
                value: String::from(SAFE_METADATA_VALUE),
            })
            .collect();
        let record = BoundaryRecord {
            schema: SchemaIdentity::current(SchemaFamily::RuntimeEvent),
            metadata,
            payload: crate::BoundaryMessage::RuntimeStopped,
        };
        let error = record
            .validate()
            .expect_err("too many metadata fields should fail");

        assert!(matches!(
            error,
            crate::CodecError::TooManyCompatibilityFields {
                limit: MAX_COMPATIBILITY_FIELDS,
            }
        ));
    }

    #[test]
    fn compatibility_leak_attempts_are_rejected() {
        let provider_error =
            CompatibilityField::new(SAFE_METADATA_NAME, UNSAFE_PROVIDER_JSON_VALUE)
                .expect_err("provider JSON should fail");
        let path_error = CompatibilityField::new(SAFE_METADATA_NAME, UNSAFE_PRIVATE_PATH_VALUE)
            .expect_err("private path should fail");

        assert!(matches!(
            provider_error,
            crate::CodecError::UnsafeCompatibilityField {
                field: METADATA_FIELD_LABEL,
                reason: PROVIDER_JSON_COMPATIBILITY_FIELD_REASON,
            }
        ));
        assert!(matches!(
            path_error,
            crate::CodecError::UnsafeCompatibilityField {
                field: METADATA_FIELD_LABEL,
                reason: PRIVATE_PATH_COMPATIBILITY_FIELD_REASON,
            }
        ));
    }

    #[test]
    fn schema_errors_do_not_echo_raw_metadata_values() {
        let error = CompatibilityField::new(SAFE_METADATA_NAME, UNSAFE_PRIVATE_PATH_VALUE)
            .expect_err("private path should fail");
        let diagnostic = error.to_string();

        assert!(!diagnostic.contains(UNSAFE_PRIVATE_PATH_VALUE));
        assert!(diagnostic.contains(METADATA_FIELD_LABEL));
    }
}
