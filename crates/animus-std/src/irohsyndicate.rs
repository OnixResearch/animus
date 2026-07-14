//! Explicit composition of admitted Iroh routes and bounded Syndicate turn packets.
//!
//! This module owns no endpoint. A caller provides an already-established packet
//! session with the security and lifecycle properties required by the Syndicate
//! transport contract.

const OUTBOUND_DIRECTION: &str = "outbound";
const INBOUND_DIRECTION: &str = "inbound";
const IROH_SYNDICATE_SESSION_FAILURE: &str = "Iroh Syndicate packet session failed";
const IROH_SYNDICATE_SCOPE_MISMATCH: &str =
    "Iroh Syndicate packet scope does not match the admitted route";

/// Caller-owned Iroh session for complete Syndicate turn packets.
///
/// One call must correspond to one complete binary Preserves packet. Implementations
/// must provide reliable, in-order, confidential, integral, peer-authenticated,
/// replay-resistant delivery and an explicit connected/disconnected lifecycle.
pub trait IrohSyndicatePacketSession {
    /// Sends one complete Syndicate packet on the admitted route and profile.
    ///
    /// # Errors
    ///
    /// Returns [`crate::IrohAdapterError`] when the caller-owned session cannot send safely.
    fn send_packet(
        &mut self,
        route: &crate::IrohRouteDescriptor,
        syndicate_profile: &str,
        packet: &[u8],
    ) -> Result<(), crate::IrohAdapterError>;

    /// Receives one complete Syndicate packet from the admitted route and profile.
    ///
    /// # Errors
    ///
    /// Returns [`crate::IrohAdapterError`] when the caller-owned session cannot receive safely.
    fn receive_packet(
        &mut self,
        route: &crate::IrohRouteDescriptor,
        syndicate_profile: &str,
    ) -> Result<Vec<u8>, crate::IrohAdapterError>;
}

/// Iroh route/session bridge implementing the bounded Syndicate packet transport port.
pub struct IrohSyndicatePacketTransport<T> {
    route: crate::IrohRouteDescriptor,
    session: T,
}

impl<T> IrohSyndicatePacketTransport<T> {
    /// Binds a caller-owned packet session to one admitted Iroh route.
    ///
    /// # Errors
    ///
    /// Returns [`crate::IrohAdapterError`] when remote admission is not granted.
    pub fn new(
        route: crate::IrohRouteDescriptor,
        session: T,
    ) -> Result<Self, crate::IrohAdapterError> {
        // r[impl agent_harness_runtime.syndicate_iroh_bridge.transport]
        crate::admit_iroh_scope(&route, route.scope())?;
        Ok(Self { route, session })
    }

    /// Returns the admitted Iroh route bound to this packet session.
    #[must_use]
    pub const fn route(&self) -> &crate::IrohRouteDescriptor {
        &self.route
    }

    /// Returns the fixed Syndicate profile carried by this session.
    #[must_use]
    pub const fn syndicate_profile() -> &'static str {
        animus_preserves::SYNDICATE_PROTOCOL_PROFILE
    }

    /// Returns the caller-owned packet session.
    #[must_use]
    pub const fn session(&self) -> &T {
        &self.session
    }

    /// Returns mutable access to the caller-owned packet session.
    pub fn session_mut(&mut self) -> &mut T {
        &mut self.session
    }
}

impl<T> crate::SyndicatePacketTransport for IrohSyndicatePacketTransport<T>
where
    T: IrohSyndicatePacketSession,
{
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), crate::SyndicateProtocolError> {
        // r[impl agent_harness_runtime.syndicate_iroh_bridge.framing]
        validate_route_packet(&self.route, PacketDirection::Outbound, packet)?;
        self.session
            .send_packet(&self.route, Self::syndicate_profile(), packet)
            .map_err(map_session_error)
    }

    fn receive_packet(&mut self) -> Result<Vec<u8>, crate::SyndicateProtocolError> {
        let packet = self
            .session
            .receive_packet(&self.route, Self::syndicate_profile())
            .map_err(map_session_error)?;
        validate_route_packet(&self.route, PacketDirection::Inbound, &packet)?;
        Ok(packet)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PacketDirection {
    Outbound,
    Inbound,
}

impl PacketDirection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Outbound => OUTBOUND_DIRECTION,
            Self::Inbound => INBOUND_DIRECTION,
        }
    }
}

fn validate_route_packet(
    route: &crate::IrohRouteDescriptor,
    direction: PacketDirection,
    packet: &[u8],
) -> Result<(), crate::SyndicateProtocolError> {
    // r[impl agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
    validate_route_packet_size(route, direction, packet.len())?;
    let turn = animus_preserves::from_syndicate_turn_bytes(packet)?;
    for turn_event in turn.events() {
        let record = turn_event
            .event()
            .assertion()
            .or_else(|| turn_event.event().message_body());
        if let Some(record) = record {
            validate_record_scope(route, record)?;
        }
    }
    Ok(())
}

fn validate_route_packet_size(
    route: &crate::IrohRouteDescriptor,
    direction: PacketDirection,
    actual: usize,
) -> Result<(), crate::SyndicateProtocolError> {
    let limit = route_packet_limit(route);
    if actual <= limit {
        return Ok(());
    }
    Err(crate::SyndicateProtocolError::PacketTooLarge {
        direction: direction.as_str(),
        actual,
        limit,
    })
}

fn validate_record_scope(
    route: &crate::IrohRouteDescriptor,
    record: &animus_preserves::DataspaceRecord,
) -> Result<(), crate::SyndicateProtocolError> {
    let payload = record.clone().into_payload();
    let kind = payload.kind();
    let scope = match kind {
        animus_preserves::DataspacePayloadKind::Scope => payload.try_into_scope()?,
        animus_preserves::DataspacePayloadKind::Assertion => {
            payload.try_into_assertion()?.scope().clone()
        }
        animus_preserves::DataspacePayloadKind::Subscription => {
            payload.try_into_subscription()?.scope().clone()
        }
        animus_preserves::DataspacePayloadKind::OrchestrationEvent => {
            payload.try_into_orchestration_event()?.scope().clone()
        }
    };
    if &scope == route.scope() {
        return Ok(());
    }
    Err(crate::SyndicateProtocolError::transport(
        IROH_SYNDICATE_SCOPE_MISMATCH,
    ))
}

fn route_packet_limit(route: &crate::IrohRouteDescriptor) -> usize {
    match usize::try_from(route.max_frame_bytes()) {
        Ok(limit) => limit,
        Err(_unsupported_pointer_width) => usize::MAX,
    }
}

fn map_session_error(_error: crate::IrohAdapterError) -> crate::SyndicateProtocolError {
    crate::SyndicateProtocolError::transport(IROH_SYNDICATE_SESSION_FAILURE)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    const TENANT: &str = "tenant:local";
    const WORKSPACE: &str = "workspace:main";
    const OTHER_WORKSPACE: &str = "workspace:other";
    const SESSION: &str = "session:iroh-syndicate";
    const BRANCH: &str = "branch:main";
    const FACET: &str = "facet:transport";
    const ACTOR_KIND: &str = "agent";
    const ACTOR_ID: &str = "agent:main";
    const NODE_ID: &str = "node:remote";
    const TOPIC_ID: &str = "topic:workspace-main";
    const ADMISSION_ADMITTED: &str = "admitted";
    const ADMISSION_DENIED: &str = "denied";
    const ASSERTION_ID: &str = "assertion:active";
    const EVENT_ID: &str = "event:cancel";
    const REDACTION_PROMPT: &str = "prompt_omitted";
    const SAFE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const LEAKING_FAILURE: &str = "raw_prompt: secret";
    const OID: u64 = 0;
    const HANDLE: u64 = 7;
    const ITEM_COUNT: u32 = 1;
    const ROUTE_PACKET_BYTES: u32 = 4096;
    const TINY_ROUTE_PACKET_BYTES: u32 = 1;
    const ZERO_ROUTE_PACKET_BYTES: u32 = 0;
    const EXPECTED_PACKET_COUNT: usize = 1;

    #[derive(Default)]
    struct MemoryIrohSyndicateSession {
        sent: Vec<Vec<u8>>,
        incoming: VecDeque<Vec<u8>>,
        profiles: Vec<String>,
        fail_send: bool,
        fail_receive: bool,
    }

    impl IrohSyndicatePacketSession for MemoryIrohSyndicateSession {
        fn send_packet(
            &mut self,
            _route: &crate::IrohRouteDescriptor,
            syndicate_profile: &str,
            packet: &[u8],
        ) -> Result<(), crate::IrohAdapterError> {
            if self.fail_send {
                return Err(crate::IrohAdapterError::Transport {
                    reason: LEAKING_FAILURE,
                });
            }
            self.profiles.push(syndicate_profile.to_owned());
            self.sent.push(packet.to_vec());
            Ok(())
        }

        fn receive_packet(
            &mut self,
            _route: &crate::IrohRouteDescriptor,
            syndicate_profile: &str,
        ) -> Result<Vec<u8>, crate::IrohAdapterError> {
            if self.fail_receive {
                return Err(crate::IrohAdapterError::Transport {
                    reason: LEAKING_FAILURE,
                });
            }
            self.profiles.push(syndicate_profile.to_owned());
            self.incoming
                .pop_front()
                .ok_or(crate::IrohAdapterError::LiveBackendUnavailable)
        }
    }

    fn scope_for(workspace: &str) -> animus_adapters::Scope {
        animus_adapters::Scope::new(animus_adapters::ScopeDraft {
            tenant_id: TENANT,
            workspace_id: workspace,
            session_id: SESSION,
            branch_id: BRANCH,
            facet_id: FACET,
            actor_kind: ACTOR_KIND,
            actor_id: ACTOR_ID,
        })
        .expect("scope should validate")
    }

    fn scope() -> animus_adapters::Scope {
        scope_for(WORKSPACE)
    }

    fn route(admission: &str, max_frame_bytes: u32) -> crate::IrohRouteDescriptor {
        crate::IrohRouteDescriptor::new(crate::IrohRouteDescriptorDraft {
            node_id: NODE_ID,
            topic_id: TOPIC_ID,
            protocol_id: crate::IROH_AGENT_COMMS_PROTOCOL,
            scope: scope(),
            ticket_hash: Some(SAFE_HASH),
            remote_admission: admission,
            max_frame_bytes,
        })
        .expect("route descriptor should validate")
    }

    fn assertion_for(workspace: &str) -> animus_adapters::Assertion {
        animus_adapters::Assertion::new(animus_adapters::AssertionDraft {
            stable_id: ASSERTION_ID,
            kind: "agent_lifecycle",
            status: "active",
            scope: scope_for(workspace),
            redactions: &[REDACTION_PROMPT],
            content_hash: Some(SAFE_HASH),
            item_count: ITEM_COUNT,
            lease_expires_at: None,
        })
        .expect("assertion should validate")
    }

    fn assertion() -> animus_adapters::Assertion {
        assertion_for(WORKSPACE)
    }

    fn orchestration_event_for(workspace: &str) -> animus_adapters::OrchestrationEvent {
        animus_adapters::OrchestrationEvent::new(animus_adapters::OrchestrationEventDraft {
            stable_id: EVENT_ID,
            kind: "cancelled",
            scope: scope_for(workspace),
            target_owner: None,
        })
        .expect("orchestration event should validate")
    }

    #[test]
    fn exact_packets_cross_the_admitted_iroh_session_in_both_directions() {
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.transport]
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.framing]
        let turn = animus_preserves::SyndicateTurn::assertion(OID, HANDLE, &assertion())
            .expect("turn should validate");
        let packet = animus_preserves::to_syndicate_turn_bytes(&turn).expect("turn should encode");
        let mut session = MemoryIrohSyndicateSession::default();
        session.incoming.push_back(packet.clone());
        let bridge = IrohSyndicatePacketTransport::new(
            route(ADMISSION_ADMITTED, ROUTE_PACKET_BYTES),
            session,
        )
        .expect("admitted bridge should build");
        let mut protocol = crate::SyndicateProtocolAdapter::with_default_packet_bound(bridge)
            .expect("protocol adapter should build");

        protocol.send_turn(&turn).expect("turn should send");
        let received = protocol.receive_turn().expect("turn should receive");

        assert_eq!(received, turn);
        assert!(protocol.outbound_handles().contains(HANDLE));
        assert!(protocol.inbound_handles().contains(HANDLE));
        assert_eq!(
            protocol.transport().session().sent,
            vec![packet],
            "one complete encoded turn must cross one session effect"
        );
        assert_eq!(
            protocol.transport().session().profiles.len(),
            EXPECTED_PACKET_COUNT + EXPECTED_PACKET_COUNT
        );
        assert!(
            protocol
                .transport()
                .session()
                .profiles
                .iter()
                .all(|profile| profile == animus_preserves::SYNDICATE_PROTOCOL_PROFILE)
        );
    }

    #[test]
    fn denied_and_zero_bound_routes_fail_before_session_use() {
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
        let denied = IrohSyndicatePacketTransport::new(
            route(ADMISSION_DENIED, ROUTE_PACKET_BYTES),
            MemoryIrohSyndicateSession::default(),
        );
        assert!(matches!(
            denied,
            Err(crate::IrohAdapterError::AdmissionRequired)
        ));

        let zero = crate::IrohRouteDescriptor::new(crate::IrohRouteDescriptorDraft {
            node_id: NODE_ID,
            topic_id: TOPIC_ID,
            protocol_id: crate::IROH_AGENT_COMMS_PROTOCOL,
            scope: scope(),
            ticket_hash: Some(SAFE_HASH),
            remote_admission: ADMISSION_ADMITTED,
            max_frame_bytes: ZERO_ROUTE_PACKET_BYTES,
        });
        assert!(matches!(
            zero,
            Err(crate::IrohAdapterError::UnsafeRoute { .. })
        ));
    }

    #[test]
    fn route_bounds_reject_packets_and_preserve_handle_state() {
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.framing]
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.verification]
        let bridge = IrohSyndicatePacketTransport::new(
            route(ADMISSION_ADMITTED, TINY_ROUTE_PACKET_BYTES),
            MemoryIrohSyndicateSession::default(),
        )
        .expect("bridge should build");
        let mut protocol = crate::SyndicateProtocolAdapter::with_default_packet_bound(bridge)
            .expect("protocol adapter should build");

        let outbound = protocol
            .publish_assertion(OID, HANDLE, &assertion())
            .expect_err("route must reject an oversized packet");

        assert!(matches!(
            outbound,
            crate::SyndicateProtocolError::PacketTooLarge {
                direction: OUTBOUND_DIRECTION,
                ..
            }
        ));
        assert!(protocol.outbound_handles().is_empty());
        assert!(protocol.transport().session().sent.is_empty());

        let oversized =
            vec![0; route_packet_limit(protocol.transport().route()) + EXPECTED_PACKET_COUNT];
        protocol
            .transport_mut()
            .session_mut()
            .incoming
            .push_back(oversized);
        let inbound = protocol
            .receive_turn()
            .expect_err("route must reject oversized inbound bytes before decoding");
        assert!(matches!(
            inbound,
            crate::SyndicateProtocolError::PacketTooLarge {
                direction: INBOUND_DIRECTION,
                ..
            }
        ));
        assert!(protocol.inbound_handles().is_empty());
    }

    #[test]
    fn cross_scope_packets_fail_before_delivery_or_handle_commit() {
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
        let cross_scope_turn = animus_preserves::SyndicateTurn::assertion(
            OID,
            HANDLE,
            &assertion_for(OTHER_WORKSPACE),
        )
        .expect("cross-scope turn remains structurally valid");
        let cross_scope_packet = animus_preserves::to_syndicate_turn_bytes(&cross_scope_turn)
            .expect("cross-scope turn should encode");
        let bridge = IrohSyndicatePacketTransport::new(
            route(ADMISSION_ADMITTED, ROUTE_PACKET_BYTES),
            MemoryIrohSyndicateSession::default(),
        )
        .expect("bridge should build");
        let mut outbound = crate::SyndicateProtocolAdapter::with_default_packet_bound(bridge)
            .expect("protocol adapter should build");

        let send = outbound
            .send_turn(&cross_scope_turn)
            .expect_err("cross-scope outbound packet must fail");
        assert!(matches!(
            send,
            crate::SyndicateProtocolError::Transport { .. }
        ));
        assert!(outbound.outbound_handles().is_empty());
        assert!(outbound.transport().session().sent.is_empty());

        let cross_scope_message = animus_preserves::SyndicateTurn::orchestration_message(
            OID,
            &orchestration_event_for(OTHER_WORKSPACE),
        )
        .expect("cross-scope message remains structurally valid");
        let message_send = outbound
            .send_turn(&cross_scope_message)
            .expect_err("cross-scope outbound message must fail");
        assert!(matches!(
            message_send,
            crate::SyndicateProtocolError::Transport { .. }
        ));
        assert!(outbound.transport().session().sent.is_empty());

        let mut session = MemoryIrohSyndicateSession::default();
        session.incoming.push_back(cross_scope_packet);
        let bridge = IrohSyndicatePacketTransport::new(
            route(ADMISSION_ADMITTED, ROUTE_PACKET_BYTES),
            session,
        )
        .expect("bridge should build");
        let mut inbound = crate::SyndicateProtocolAdapter::with_default_packet_bound(bridge)
            .expect("protocol adapter should build");
        let receive = inbound
            .receive_turn()
            .expect_err("cross-scope inbound packet must fail");
        assert!(matches!(
            receive,
            crate::SyndicateProtocolError::Transport { .. }
        ));
        assert!(inbound.inbound_handles().is_empty());
    }

    #[test]
    fn session_failures_are_bounded_and_do_not_commit_handles() {
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
        // r[verify agent_harness_runtime.syndicate_iroh_bridge.verification]
        let session = MemoryIrohSyndicateSession {
            fail_send: true,
            fail_receive: true,
            ..MemoryIrohSyndicateSession::default()
        };
        let bridge = IrohSyndicatePacketTransport::new(
            route(ADMISSION_ADMITTED, ROUTE_PACKET_BYTES),
            session,
        )
        .expect("bridge should build");
        let mut protocol = crate::SyndicateProtocolAdapter::with_default_packet_bound(bridge)
            .expect("protocol adapter should build");

        let send = protocol
            .publish_assertion(OID, HANDLE, &assertion())
            .expect_err("session send must fail");
        let receive = protocol
            .receive_turn()
            .expect_err("session receive must fail");

        assert!(matches!(
            send,
            crate::SyndicateProtocolError::Transport { .. }
        ));
        assert!(matches!(
            receive,
            crate::SyndicateProtocolError::Transport { .. }
        ));
        assert!(!send.to_string().contains("secret"));
        assert!(!receive.to_string().contains("secret"));
        assert!(protocol.outbound_handles().is_empty());
        assert!(protocol.inbound_handles().is_empty());
    }
}
