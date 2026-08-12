# agent-harness-runtime Change Spec: OpenClaw gateway control plane

## Requirements

### Requirement: Bounded OpenClaw gateway reference

r[agent_harness_runtime.openclaw_gateway.scope] Animus MUST treat OpenClaw's gateway as a bounded reference for optional host-owned control-plane protocol shape, and MUST NOT adopt OpenClaw's WebSocket server, HTTP routes, channel connectors, provider sessions, dashboard, credential stores, daemon supervision, or global state as reusable default behavior.

#### Scenario: Gateway profile is data only

- GIVEN an embedder wants a daemon or attach-style host shell
- WHEN it uses the OpenClaw-inspired gateway profile
- THEN reusable Animus crates validate protocol descriptors and frame DTOs only
- AND server startup, sockets, network binding, auth storage, dashboards, providers, and channel connectors remain host-owned.

### Requirement: Gateway connect handshake

r[agent_harness_runtime.openclaw_gateway.handshake] Gateway protocol admission MUST require a first-frame connect handshake containing protocol range, client identity, role, scopes, advertised capabilities, challenge evidence, and safe hello metadata before prompt, attach, node, session, or tool requests are accepted.

#### Scenario: Non-connect first frame is rejected

- GIVEN a gateway transport receives a request frame before a valid connect handshake
- WHEN protocol validation runs
- THEN validation returns a handshake-required diagnostic
- AND no runtime request, session mutation, node invocation, or tool dispatch is admitted.

### Requirement: Gateway frame validation

r[agent_harness_runtime.openclaw_gateway.frames] Gateway request, response, and event frames MUST carry typed correlation IDs, method or event names, bounded payload classes, safe error shapes, optional sequence metadata, and optional state-version or snapshot metadata without embedding raw prompts, provider payloads, credentials, full path dumps, raw tool bodies, or unbounded output.

#### Scenario: Unsafe error payload is rejected

- GIVEN a response frame includes a credential-looking token or raw provider payload in its error details
- WHEN frame validation runs
- THEN validation rejects or redacts the unsafe details with a typed leak diagnostic
- AND the frame is not eligible for durable evidence.

### Requirement: Gateway idempotency metadata

r[agent_harness_runtime.openclaw_gateway.idempotency] Side-effecting gateway methods MUST declare idempotency requirements and MUST be rejected before host dispatch when a required idempotency identity is missing, malformed, stale, or mismatched for the requested method and actor.

#### Scenario: Retried send remains bounded

- GIVEN a side-effecting send-like gateway request is retried after transport failure
- WHEN the idempotency identity matches the original method, actor, and safe target metadata
- THEN the host may return the prior safe outcome or dispatch according to its dedupe policy
- AND reusable runtime code does not maintain dedupe cache storage or claim duplicate-prevention effects by itself.

### Requirement: Gateway adaptation verification

r[agent_harness_runtime.openclaw_gateway.verification] The OpenClaw gateway adaptation MUST include positive connect, frame, event, snapshot, and idempotency tests plus negative non-connect, protocol-mismatch, unsafe-payload, missing-idempotency, stale-state, and ambient-transport fallback tests before archive.

#### Scenario: Closeout evidence is bounded

- GIVEN maintainers intend to archive the OpenClaw gateway adaptation
- WHEN closeout validation runs
- THEN evidence names focused protocol fixtures, documentation, Cairn validation, and Cairn gates
- AND any skipped daemon, WebSocket, dashboard, provider, or channel checks are listed as non-claims with next-best validation.
