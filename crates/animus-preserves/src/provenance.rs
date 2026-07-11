const FAMILY: crate::SchemaFamily = crate::SchemaFamily::SessionProvenanceRecord;
const SESSION_LABEL_FIELD: &str = "session_label";
const SESSION_HASH_FIELD: &str = "session_hash";
const RECEIPT_HASH_FIELD: &str = "receipt_hash";

/// Versioned compatibility record for safe session identity and provenance hashes.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionProvenanceRecord {
    schema: crate::SchemaIdentity,
    session_label: String,
    session_hash: String,
    receipt_hash: String,
    metadata: Vec<crate::CompatibilityField>,
}

impl SessionProvenanceRecord {
    /// Builds a compatibility record from an owning runtime session DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when a safe field or metadata value is invalid.
    pub fn from_session_identity(
        identity: &animus_runtime::session_persistence::SessionIdentity,
        receipt_hash: &str,
        metadata: Vec<crate::CompatibilityField>,
    ) -> Result<Self, crate::CodecError> {
        let record = Self {
            schema: crate::SchemaIdentity::current(FAMILY),
            session_label: identity.session_label.clone(),
            session_hash: identity.session_hash.clone(),
            receipt_hash: String::from(receipt_hash),
            metadata,
        };
        record.validate()?;
        Ok(record)
    }

    /// Returns the schema identity.
    #[must_use]
    pub const fn schema(&self) -> crate::SchemaIdentity {
        self.schema
    }

    /// Returns the safe session label.
    #[must_use]
    pub fn session_label(&self) -> &str {
        &self.session_label
    }

    /// Returns the safe session hash.
    #[must_use]
    pub fn session_hash(&self) -> &str {
        &self.session_hash
    }

    /// Returns the safe provenance receipt hash.
    #[must_use]
    pub fn receipt_hash(&self) -> &str {
        &self.receipt_hash
    }

    /// Returns bounded compatibility metadata.
    #[must_use]
    pub fn metadata(&self) -> &[crate::CompatibilityField] {
        &self.metadata
    }

    /// Reconstructs the owning runtime session DTO after revalidation.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the record is unsupported or unsafe.
    pub fn try_into_session_identity(
        &self,
    ) -> Result<animus_runtime::session_persistence::SessionIdentity, crate::CodecError> {
        self.validate()?;
        Ok(animus_runtime::session_persistence::SessionIdentity {
            session_label: self.session_label.clone(),
            session_hash: self.session_hash.clone(),
        })
    }

    /// Validates schema identity, safe fields, and compatibility metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the record is unsupported or unsafe.
    pub fn validate(&self) -> Result<(), crate::CodecError> {
        crate::schema::validate_schema_identity(self.schema)?;
        if self.schema.family() != FAMILY {
            return Err(crate::CodecError::WrongSchemaFamily {
                expected: FAMILY.as_str(),
                actual: self.schema.family().as_str(),
            });
        }
        crate::schema::validate_family_value(SESSION_LABEL_FIELD, &self.session_label)?;
        crate::schema::validate_family_value(SESSION_HASH_FIELD, &self.session_hash)?;
        crate::schema::validate_family_value(RECEIPT_HASH_FIELD, &self.receipt_hash)?;
        crate::schema::validate_metadata(&self.metadata)
    }
}

/// Encodes a validated session/provenance record as Preserves text.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_text(record: &SessionProvenanceRecord) -> Result<String, crate::CodecError> {
    record.validate()?;
    crate::family_codec::encode_text(record)
}

/// Decodes and revalidates a session/provenance record from Preserves text.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or safe fields are invalid.
pub fn from_text(text: &str) -> Result<SessionProvenanceRecord, crate::CodecError> {
    let record: SessionProvenanceRecord = crate::family_codec::decode_text(text)?;
    record.validate()?;
    Ok(record)
}

/// Encodes a validated session/provenance record as Preserves binary.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_bytes(record: &SessionProvenanceRecord) -> Result<Vec<u8>, crate::CodecError> {
    record.validate()?;
    crate::family_codec::encode_bytes(record)
}

/// Decodes and revalidates a session/provenance record from Preserves binary.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or safe fields are invalid.
pub fn from_bytes(bytes: &[u8]) -> Result<SessionProvenanceRecord, crate::CodecError> {
    let record: SessionProvenanceRecord = crate::family_codec::decode_bytes(bytes)?;
    record.validate()?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION_LABEL: &str = "session-primary";
    const SESSION_HASH: &str = "blake3:session-hash";
    const RECEIPT_HASH: &str = "blake3:receipt-hash";
    const PRIVATE_PATH: &str = "/home/example/private-session.json";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;

    fn session_identity() -> animus_runtime::session_persistence::SessionIdentity {
        animus_runtime::session_persistence::SessionIdentity {
            session_label: String::from(SESSION_LABEL),
            session_hash: String::from(SESSION_HASH),
        }
    }

    fn record() -> SessionProvenanceRecord {
        SessionProvenanceRecord::from_session_identity(
            &session_identity(),
            RECEIPT_HASH,
            vec![
                crate::CompatibilityField::new("source", "bounded")
                    .expect("safe metadata should validate"),
            ],
        )
        .expect("session/provenance record should validate")
    }

    #[test]
    fn session_provenance_record_round_trips_text_binary_and_owner_dto() {
        let record = record();
        let text = to_text(&record).expect("record should encode as text");
        let bytes = to_bytes(&record).expect("record should encode as binary");

        assert_eq!(from_text(&text).expect("text should decode"), record);
        assert_eq!(from_bytes(&bytes).expect("binary should decode"), record);
        assert_eq!(
            record
                .try_into_session_identity()
                .expect("session DTO should revalidate"),
            session_identity()
        );
    }

    #[test]
    fn unsupported_wrong_family_malformed_and_leaking_records_fail_closed() {
        let mut unsupported = record();
        unsupported.schema = crate::SchemaIdentity::new(FAMILY, UNSUPPORTED_SCHEMA_VERSION);
        let mut wrong_family = record();
        wrong_family.schema = crate::SchemaIdentity::current(crate::SchemaFamily::PolicyVisibleDto);
        let mut leaking = record();
        leaking.session_label = String::from(PRIVATE_PATH);

        assert!(matches!(
            to_text(&unsupported),
            Err(crate::CodecError::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            to_text(&wrong_family),
            Err(crate::CodecError::WrongSchemaFamily { .. })
        ));
        assert!(from_text("<SessionProvenanceRecord").is_err());
        let error = to_text(&leaking).expect_err("private path must fail");
        assert!(!error.to_string().contains(PRIVATE_PATH));
    }
}
