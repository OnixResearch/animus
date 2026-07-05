const DATASPACE_FIELD: &str = "dataspace";
const DATASPACE_SCHEMA_FAMILY: crate::SchemaFamily = crate::SchemaFamily::DataspaceDto;
const MAX_DATASPACE_LIST_ITEMS: usize = 8;
const UNSAFE_DATASPACE_REASON: &str = "dataspace payload must contain only safe IDs, labels, hashes, counts, leases, and redaction markers";
const UNBOUNDED_DATASPACE_REASON: &str =
    "dataspace payload list exceeds the bounded preservation limit";
const UNSAFE_DATASPACE_MARKERS: &[&str] = &[
    "/home/",
    "/Users/",
    "raw_prompt:",
    "provider_payload:",
    "credential:",
    "file_body:",
    "tool_input:",
    "tool_output:",
    "model_output:",
    "unbounded_log:",
    "begin private key",
    "api_key=",
    "aws_secret_access_key",
    "sk-",
];

/// Dataspace payload kind stored in a versioned Preserves record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataspacePayloadKind {
    /// Dataspace scope boundary payload.
    Scope,
    /// Bounded dataspace assertion payload.
    Assertion,
    /// Safe dataspace subscription payload.
    Subscription,
    /// Host orchestration event payload.
    OrchestrationEvent,
}

impl DataspacePayloadKind {
    /// Returns the stable payload-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scope => "scope",
            Self::Assertion => "assertion",
            Self::Subscription => "subscription",
            Self::OrchestrationEvent => "orchestration_event",
        }
    }
}

/// Serializable scope payload used by dataspace Preserves records.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DataspaceScopePayload {
    tenant_id: String,
    workspace_id: String,
    session_id: String,
    branch_id: String,
    facet_id: String,
    actor_kind: String,
    actor_id: String,
}

impl DataspaceScopePayload {
    /// Builds a Preserves scope payload from a validated adapter DTO.
    #[must_use]
    pub fn from_scope(scope: &animus_adapters::Scope) -> Self {
        Self {
            tenant_id: String::from(scope.tenant_id()),
            workspace_id: String::from(scope.workspace_id()),
            session_id: String::from(scope.session_id()),
            branch_id: String::from(scope.branch_id()),
            facet_id: String::from(scope.facet_id()),
            actor_kind: String::from(scope.actor_kind().as_str()),
            actor_id: String::from(scope.actor_id()),
        }
    }

    /// Converts the payload into a validated adapter scope DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when a field is unsafe or invalid.
    pub fn try_into_scope(&self) -> Result<animus_adapters::Scope, crate::CodecError> {
        validate_dataspace_texts([
            self.tenant_id.as_str(),
            self.workspace_id.as_str(),
            self.session_id.as_str(),
            self.branch_id.as_str(),
            self.facet_id.as_str(),
            self.actor_kind.as_str(),
            self.actor_id.as_str(),
        ])?;
        Ok(animus_adapters::Scope::new(animus_adapters::ScopeDraft {
            tenant_id: &self.tenant_id,
            workspace_id: &self.workspace_id,
            session_id: &self.session_id,
            branch_id: &self.branch_id,
            facet_id: &self.facet_id,
            actor_kind: &self.actor_kind,
            actor_id: &self.actor_id,
        })?)
    }
}

/// Typed dataspace Preserves payload.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum DataspaceMessage {
    /// Dataspace scope boundary payload.
    Scope {
        /// Safe scope payload.
        scope: DataspaceScopePayload,
    },
    /// Bounded dataspace assertion payload.
    Assertion {
        /// Stable assertion identity.
        stable_id: String,
        /// Assertion kind label.
        kind: String,
        /// Assertion status label.
        status: String,
        /// Scoped assertion owner.
        scope: DataspaceScopePayload,
        /// Redaction marker labels.
        redactions: Vec<String>,
        /// Optional BLAKE3 safe content hash.
        content_hash: Option<String>,
        /// Bounded item count.
        item_count: u32,
        /// Optional deterministic lease expiry tick.
        lease_expires_at: Option<u64>,
    },
    /// Safe subscription payload.
    Subscription {
        /// Subscription scope boundary.
        scope: DataspaceScopePayload,
        /// Assertion kind labels accepted by the query.
        kinds: Vec<String>,
        /// Raw payload request bit; validated fail-closed when true.
        include_raw_payloads: bool,
        /// Cross-workspace query bit; validated fail-closed when true.
        cross_workspace: bool,
    },
    /// Host orchestration event payload.
    OrchestrationEvent {
        /// Stable event identity.
        stable_id: String,
        /// Event kind label.
        kind: String,
        /// Affected scope boundary.
        scope: DataspaceScopePayload,
        /// Optional ownership-transfer target owner.
        target_owner: Option<String>,
    },
}

impl DataspaceMessage {
    /// Builds a Preserves scope message from a validated adapter DTO.
    #[must_use]
    pub fn from_scope(scope: &animus_adapters::Scope) -> Self {
        Self::Scope {
            scope: DataspaceScopePayload::from_scope(scope),
        }
    }

    /// Builds a Preserves assertion message from a validated adapter DTO.
    #[must_use]
    pub fn from_assertion(assertion: &animus_adapters::Assertion) -> Self {
        Self::Assertion {
            stable_id: String::from(assertion.stable_id()),
            kind: String::from(assertion_kind_label(assertion.kind())),
            status: String::from(assertion_status_label(assertion.status())),
            scope: DataspaceScopePayload::from_scope(assertion.scope()),
            redactions: assertion
                .redactions()
                .iter()
                .map(|redaction| String::from(redaction_label(*redaction)))
                .collect(),
            content_hash: assertion
                .content_hash()
                .map(animus_adapters::SafeHash::as_str)
                .map(String::from),
            item_count: assertion.item_count(),
            lease_expires_at: assertion.lease_expires_at(),
        }
    }

    /// Builds a Preserves subscription message from a validated adapter DTO.
    #[must_use]
    pub fn from_subscription(subscription: &animus_adapters::Subscription) -> Self {
        Self::Subscription {
            scope: DataspaceScopePayload::from_scope(subscription.scope()),
            kinds: subscription
                .kinds()
                .iter()
                .map(|kind| String::from(assertion_kind_label(*kind)))
                .collect(),
            include_raw_payloads: false,
            cross_workspace: false,
        }
    }

    /// Builds a Preserves orchestration event message from a validated adapter DTO.
    #[must_use]
    pub fn from_orchestration_event(event: &animus_adapters::OrchestrationEvent) -> Self {
        Self::OrchestrationEvent {
            stable_id: String::from(event.stable_id()),
            kind: String::from(event.kind().as_str()),
            scope: DataspaceScopePayload::from_scope(event.scope()),
            target_owner: event.target_owner().map(String::from),
        }
    }

    /// Returns the typed dataspace payload kind.
    #[must_use]
    pub const fn kind(&self) -> DataspacePayloadKind {
        match self {
            Self::Scope { .. } => DataspacePayloadKind::Scope,
            Self::Assertion { .. } => DataspacePayloadKind::Assertion,
            Self::Subscription { .. } => DataspacePayloadKind::Subscription,
            Self::OrchestrationEvent { .. } => DataspacePayloadKind::OrchestrationEvent,
        }
    }

    /// Converts this message into a validated adapter scope DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or fields are unsafe.
    pub fn try_into_scope(self) -> Result<animus_adapters::Scope, crate::CodecError> {
        match self {
            Self::Scope { scope } => scope.try_into_scope(),
            other => Err(crate::CodecError::WrongDataspaceKind {
                expected: DataspacePayloadKind::Scope.as_str(),
                actual: other.kind().as_str(),
            }),
        }
    }

    /// Converts this message into a validated adapter assertion DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or fields are unsafe.
    pub fn try_into_assertion(self) -> Result<animus_adapters::Assertion, crate::CodecError> {
        match self {
            Self::Assertion {
                stable_id,
                kind,
                status,
                scope,
                redactions,
                content_hash,
                item_count,
                lease_expires_at,
            } => {
                validate_dataspace_list(&redactions)?;
                validate_dataspace_texts([stable_id.as_str(), kind.as_str(), status.as_str()])?;
                validate_dataspace_texts(redactions.iter().map(String::as_str))?;
                if let Some(hash) = content_hash.as_deref() {
                    validate_dataspace_texts([hash])?;
                }
                let scope = scope.try_into_scope()?;
                let redaction_refs = redactions.iter().map(String::as_str).collect::<Vec<_>>();
                Ok(animus_adapters::Assertion::new(
                    animus_adapters::AssertionDraft {
                        stable_id: &stable_id,
                        kind: &kind,
                        status: &status,
                        scope,
                        redactions: &redaction_refs,
                        content_hash: content_hash.as_deref(),
                        item_count,
                        lease_expires_at,
                    },
                )?)
            }
            other => Err(crate::CodecError::WrongDataspaceKind {
                expected: DataspacePayloadKind::Assertion.as_str(),
                actual: other.kind().as_str(),
            }),
        }
    }

    /// Converts this message into a validated adapter subscription DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or fields are unsafe.
    pub fn try_into_subscription(self) -> Result<animus_adapters::Subscription, crate::CodecError> {
        match self {
            Self::Subscription {
                scope,
                kinds,
                include_raw_payloads,
                cross_workspace,
            } => {
                validate_dataspace_list(&kinds)?;
                validate_dataspace_texts(kinds.iter().map(String::as_str))?;
                let scope = scope.try_into_scope()?;
                let kind_refs = kinds.iter().map(String::as_str).collect::<Vec<_>>();
                animus_adapters::Subscription::new(animus_adapters::SubscriptionDraft {
                    scope,
                    kinds: &kind_refs,
                    include_raw_payloads,
                    cross_workspace,
                })
                .map_err(|issue| subscription_issue(&issue))
            }
            other => Err(crate::CodecError::WrongDataspaceKind {
                expected: DataspacePayloadKind::Subscription.as_str(),
                actual: other.kind().as_str(),
            }),
        }
    }

    /// Converts this message into a validated adapter orchestration event DTO.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the message kind differs or fields are unsafe.
    pub fn try_into_orchestration_event(
        self,
    ) -> Result<animus_adapters::OrchestrationEvent, crate::CodecError> {
        match self {
            Self::OrchestrationEvent {
                stable_id,
                kind,
                scope,
                target_owner,
            } => {
                validate_dataspace_texts([stable_id.as_str(), kind.as_str()])?;
                if let Some(owner) = target_owner.as_deref() {
                    validate_dataspace_texts([owner])?;
                }
                let scope = scope.try_into_scope()?;
                Ok(animus_adapters::OrchestrationEvent::new(
                    animus_adapters::OrchestrationEventDraft {
                        stable_id: &stable_id,
                        kind: &kind,
                        scope,
                        target_owner: target_owner.as_deref(),
                    },
                )?)
            }
            other => Err(crate::CodecError::WrongDataspaceKind {
                expected: DataspacePayloadKind::OrchestrationEvent.as_str(),
                actual: other.kind().as_str(),
            }),
        }
    }

    fn validate(&self) -> Result<(), crate::CodecError> {
        match self.clone() {
            Self::Scope { .. } => self.clone().try_into_scope().map(|_scope| ()),
            Self::Assertion { .. } => self.clone().try_into_assertion().map(|_assertion| ()),
            Self::Subscription { .. } => self.clone().try_into_subscription().map(|_query| ()),
            Self::OrchestrationEvent { .. } => {
                self.clone().try_into_orchestration_event().map(|_event| ())
            }
        }
    }
}

/// Versioned Preserves record for safe dataspace DTO payloads.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DataspaceRecord {
    schema: crate::SchemaIdentity,
    payload: DataspaceMessage,
}

impl DataspaceRecord {
    /// Builds a versioned dataspace record from a typed payload.
    #[must_use]
    pub fn from_message(payload: &DataspaceMessage) -> Self {
        Self {
            schema: crate::SchemaIdentity::current(DATASPACE_SCHEMA_FAMILY),
            payload: payload.clone(),
        }
    }

    /// Builds a versioned dataspace record from a validated scope DTO.
    #[must_use]
    pub fn from_scope(scope: &animus_adapters::Scope) -> Self {
        Self::from_message(&DataspaceMessage::from_scope(scope))
    }

    /// Builds a versioned dataspace record from a validated assertion DTO.
    #[must_use]
    pub fn from_assertion(assertion: &animus_adapters::Assertion) -> Self {
        Self::from_message(&DataspaceMessage::from_assertion(assertion))
    }

    /// Builds a versioned dataspace record from a validated subscription DTO.
    #[must_use]
    pub fn from_subscription(subscription: &animus_adapters::Subscription) -> Self {
        Self::from_message(&DataspaceMessage::from_subscription(subscription))
    }

    /// Builds a versioned dataspace record from a validated orchestration event DTO.
    #[must_use]
    pub fn from_orchestration_event(event: &animus_adapters::OrchestrationEvent) -> Self {
        Self::from_message(&DataspaceMessage::from_orchestration_event(event))
    }

    /// Returns the declared schema identity.
    #[must_use]
    pub const fn schema(&self) -> crate::SchemaIdentity {
        self.schema
    }

    /// Returns the typed payload kind.
    #[must_use]
    pub const fn payload_kind(&self) -> DataspacePayloadKind {
        self.payload.kind()
    }

    /// Returns the typed dataspace payload.
    #[must_use]
    pub const fn payload(&self) -> &DataspaceMessage {
        &self.payload
    }

    /// Consumes the record and returns the typed dataspace payload.
    #[must_use]
    pub fn into_payload(self) -> DataspaceMessage {
        self.payload
    }

    /// Validates schema version, family, payload kind, and safe DTO fields.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the record is unsupported or unsafe.
    pub fn validate(&self) -> Result<(), crate::CodecError> {
        self.schema.validate_supported()?;
        if self.schema.family() != DATASPACE_SCHEMA_FAMILY {
            return Err(crate::CodecError::WrongSchemaFamily {
                expected: DATASPACE_SCHEMA_FAMILY.as_str(),
                actual: self.schema.family().as_str(),
            });
        }
        self.payload.validate()
    }

    pub(crate) fn from_decoded(record: Self) -> Result<Self, crate::CodecError> {
        record.validate()?;
        Ok(record)
    }
}

/// Encodes one versioned dataspace record as Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_dataspace_record_text(record: &DataspaceRecord) -> Result<String, crate::CodecError> {
    record.validate()?;
    encode_text(record)
}

/// Decodes one versioned dataspace record from Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or DTO validation fails.
pub fn from_dataspace_record_text(text: &str) -> Result<DataspaceRecord, crate::CodecError> {
    let record = preserves::de::from_text(text)?;
    DataspaceRecord::from_decoded(record)
}

/// Encodes one versioned dataspace record as Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when schema validation or serialization fails.
pub fn to_dataspace_record_bytes(record: &DataspaceRecord) -> Result<Vec<u8>, crate::CodecError> {
    record.validate()?;
    encode_bytes(record)
}

/// Decodes one versioned dataspace record from Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, schema, or DTO validation fails.
pub fn from_dataspace_record_bytes(bytes: &[u8]) -> Result<DataspaceRecord, crate::CodecError> {
    let record = preserves::de::from_bytes(bytes)?;
    DataspaceRecord::from_decoded(record)
}

fn assertion_kind_label(kind: animus_adapters::AssertionKind) -> &'static str {
    match kind {
        animus_adapters::AssertionKind::RuntimeEvent => "runtime_event",
        animus_adapters::AssertionKind::ToolState => "tool_state",
        animus_adapters::AssertionKind::BackgroundWork => "background_work",
        animus_adapters::AssertionKind::SessionSummary => "session_summary",
        animus_adapters::AssertionKind::ProvenanceSummary => "provenance_summary",
        animus_adapters::AssertionKind::AgentLifecycle => "agent_lifecycle",
        animus_adapters::AssertionKind::SubagentLifecycle => "subagent_lifecycle",
        animus_adapters::AssertionKind::Cancellation => "cancellation",
        animus_adapters::AssertionKind::OwnershipTransfer => "ownership_transfer",
    }
}

fn assertion_status_label(status: animus_adapters::AssertionStatus) -> &'static str {
    match status {
        animus_adapters::AssertionStatus::Active => "active",
        animus_adapters::AssertionStatus::Completed => "completed",
        animus_adapters::AssertionStatus::Cancelled => "cancelled",
        animus_adapters::AssertionStatus::Expired => "expired",
        animus_adapters::AssertionStatus::Failed => "failed",
        animus_adapters::AssertionStatus::Blocked => "blocked",
    }
}

fn redaction_label(redaction: animus_adapters::AssertionRedaction) -> &'static str {
    match redaction {
        animus_adapters::AssertionRedaction::PromptOmitted => "prompt_omitted",
        animus_adapters::AssertionRedaction::ProviderPayloadOmitted => "provider_payload_omitted",
        animus_adapters::AssertionRedaction::CredentialOmitted => "credential_omitted",
        animus_adapters::AssertionRedaction::PathRedacted => "path_redacted",
        animus_adapters::AssertionRedaction::FileBodyOmitted => "file_body_omitted",
        animus_adapters::AssertionRedaction::ToolPayloadOmitted => "tool_payload_omitted",
        animus_adapters::AssertionRedaction::ModelOutputOmitted => "model_output_omitted",
        animus_adapters::AssertionRedaction::LogsBounded => "logs_bounded",
    }
}

fn subscription_issue(issue: &animus_adapters::SubscriptionIssue) -> crate::CodecError {
    crate::CodecError::InvalidRuntimePayload {
        field: DATASPACE_FIELD,
        reason: issue.as_str(),
    }
}

fn validate_dataspace_list<T>(items: &[T]) -> Result<(), crate::CodecError> {
    if items.len() > MAX_DATASPACE_LIST_ITEMS {
        return Err(crate::CodecError::UnsafeDataspacePayload {
            field: DATASPACE_FIELD,
            reason: UNBOUNDED_DATASPACE_REASON,
        });
    }
    Ok(())
}

fn validate_dataspace_texts<'a>(
    values: impl IntoIterator<Item = &'a str>,
) -> Result<(), crate::CodecError> {
    for value in values {
        validate_dataspace_text(value)?;
    }
    Ok(())
}

fn validate_dataspace_text(value: &str) -> Result<(), crate::CodecError> {
    let lower = value.to_ascii_lowercase();
    for marker in UNSAFE_DATASPACE_MARKERS {
        if lower.contains(marker) {
            return Err(crate::CodecError::UnsafeDataspacePayload {
                field: DATASPACE_FIELD,
                reason: UNSAFE_DATASPACE_REASON,
            });
        }
    }
    Ok(())
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

    const TENANT: &str = "tenant:local";
    const WORKSPACE: &str = "workspace:main";
    const SESSION: &str = "session:run";
    const BRANCH: &str = "branch:main";
    const FACET: &str = "facet:loop";
    const ACTOR_KIND: &str = "agent";
    const ACTOR_ID: &str = "agent:main";
    const ASSERTION_ID: &str = "assertion:runtime";
    const ASSERTION_KIND: &str = "runtime_event";
    const ASSERTION_STATUS: &str = "active";
    const REDACTION_PROMPT: &str = "prompt_omitted";
    const REDACTION_PROVIDER: &str = "provider_payload_omitted";
    const REDACTION_CREDENTIAL: &str = "credential_omitted";
    const REDACTION_LOGS: &str = "logs_bounded";
    const SAFE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const EVENT_ID: &str = "event:cancel";
    const EVENT_KIND: &str = "cancelled";
    const UNSAFE_RAW_PROMPT: &str = "raw_prompt: private text";
    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;
    const ITEM_COUNT: u32 = 3;
    const LEASE_EXPIRES_AT: u64 = 42;
    const TOO_MANY_ITEMS: usize = MAX_DATASPACE_LIST_ITEMS + 1;

    fn scope() -> animus_adapters::Scope {
        animus_adapters::Scope::new(animus_adapters::ScopeDraft {
            tenant_id: TENANT,
            workspace_id: WORKSPACE,
            session_id: SESSION,
            branch_id: BRANCH,
            facet_id: FACET,
            actor_kind: ACTOR_KIND,
            actor_id: ACTOR_ID,
        })
        .expect("scope should validate")
    }

    fn assertion() -> animus_adapters::Assertion {
        animus_adapters::Assertion::new(animus_adapters::AssertionDraft {
            stable_id: ASSERTION_ID,
            kind: ASSERTION_KIND,
            status: ASSERTION_STATUS,
            scope: scope(),
            redactions: &[
                REDACTION_PROMPT,
                REDACTION_PROVIDER,
                REDACTION_CREDENTIAL,
                REDACTION_LOGS,
            ],
            content_hash: Some(SAFE_HASH),
            item_count: ITEM_COUNT,
            lease_expires_at: Some(LEASE_EXPIRES_AT),
        })
        .expect("assertion should validate")
    }

    fn subscription() -> animus_adapters::Subscription {
        animus_adapters::Subscription::new(animus_adapters::SubscriptionDraft {
            scope: scope(),
            kinds: &[ASSERTION_KIND],
            include_raw_payloads: false,
            cross_workspace: false,
        })
        .expect("subscription should validate")
    }

    fn event() -> animus_adapters::OrchestrationEvent {
        animus_adapters::OrchestrationEvent::new(animus_adapters::OrchestrationEventDraft {
            stable_id: EVENT_ID,
            kind: EVENT_KIND,
            scope: scope(),
            target_owner: None,
        })
        .expect("event should validate")
    }

    fn assert_text_and_binary_round_trip(record: &DataspaceRecord) {
        let text = to_dataspace_record_text(record).expect("dataspace record should encode text");
        let decoded_text =
            from_dataspace_record_text(&text).expect("dataspace record should decode text");
        let bytes =
            to_dataspace_record_bytes(record).expect("dataspace record should encode binary");
        let decoded_bytes =
            from_dataspace_record_bytes(&bytes).expect("dataspace record should decode binary");

        assert_eq!(&decoded_text, record);
        assert_eq!(&decoded_bytes, record);
    }

    #[test]
    fn dataspace_scope_assertion_subscription_and_event_round_trip() {
        // r[verify agent_harness_runtime.preserves_dataspace.round_trip.valid]
        let scope_record = DataspaceRecord::from_scope(&scope());
        let assertion_record = DataspaceRecord::from_assertion(&assertion());
        let subscription_record = DataspaceRecord::from_subscription(&subscription());
        let event_record = DataspaceRecord::from_orchestration_event(&event());

        for record in [
            &scope_record,
            &assertion_record,
            &subscription_record,
            &event_record,
        ] {
            assert_eq!(record.schema().family(), crate::SchemaFamily::DataspaceDto);
            assert_text_and_binary_round_trip(record);
        }

        let decoded = assertion_record
            .clone()
            .into_payload()
            .try_into_assertion()
            .expect("assertion payload should decode");
        assert_eq!(decoded.scope().tenant_id(), TENANT);
        assert_eq!(decoded.status(), animus_adapters::AssertionStatus::Active);
        assert_eq!(
            decoded
                .content_hash()
                .map(animus_adapters::SafeHash::as_str),
            Some(SAFE_HASH)
        );
        assert_eq!(decoded.lease_expires_at(), Some(LEASE_EXPIRES_AT));
    }

    #[test]
    fn dataspace_records_reject_unsupported_versions_wrong_families_and_wrong_kinds() {
        // r[verify agent_harness_runtime.preserves_dataspace.malformed.rejects]
        let unsupported = DataspaceRecord {
            schema: crate::SchemaIdentity::new(
                crate::SchemaFamily::DataspaceDto,
                UNSUPPORTED_SCHEMA_VERSION,
            ),
            payload: DataspaceMessage::from_scope(&scope()),
        };
        let wrong_family = DataspaceRecord {
            schema: crate::SchemaIdentity::current(crate::SchemaFamily::AdapterMessage),
            payload: DataspaceMessage::from_scope(&scope()),
        };
        let wrong_kind = DataspaceMessage::from_scope(&scope())
            .try_into_assertion()
            .expect_err("wrong dataspace kind should fail");

        assert!(matches!(
            unsupported.validate(),
            Err(crate::CodecError::UnsupportedSchemaVersion {
                family: "dataspace_dto",
                version: UNSUPPORTED_SCHEMA_VERSION,
                ..
            })
        ));
        assert!(matches!(
            wrong_family.validate(),
            Err(crate::CodecError::WrongSchemaFamily {
                expected: "dataspace_dto",
                actual: "adapter_message",
            })
        ));
        assert!(matches!(
            wrong_kind,
            crate::CodecError::WrongDataspaceKind {
                expected: "assertion",
                actual: "scope",
            }
        ));
    }

    #[test]
    fn dataspace_decode_rejects_unsafe_cross_scope_raw_and_unbounded_payloads() {
        // r[verify agent_harness_runtime.preserves_dataspace.safe_payloads.rejects_raw]
        // r[verify agent_harness_runtime.preserves_dataspace.malformed.rejects]
        let unsafe_scope = DataspaceMessage::Scope {
            scope: DataspaceScopePayload {
                tenant_id: String::from(TENANT),
                workspace_id: String::from(UNSAFE_RAW_PROMPT),
                session_id: String::from(SESSION),
                branch_id: String::from(BRANCH),
                facet_id: String::from(FACET),
                actor_kind: String::from(ACTOR_KIND),
                actor_id: String::from(ACTOR_ID),
            },
        };
        let cross_scope = DataspaceMessage::Subscription {
            scope: DataspaceScopePayload::from_scope(&scope()),
            kinds: vec![String::from(ASSERTION_KIND)],
            include_raw_payloads: false,
            cross_workspace: true,
        };
        let raw_payload = DataspaceMessage::Subscription {
            scope: DataspaceScopePayload::from_scope(&scope()),
            kinds: vec![String::from(ASSERTION_KIND)],
            include_raw_payloads: true,
            cross_workspace: false,
        };
        let unbounded = DataspaceMessage::Subscription {
            scope: DataspaceScopePayload::from_scope(&scope()),
            kinds: std::iter::repeat_n(String::from(ASSERTION_KIND), TOO_MANY_ITEMS).collect(),
            include_raw_payloads: false,
            cross_workspace: false,
        };

        assert!(matches!(
            unsafe_scope.validate(),
            Err(crate::CodecError::UnsafeDataspacePayload { .. })
        ));
        assert!(matches!(
            cross_scope.validate(),
            Err(crate::CodecError::InvalidRuntimePayload {
                reason: "DATASPACE_CROSS_WORKSPACE_SCOPE",
                ..
            })
        ));
        assert!(matches!(
            raw_payload.validate(),
            Err(crate::CodecError::InvalidRuntimePayload {
                reason: "DATASPACE_RAW_PAYLOAD_REQUESTED",
                ..
            })
        ));
        assert!(matches!(
            unbounded.validate(),
            Err(crate::CodecError::UnsafeDataspacePayload {
                reason: UNBOUNDED_DATASPACE_REASON,
                ..
            })
        ));
    }

    #[test]
    fn dataspace_errors_do_not_echo_raw_material() {
        let unsafe_scope = DataspaceMessage::Scope {
            scope: DataspaceScopePayload {
                tenant_id: String::from(TENANT),
                workspace_id: String::from(UNSAFE_RAW_PROMPT),
                session_id: String::from(SESSION),
                branch_id: String::from(BRANCH),
                facet_id: String::from(FACET),
                actor_kind: String::from(ACTOR_KIND),
                actor_id: String::from(ACTOR_ID),
            },
        };
        let diagnostic = unsafe_scope
            .validate()
            .expect_err("unsafe dataspace payload should fail")
            .to_string();

        assert!(!diagnostic.contains(UNSAFE_RAW_PROMPT));
        assert!(diagnostic.contains(DATASPACE_FIELD));
    }
}
