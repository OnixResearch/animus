# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Iroh and Syndicate composition is explicitly optional

r[agent_harness_runtime.syndicate_iroh_bridge.feature] Animus MUST expose the Iroh-to-Syndicate packet composition only behind a non-default `iroh-syndicate-host` feature that composes the existing `iroh-host` and `syndicate-host` seams, while either underlying feature MUST remain independently usable.

#### Scenario: Default and independent feature builds remain isolated

- GIVEN a consumer does not select `iroh-syndicate-host`
- WHEN Animus builds with defaults, only `iroh-host`, or only `syndicate-host`
- THEN no Iroh-to-Syndicate bridge API or live network dependency may be required
- AND reusable core, adapter, and runtime crates MUST remain free of endpoint authority.

### Requirement: Admitted Iroh packet session implements the Syndicate transport port

r[agent_harness_runtime.syndicate_iroh_bridge.transport] Animus MUST provide a caller-owned Iroh packet-session port and a bridge that binds one admitted `IROH_AGENT_COMMS_PROTOCOL` route to `SYNDICATE_PROTOCOL_PROFILE` and implements `SyndicatePacketTransport` without reinterpreting turns as legacy direct agent-comms frames.

#### Scenario: Host composes an admitted Iroh session

- GIVEN a validated admitted Iroh route and a caller-owned secure ordered packet session
- WHEN the host wraps them in the composition bridge and constructs `SyndicateProtocolAdapter`
- THEN one Syndicate turn MAY be sent or received through the session port
- AND Syndicate MUST remain the sole coordination protocol for that logical session.

### Requirement: Composition preserves packet boundaries and route ceilings

r[agent_harness_runtime.syndicate_iroh_bridge.framing] The bridge MUST preserve exactly one complete binary Syndicate turn packet per caller-owned session effect and MUST enforce the Iroh route byte ceiling and exact route-scope equality for every assertion/message body before outbound effects and before returning inbound bytes to the Syndicate decoder.

#### Scenario: Route ceiling is stricter than protocol ceiling

- GIVEN a valid Syndicate turn packet exceeds the admitted Iroh route's maximum frame bytes
- WHEN the bridge attempts to send it
- THEN the bridge MUST return a bounded packet-size error before calling the session
- AND the outer Syndicate adapter MUST NOT commit its planned assertion-handle state.

#### Scenario: Structurally valid turn crosses an admitted scope

- GIVEN a valid turn contains an assertion or message for another tenant, workspace, session, branch, facet, or actor scope
- WHEN outbound or inbound Iroh composition admission runs
- THEN the packet MUST be rejected before session send or DTO delivery
- AND no assertion handle may be committed for the rejected turn.

### Requirement: Composition fails closed without network authority

r[agent_harness_runtime.syndicate_iroh_bridge.fail_closed] The bridge MUST reject denied or missing route admission before session construction, map session failures to bounded errors without packet, ticket, key, relay, credential, prompt, provider, file, tool, model, or log bodies, and MUST NOT construct endpoints, sockets, tickets, keys, relays, discovery, retries, or persistence.

#### Scenario: Session send fails after valid protocol planning

- GIVEN a valid turn and admitted route pass pure planning and bounds
- WHEN the caller-owned Iroh session reports a send failure
- THEN the bridge MUST return a bounded transport failure
- AND the Syndicate adapter MUST leave outbound handle state unchanged and require host-owned session reset or reconciliation.

### Requirement: Iroh Syndicate composition has positive and negative evidence

r[agent_harness_runtime.syndicate_iroh_bridge.verification] The bridge MUST include positive and negative fake-session tests for exact packet preservation, bidirectional delivery, route admission, route byte ceilings, unavailable sessions, safe error mapping, and handle non-commit, plus combined-feature, facade, catalog, formatting, clippy, and Cairn validation evidence.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to archive the Iroh Syndicate composition change
- WHEN closeout evidence is reviewed
- THEN deterministic combined-feature and lifecycle gates MUST pass
- AND live Iroh endpoint interoperability, remote peer correctness, relay behavior, retry safety, and full Syndicate capability support MUST remain explicit non-claims.
