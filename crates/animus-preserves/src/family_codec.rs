const MALFORMED_TEXT_REASON: &str = "malformed family-specific Preserves text";
const MALFORMED_BINARY_REASON: &str = "malformed family-specific Preserves binary";
const CODEC_FIELD: &str = "preserves_family_record";

pub(crate) fn encode_text<T: serde::Serialize>(payload: &T) -> Result<String, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload)?;
    Ok(String::from_utf8(buffer)?)
}

pub(crate) fn encode_bytes<T: serde::Serialize>(payload: &T) -> Result<Vec<u8>, crate::CodecError> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::PackedWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload)?;
    Ok(buffer)
}

pub(crate) fn decode_text<T>(text: &str) -> Result<T, crate::CodecError>
where
    T: serde::de::DeserializeOwned,
{
    preserves::de::from_text(text).map_err(|_| crate::CodecError::InvalidRuntimePayload {
        field: CODEC_FIELD,
        reason: MALFORMED_TEXT_REASON,
    })
}

pub(crate) fn decode_bytes<T>(bytes: &[u8]) -> Result<T, crate::CodecError>
where
    T: serde::de::DeserializeOwned,
{
    preserves::de::from_bytes(bytes).map_err(|_| crate::CodecError::InvalidRuntimePayload {
        field: CODEC_FIELD,
        reason: MALFORMED_BINARY_REASON,
    })
}
