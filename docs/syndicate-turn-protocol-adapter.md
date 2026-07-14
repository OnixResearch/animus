# Syndicate turn protocol adapter

Animus exposes a capability-free Syndicate Protocol v1 profile behind the non-default `syndicate-host` feature. The profile label is `syndicate.protocol.v1.animus-safe`.

## Layering

```text
Animus assertion/orchestration DTOs
  -> validated versioned DataspaceRecord bodies
  -> Syndicate Turn packets in Preserves binary or text syntax
  -> caller-owned reliable, ordered, authenticated transport
```

Syndicate supplies coordination semantics and Preserves packet shapes. Iroh, local IPC, TLS streams, or another admitted host service supplies transport. Enabling this adapter does not construct Iroh endpoints, sockets, TLS sessions, WebSockets, daemon clients, credentials, relays, or discovery state.

## Supported packet profile

The codec preserves the Syndicate Protocol v1 turn shape without an Animus envelope:

```preserves-schema
Turn      = [TurnEvent ...].
TurnEvent = [@oid Oid @event Event].
Event     = Assert / Retract / Message.
Assert    = <A @assertion AnimusDataspaceRecord @handle Handle>.
Retract   = <R @handle Handle>.
Message   = <M @body AnimusDataspaceRecord>.
```

Outbound helpers project validated Animus assertions to `A` events, retractions to `R` events, and orchestration events to `M` events. Decode reconstructs each `DataspaceRecord` and reruns its schema-version, family, payload, redaction, scope, and owning DTO validation.

`SyndicateProtocolAdapter<T>` wraps a caller-owned `SyndicatePacketTransport`. It sends or receives one complete binary Preserves packet per call, checks configured packet bounds, and tracks separate inbound and outbound assertion-handle sets. An assert must use an inactive handle; a retract must name a live handle. Planned state is committed only after a successful local transport call.

## Transport requirements

The caller-owned session must provide:

- reliable and in-order packet delivery;
- a defined connected/disconnected lifecycle;
- confidentiality, integrity, peer authenticity, and replay resistance;
- one self-delimiting Preserves packet per adapter call; and
- session reset or reconciliation after ambiguous send failures.

Iroh can satisfy the transport role when the host binds this profile during route admission and records its own endpoint, key, relay, retry, and session evidence.

## Capability boundary

The first profile rejects:

- `<S>` synchronization events;
- embedded WireRefs and transient references;
- attenuation caveats and rewrite templates;
- extension, error, and arbitrary non-turn packets;
- arbitrary assertion bodies outside validated Animus dataspace records;
- stale retractions and duplicate active handles; and
- malformed or oversized packets.

It does not implement a full Syndicate relay, import/export membranes, capability proxying, caveat enforcement, well-known OID setup, discovery, or automatic assertion retraction on session teardown. Those require a separate authority and lifecycle change.

## Focused checks

```sh
nix develop -c cargo test -p animus-preserves syndicate --locked
nix develop -c cargo test -p animus-std --features syndicate-host syndicate --locked
nix develop -c cargo check -p animus --features syndicate-host --locked
nix develop -c scripts/cairn/check-preserves-boundary-schemas --root .
```

Protocol reference: [Syndicate Protocol](https://synit.org/book/protocol.html).
