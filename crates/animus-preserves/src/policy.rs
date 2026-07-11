const FAMILY: crate::SchemaFamily = crate::SchemaFamily::PolicyVisibleDto;
const POLICY_HASH_FIELD: &str = "policy_hash";
const POLICY_DECISION_FIELD: &str = "policy_decision";
const UNKNOWN_REASON: &str = "unknown policy decision reason";

/// Serializable policy decision that reconstructs an owning adapter DTO.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum PolicyDecisionRecord {
    /// Policy allowed the request.
    Allow,
    /// Policy denied the request with a safe reason class.
    Deny {
        /// Stable reason label.
        reason: String,
    },
}

impl PolicyDecisionRecord {
    /// Builds a serializable decision from an owning adapter DTO.
    #[must_use]
    pub fn from_runtime_policy_decision(decision: animus_adapters::RuntimePolicyDecision) -> Self {
        match decision {
            animus_adapters::RuntimePolicyDecision::Allow => Self::Allow,
            animus_adapters::RuntimePolicyDecision::Deny { reason } => Self::Deny {
                reason: String::from(reason.as_str()),
            },
        }
    }

    /// Reconstructs and revalidates the owning policy decision DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when a reason label is unsupported.
    pub fn try_into_runtime_policy_decision(
        &self,
    ) -> Result<animus_adapters::RuntimePolicyDecision, crate::CodecError> {
        match self {
            Self::Allow => Ok(animus_adapters::RuntimePolicyDecision::Allow),
            Self::Deny { reason } => Ok(animus_adapters::RuntimePolicyDecision::Deny {
                reason: parse_reason(reason)?,
            }),
        }
    }
}

/// Versioned compatibility record for policy-visible decisions and safe metadata.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyVisibleRecord {
    schema: crate::SchemaIdentity,
    policy_hash: String,
    decision: PolicyDecisionRecord,
    metadata: Vec<crate::CompatibilityField>,
}

impl PolicyVisibleRecord {
    /// Builds a policy-visible compatibility record.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when safe fields or metadata are invalid.
    pub fn new(
        policy_hash: &str,
        decision: animus_adapters::RuntimePolicyDecision,
        metadata: Vec<crate::CompatibilityField>,
    ) -> Result<Self, crate::CodecError> {
        let record = Self {
            schema: crate::SchemaIdentity::current(FAMILY),
            policy_hash: String::from(policy_hash),
            decision: PolicyDecisionRecord::from_runtime_policy_decision(decision),
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

    /// Returns the safe policy hash.
    #[must_use]
    pub fn policy_hash(&self) -> &str {
        &self.policy_hash
    }

    /// Returns the serialized policy decision.
    #[must_use]
    pub const fn decision(&self) -> &PolicyDecisionRecord {
        &self.decision
    }

    /// Returns bounded compatibility metadata.
    #[must_use]
    pub fn metadata(&self) -> &[crate::CompatibilityField] {
        &self.metadata
    }

    /// Validates schema identity, policy fields, owning DTO, and metadata.
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
        crate::schema::validate_family_value(POLICY_HASH_FIELD, &self.policy_hash)?;
        self.decision.try_into_runtime_policy_decision()?;
        crate::schema::validate_metadata(&self.metadata)
    }
}

/// Encodes a validated policy-visible record as Preserves text.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_text(record: &PolicyVisibleRecord) -> Result<String, crate::CodecError> {
    record.validate()?;
    crate::family_codec::encode_text(record)
}

/// Decodes and revalidates a policy-visible record from Preserves text.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or owning DTO fields are invalid.
pub fn from_text(text: &str) -> Result<PolicyVisibleRecord, crate::CodecError> {
    let record: PolicyVisibleRecord = crate::family_codec::decode_text(text)?;
    record.validate()?;
    Ok(record)
}

/// Encodes a validated policy-visible record as Preserves binary.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_bytes(record: &PolicyVisibleRecord) -> Result<Vec<u8>, crate::CodecError> {
    record.validate()?;
    crate::family_codec::encode_bytes(record)
}

/// Decodes and revalidates a policy-visible record from Preserves binary.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or owning DTO fields are invalid.
pub fn from_bytes(bytes: &[u8]) -> Result<PolicyVisibleRecord, crate::CodecError> {
    let record: PolicyVisibleRecord = crate::family_codec::decode_bytes(bytes)?;
    record.validate()?;
    Ok(record)
}

fn parse_reason(reason: &str) -> Result<animus_adapters::Reason, crate::CodecError> {
    match reason {
        "not_injected" => Ok(animus_adapters::Reason::NotInjected),
        "host_unavailable" => Ok(animus_adapters::Reason::HostUnavailable),
        "capability_denied" => Ok(animus_adapters::Reason::CapabilityDenied),
        "policy_denied" => Ok(animus_adapters::Reason::PolicyDenied),
        "cancelled" => Ok(animus_adapters::Reason::Cancelled),
        "deadline_exceeded" => Ok(animus_adapters::Reason::DeadlineExceeded),
        "invalid_boundary_payload" => Ok(animus_adapters::Reason::InvalidBoundaryPayload),
        "no_decision" => Ok(animus_adapters::Reason::NoDecision),
        "mismatched_confirmation" => Ok(animus_adapters::Reason::MismatchedConfirmation),
        "stale_confirmation" => Ok(animus_adapters::Reason::StaleConfirmation),
        "completed" => Ok(animus_adapters::Reason::Completed),
        _ => Err(crate::CodecError::InvalidRuntimePayload {
            field: POLICY_DECISION_FIELD,
            reason: UNKNOWN_REASON,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POLICY_HASH: &str = "blake3:policy-hash";
    const PROVIDER_JSON: &str = "{\"raw_provider_payload\":true}";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;

    fn decision() -> animus_adapters::RuntimePolicyDecision {
        animus_adapters::RuntimePolicyDecision::Deny {
            reason: animus_adapters::Reason::PolicyDenied,
        }
    }

    fn record() -> PolicyVisibleRecord {
        PolicyVisibleRecord::new(
            POLICY_HASH,
            decision(),
            vec![
                crate::CompatibilityField::new("rule", "deny-write")
                    .expect("safe metadata should validate"),
            ],
        )
        .expect("policy record should validate")
    }

    #[test]
    fn policy_visible_record_round_trips_text_binary_and_owner_dto() {
        let record = record();
        let text = to_text(&record).expect("record should encode as text");
        let bytes = to_bytes(&record).expect("record should encode as binary");

        assert_eq!(from_text(&text).expect("text should decode"), record);
        assert_eq!(from_bytes(&bytes).expect("binary should decode"), record);
        assert_eq!(
            record
                .decision()
                .try_into_runtime_policy_decision()
                .expect("decision should revalidate"),
            decision()
        );
    }

    #[test]
    fn unsupported_wrong_family_unknown_reason_and_leaking_records_fail_closed() {
        let mut unsupported = record();
        unsupported.schema = crate::SchemaIdentity::new(FAMILY, UNSUPPORTED_SCHEMA_VERSION);
        let mut wrong_family = record();
        wrong_family.schema = crate::SchemaIdentity::current(crate::SchemaFamily::RuntimeEvent);
        let mut unknown_reason = record();
        unknown_reason.decision = PolicyDecisionRecord::Deny {
            reason: String::from("ambient_allow"),
        };
        let mut leaking = record();
        leaking.policy_hash = String::from(PROVIDER_JSON);

        assert!(matches!(
            to_text(&unsupported),
            Err(crate::CodecError::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            to_text(&wrong_family),
            Err(crate::CodecError::WrongSchemaFamily { .. })
        ));
        assert!(matches!(
            to_text(&unknown_reason),
            Err(crate::CodecError::InvalidRuntimePayload { .. })
        ));
        assert!(from_text("<PolicyVisibleRecord").is_err());
        let error = to_text(&leaking).expect_err("provider JSON must fail");
        assert!(!error.to_string().contains(PROVIDER_JSON));
    }
}
