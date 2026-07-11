/// Encodes one validated adapter record as Preserves text.
///
/// # Errors
///
/// Returns [`crate::Error`] when validation or serialization fails.
pub fn to_record_text(record: &crate::AdapterRecord) -> Result<String, crate::Error> {
    record.validate()?;
    encode_text(record)
}

/// Decodes and revalidates one adapter record from Preserves text.
///
/// # Errors
///
/// Returns [`crate::Error`] when syntax, schema, metadata, or the owning DTO is invalid.
pub fn from_record_text(text: &str) -> Result<crate::AdapterRecord, crate::Error> {
    let record = preserves::de::from_text(text).map_err(|_| crate::Error::MalformedText)?;
    crate::AdapterRecord::from_decoded(record)
}

/// Encodes one adapter message as a versioned Preserves text record.
///
/// # Errors
///
/// Returns [`crate::Error`] when validation or serialization fails.
pub fn to_text(message: &crate::AdapterMessage) -> Result<String, crate::Error> {
    to_record_text(&crate::AdapterRecord::from_message(message))
}

/// Decodes one adapter message from a versioned Preserves text record.
///
/// # Errors
///
/// Returns [`crate::Error`] when syntax, schema, metadata, or the owning DTO is invalid.
pub fn from_text(text: &str) -> Result<crate::AdapterMessage, crate::Error> {
    Ok(from_record_text(text)?.into_payload())
}

/// Encodes one validated adapter record as Preserves binary.
///
/// # Errors
///
/// Returns [`crate::Error`] when validation or serialization fails.
pub fn to_record_bytes(record: &crate::AdapterRecord) -> Result<Vec<u8>, crate::Error> {
    record.validate()?;
    encode_bytes(record)
}

/// Decodes and revalidates one adapter record from Preserves binary.
///
/// # Errors
///
/// Returns [`crate::Error`] when syntax, schema, metadata, or the owning DTO is invalid.
pub fn from_record_bytes(bytes: &[u8]) -> Result<crate::AdapterRecord, crate::Error> {
    let record = preserves::de::from_bytes(bytes).map_err(|_| crate::Error::MalformedBinary)?;
    crate::AdapterRecord::from_decoded(record)
}

/// Encodes one adapter message as a versioned Preserves binary record.
///
/// # Errors
///
/// Returns [`crate::Error`] when validation or serialization fails.
pub fn to_bytes(message: &crate::AdapterMessage) -> Result<Vec<u8>, crate::Error> {
    to_record_bytes(&crate::AdapterRecord::from_message(message))
}

/// Decodes one adapter message from a versioned Preserves binary record.
///
/// # Errors
///
/// Returns [`crate::Error`] when syntax, schema, metadata, or the owning DTO is invalid.
pub fn from_bytes(bytes: &[u8]) -> Result<crate::AdapterMessage, crate::Error> {
    Ok(from_record_bytes(bytes)?.into_payload())
}

fn encode_text<T: serde::Serialize>(payload: &T) -> Result<String, crate::Error> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::TextWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload).map_err(|_| crate::Error::Serialization)?;
    String::from_utf8(buffer).map_err(|_| crate::Error::Serialization)
}

fn encode_bytes<T: serde::Serialize>(payload: &T) -> Result<Vec<u8>, crate::Error> {
    let mut buffer = Vec::new();
    let mut writer = preserves::value::PackedWriter::new(&mut buffer);
    preserves::ser::to_writer(&mut writer, payload).map_err(|_| crate::Error::Serialization)?;
    Ok(buffer)
}
