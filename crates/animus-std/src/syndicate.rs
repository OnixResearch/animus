//! Host-owned Syndicate Protocol packet adapter.
//!
//! This shell preserves one binary Preserves packet per Syndicate turn while a
//! caller-owned transport supplies session security, ordering, reliability, and
//! lifecycle. Capability membranes and live network construction remain outside
//! this crate.

use std::collections::BTreeSet;

/// Default maximum encoded Syndicate packet size.
pub const DEFAULT_SYNDICATE_MAX_PACKET_BYTES: usize = 64 * 1024;

const OUTBOUND_DIRECTION: &str = "outbound";
const INBOUND_DIRECTION: &str = "inbound";
const INVALID_PACKET_BOUND_REASON: &str = "maximum Syndicate packet bytes must be non-zero";

/// Caller-owned transport for complete Syndicate Preserves packets.
///
/// Implementations must provide a reliable, in-order, confidential, integral,
/// authentic, replay-resistant session and preserve one packet per call.
pub trait SyndicatePacketTransport {
    /// Sends one complete binary Preserves packet.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when the host cannot send the packet safely.
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), SyndicateProtocolError>;

    /// Receives one complete binary Preserves packet.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when the host cannot receive a complete packet safely.
    fn receive_packet(&mut self) -> Result<Vec<u8>, SyndicateProtocolError>;
}

/// Directional assertion-handle state for one Syndicate transport session.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SyndicateHandleState {
    active: BTreeSet<u64>,
}

impl SyndicateHandleState {
    /// Returns whether a handle is currently active.
    #[must_use]
    pub fn contains(&self, handle: u64) -> bool {
        self.active.contains(&handle)
    }

    /// Returns the number of active assertion handles.
    #[must_use]
    pub fn len(&self) -> usize {
        self.active.len()
    }

    /// Returns whether no assertion handles are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.active.is_empty()
    }
}

/// Purely plans the next directional handle state for one ordered turn.
///
/// # Errors
///
/// Returns [`SyndicateProtocolError`] when an assert reuses a live handle or a
/// retract names an unknown handle.
pub fn plan_syndicate_handle_transition(
    current: &SyndicateHandleState,
    turn: &animus_preserves::SyndicateTurn,
) -> Result<SyndicateHandleState, SyndicateProtocolError> {
    // r[impl agent_harness_runtime.syndicate_turn_adapter.lifecycle]
    turn.validate()?;
    let mut next = current.clone();
    for turn_event in turn.events() {
        match turn_event.event() {
            animus_preserves::SyndicateEvent::Assert { handle, .. } => {
                if !next.active.insert(*handle) {
                    return Err(SyndicateProtocolError::DuplicateHandle { handle: *handle });
                }
            }
            animus_preserves::SyndicateEvent::Retract { handle } => {
                if !next.active.remove(handle) {
                    return Err(SyndicateProtocolError::UnknownHandle { handle: *handle });
                }
            }
            animus_preserves::SyndicateEvent::Message { .. } => {}
        }
    }
    Ok(next)
}

/// Error returned by the bounded Syndicate protocol adapter.
#[derive(Debug)]
pub enum SyndicateProtocolError {
    /// Preserves or dataspace DTO validation failed.
    Codec {
        /// Source codec failure.
        source: animus_preserves::CodecError,
    },
    /// An assert attempted to reuse a live connection-scoped handle.
    DuplicateHandle {
        /// Reused handle.
        handle: u64,
    },
    /// A retract named a handle that is not active in this direction.
    UnknownHandle {
        /// Unknown handle.
        handle: u64,
    },
    /// Configured packet bound was zero.
    InvalidPacketBound,
    /// Encoded or received packet exceeded the configured bound.
    PacketTooLarge {
        /// Safe packet direction label.
        direction: &'static str,
        /// Observed packet size.
        actual: usize,
        /// Configured packet size limit.
        limit: usize,
    },
    /// Caller-owned transport failed safely.
    Transport {
        /// Safe bounded reason without packet or credential content.
        reason: &'static str,
    },
}

impl SyndicateProtocolError {
    /// Builds a safe caller-owned transport failure.
    #[must_use]
    pub const fn transport(reason: &'static str) -> Self {
        Self::Transport { reason }
    }
}

impl From<animus_preserves::CodecError> for SyndicateProtocolError {
    fn from(source: animus_preserves::CodecError) -> Self {
        Self::Codec { source }
    }
}

impl core::fmt::Display for SyndicateProtocolError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Codec { source } => write!(formatter, "Syndicate packet codec failed: {source}"),
            Self::DuplicateHandle { handle } => {
                write!(
                    formatter,
                    "Syndicate assertion handle {handle} is already active"
                )
            }
            Self::UnknownHandle { handle } => {
                write!(
                    formatter,
                    "Syndicate assertion handle {handle} is not active"
                )
            }
            Self::InvalidPacketBound => formatter.write_str(INVALID_PACKET_BOUND_REASON),
            Self::PacketTooLarge {
                direction,
                actual,
                limit,
            } => write!(
                formatter,
                "{direction} Syndicate packet has {actual} bytes; limit is {limit}"
            ),
            Self::Transport { reason } => write!(formatter, "Syndicate transport failed: {reason}"),
        }
    }
}

impl std::error::Error for SyndicateProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Codec { source } => Some(source),
            Self::DuplicateHandle { .. }
            | Self::UnknownHandle { .. }
            | Self::InvalidPacketBound
            | Self::PacketTooLarge { .. }
            | Self::Transport { .. } => None,
        }
    }
}

/// Feature-gated host adapter for the capability-free Syndicate turn profile.
pub struct SyndicateProtocolAdapter<T> {
    transport: T,
    max_packet_bytes: usize,
    outbound_handles: SyndicateHandleState,
    inbound_handles: SyndicateHandleState,
}

impl<T> SyndicateProtocolAdapter<T>
where
    T: SyndicatePacketTransport,
{
    /// Builds an adapter with an explicit non-zero packet bound.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError::InvalidPacketBound`] when the bound is zero.
    pub fn new(transport: T, max_packet_bytes: usize) -> Result<Self, SyndicateProtocolError> {
        if max_packet_bytes == 0 {
            return Err(SyndicateProtocolError::InvalidPacketBound);
        }
        Ok(Self {
            transport,
            max_packet_bytes,
            outbound_handles: SyndicateHandleState::default(),
            inbound_handles: SyndicateHandleState::default(),
        })
    }

    /// Builds an adapter with [`DEFAULT_SYNDICATE_MAX_PACKET_BYTES`].
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] if the default bound becomes invalid.
    pub fn with_default_packet_bound(transport: T) -> Result<Self, SyndicateProtocolError> {
        Self::new(transport, DEFAULT_SYNDICATE_MAX_PACKET_BYTES)
    }

    /// Returns the wrapped caller-owned transport.
    #[must_use]
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// Returns mutable access to the wrapped caller-owned transport.
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }

    /// Returns the configured packet bound.
    #[must_use]
    pub const fn max_packet_bytes(&self) -> usize {
        self.max_packet_bytes
    }

    /// Returns outbound assertion-handle state.
    #[must_use]
    pub const fn outbound_handles(&self) -> &SyndicateHandleState {
        &self.outbound_handles
    }

    /// Returns inbound assertion-handle state.
    #[must_use]
    pub const fn inbound_handles(&self) -> &SyndicateHandleState {
        &self.inbound_handles
    }

    /// Sends one complete turn after pure handle planning and packet bounding.
    ///
    /// The planned state is committed only after the transport reports success.
    /// A host must reset or reconcile the session after an ambiguous transport
    /// failure rather than assuming the remote peer received nothing.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when lifecycle, encoding, bounds, or transport fail.
    pub fn send_turn(
        &mut self,
        turn: &animus_preserves::SyndicateTurn,
    ) -> Result<(), SyndicateProtocolError> {
        // r[impl agent_harness_runtime.syndicate_turn_adapter.host]
        let next = plan_syndicate_handle_transition(&self.outbound_handles, turn)?;
        let packet = animus_preserves::to_syndicate_turn_bytes(turn)?;
        validate_packet_size(OUTBOUND_DIRECTION, packet.len(), self.max_packet_bytes)?;
        self.transport.send_packet(&packet)?;
        self.outbound_handles = next;
        Ok(())
    }

    /// Receives, bounds, decodes, and commits one complete turn packet.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when transport, bounds, decoding, or lifecycle fail.
    pub fn receive_turn(
        &mut self,
    ) -> Result<animus_preserves::SyndicateTurn, SyndicateProtocolError> {
        let packet = self.transport.receive_packet()?;
        validate_packet_size(INBOUND_DIRECTION, packet.len(), self.max_packet_bytes)?;
        let turn = animus_preserves::from_syndicate_turn_bytes(&packet)?;
        let next = plan_syndicate_handle_transition(&self.inbound_handles, &turn)?;
        self.inbound_handles = next;
        Ok(turn)
    }

    /// Projects and sends one assertion turn.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when projection, lifecycle, bounds, or transport fail.
    pub fn publish_assertion(
        &mut self,
        oid: u64,
        handle: u64,
        assertion: &animus_adapters::Assertion,
    ) -> Result<(), SyndicateProtocolError> {
        let turn = animus_preserves::SyndicateTurn::assertion(oid, handle, assertion)?;
        self.send_turn(&turn)
    }

    /// Projects and sends one retraction turn.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when lifecycle, bounds, or transport fail.
    pub fn retract_assertion(
        &mut self,
        oid: u64,
        handle: u64,
    ) -> Result<(), SyndicateProtocolError> {
        let turn = animus_preserves::SyndicateTurn::retraction(oid, handle)?;
        self.send_turn(&turn)
    }

    /// Projects and sends one orchestration event as a Syndicate message turn.
    ///
    /// # Errors
    ///
    /// Returns [`SyndicateProtocolError`] when projection, bounds, or transport fail.
    pub fn send_orchestration_event(
        &mut self,
        oid: u64,
        event: &animus_adapters::OrchestrationEvent,
    ) -> Result<(), SyndicateProtocolError> {
        let turn = animus_preserves::SyndicateTurn::orchestration_message(oid, event)?;
        self.send_turn(&turn)
    }
}

fn validate_packet_size(
    direction: &'static str,
    actual: usize,
    limit: usize,
) -> Result<(), SyndicateProtocolError> {
    if actual <= limit {
        return Ok(());
    }
    Err(SyndicateProtocolError::PacketTooLarge {
        direction,
        actual,
        limit,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

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
    const TRANSPORT_FAILURE: &str = "fixture transport unavailable";
    const OID: u64 = 0;
    const HANDLE: u64 = 7;
    const ITEM_COUNT: u32 = 1;
    const TINY_PACKET_BOUND: usize = 1;
    const EXPECTED_SENT_PACKET_COUNT: usize = 3;

    #[derive(Default)]
    struct MemoryPacketTransport {
        sent: Vec<Vec<u8>>,
        incoming: VecDeque<Vec<u8>>,
        fail_send: bool,
        fail_receive: bool,
    }

    impl SyndicatePacketTransport for MemoryPacketTransport {
        fn send_packet(&mut self, packet: &[u8]) -> Result<(), SyndicateProtocolError> {
            if self.fail_send {
                return Err(SyndicateProtocolError::transport(TRANSPORT_FAILURE));
            }
            self.sent.push(packet.to_vec());
            Ok(())
        }

        fn receive_packet(&mut self) -> Result<Vec<u8>, SyndicateProtocolError> {
            if self.fail_receive {
                return Err(SyndicateProtocolError::transport(TRANSPORT_FAILURE));
            }
            self.incoming
                .pop_front()
                .ok_or_else(|| SyndicateProtocolError::transport(TRANSPORT_FAILURE))
        }
    }

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
    fn adapter_preserves_turn_packets_and_directional_handle_state() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.host]
        // r[verify agent_harness_runtime.syndicate_turn_adapter.lifecycle]
        let mut adapter =
            SyndicateProtocolAdapter::with_default_packet_bound(MemoryPacketTransport::default())
                .expect("adapter should build");

        adapter
            .publish_assertion(OID, HANDLE, &assertion())
            .expect("assertion turn should send");
        adapter
            .send_orchestration_event(OID, &orchestration_event())
            .expect("message turn should send");
        adapter
            .retract_assertion(OID, HANDLE)
            .expect("retraction turn should send");

        assert!(adapter.outbound_handles().is_empty());
        assert_eq!(adapter.transport().sent.len(), EXPECTED_SENT_PACKET_COUNT);
        let first = animus_preserves::from_syndicate_turn_bytes(&adapter.transport().sent[0])
            .expect("sent packet should decode");
        assert_eq!(first.events()[0].oid(), OID);
        assert_eq!(
            first.events()[0].event().kind(),
            animus_preserves::SyndicateEventKind::Assert
        );
    }

    #[test]
    fn inbound_turn_is_bounded_decoded_and_committed() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.host]
        let turn = animus_preserves::SyndicateTurn::assertion(OID, HANDLE, &assertion())
            .expect("turn should build");
        let packet = animus_preserves::to_syndicate_turn_bytes(&turn).expect("turn should encode");
        let mut transport = MemoryPacketTransport::default();
        transport.incoming.push_back(packet);
        let mut adapter = SyndicateProtocolAdapter::with_default_packet_bound(transport)
            .expect("adapter should build");

        let received = adapter.receive_turn().expect("turn should receive");

        assert_eq!(received, turn);
        assert!(adapter.inbound_handles().contains(HANDLE));
        assert!(adapter.outbound_handles().is_empty());
    }

    #[test]
    fn duplicate_stale_and_failed_transitions_do_not_commit() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.fail_closed]
        let mut adapter =
            SyndicateProtocolAdapter::with_default_packet_bound(MemoryPacketTransport::default())
                .expect("adapter should build");
        adapter
            .publish_assertion(OID, HANDLE, &assertion())
            .expect("initial assertion should send");

        let duplicate = adapter
            .publish_assertion(OID, HANDLE, &assertion())
            .expect_err("duplicate handle must fail");
        assert!(matches!(
            duplicate,
            SyndicateProtocolError::DuplicateHandle { .. }
        ));
        assert!(adapter.outbound_handles().contains(HANDLE));

        adapter
            .retract_assertion(OID, HANDLE)
            .expect("live handle should retract");
        let stale = adapter
            .retract_assertion(OID, HANDLE)
            .expect_err("stale retraction must fail");
        assert!(matches!(
            stale,
            SyndicateProtocolError::UnknownHandle { .. }
        ));
        assert!(adapter.outbound_handles().is_empty());

        adapter.transport_mut().fail_send = true;
        let failed = adapter
            .publish_assertion(OID, HANDLE, &assertion())
            .expect_err("transport failure must propagate");
        assert!(matches!(failed, SyndicateProtocolError::Transport { .. }));
        assert!(adapter.outbound_handles().is_empty());
    }

    #[test]
    fn packet_bounds_apply_before_send_and_decode() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.verification]
        let mut outbound =
            SyndicateProtocolAdapter::new(MemoryPacketTransport::default(), TINY_PACKET_BOUND)
                .expect("tiny bounded adapter should build");
        let too_large = outbound
            .publish_assertion(OID, HANDLE, &assertion())
            .expect_err("oversized outbound packet must fail");
        assert!(matches!(
            too_large,
            SyndicateProtocolError::PacketTooLarge {
                direction: OUTBOUND_DIRECTION,
                ..
            }
        ));
        assert!(outbound.transport().sent.is_empty());
        assert!(outbound.outbound_handles().is_empty());

        let turn = animus_preserves::SyndicateTurn::assertion(OID, HANDLE, &assertion())
            .expect("turn should build");
        let packet = animus_preserves::to_syndicate_turn_bytes(&turn).expect("turn should encode");
        let mut transport = MemoryPacketTransport::default();
        transport.incoming.push_back(packet);
        let mut inbound = SyndicateProtocolAdapter::new(transport, TINY_PACKET_BOUND)
            .expect("tiny bounded adapter should build");
        let too_large = inbound
            .receive_turn()
            .expect_err("oversized inbound packet must fail");
        assert!(matches!(
            too_large,
            SyndicateProtocolError::PacketTooLarge {
                direction: INBOUND_DIRECTION,
                ..
            }
        ));
        assert!(inbound.inbound_handles().is_empty());
    }

    #[test]
    fn zero_packet_bound_is_rejected() {
        // r[verify agent_harness_runtime.syndicate_turn_adapter.verification]
        let result = SyndicateProtocolAdapter::new(MemoryPacketTransport::default(), 0);
        assert!(matches!(
            result,
            Err(SyndicateProtocolError::InvalidPacketBound)
        ));
    }
}
