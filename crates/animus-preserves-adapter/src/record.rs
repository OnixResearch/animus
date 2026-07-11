/// Versioned adapter-only Preserves record.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRecord {
    schema: animus_preserves_schema::SchemaIdentity,
    metadata: Vec<crate::CompatibilityField>,
    payload: crate::AdapterMessage,
}

impl AdapterRecord {
    /// Builds a current adapter record without compatibility metadata.
    #[must_use]
    pub fn from_message(payload: &crate::AdapterMessage) -> Self {
        Self {
            schema: animus_preserves_schema::SchemaIdentity::current(
                animus_preserves_schema::SchemaFamily::AdapterMessage,
            ),
            metadata: Vec::new(),
            payload: payload.clone(),
        }
    }

    /// Builds a current adapter record with validated compatibility metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when metadata or the owning adapter DTO is invalid.
    pub fn with_metadata(
        payload: crate::AdapterMessage,
        metadata: Vec<crate::CompatibilityField>,
    ) -> Result<Self, crate::Error> {
        let record = Self {
            schema: animus_preserves_schema::SchemaIdentity::current(
                animus_preserves_schema::SchemaFamily::AdapterMessage,
            ),
            metadata,
            payload,
        };
        record.validate()?;
        Ok(record)
    }

    /// Returns the schema identity.
    #[must_use]
    pub const fn schema(&self) -> animus_preserves_schema::SchemaIdentity {
        self.schema
    }

    /// Returns safe compatibility metadata.
    #[must_use]
    pub fn metadata(&self) -> &[crate::CompatibilityField] {
        &self.metadata
    }

    /// Returns the typed adapter payload.
    #[must_use]
    pub const fn payload(&self) -> &crate::AdapterMessage {
        &self.payload
    }

    /// Consumes the record and returns its typed adapter payload.
    #[must_use]
    pub fn into_payload(self) -> crate::AdapterMessage {
        self.payload
    }

    /// Validates schema, metadata, and the owning adapter DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the record is unsupported, mismatched, or unsafe.
    pub fn validate(&self) -> Result<(), crate::Error> {
        validate_schema(self.schema)?;
        crate::metadata::validate_metadata(&self.metadata)?;
        self.payload.validate()
    }

    pub(crate) fn from_decoded(record: Self) -> Result<Self, crate::Error> {
        record.validate()?;
        Ok(record)
    }

    #[cfg(test)]
    pub(crate) fn unchecked(
        schema: animus_preserves_schema::SchemaIdentity,
        metadata: Vec<crate::CompatibilityField>,
        payload: crate::AdapterMessage,
    ) -> Self {
        Self {
            schema,
            metadata,
            payload,
        }
    }
}

fn validate_schema(schema: animus_preserves_schema::SchemaIdentity) -> Result<(), crate::Error> {
    if let Err(error) = schema.validate_supported() {
        return Err(crate::Error::UnsupportedSchemaVersion {
            version: error.version(),
            supported: error.supported(),
        });
    }
    if schema.family() != animus_preserves_schema::SchemaFamily::AdapterMessage {
        return Err(crate::Error::WrongSchemaFamily {
            actual: schema.family().as_str(),
        });
    }
    Ok(())
}
