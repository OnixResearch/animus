## Context

The bounded Syndicate profile emits one binary Preserves packet per turn and commits directional assertion-handle state only after `SyndicatePacketTransport` reports success. The Iroh seam validates safe route metadata and remote admission but intentionally owns no endpoint. The existing `IrohAgentTransport` carries typed direct assertion/event operations and redacted frame metadata; extending it with raw Syndicate packets would overlap coordination semantics and force unrelated implementations to support a second protocol mode.

The composition must therefore adapt a distinct caller-owned packet session into `SyndicatePacketTransport`, while reusing `IrohRouteDescriptor` only for safe route identity, admission, and byte ceilings.

## Decisions

### 1. Use an explicit composition feature

**Choice:** Add `iroh-syndicate-host = ["iroh-host", "syndicate-host"]` to `animus-std` and the root facade. Compile and export the bridge only under that feature.

**Rationale:** Explicit opt-in prevents enabling either independent seam from silently changing its public protocol surface and makes combined-feature verification straightforward.

### 2. Keep the packet session distinct from direct Iroh agent-comms

**Choice:** Add `IrohSyndicatePacketSession` with one send and one receive operation over an `IrohRouteDescriptor`. Add `IrohSyndicatePacketTransport<T>` that owns one admitted route and one caller-provided session and implements `SyndicatePacketTransport`.

**Rationale:** A separate port makes it impossible for Animus to reinterpret legacy metadata frames as Syndicate turns. Hosts may use the same underlying endpoint implementation, but must bind a session to exactly one coordination profile themselves.

### 3. Reuse the existing route protocol and profile binding

**Choice:** Require the existing `IROH_AGENT_COMMS_PROTOCOL` route descriptor and expose the fixed `SYNDICATE_PROTOCOL_PROFILE` through the bridge. Constructor admission verifies the route is admitted for its own scope. No second Animus envelope or ALPN string is introduced.

**Rationale:** The prior architecture decision makes `animus.agent-comms.v1` an Animus profile over Syndicate turns rather than a competing coordination protocol. The application profile remains explicit in the bridge API and documentation.

### 4. Enforce route bounds and scope in a pure core before effects

**Choice:** A deterministic packet admission function first compares packet length with the route's configured maximum in either direction, then decodes the already-validated turn packet only to require every assertion/message body scope to equal the admitted route scope. The shell runs admission before outbound send and immediately after inbound receive. Diagnostics contain only safe classes, direction, observed length, and configured limit.

**Rationale:** The outer Syndicate adapter has its own bound but no route knowledge. The transport route can be stricter, and a structurally valid cross-tenant turn must not escape over an admitted route. Outbound route rejection must happen before side effects.

### 5. Preserve existing handle commit semantics

**Choice:** The bridge decodes only for route-scope admission and owns no assertion handles or delivered DTO state. `SyndicateProtocolAdapter<IrohSyndicatePacketTransport<T>>` remains the only stateful protocol shell. A failed bridge send returns `SyndicateProtocolError`, so the outer adapter does not commit its planned handle transition.

**Rationale:** Duplicating protocol state in the transport layer would create divergent lifecycle authorities, while omitting scope admission would permit valid cross-route DTOs.

### 6. Keep live Iroh authority caller-owned

**Choice:** The session trait documents the required reliability, ordering, confidentiality, integrity, peer authenticity, replay resistance, one-packet framing, and connected lifecycle. Animus does not add `iroh`, async runtime, socket, ticket, key, relay, discovery, retry, clock, filesystem, or persistence dependencies.

**Rationale:** The reusable composition seam can be tested deterministically while production network claims remain bounded to host evidence.

## Alternatives considered

- **Extend `IrohAgentTransport`: rejected.** Its typed assertion/query/event contract represents the older direct coordination path and would force all implementations to add raw packet methods or hidden defaults.
- **Implement `SyndicatePacketTransport` directly for `IrohAgentCommsAdapter<T>`: rejected.** That wrapper is semantically tied to direct metadata frames and has no receive port; combining both modes obscures which coordination protocol owns a session.
- **Document composition without code: rejected.** It leaves route admission, packet boundaries, and size-error behavior to ad hoc host implementations.

## Risks / Trade-offs

- The bridge proves only deterministic composition over fake sessions; a real Iroh host still needs independent endpoint and peer-admission evidence.
- A session send error can be delivery-ambiguous. The host must discard or reconcile the session rather than retrying under assumed non-delivery.
- Existing direct `IrohAgentCommsAdapter` behavior remains for compatibility, but documentation forbids mixing it with Syndicate turn packets on the same logical session.
- The route and outer Syndicate adapter can have different size ceilings; the effective ceiling is the smaller one.

## Non-Goals

- Constructing an Iroh endpoint, stream, ticket, node address, relay, key, discovery service, async executor, retry loop, durable session, or reconnect policy.
- Implementing Syndicate `S`, WireRefs, membranes, caveats, relay actors, automatic disconnect retraction, or full capability interoperability.
- Claiming live network interoperability from in-memory tests.
