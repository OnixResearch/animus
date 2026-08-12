# agent-harness-runtime Change Spec: Iroh agent-comms host adapter

## Requirements

### Requirement: Iroh route descriptor

r[agent_harness_runtime.iroh_agent_comms_host.route] The `iroh-host` seam MUST validate an Iroh agent-comms route descriptor containing safe node and topic labels, the supported protocol or ALPN label, explicit scope, maximum frame bytes, remote-admission status, and optional BLAKE3 ticket or node-address hash, and MUST NOT accept raw tickets, node-address bodies, relay URLs, private keys, authorization headers, or credential material.

#### Scenario: Raw Iroh ticket is rejected

r[agent_harness_runtime.iroh_agent_comms_host.route.raw_ticket]
- GIVEN a host attempts to configure an Iroh route using a raw ticket, relay URL, private key, or authorization header
- WHEN route validation runs
- THEN validation rejects the descriptor with a safe route diagnostic
- AND no endpoint, relay, or transport object is constructed by reusable code.

### Requirement: Neutral agent-comms mapping

r[agent_harness_runtime.iroh_agent_comms_host.neutral_mapping] The Iroh host adapter MUST map Iroh-specific route metadata into the neutral agent-comms route and MUST use neutral frame projection, admission, redaction, scope, hash, frame-size, and receipt helpers for dataspace assertions, orchestration events, headless metadata, and subagent lifecycle metadata.

#### Scenario: Iroh publish uses neutral frame projection

r[agent_harness_runtime.iroh_agent_comms_host.neutral_mapping.assertion]
- GIVEN a valid redacted dataspace assertion and admitted Iroh route
- WHEN the Iroh adapter prepares to publish
- THEN it produces a neutral agent-comms frame with route hash, payload hash, count bounds, and redaction evidence
- AND Iroh-specific code does not duplicate generic frame-kind, redaction, receipt, or leak-validation logic.

### Requirement: Host-injected Iroh transport

r[agent_harness_runtime.iroh_agent_comms_host.injected_transport] Iroh communication effects MUST cross a caller-supplied host transport port for publish, query or subscribe, and event delivery over already validated routes and frames, and reusable crates MUST NOT construct live Iroh endpoints, dial peers, parse tickets, configure relays, read keys, discover services, retry network calls, read clocks, or persist remote-session state.

#### Scenario: Fake transport receives only validated metadata

r[agent_harness_runtime.iroh_agent_comms_host.injected_transport.fake]
- GIVEN tests use an in-memory fake Iroh transport
- WHEN publish, query, and event delivery are invoked through the adapter
- THEN the fake receives only validated route metadata and neutral frame metadata
- AND no live Iroh crate, endpoint, socket, relay, credential store, or filesystem state is required.

### Requirement: Iroh fail-closed safety

r[agent_harness_runtime.iroh_agent_comms_host.fail_closed] The Iroh host adapter MUST fail closed before transport effects for invalid protocol labels, missing or denied admission, prompt-required admission, scope mismatch, oversized frames, invalid hashes, missing redaction evidence, raw tickets or node-addresses, relay URLs, private keys, credentials, prompts, provider payloads, file bodies, tool payloads, model outputs, unbounded logs, unavailable transports, and ambient fallback attempts.

#### Scenario: Cross-scope frame never reaches transport

r[agent_harness_runtime.iroh_agent_comms_host.fail_closed.scope]
- GIVEN an admitted Iroh route scoped to one tenant and workspace
- WHEN a caller attempts to publish an assertion or event scoped to a different tenant or workspace
- THEN the adapter returns a scope-mismatch diagnostic
- AND the host transport publish method is not called.

### Requirement: Iroh docs and receipts

r[agent_harness_runtime.iroh_agent_comms_host.docs_receipts] Iroh adapter docs and receipts MUST record safe protocol labels, transport-family labels, route hashes, ticket or node-address hashes, frame hashes, admission classes, status classes, counts, redaction evidence, and non-claims, and MUST identify live endpoint setup, ticket handling, relay selection, authentication, retries, persistence, and smoke-test evidence as host-owned responsibilities.

#### Scenario: Iroh receipt does not expose ticket material

r[agent_harness_runtime.iroh_agent_comms_host.docs_receipts.no_ticket_leak]
- GIVEN an Iroh dispatch receipt candidate contains a raw ticket, node address body, relay URL with credentials, private key, authorization header, or endpoint debug dump
- WHEN receipt validation runs
- THEN validation rejects the candidate with a leak diagnostic
- AND durable evidence contains only safe labels, hashes, counts, and redaction markers.

### Requirement: Iroh host adapter verification

r[agent_harness_runtime.iroh_agent_comms_host.verification] Iroh host adapter adoption MUST include positive fake-transport publish, query, orchestration-event, neutral-frame mapping, and feature-export tests plus negative invalid-protocol, raw-ticket, missing-admission, prompt-required-admission, scope-mismatch, oversized-frame, invalid-hash, missing-redaction, unavailable-transport, no-ambient-fallback, and receipt-leak tests.

#### Scenario: Closeout evidence covers Iroh host seam

r[agent_harness_runtime.iroh_agent_comms_host.verification.closeout]
- GIVEN maintainers intend to archive the Iroh host adapter change
- WHEN closeout validation runs
- THEN evidence names route validation, neutral mapping, injected transport, fail-closed safety, docs, receipts, feature exports, focused tests, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
