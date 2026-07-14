//! Capability-free Syndicate Protocol turn profile over validated Animus dataspace records.
//!
//! The wire shape follows Syndicate Protocol v1 for turns and the `A`, `R`, and
//! `M` event variants. Embedded `WireRef`s, `S` synchronization events, caveats,
//! extensions, and protocol error packets are deliberately unsupported.

use preserves::value::{IOValue, NestedValue, Value};

const ASSERT_EVENT_LABEL: &str = "A";
const RETRACT_EVENT_LABEL: &str = "R";
const MESSAGE_EVENT_LABEL: &str = "M";
const SYNC_EVENT_LABEL: &str = "S";
const ASSERT_EVENT_ARITY: usize = 2;
const RETRACT_EVENT_ARITY: usize = 1;
const MESSAGE_EVENT_ARITY: usize = 1;
const TURN_EVENT_FIELDS: usize = 2;
const INVALID_PACKET_REASON: &str = "packet is not a supported Syndicate turn";
const INVALID_TURN_EVENT_REASON: &str = "turn event must be an [Oid Event] sequence";
const INVALID_OID_REASON: &str = "turn OID must be a non-negative 64-bit integer";
const INVALID_HANDLE_REASON: &str = "assertion handle must be a non-negative 64-bit integer";
const INVALID_BODY_REASON: &str = "event body is not a validated Animus dataspace record";
const UNSUPPORTED_EVENT_REASON: &str =
    "event is outside the capability-free A/R/M Syndicate profile";
const UNBOUNDED_TURN_REASON: &str = "turn exceeds the bounded event count";

/// Stable route/session profile label for the capability-free Syndicate Protocol v1 adapter.
pub const SYNDICATE_PROTOCOL_PROFILE: &str = "syndicate.protocol.v1.animus-safe";
/// Maximum number of ordered events accepted in one turn packet.
pub const MAX_SYNDICATE_TURN_EVENTS: usize = 64;

/// Supported Syndicate Protocol event kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyndicateEventKind {
    /// Publish one assertion under a connection-scoped handle.
    Assert,
    /// Retract one previously published connection-scoped handle.
    Retract,
    /// Deliver one ephemeral message body.
    Message,
}

/// Capability-free Syndicate Protocol event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyndicateEvent {
    /// `<A Assertion Handle>` with a validated Animus dataspace record body.
    Assert {
        /// Validated assertion record.
        assertion: crate::DataspaceRecord,
        /// Connection-scoped assertion handle.
        handle: u64,
    },
    /// `<R Handle>` for one live assertion handle.
    Retract {
        /// Connection-scoped assertion handle.
        handle: u64,
    },
    /// `<M Body>` with a validated Animus dataspace record body.
    Message {
        /// Validated message body.
        body: crate::DataspaceRecord,
    },
}

impl SyndicateEvent {
    /// Returns the supported event kind.
    #[must_use]
    pub const fn kind(&self) -> SyndicateEventKind {
        match self {
            Self::Assert { .. } => SyndicateEventKind::Assert,
            Self::Retract { .. } => SyndicateEventKind::Retract,
            Self::Message { .. } => SyndicateEventKind::Message,
        }
    }

    /// Returns the assertion handle for assert and retract events.
    #[must_use]
    pub const fn handle(&self) -> Option<u64> {
        match self {
            Self::Assert { handle, .. } | Self::Retract { handle } => Some(*handle),
            Self::Message { .. } => None,
        }
    }

    /// Returns the validated assertion record when this is an assert event.
    #[must_use]
    pub const fn assertion(&self) -> Option<&crate::DataspaceRecord> {
        match self {
            Self::Assert { assertion, .. } => Some(assertion),
            Self::Retract { .. } | Self::Message { .. } => None,
        }
    }

    /// Returns the validated message record when this is a message event.
    #[must_use]
    pub const fn message_body(&self) -> Option<&crate::DataspaceRecord> {
        match self {
            Self::Message { body } => Some(body),
            Self::Assert { .. } | Self::Retract { .. } => None,
        }
    }

    fn validate(&self) -> Result<(), crate::CodecError> {
        match self {
            Self::Assert { assertion, .. } => {
                assertion.validate()?;
                if assertion.payload_kind() != crate::DataspacePayloadKind::Assertion {
                    return Err(invalid_packet(UNSUPPORTED_EVENT_REASON));
                }
                Ok(())
            }
            Self::Retract { .. } => Ok(()),
            Self::Message { body } => body.validate(),
        }
    }
}

/// One ordered Syndicate turn entry, represented on the wire as `[Oid Event]`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyndicateTurnEvent {
    oid: u64,
    event: SyndicateEvent,
}

impl SyndicateTurnEvent {
    /// Builds an assertion event from an already validated Animus assertion DTO.
    #[must_use]
    pub fn from_assertion(oid: u64, handle: u64, assertion: &animus_adapters::Assertion) -> Self {
        // r[impl agent_harness_runtime.syndicate_turn_adapter.projection]
        Self {
            oid,
            event: SyndicateEvent::Assert {
                assertion: crate::DataspaceRecord::from_assertion(assertion),
                handle,
            },
        }
    }

    /// Builds a retraction event.
    #[must_use]
    pub const fn retract(oid: u64, handle: u64) -> Self {
        Self {
            oid,
            event: SyndicateEvent::Retract { handle },
        }
    }

    /// Builds a message event from a validated dataspace record.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the body is unsupported or unsafe.
    pub fn message(oid: u64, body: crate::DataspaceRecord) -> Result<Self, crate::CodecError> {
        body.validate()?;
        Ok(Self {
            oid,
            event: SyndicateEvent::Message { body },
        })
    }

    /// Builds an orchestration-message event from an already validated Animus DTO.
    #[must_use]
    pub fn from_orchestration_event(oid: u64, event: &animus_adapters::OrchestrationEvent) -> Self {
        Self {
            oid,
            event: SyndicateEvent::Message {
                body: crate::DataspaceRecord::from_orchestration_event(event),
            },
        }
    }

    /// Returns the receiving entity OID.
    #[must_use]
    pub const fn oid(&self) -> u64 {
        self.oid
    }

    /// Returns the turn event.
    #[must_use]
    pub const fn event(&self) -> &SyndicateEvent {
        &self.event
    }

    fn validate(&self) -> Result<(), crate::CodecError> {
        self.event.validate()
    }
}

/// Ordered capability-free Syndicate Protocol turn packet.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SyndicateTurn {
    events: Vec<SyndicateTurnEvent>,
}

impl SyndicateTurn {
    /// Builds and validates an ordered turn packet.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the event count or a body is invalid.
    pub fn new(events: Vec<SyndicateTurnEvent>) -> Result<Self, crate::CodecError> {
        let turn = Self { events };
        turn.validate()?;
        Ok(turn)
    }

    /// Builds a one-event assertion turn.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the resulting turn is invalid.
    pub fn assertion(
        oid: u64,
        handle: u64,
        assertion: &animus_adapters::Assertion,
    ) -> Result<Self, crate::CodecError> {
        Self::new(vec![SyndicateTurnEvent::from_assertion(
            oid, handle, assertion,
        )])
    }

    /// Builds a one-event retraction turn.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the resulting turn is invalid.
    pub fn retraction(oid: u64, handle: u64) -> Result<Self, crate::CodecError> {
        Self::new(vec![SyndicateTurnEvent::retract(oid, handle)])
    }

    /// Builds a one-event orchestration-message turn.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the resulting turn is invalid.
    pub fn orchestration_message(
        oid: u64,
        event: &animus_adapters::OrchestrationEvent,
    ) -> Result<Self, crate::CodecError> {
        Self::new(vec![SyndicateTurnEvent::from_orchestration_event(
            oid, event,
        )])
    }

    /// Returns ordered turn events.
    #[must_use]
    pub fn events(&self) -> &[SyndicateTurnEvent] {
        &self.events
    }

    /// Returns whether the turn has no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the number of ordered turn events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Validates the event bound and every typed body.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CodecError`] when the turn is unsafe or unbounded.
    pub fn validate(&self) -> Result<(), crate::CodecError> {
        if self.events.len() > MAX_SYNDICATE_TURN_EVENTS {
            return Err(invalid_packet(UNBOUNDED_TURN_REASON));
        }
        for event in &self.events {
            event.validate()?;
        }
        Ok(())
    }
}

/// Encodes a turn using Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_syndicate_turn_text(turn: &SyndicateTurn) -> Result<String, crate::CodecError> {
    // r[impl agent_harness_runtime.syndicate_turn_adapter.wire]
    let value = turn_to_value(turn)?;
    crate::family_codec::encode_text(&value)
}

/// Decodes a turn from Preserves text syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, shape, body validation, or bounds fail.
pub fn from_syndicate_turn_text(text: &str) -> Result<SyndicateTurn, crate::CodecError> {
    let value: IOValue =
        preserves::de::from_text(text).map_err(|_error| invalid_packet(INVALID_PACKET_REASON))?;
    turn_from_value(&value)
}

/// Encodes a turn using Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when validation or serialization fails.
pub fn to_syndicate_turn_bytes(turn: &SyndicateTurn) -> Result<Vec<u8>, crate::CodecError> {
    let value = turn_to_value(turn)?;
    crate::family_codec::encode_bytes(&value)
}

/// Decodes a turn from Preserves binary syntax.
///
/// # Errors
///
/// Returns [`crate::CodecError`] when syntax, shape, body validation, or bounds fail.
pub fn from_syndicate_turn_bytes(bytes: &[u8]) -> Result<SyndicateTurn, crate::CodecError> {
    let value: IOValue =
        preserves::de::from_bytes(bytes).map_err(|_error| invalid_packet(INVALID_PACKET_REASON))?;
    turn_from_value(&value)
}

fn turn_to_value(turn: &SyndicateTurn) -> Result<IOValue, crate::CodecError> {
    turn.validate()?;
    let events = turn
        .events()
        .iter()
        .map(turn_event_to_value)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Value::Sequence(events).wrap())
}

fn turn_event_to_value(event: &SyndicateTurnEvent) -> Result<IOValue, crate::CodecError> {
    event.validate()?;
    Ok(Value::Sequence(vec![
        Value::from(event.oid()).wrap(),
        event_to_value(event.event()),
    ])
    .wrap())
}

fn event_to_value(event: &SyndicateEvent) -> IOValue {
    let fields = match event {
        SyndicateEvent::Assert { assertion, handle } => vec![
            preserves::value::to_value(assertion),
            Value::from(*handle).wrap(),
        ],
        SyndicateEvent::Retract { handle } => vec![Value::from(*handle).wrap()],
        SyndicateEvent::Message { body } => vec![preserves::value::to_value(body)],
    };
    let label = match event.kind() {
        SyndicateEventKind::Assert => ASSERT_EVENT_LABEL,
        SyndicateEventKind::Retract => RETRACT_EVENT_LABEL,
        SyndicateEventKind::Message => MESSAGE_EVENT_LABEL,
    };
    simple_record(label, fields)
}

fn turn_from_value(value: &IOValue) -> Result<SyndicateTurn, crate::CodecError> {
    let Value::Sequence(events) = value.value() else {
        return Err(invalid_packet(INVALID_PACKET_REASON));
    };
    if events.len() > MAX_SYNDICATE_TURN_EVENTS {
        return Err(invalid_packet(UNBOUNDED_TURN_REASON));
    }
    let decoded = events
        .iter()
        .map(turn_event_from_value)
        .collect::<Result<Vec<_>, _>>()?;
    SyndicateTurn::new(decoded)
}

fn turn_event_from_value(value: &IOValue) -> Result<SyndicateTurnEvent, crate::CodecError> {
    let Value::Sequence(fields) = value.value() else {
        return Err(invalid_packet(INVALID_TURN_EVENT_REASON));
    };
    if fields.len() != TURN_EVENT_FIELDS {
        return Err(invalid_packet(INVALID_TURN_EVENT_REASON));
    }
    let oid = fields[0]
        .value()
        .as_u64()
        .ok_or_else(|| invalid_packet(INVALID_OID_REASON))?;
    let event = event_from_value(&fields[1])?;
    let turn_event = SyndicateTurnEvent { oid, event };
    turn_event.validate()?;
    Ok(turn_event)
}

fn event_from_value(value: &IOValue) -> Result<SyndicateEvent, crate::CodecError> {
    // r[impl agent_harness_runtime.syndicate_turn_adapter.fail_closed]
    if let Some([assertion_value, handle_value]) = value
        .value()
        .as_simple_record(ASSERT_EVENT_LABEL, Some(ASSERT_EVENT_ARITY))
    {
        let assertion: crate::DataspaceRecord = preserves::value::from_value(assertion_value)
            .map_err(|_error| invalid_packet(INVALID_BODY_REASON))?;
        assertion.validate()?;
        if assertion.payload_kind() != crate::DataspacePayloadKind::Assertion {
            return Err(invalid_packet(UNSUPPORTED_EVENT_REASON));
        }
        return Ok(SyndicateEvent::Assert {
            assertion,
            handle: handle_from_value(handle_value)?,
        });
    }
    if let Some([handle_value]) = value
        .value()
        .as_simple_record(RETRACT_EVENT_LABEL, Some(RETRACT_EVENT_ARITY))
    {
        return Ok(SyndicateEvent::Retract {
            handle: handle_from_value(handle_value)?,
        });
    }
    if let Some([body_value]) = value
        .value()
        .as_simple_record(MESSAGE_EVENT_LABEL, Some(MESSAGE_EVENT_ARITY))
    {
        let body: crate::DataspaceRecord = preserves::value::from_value(body_value)
            .map_err(|_error| invalid_packet(INVALID_BODY_REASON))?;
        body.validate()?;
        return Ok(SyndicateEvent::Message { body });
    }
    if value
        .value()
        .as_simple_record(SYNC_EVENT_LABEL, None)
        .is_some()
    {
        return Err(invalid_packet(UNSUPPORTED_EVENT_REASON));
    }
    Err(invalid_packet(UNSUPPORTED_EVENT_REASON))
}

fn handle_from_value(value: &IOValue) -> Result<u64, crate::CodecError> {
    value
        .value()
        .as_u64()
        .ok_or_else(|| invalid_packet(INVALID_HANDLE_REASON))
}

fn simple_record(label: &str, fields: Vec<IOValue>) -> IOValue {
    let mut record = Value::<IOValue>::simple_record(label, fields.len());
    record.fields_vec_mut().extend(fields);
    record.finish().wrap()
}

fn invalid_packet(reason: &'static str) -> crate::CodecError {
    crate::CodecError::InvalidSyndicatePacket { reason }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TENANT: &str = "tenant:local";
    const WORKSPACE: &str = "workspace:main";
    const SESSION: &str = "session:turn";
    const BRANCH: &str = "branch:main";
    const FACET: &str = "facet:relay";
    const ACTOR_KIND: &str = "agent";
    const ACTOR_ID: &str = "agent:main";
    const ASSERTION_ID: &str = "assertion:active";
    const EVENT_ID: &str = "event:cancel";
    const SAFE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const REDACTION_PROMPT: &str = "prompt_omitted";
    const OID: u64 = 0;
    const HANDLE: u64 = 7;
    const SECOND_HANDLE: u64 = 8;
    const ITEM_COUNT: u32 = 1;

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
            kind: "agent_lifecycle",
            status: "active",
            scope: scope(),
            redactions: &[REDACTION_PROMPT],
            content_hash: Some(SAFE_HASH),
            item_count: ITEM_COUNT,
            lease_expires_at: None,
        })
        .expect("assertion should validate")
    }

    fn orchestration_event() -> animus_adapters::OrchestrationEvent {
        animus_adapters::OrchestrationEvent::new(animus_adapters::OrchestrationEventDraft {
            stable_id: EVENT_ID,
            kind: "cancelled",
            scope: scope(),
            target_owner: None,
        })
        .expect("event should validate")
    }

    #[test]
    fn exact_turn_profile_round_trips_text_and_binary_in_order() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.wire]
        // r[verify agent_harness_runtime.syndicate_turn_adapter.projection]
        let turn = SyndicateTurn::new(vec![
            SyndicateTurnEvent::from_assertion(OID, HANDLE, &assertion()),
            SyndicateTurnEvent::from_orchestration_event(OID, &orchestration_event()),
            SyndicateTurnEvent::retract(OID, HANDLE),
        ])
        .expect("turn should validate");

        let text = to_syndicate_turn_text(&turn).expect("text should encode");
        let bytes = to_syndicate_turn_bytes(&turn).expect("binary should encode");

        let expected_tail = format!("<R {HANDLE}>]]");
        assert!(text.starts_with("[[0, <A "), "unexpected turn text: {text}");
        assert!(text.contains("<M "), "unexpected turn text: {text}");
        assert!(
            text.ends_with(&expected_tail),
            "unexpected turn text: {text}"
        );
        assert_eq!(
            from_syndicate_turn_text(&text).expect("text should decode"),
            turn
        );
        assert_eq!(
            from_syndicate_turn_bytes(&bytes).expect("binary should decode"),
            turn
        );
    }

    #[test]
    fn unsupported_capabilities_and_unsafe_bodies_fail_closed() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.fail_closed]
        let sync = from_syndicate_turn_text("[[0 <S #:#t>]]")
            .expect_err("embedded sync capability must fail");
        let arbitrary_packet = format!("[[0 <A \"raw_prompt: secret\" {HANDLE}>]]");
        let arbitrary = from_syndicate_turn_text(&arbitrary_packet)
            .expect_err("arbitrary assertion body must fail");
        let negative_handle =
            from_syndicate_turn_text("[[0 <R -1>]]").expect_err("negative handle must fail");
        let malformed = from_syndicate_turn_text("<M>").expect_err("non-turn packet must fail");
        let malformed_sensitive = from_syndicate_turn_text("[[0 <A \"raw_prompt: secret")
            .expect_err("malformed sensitive packet must fail safely");

        assert!(
            matches!(&sync, crate::CodecError::InvalidSyndicatePacket { .. }),
            "unexpected sync error: {sync}"
        );
        assert!(!arbitrary.to_string().contains("secret"));
        assert!(
            matches!(
                &negative_handle,
                crate::CodecError::InvalidSyndicatePacket { .. }
            ),
            "unexpected handle error: {negative_handle}"
        );
        assert!(
            matches!(&malformed, crate::CodecError::InvalidSyndicatePacket { .. }),
            "unexpected malformed error: {malformed}"
        );
        assert!(!malformed_sensitive.to_string().contains("secret"));
    }

    #[test]
    fn turn_event_bound_is_enforced() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.verification]
        let events = (0..=MAX_SYNDICATE_TURN_EVENTS)
            .map(|index| SyndicateTurnEvent::retract(OID, index as u64))
            .collect();
        let error = SyndicateTurn::new(events).expect_err("oversized turn must fail");
        assert!(matches!(
            error,
            crate::CodecError::InvalidSyndicatePacket { .. }
        ));
    }

    #[test]
    fn assertion_event_rejects_non_assertion_dataspace_record() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.projection]
        let body = crate::DataspaceRecord::from_orchestration_event(&orchestration_event());
        let turn = SyndicateTurn::new(vec![SyndicateTurnEvent {
            oid: OID,
            event: SyndicateEvent::Assert {
                assertion: body,
                handle: SECOND_HANDLE,
            },
        }]);
        assert!(matches!(
            turn,
            Err(crate::CodecError::InvalidSyndicatePacket { .. })
        ));
    }
}
