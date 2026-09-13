#[cfg(test)]
mod tests;
mod traversal;

/// Reviewed JSON schema bound to one canonical tool-schema label.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenAiToolSchemaBinding {
    label: String,
    schema: serde_json::Value,
}

impl OpenAiToolSchemaBinding {
    /// Parses and admits one strict function schema.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when the label, byte bound, JSON, or strict-object shape is invalid.
    pub fn from_json(label: &str, schema_json: &[u8]) -> Result<Self, crate::HostError> {
        super::validate_label(label)?;
        if schema_json.len() > super::MAX_SCHEMA_BYTES {
            return Err(crate::HostError::InvalidToolSchema {
                reason: super::SCHEMA_BOUND_REASON,
            });
        }
        let schema: serde_json::Value = serde_json::from_slice(schema_json).map_err(|_error| {
            crate::HostError::InvalidToolSchema {
                reason: super::SCHEMA_JSON_REASON,
            }
        })?;
        traversal::validate(&schema)?;
        Ok(Self {
            label: String::from(label),
            schema,
        })
    }

    /// Returns the canonical schema label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub(crate) const fn schema(&self) -> &serde_json::Value {
        &self.schema
    }
}

pub(super) fn schema_map(
    schemas: &[OpenAiToolSchemaBinding],
) -> Result<std::collections::BTreeMap<&str, &serde_json::Value>, crate::HostError> {
    let mut output = std::collections::BTreeMap::new();
    for binding in schemas {
        if output.insert(binding.label(), binding.schema()).is_some() {
            return Err(crate::HostError::InvalidToolSchema {
                reason: super::DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(output)
}
