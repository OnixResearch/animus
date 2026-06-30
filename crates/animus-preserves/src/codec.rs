/// Encodes one boundary message as Preserves text syntax.
///
/// This legacy helper is an internal DTO round trip. Use [`to_versioned_text`]
/// for stable external wire payloads.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves serialization fails.
pub fn to_text(message: &crate::BoundaryMessage) -> Result<String, crate::CodecError> {
    encode_text(message)
}

/// Decodes one boundary message from Preserves text syntax.
///
/// This legacy helper is an internal DTO round trip. Use [`from_versioned_text`]
/// for stable external wire payloads.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves deserialization fails.
pub fn from_text(text: &str) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(preserves::de::from_text(text)?)
}

/// Encodes one versioned boundary record as Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_record_text(record: &crate::BoundaryRecord) -> Result<String, crate::CodecError> {
    record.validate()?;
    encode_text(record)
}

/// Decodes one versioned boundary record from Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or metadata validation fails.
pub fn from_record_text(text: &str) -> Result<crate::BoundaryRecord, crate::CodecError> {
    let record = preserves::de::from_text(text)?;
    crate::BoundaryRecord::from_decoded(record)
}

/// Encodes one boundary message as a versioned Preserves text record.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_versioned_text(message: &crate::BoundaryMessage) -> Result<String, crate::CodecError> {
    to_record_text(&crate::BoundaryRecord::from_message(message))
}

/// Decodes one boundary message from a versioned Preserves text record.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or metadata validation fails.
pub fn from_versioned_text(text: &str) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(from_record_text(text)?.into_payload())
}

/// Encodes one boundary message as Preserves binary syntax.
///
/// This legacy helper is an internal DTO round trip. Use [`to_versioned_bytes`]
/// for stable external wire payloads.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves serialization fails.
pub fn to_bytes(message: &crate::BoundaryMessage) -> Result<Vec<u8>, crate::CodecError> {
    encode_bytes(message)
}

/// Decodes one boundary message from Preserves binary syntax.
///
/// This legacy helper is an internal DTO round trip. Use [`from_versioned_bytes`]
/// for stable external wire payloads.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves deserialization fails.
pub fn from_bytes(bytes: &[u8]) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(preserves::de::from_bytes(bytes)?)
}

/// Encodes one versioned boundary record as Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_record_bytes(record: &crate::BoundaryRecord) -> Result<Vec<u8>, crate::CodecError> {
    record.validate()?;
    encode_bytes(record)
}

/// Decodes one versioned boundary record from Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or metadata validation fails.
pub fn from_record_bytes(bytes: &[u8]) -> Result<crate::BoundaryRecord, crate::CodecError> {
    let record = preserves::de::from_bytes(bytes)?;
    crate::BoundaryRecord::from_decoded(record)
}

/// Encodes one boundary message as a versioned Preserves binary record.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_versioned_bytes(message: &crate::BoundaryMessage) -> Result<Vec<u8>, crate::CodecError> {
    to_record_bytes(&crate::BoundaryRecord::from_message(message))
}

/// Decodes one boundary message from a versioned Preserves binary record.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or metadata validation fails.
pub fn from_versioned_bytes(bytes: &[u8]) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(from_record_bytes(bytes)?.into_payload())
}

fn encode_text<T: serde::Serialize>(payload: &T) -> Result<String, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload)?;

    Ok(String::from_utf8(buffer)?)
}

fn encode_bytes<T: serde::Serialize>(payload: &T) -> Result<Vec<u8>, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::PackedWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const INVALID_PRESERVES_TEXT: &str = "<ModelRequest";
    const INVALID_PRESERVES_BINARY: &[u8] = b"not preserves binary";
    const MISSING_PAYLOAD_RECORD_TEXT: &str =
        "<BoundaryRecord <SchemaIdentity <runtime_event> 1> []>";
    const UNSUPPORTED_VERSION_RECORD_TEXT: &str =
        "<BoundaryRecord <SchemaIdentity <runtime_event> 2> [] <RuntimeStopped>>";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;

    #[test]
    fn text_round_trips_message() {
        let message = crate::BoundaryMessage::ModelRequest {
            prompt: String::from(MODEL_PROMPT),
        };
        let text = to_text(&message).expect("message should encode as text");
        let decoded = from_text(&text).expect("message should decode from text");

        assert_eq!(decoded, message);
    }

    #[test]
    fn binary_round_trips_message() {
        let message = crate::BoundaryMessage::ModelRequest {
            prompt: String::from(MODEL_PROMPT),
        };
        let bytes = to_bytes(&message).expect("message should encode as binary");
        let decoded = from_bytes(&bytes).expect("message should decode from binary");

        assert_eq!(decoded, message);
    }

    #[test]
    fn versioned_text_round_trips_message() {
        let message = crate::BoundaryMessage::ModelRequest {
            prompt: String::from(MODEL_PROMPT),
        };
        let text = to_versioned_text(&message).expect("record should encode as text");
        let decoded = from_versioned_text(&text).expect("record should decode from text");

        assert_eq!(decoded, message);
    }

    #[test]
    fn versioned_binary_round_trips_message() {
        let message = crate::BoundaryMessage::ModelRequest {
            prompt: String::from(MODEL_PROMPT),
        };
        let bytes = to_versioned_bytes(&message).expect("record should encode as binary");
        let decoded = from_versioned_bytes(&bytes).expect("record should decode from binary");

        assert_eq!(decoded, message);
    }

    #[test]
    fn text_decode_rejects_invalid_preserves() {
        let error = from_text(INVALID_PRESERVES_TEXT).expect_err("invalid text should fail");

        assert!(matches!(error, crate::CodecError::Preserves { .. }));
    }

    #[test]
    fn binary_decode_rejects_invalid_preserves() {
        let error = from_bytes(INVALID_PRESERVES_BINARY).expect_err("invalid binary should fail");

        assert!(matches!(error, crate::CodecError::Preserves { .. }));
    }

    #[test]
    fn versioned_text_decode_rejects_missing_payload() {
        let error =
            from_record_text(MISSING_PAYLOAD_RECORD_TEXT).expect_err("missing payload should fail");

        assert!(matches!(error, crate::CodecError::Preserves { .. }));
    }

    #[test]
    fn versioned_text_decode_rejects_unsupported_version() {
        let error = from_record_text(UNSUPPORTED_VERSION_RECORD_TEXT)
            .expect_err("unsupported version should fail");

        assert!(matches!(
            error,
            crate::CodecError::UnsupportedSchemaVersion {
                family: "runtime_event",
                version: UNSUPPORTED_SCHEMA_VERSION,
                ..
            }
        ));
    }
}
