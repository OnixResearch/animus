/// Encodes one boundary message as Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves serialization fails.
pub fn to_text(message: &crate::BoundaryMessage) -> Result<String, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, message)?;

    Ok(String::from_utf8(buffer)?)
}

/// Decodes one boundary message from Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves deserialization fails.
pub fn from_text(text: &str) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(preserves::de::from_text(text)?)
}

/// Encodes one boundary message as Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves serialization fails.
pub fn to_bytes(message: &crate::BoundaryMessage) -> Result<Vec<u8>, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::PackedWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, message)?;

    Ok(buffer)
}

/// Decodes one boundary message from Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when Preserves deserialization fails.
pub fn from_bytes(bytes: &[u8]) -> Result<crate::BoundaryMessage, crate::CodecError> {
    Ok(preserves::de::from_bytes(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const INVALID_PRESERVES_TEXT: &str = "<ModelRequest";
    const INVALID_PRESERVES_BINARY: &[u8] = b"not preserves binary";

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
    fn text_decode_rejects_invalid_preserves() {
        let error = from_text(INVALID_PRESERVES_TEXT).expect_err("invalid text should fail");

        assert!(matches!(error, crate::CodecError::Preserves { .. }));
    }

    #[test]
    fn binary_decode_rejects_invalid_preserves() {
        let error = from_bytes(INVALID_PRESERVES_BINARY).expect_err("invalid binary should fail");

        assert!(matches!(error, crate::CodecError::Preserves { .. }));
    }
}
