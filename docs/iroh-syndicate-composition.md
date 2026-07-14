# Iroh transport composition for Syndicate turns

Animus composes its bounded Syndicate turn profile with its host-owned Iroh route seam behind the non-default `iroh-syndicate-host` feature. This feature enables `iroh-host` and `syndicate-host`; either underlying feature remains independently usable.

## Layering

```text
SyndicateProtocolAdapter
  -> one validated binary Preserves turn packet
  -> IrohSyndicatePacketTransport
  -> caller-owned IrohSyndicatePacketSession
  -> host-owned endpoint / secure ordered stream
```

`IrohSyndicatePacketTransport<T>` binds one admitted `IrohRouteDescriptor` using `animus.agent-comms.v1` to one caller-owned session and implements `SyndicatePacketTransport`. The fixed application profile supplied on every session call is `syndicate.protocol.v1.animus-safe`.

The bridge does not add an Animus frame envelope, convert turns into redacted `AgentCommsFrame` metadata, split packets, or join packets. It decodes the packet only for fail-closed route-scope admission; the enclosing Syndicate adapter remains the owner of protocol DTO delivery and handle state. Exactly one complete binary turn packet crosses each session effect.

## Host session contract

An `IrohSyndicatePacketSession` implementation must supply an already-established session with:

- reliable, in-order delivery;
- confidentiality, integrity, peer authenticity, and replay resistance;
- one complete packet per send/receive call;
- explicit connected and disconnected lifecycle handling; and
- host-owned endpoint, ticket, key, relay, admission, retry, and reconciliation policy.

Animus does not depend on the Iroh crate or construct endpoints, sockets, tickets, keys, relays, discovery clients, retry loops, async executors, clocks, files, or durable session state.

## Admission and bounds

Construction fails unless the `IrohRouteDescriptor` has admitted remote access for its scope. The bridge enforces the route's `max_frame_bytes` and requires every assertion/message body to match the exact admitted route scope before an outbound session effect and immediately after an inbound receive. The enclosing `SyndicateProtocolAdapter` also enforces its own packet ceiling, so the effective limit is the smaller configured value.

If session send fails, the bridge returns a bounded `SyndicateProtocolError::Transport`. The enclosing protocol adapter does not commit its planned assertion handles. Because remote delivery can still be ambiguous, the host must discard or reconcile the session rather than retrying under an assumption of non-delivery.

## One coordination protocol per session

Do not use `IrohAgentCommsAdapter` direct assertion/event operations and `IrohSyndicatePacketTransport` on the same logical session. The former is retained as a compatibility seam; the latter makes Syndicate the coordination protocol and Iroh only the transport. A host can share endpoint infrastructure, but must bind each logical session to one application profile.

## Non-claims

In-memory conformance tests establish packet preservation, route admission, bounds, safe failure mapping, and Syndicate handle non-commit. They do not establish live Iroh endpoint interoperability, remote peer correctness, relay behavior, retry safety, availability, disconnect cleanup, or full Syndicate WireRef/capability support.

Focused checks:

```sh
nix develop -c cargo test -p animus-std --features iroh-syndicate-host irohsyndicate --locked
nix develop -c cargo test -p animus-std --features iroh-syndicate-host --locked
nix develop -c cargo check -p animus --features iroh-syndicate-host --locked
```
