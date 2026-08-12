# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Bounded Syndicate turn wire profile

r[agent_harness_runtime.syndicate_turn_adapter.wire] Animus MUST provide a versioned capability-free Syndicate Protocol profile that encodes one turn as a Preserves sequence of ordered `[Oid Event]` entries and supports exact `<A Assertion Handle>`, `<R Handle>`, and `<M Body>` event shapes with validated Animus dataspace records as assertion and message bodies.

#### Scenario: Safe turn round-trips without an Animus envelope

- GIVEN a turn contains bounded assertion, retraction, and orchestration-message events
- WHEN the turn is encoded and decoded through text or binary Preserves syntax
- THEN event order, OIDs, handles, event kinds, and validated dataspace DTO meaning MUST be preserved
- AND the top-level packet MUST remain the Syndicate turn sequence rather than an Animus-specific wrapper record.

### Requirement: Turn bodies reuse owning DTO validation

r[agent_harness_runtime.syndicate_turn_adapter.projection] Outbound assertions and orchestration events MUST be projected from validated Animus DTOs, and every decoded assertion or message body MUST reconstruct a versioned `DataspaceRecord` and rerun its schema-family, version, redaction, scope, list-bound, and owning DTO validators before acceptance.

#### Scenario: Unsafe body is rejected after decoding

- GIVEN a syntactically valid turn body contains a wrong schema family, unsupported version, cross-scope/raw subscription request, raw prompt, provider payload, credential, private path, file/tool/model body, or unbounded list
- WHEN the Syndicate turn codec decodes it
- THEN decoding MUST fail with a bounded safe error class
- AND no accepted assertion, orchestration event, or transport receipt may contain the rejected body.

### Requirement: Syndicate packet transport remains injected

r[agent_harness_runtime.syndicate_turn_adapter.host] Animus MUST expose the Syndicate protocol adapter only behind the non-default `syndicate-host` feature over a caller-owned packet transport that supplies complete reliable, in-order, confidential, integral, authentic, replay-resistant session packets, while reusable crates MUST NOT construct sockets, Iroh endpoints, TLS sessions, WebSockets, daemon clients, credentials, or discovery state.

#### Scenario: Iroh carries Syndicate packets without layer confusion

- GIVEN a host has admitted an Iroh or other secure ordered session for the `syndicate.protocol.v1.animus-safe` profile
- WHEN the host supplies that session through the packet transport port
- THEN the adapter MAY send and receive bounded Preserves turn packets over it
- AND Syndicate remains the coordination protocol while Iroh or the selected host remains the transport.

### Requirement: Assertion-handle lifecycle fails closed

r[agent_harness_runtime.syndicate_turn_adapter.lifecycle] The adapter MUST validate inbound and outbound turns against separate explicit active-handle sets, require every asserted handle to be unused, require every retracted handle to be live, preserve event and turn order, and commit the planned handle state only after the corresponding packet effect succeeds.

#### Scenario: Duplicate assertion handle is denied before send

- GIVEN an outbound assertion handle is already active in the current transport session
- WHEN another turn attempts to assert that handle before retraction
- THEN pure lifecycle planning MUST reject the turn before encoding or transport effects
- AND the prior active-handle state MUST remain unchanged.

#### Scenario: Transport fails after valid planning

- GIVEN a turn has a valid planned handle transition
- WHEN the injected send effect reports failure
- THEN the adapter MUST return a typed transport failure and MUST NOT commit the planned local handle state
- AND documentation MUST require session reset or host reconciliation rather than claiming remote non-delivery.

### Requirement: Unsupported capability semantics are rejected

r[agent_harness_runtime.syndicate_turn_adapter.fail_closed] The initial Syndicate profile MUST reject `S` synchronization events, embedded WireRefs, attenuation caveats, extension packets, protocol error packets, arbitrary unvalidated assertion bodies, malformed Preserves, and packets exceeding configured bounds, and MUST NOT claim full Syndicate relay or object-capability interoperability.

#### Scenario: Capability-bearing packet reaches the adapter

- GIVEN a packet contains an embedded WireRef, `<S>` synchronization event, caveat, or unknown extension
- WHEN packet decoding or admission runs
- THEN the packet MUST be rejected before DTO delivery or handle-state mutation
- AND no membrane, capability proxy, attenuation, or fallback transport behavior may be synthesized.

### Requirement: Syndicate protocol adapter has positive and negative evidence

r[agent_harness_runtime.syndicate_turn_adapter.verification] The adaptation MUST include positive and negative tests for exact text and binary turn fixtures, ordering, projection, DTO revalidation, handle lifecycle, frame bounds, transport failure, state non-commit, feature isolation, and capability non-claims, plus focused Preserves, std-host, root-feature, boundary-schema, and Cairn validation evidence.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to archive the Syndicate protocol-adapter change
- WHEN closeout evidence is reviewed
- THEN it MUST name passing codec, host-adapter, feature, formatting, clippy, schema-boundary, and Cairn gates
- AND full WireRef/capability interoperability and live production transport MUST remain explicit non-claims or separately evidenced future work.
