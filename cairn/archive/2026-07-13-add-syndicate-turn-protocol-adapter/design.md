## Context

Syndicate Protocol carries Preserves packets over a reliable, ordered, secure session. Its central packet is a turn: a sequence of `[Oid Event]` pairs where events assert (`<A body handle>`), retract (`<R handle>`), deliver a message (`<M body>`), or synchronize through an embedded capability (`<S #:#t>`). Animus already owns bounded dataspace assertion, subscription, and orchestration DTOs plus Preserves text/binary records, but the current `syndicate-host` seam stops above this wire contract.

The first adapter must provide useful interoperability without pretending to implement the protocol's stateful capability membrane. It therefore adopts the exact first-order turn shapes for `A`, `R`, and `M`, restricts bodies to validated Animus dataspace records, and rejects embedded capabilities and `S` events.

## Decisions

### 1. Implement a named capability-free protocol profile

**Choice:** Publish the profile label `syndicate.protocol.v1.animus-safe` and encode turns exactly as Preserves sequences of `[Oid Event]`, with `<A DataspaceRecord Handle>`, `<R Handle>`, and `<M DataspaceRecord>` events. OIDs and handles are non-negative integers. The codec does not wrap turns in an Animus-specific envelope.

**Rationale:** Exact first-order packet shapes preserve interoperability and turn boundaries. A distinct profile label prevents the supported subset from being mistaken for full Syndicate Protocol conformance.

### 2. Keep Preserves conversion in the boundary crate

**Choice:** `animus-preserves` owns manual conversion between typed turn values and Preserves values. Decoded assertion/message bodies must reconstruct `DataspaceRecord` and rerun its owning schema and Animus DTO validators before the turn is accepted.

**Rationale:** Reusable runtime and adapter logic should not manipulate `preserves::value` directly, while the existing boundary crate already owns codec and safe-payload policy.

### 3. Keep live transport injected and feature-gated

**Choice:** `animus-std` exposes `SyndicateProtocolAdapter<T>` only under `syndicate-host`. It wraps a caller-owned `SyndicatePacketTransport` that sends and receives one complete binary Preserves packet per call. The adapter does not create sockets, Iroh endpoints, TLS sessions, WebSockets, daemon clients, credentials, or discovery state.

**Rationale:** Syndicate Protocol and Iroh occupy different layers. A host may carry these packets over Iroh or another secure ordered transport without coupling reusable crates to one network implementation.

### 4. Separate pure handle planning from transport effects

**Choice:** A deterministic planner validates each turn against an explicit active-handle set and returns the next set. Assert requires an unused handle; retract requires a live handle; message leaves the set unchanged. The shell plans, encodes, checks size, performs one transport call, and commits the planned set only after success. Inbound and outbound handle sets remain separate.

**Rationale:** Handle lifecycle is protocol state, but its decisions can be tested without network effects. Separate directional sets reflect the symmetric protocol and prevent cross-direction aliasing.

### 5. Reject capability semantics until membranes are implemented

**Choice:** The codec rejects `S` events, embedded values, extension packets, protocol error packets, and arbitrary assertion bodies in the first profile. WireRefs, attenuation caveats, well-known capability setup, membrane reference counting, transient-reference checks, and session teardown retraction are non-goals.

**Rationale:** Accepting capability-bearing packets without a correct membrane and attenuation implementation would create misleading interoperability and authority claims.

### 6. Bind packet size before and after transport

**Choice:** The host adapter has an explicit non-zero maximum packet size. Outbound encoded bytes are checked before send; inbound bytes are checked before decode. Diagnostics report only safe classes and limits, never rejected packet bodies.

**Rationale:** Preserves validation does not itself bound memory or transport amplification.

## Risks / Trade-offs

- Peers expecting full WireRef and synchronization behavior cannot use this restricted profile. The profile label and docs make that blocker explicit.
- The upstream protocol has no built-in version-negotiation packet. Hosts must bind the profile during route/session admission.
- A transport failure may have ambiguous remote delivery despite local non-commit. Hosts must provide session-reset behavior rather than retrying a possibly delivered turn under stale handle state.
- Dataspace records are more restrictive than Syndicate's `Assertion = any`; this deliberately trades generality for Animus safety and DTO revalidation.

## Non-Goals

- Implementing a Syndicate daemon/client crate, full relay actor, dataspace runtime, WireRef membrane, caveat evaluator, capability attenuation, discovery, TLS, Iroh endpoint, retry loop, or durable session state.
- Replacing Animus's ledger, provenance, admission, authorization, or neutral agent-comms contracts.
- Claiming compatibility with arbitrary Syndicate Protocol peers outside the declared capability-free profile.
