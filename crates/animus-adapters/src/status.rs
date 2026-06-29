use alloc::{string::String, vec::Vec};

const STATUS_OBSERVATION_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("status_observation_kind");
const STATUS_STATE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("semantic_status_state");
const STATUS_SOURCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("status_source_id");
const STATUS_TARGET_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("status_target_id");
const AGENT_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("agent_label");
const SESSION_REFERENCE_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("session_reference_kind");
const SESSION_REFERENCE_VALUE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("session_reference_value");
const STATUS_MESSAGE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("status_message");
const PRESENTATION_TITLE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("presentation_title");
const DISPLAY_AGENT_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("display_agent_name");
const ACTIVITY_TEXT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("activity_text");
const CUSTOM_STATE_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("custom_state_label");
const EVIDENCE_HASH_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("status_evidence_hash");

const STATE_IDLE: &str = "idle";
const STATE_WORKING: &str = "working";
const STATE_BLOCKED: &str = "blocked";
const STATE_DONE: &str = "done";
const STATE_UNKNOWN: &str = "unknown";

const KIND_TRUSTED_LIFECYCLE: &str = "trusted_lifecycle";
const KIND_UNTRUSTED_LIFECYCLE: &str = "untrusted_lifecycle";
const KIND_HEURISTIC_FALLBACK: &str = "heuristic_fallback";

const SESSION_REFERENCE_KIND_NATIVE: &str = "native";
const EVIDENCE_HASH_NOT_HEX_REASON: &str = "evidence hash must contain lowercase hex only";
const EVIDENCE_HASH_LENGTH_REASON: &str = "evidence hash must use the configured BLAKE3 hex length";

/// Character length of a lowercase BLAKE3 evidence hash in hex form.
pub const SAFE_EVIDENCE_HASH_HEX_CHARS: usize = 64;

/// Semantic lifecycle state shared by runtime hosts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticAgentState {
    /// The agent is available for work.
    Idle,
    /// The agent is actively working.
    Working,
    /// The agent needs attention or cannot continue.
    Blocked,
    /// The agent completed its current work.
    Done,
    /// The current semantic state is unknown.
    Unknown,
}

impl SemanticAgentState {
    /// Parses a boundary status state string.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the value is empty, too long,
    /// contains a NUL character, or is not one of the supported status states.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(STATUS_STATE_FIELD, value)?;
        match value.trim() {
            STATE_IDLE => Ok(Self::Idle),
            STATE_WORKING => Ok(Self::Working),
            STATE_BLOCKED => Ok(Self::Blocked),
            STATE_DONE => Ok(Self::Done),
            STATE_UNKNOWN => Ok(Self::Unknown),
            other => Err(crate::AdapterContractError::UnsupportedStatusState {
                field: STATUS_STATE_FIELD.as_str(),
                value: String::from(other),
            }),
        }
    }

    /// Returns the stable lowercase status spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => STATE_IDLE,
            Self::Working => STATE_WORKING,
            Self::Blocked => STATE_BLOCKED,
            Self::Done => STATE_DONE,
            Self::Unknown => STATE_UNKNOWN,
        }
    }
}

/// Observation authority class supplied by a host integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentStatusObservationKind {
    /// Trusted lifecycle report; current reports own semantic state.
    TrustedLifecycle,
    /// Untrusted lifecycle report; selection records it but must not trust it.
    UntrustedLifecycle,
    /// Host heuristic fallback observation.
    HeuristicFallback,
}

impl AgentStatusObservationKind {
    /// Parses an observation-kind boundary string.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the value is empty, too long,
    /// contains a NUL character, or is not one of the supported observation kinds.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(STATUS_OBSERVATION_KIND_FIELD, value)?;
        match value.trim() {
            KIND_TRUSTED_LIFECYCLE => Ok(Self::TrustedLifecycle),
            KIND_UNTRUSTED_LIFECYCLE => Ok(Self::UntrustedLifecycle),
            KIND_HEURISTIC_FALLBACK => Ok(Self::HeuristicFallback),
            other => Err(
                crate::AdapterContractError::UnsupportedStatusObservationKind {
                    field: STATUS_OBSERVATION_KIND_FIELD.as_str(),
                    value: String::from(other),
                },
            ),
        }
    }

    /// Returns the stable lowercase observation-kind spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedLifecycle => KIND_TRUSTED_LIFECYCLE,
            Self::UntrustedLifecycle => KIND_UNTRUSTED_LIFECYCLE,
            Self::HeuristicFallback => KIND_HEURISTIC_FALLBACK,
        }
    }
}

/// Logical monotonic sequence used for deterministic freshness checks.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StatusSequence {
    value: u64,
}

impl StatusSequence {
    /// Builds a status sequence value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the raw sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.value
    }
}

/// Logical TTL for an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusTtl {
    expires_at: StatusSequence,
}

impl StatusTtl {
    /// Builds a TTL whose observation is current through `expires_at`.
    #[must_use]
    pub const fn new(expires_at: StatusSequence) -> Self {
        Self { expires_at }
    }

    /// Returns the last sequence for which the observation is current.
    #[must_use]
    pub const fn expires_at(self) -> StatusSequence {
        self.expires_at
    }

    /// Returns whether the TTL is current at `current_sequence`.
    #[must_use]
    pub const fn is_current_at(self, current_sequence: StatusSequence) -> bool {
        current_sequence.value() <= self.expires_at.value()
    }
}

/// Optional native host session reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentSessionReference {
    kind: AgentSessionReferenceKind,
    value: String,
}

impl AgentSessionReference {
    /// Builds a validated session reference.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the kind or value is invalid
    /// or the kind is unsupported.
    pub fn new(draft: AgentSessionReferenceDraft<'_>) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(
            SESSION_REFERENCE_KIND_FIELD,
            draft.kind,
        )?;
        crate::validation::validate_required_adapter_field(
            SESSION_REFERENCE_VALUE_FIELD,
            draft.value,
        )?;
        let kind = AgentSessionReferenceKind::parse(draft.kind)?;
        Ok(Self {
            kind,
            value: String::from(draft.value),
        })
    }

    /// Returns the supported session reference kind.
    #[must_use]
    pub const fn kind(&self) -> AgentSessionReferenceKind {
        self.kind
    }

    /// Returns the native session reference value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Borrowed session-reference input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentSessionReferenceDraft<'a> {
    /// Session reference kind string.
    pub kind: &'a str,
    /// Session reference value.
    pub value: &'a str,
}

/// Supported session reference kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentSessionReferenceKind {
    /// Host-native session identifier supplied by an integration.
    Native,
}

impl AgentSessionReferenceKind {
    /// Parses a session-reference kind string.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the kind is not supported.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        match value.trim() {
            SESSION_REFERENCE_KIND_NATIVE => Ok(Self::Native),
            other => Err(
                crate::AdapterContractError::UnsupportedSessionReferenceKind {
                    field: SESSION_REFERENCE_KIND_FIELD.as_str(),
                    value: String::from(other),
                },
            ),
        }
    }

    /// Returns the stable lowercase session-reference-kind spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => SESSION_REFERENCE_KIND_NATIVE,
        }
    }
}

/// Presentation-only status metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AgentStatusPresentation {
    title: Option<String>,
    display_agent_name: Option<String>,
    activity_text: Option<String>,
    custom_state_label: Option<String>,
}

impl AgentStatusPresentation {
    /// Builds validated presentation metadata.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when any supplied field is too
    /// long or contains a NUL character.
    pub fn new(
        draft: AgentStatusPresentationDraft<'_>,
    ) -> Result<Self, crate::AdapterContractError> {
        Ok(Self {
            title: optional_field(PRESENTATION_TITLE_FIELD, draft.title)?,
            display_agent_name: optional_field(DISPLAY_AGENT_NAME_FIELD, draft.display_agent_name)?,
            activity_text: optional_field(ACTIVITY_TEXT_FIELD, draft.activity_text)?,
            custom_state_label: optional_field(CUSTOM_STATE_LABEL_FIELD, draft.custom_state_label)?,
        })
    }

    /// Returns the optional presentation title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the optional display agent name.
    #[must_use]
    pub fn display_agent_name(&self) -> Option<&str> {
        self.display_agent_name.as_deref()
    }

    /// Returns the optional compact activity text.
    #[must_use]
    pub fn activity_text(&self) -> Option<&str> {
        self.activity_text.as_deref()
    }

    /// Returns the optional visual state label.
    #[must_use]
    pub fn custom_state_label(&self) -> Option<&str> {
        self.custom_state_label.as_deref()
    }
}

/// Borrowed presentation metadata input.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AgentStatusPresentationDraft<'a> {
    /// Optional title shown by presentation hosts.
    pub title: Option<&'a str>,
    /// Optional display-only agent name.
    pub display_agent_name: Option<&'a str>,
    /// Optional compact activity text.
    pub activity_text: Option<&'a str>,
    /// Optional display-only state label.
    pub custom_state_label: Option<&'a str>,
}

/// Validated semantic status observation supplied by a host adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentStatusObservation {
    kind: AgentStatusObservationKind,
    state: SemanticAgentState,
    source_id: String,
    target_id: String,
    agent_label: String,
    session: Option<AgentSessionReference>,
    message: Option<String>,
    presentation: AgentStatusPresentation,
    sequence: StatusSequence,
    ttl: Option<StatusTtl>,
    evidence_hashes: Vec<String>,
}

impl AgentStatusObservation {
    /// Builds a validated status observation from borrowed boundary fields.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when any field is malformed,
    /// unsupported, too long, or contains unsafe NUL characters.
    pub fn new(
        draft: AgentStatusObservationDraft<'_>,
    ) -> Result<Self, crate::AdapterContractError> {
        let kind = AgentStatusObservationKind::parse(draft.kind)?;
        let state = SemanticAgentState::parse(draft.state)?;
        crate::validation::validate_required_adapter_field(STATUS_SOURCE_FIELD, draft.source_id)?;
        crate::validation::validate_required_adapter_field(STATUS_TARGET_FIELD, draft.target_id)?;
        crate::validation::validate_required_adapter_field(AGENT_LABEL_FIELD, draft.agent_label)?;
        let session = draft.session.map(AgentSessionReference::new).transpose()?;
        let message = optional_field(STATUS_MESSAGE_FIELD, draft.message)?;
        let presentation = AgentStatusPresentation::new(draft.presentation)?;
        let evidence_hashes = collect_evidence_hashes(draft.evidence_hashes)?;

        Ok(Self {
            kind,
            state,
            source_id: String::from(draft.source_id),
            target_id: String::from(draft.target_id),
            agent_label: String::from(draft.agent_label),
            session,
            message,
            presentation,
            sequence: draft.sequence,
            ttl: draft.ttl,
            evidence_hashes,
        })
    }

    /// Returns the observation authority kind.
    #[must_use]
    pub const fn kind(&self) -> AgentStatusObservationKind {
        self.kind
    }

    /// Returns the semantic state reported by the observation.
    #[must_use]
    pub const fn state(&self) -> SemanticAgentState {
        self.state
    }

    /// Returns the source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        self.source_id.as_str()
    }

    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        self.target_id.as_str()
    }

    /// Returns the display-independent agent label.
    #[must_use]
    pub fn agent_label(&self) -> &str {
        self.agent_label.as_str()
    }

    /// Returns the optional host-native session reference.
    #[must_use]
    pub const fn session(&self) -> Option<&AgentSessionReference> {
        self.session.as_ref()
    }

    /// Returns the optional bounded status message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns presentation-only metadata.
    #[must_use]
    pub const fn presentation(&self) -> &AgentStatusPresentation {
        &self.presentation
    }

    /// Returns the logical sequence.
    #[must_use]
    pub const fn sequence(&self) -> StatusSequence {
        self.sequence
    }

    /// Returns the optional TTL.
    #[must_use]
    pub const fn ttl(&self) -> Option<StatusTtl> {
        self.ttl
    }

    /// Returns safe evidence hashes associated with this observation.
    #[must_use]
    pub fn evidence_hashes(&self) -> &[String] {
        self.evidence_hashes.as_slice()
    }

    /// Returns whether this observation is current at `current_sequence`.
    #[must_use]
    pub const fn is_current_at(&self, current_sequence: StatusSequence) -> bool {
        match self.ttl {
            Some(ttl) => ttl.is_current_at(current_sequence),
            None => true,
        }
    }
}

/// Borrowed status observation input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentStatusObservationDraft<'a> {
    /// Observation authority kind string.
    pub kind: &'a str,
    /// Semantic state string.
    pub state: &'a str,
    /// Stable source identifier.
    pub source_id: &'a str,
    /// Stable target identifier.
    pub target_id: &'a str,
    /// Agent label used to identify the actor without presentation overrides.
    pub agent_label: &'a str,
    /// Optional native session reference.
    pub session: Option<AgentSessionReferenceDraft<'a>>,
    /// Optional bounded lifecycle message.
    pub message: Option<&'a str>,
    /// Presentation-only metadata.
    pub presentation: AgentStatusPresentationDraft<'a>,
    /// Logical observation sequence.
    pub sequence: StatusSequence,
    /// Optional logical TTL.
    pub ttl: Option<StatusTtl>,
    /// Safe BLAKE3 evidence hashes; raw evidence is intentionally absent.
    pub evidence_hashes: &'a [&'a str],
}

/// Request sent to a host status-observation adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostStatusRequest {
    target_id: String,
}

impl HostStatusRequest {
    /// Builds a host status request for one target.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the target is empty, too
    /// long, or contains a NUL character.
    pub fn new(target_id: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(STATUS_TARGET_FIELD, target_id)?;
        Ok(Self {
            target_id: String::from(target_id),
        })
    }

    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        self.target_id.as_str()
    }
}

/// Batch returned by a host status-observation adapter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HostStatusObservationBatch {
    observations: Vec<AgentStatusObservation>,
}

impl HostStatusObservationBatch {
    /// Builds a batch from already validated observations.
    #[must_use]
    pub fn new(observations: Vec<AgentStatusObservation>) -> Self {
        Self { observations }
    }

    /// Returns validated observations.
    #[must_use]
    pub fn observations(&self) -> &[AgentStatusObservation] {
        self.observations.as_slice()
    }

    /// Consumes the batch and returns validated observations.
    #[must_use]
    pub fn into_observations(self) -> Vec<AgentStatusObservation> {
        self.observations
    }
}

fn optional_field(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<String>, crate::AdapterContractError> {
    value.map_or(Ok(None), |text| {
        crate::validation::validate_optional_adapter_field(field_name, text)?;
        Ok(Some(String::from(text)))
    })
}

fn collect_evidence_hashes(values: &[&str]) -> Result<Vec<String>, crate::AdapterContractError> {
    values
        .iter()
        .map(|value| {
            validate_evidence_hash(value)?;
            Ok(String::from(*value))
        })
        .collect()
}

fn validate_evidence_hash(value: &str) -> Result<(), crate::AdapterContractError> {
    crate::validation::validate_required_adapter_field(EVIDENCE_HASH_FIELD, value)?;
    let actual_chars = value.chars().count();
    if actual_chars != SAFE_EVIDENCE_HASH_HEX_CHARS {
        return Err(crate::AdapterContractError::InvalidEvidenceHash {
            field: EVIDENCE_HASH_FIELD.as_str(),
            reason: EVIDENCE_HASH_LENGTH_REASON,
        });
    }
    if !value
        .chars()
        .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        return Err(crate::AdapterContractError::InvalidEvidenceHash {
            field: EVIDENCE_HASH_FIELD.as_str(),
            reason: EVIDENCE_HASH_NOT_HEX_REASON,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    const TARGET_ID: &str = "pane:alpha";
    const SOURCE_ID: &str = "lifecycle:herdr";
    const AGENT_LABEL: &str = "agent-alpha";
    const SESSION_VALUE: &str = "session-1";
    const VALID_SEQUENCE: StatusSequence = StatusSequence::new(7);
    const VALID_TTL: StatusTtl = StatusTtl::new(StatusSequence::new(9));
    const VALID_HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const UPPERCASE_HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeF";
    const SHORT_HASH: &str = "0123456789abcdef";
    const MALFORMED_STATE: &str = "busy";
    const MALFORMED_SOURCE: &str = "bad\0source";
    const UNSUPPORTED_SESSION_KIND: &str = "remote";
    const NUL_BYTE_INDEX: usize = 3;

    struct StaticStatusAdapter {
        batch: HostStatusObservationBatch,
    }

    impl crate::ComponentAdapter<HostStatusRequest, HostStatusObservationBatch>
        for StaticStatusAdapter
    {
        type Error = crate::AdapterContractError;

        fn call(
            &mut self,
            _request: &HostStatusRequest,
        ) -> Result<HostStatusObservationBatch, Self::Error> {
            Ok(self.batch.clone())
        }
    }

    fn valid_draft() -> AgentStatusObservationDraft<'static> {
        AgentStatusObservationDraft {
            kind: KIND_TRUSTED_LIFECYCLE,
            state: STATE_WORKING,
            source_id: SOURCE_ID,
            target_id: TARGET_ID,
            agent_label: AGENT_LABEL,
            session: Some(AgentSessionReferenceDraft {
                kind: SESSION_REFERENCE_KIND_NATIVE,
                value: SESSION_VALUE,
            }),
            message: Some("running tests"),
            presentation: AgentStatusPresentationDraft {
                title: Some("Build pane"),
                display_agent_name: Some("Ralph"),
                activity_text: Some("checking"),
                custom_state_label: Some("needs input"),
            },
            sequence: VALID_SEQUENCE,
            ttl: Some(VALID_TTL),
            evidence_hashes: &[VALID_HASH],
        }
    }

    #[test]
    fn lifecycle_report_validation_accepts_valid_observation() {
        let observation = AgentStatusObservation::new(valid_draft())
            .expect("valid status observation should parse");

        assert_eq!(
            observation.kind(),
            AgentStatusObservationKind::TrustedLifecycle
        );
        assert_eq!(observation.state(), SemanticAgentState::Working);
        assert_eq!(observation.source_id(), SOURCE_ID);
        assert_eq!(observation.target_id(), TARGET_ID);
        assert_eq!(observation.agent_label(), AGENT_LABEL);
        assert_eq!(observation.message(), Some("running tests"));
        assert_eq!(observation.sequence(), VALID_SEQUENCE);
        assert_eq!(observation.ttl(), Some(VALID_TTL));
        assert_eq!(observation.evidence_hashes(), &[String::from(VALID_HASH)]);
        assert_eq!(
            observation.presentation().custom_state_label(),
            Some("needs input")
        );
        let session = observation.session().expect("session should be present");
        assert_eq!(session.kind(), AgentSessionReferenceKind::Native);
        assert_eq!(session.value(), SESSION_VALUE);
    }

    #[test]
    fn host_status_adapter_uses_typed_observation_contract() {
        let observation = AgentStatusObservation::new(valid_draft())
            .expect("valid status observation should parse");
        let mut adapter = StaticStatusAdapter {
            batch: HostStatusObservationBatch::new(vec![observation.clone()]),
        };
        let request = HostStatusRequest::new(TARGET_ID).expect("valid target should parse");
        let batch = <StaticStatusAdapter as crate::ComponentAdapter<
            HostStatusRequest,
            HostStatusObservationBatch,
        >>::call(&mut adapter, &request)
        .expect("static adapter should return its typed batch");

        assert_eq!(request.target_id(), TARGET_ID);
        assert_eq!(batch.observations(), &[observation]);
    }

    #[test]
    fn lifecycle_report_rejects_malformed_state() {
        let mut draft = valid_draft();
        draft.state = MALFORMED_STATE;

        let error =
            AgentStatusObservation::new(draft).expect_err("malformed state should be rejected");

        assert_eq!(
            error,
            crate::AdapterContractError::UnsupportedStatusState {
                field: STATUS_STATE_FIELD.as_str(),
                value: String::from(MALFORMED_STATE),
            }
        );
    }

    #[test]
    fn lifecycle_report_rejects_malformed_source() {
        let mut draft = valid_draft();
        draft.source_id = MALFORMED_SOURCE;

        let error =
            AgentStatusObservation::new(draft).expect_err("malformed source should be rejected");

        assert_eq!(
            error,
            crate::AdapterContractError::NulCharacter {
                field: STATUS_SOURCE_FIELD.as_str(),
                byte_index: NUL_BYTE_INDEX,
            }
        );
    }

    #[test]
    fn lifecycle_report_rejects_missing_target() {
        let mut draft = valid_draft();
        draft.target_id = " ";

        let error =
            AgentStatusObservation::new(draft).expect_err("missing target should be rejected");

        assert_eq!(
            error,
            crate::AdapterContractError::EmptyField {
                field: STATUS_TARGET_FIELD.as_str(),
            }
        );
    }

    #[test]
    fn lifecycle_report_rejects_unsupported_session_reference() {
        let mut draft = valid_draft();
        draft.session = Some(AgentSessionReferenceDraft {
            kind: UNSUPPORTED_SESSION_KIND,
            value: SESSION_VALUE,
        });

        let error = AgentStatusObservation::new(draft)
            .expect_err("unsupported session reference should be rejected");

        assert_eq!(
            error,
            crate::AdapterContractError::UnsupportedSessionReferenceKind {
                field: SESSION_REFERENCE_KIND_FIELD.as_str(),
                value: String::from(UNSUPPORTED_SESSION_KIND),
            }
        );
    }

    #[test]
    fn lifecycle_report_rejects_invalid_evidence_hashes() {
        let mut short_draft = valid_draft();
        short_draft.evidence_hashes = &[SHORT_HASH];
        let short_error = AgentStatusObservation::new(short_draft)
            .expect_err("short evidence hash should be rejected");

        assert_eq!(
            short_error,
            crate::AdapterContractError::InvalidEvidenceHash {
                field: EVIDENCE_HASH_FIELD.as_str(),
                reason: EVIDENCE_HASH_LENGTH_REASON,
            }
        );

        let mut uppercase_draft = valid_draft();
        uppercase_draft.evidence_hashes = &[UPPERCASE_HASH];
        let uppercase_error = AgentStatusObservation::new(uppercase_draft)
            .expect_err("uppercase evidence hash should be rejected");

        assert_eq!(
            uppercase_error,
            crate::AdapterContractError::InvalidEvidenceHash {
                field: EVIDENCE_HASH_FIELD.as_str(),
                reason: EVIDENCE_HASH_NOT_HEX_REASON,
            }
        );
    }
}
