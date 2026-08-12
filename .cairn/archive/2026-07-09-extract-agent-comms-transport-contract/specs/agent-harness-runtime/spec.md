# agent-harness-runtime Change Spec: Agent-comms transport contract

## Requirements

### Requirement: Transport-neutral route and frame DTOs

r[agent_harness_runtime.agent_comms_transport.route_frame] Animus MUST define transport-neutral agent-comms route, admission, frame-kind, frame, redaction, and receipt DTOs that carry safe labels, explicit tenant/workspace/session scope, BLAKE3 route and payload hashes, maximum frame bounds, policy or admission metadata, and bounded item counts without naming a concrete transport as the shared contract.

#### Scenario: Iroh and IPC use the same frame contract

r[agent_harness_runtime.agent_comms_transport.route_frame.shared]
- GIVEN one host wants Iroh transport and another host wants local IPC transport
- WHEN each host validates a route and projects a headless event frame
- THEN both routes and frames use the same transport-neutral DTO shape
- AND transport-specific node IDs, socket paths, tickets, or endpoint bodies do not appear in the shared frame payload.

### Requirement: Admission and scope before effects

r[agent_harness_runtime.agent_comms_transport.admission] Agent-comms dispatch MUST validate remote admission, policy evidence, supported transport family, route scope, frame-size ceiling, BLAKE3 hashes, redaction evidence, and unsafe raw-material markers before any host transport effect is invoked.

#### Scenario: Missing admission blocks dispatch

r[agent_harness_runtime.agent_comms_transport.admission.missing]
- GIVEN a route has missing, denied, or prompt-required admission
- WHEN a caller attempts to publish, query, or deliver an event over that route
- THEN validation returns an admission-required diagnostic
- AND no socket, endpoint, relay, daemon, or host transport method is called.

### Requirement: Safe frame projections

r[agent_harness_runtime.agent_comms_transport.projection] Agent-comms frame projection MUST consume existing Animus DTOs for dataspace assertions, orchestration events, headless command/event metadata, and subagent lifecycle metadata, and MUST emit only safe labels, status classes, counts, BLAKE3 hashes, correlation IDs, and redaction markers.

#### Scenario: Dataspace assertion becomes hash-only frame metadata

r[agent_harness_runtime.agent_comms_transport.projection.dataspace]
- GIVEN a redacted dataspace assertion contains a safe status, scope, item count, content hash, and redaction markers
- WHEN it is projected into an agent-comms frame
- THEN the frame records kind, route hash, payload metadata hash, byte and item counts, and redaction evidence
- AND it does not include raw prompt text, provider payloads, file bodies, tool payloads, model output, credentials, or unbounded logs.

### Requirement: Host-injected transport port

r[agent_harness_runtime.agent_comms_transport.injected_port] Agent-comms effects MUST use a caller-supplied host transport port for publish, query or subscribe, and event delivery over validated routes and frames, and reusable crates MUST NOT construct live network clients, sockets, endpoints, relays, daemons, service discovery, credential stores, retry loops, or persistence backends.

#### Scenario: Missing host transport fails closed

r[agent_harness_runtime.agent_comms_transport.injected_port.missing]
- GIVEN a valid route and frame require a transport effect
- WHEN no host transport is injected or the injected transport reports unavailable
- THEN the adapter returns a typed unavailable-transport diagnostic
- AND no fallback transport, shell command, endpoint discovery, or credential lookup occurs.

### Requirement: Safe agent-comms receipts

r[agent_harness_runtime.agent_comms_transport.receipts] Agent-comms route admission, frame projection, dispatch, query, cancellation, timeout, unavailable transport, denial, and completion decisions MUST produce safe receipts containing route hash, frame hash, transport family label, scope hash, admission class, policy hash, frame kind, counts, status class, and redaction evidence without raw payload or transport secrets.

#### Scenario: Receipt rejects raw transport and prompt material

r[agent_harness_runtime.agent_comms_transport.receipts.rejects_leaks]
- GIVEN an agent-comms receipt candidate contains a raw Iroh ticket, socket path dump, URL with credentials, authorization header, prompt body, provider payload, credential, full host path, tool body, model output, or unbounded log body
- WHEN receipt validation runs
- THEN validation rejects the candidate with a leak diagnostic
- AND durable evidence records only safe labels, hashes, counts, and redaction classes.

### Requirement: Migration from Iroh-named shared logic

r[agent_harness_runtime.agent_comms_transport.migration] Existing Iroh-named shared frame, admission, scope, redaction, and projection logic SHOULD be migrated behind neutral agent-comms names before additional transports depend on the surface, with compatibility aliases allowed only when they do not expose Iroh semantics as the generic contract.

#### Scenario: Iroh adapter becomes thin transport-specific layer

r[agent_harness_runtime.agent_comms_transport.migration.iroh]
- GIVEN the neutral agent-comms route and frame DTOs exist
- WHEN the Iroh host adapter validates an Iroh route and publishes a frame
- THEN Iroh-specific fields are translated into neutral route metadata and ticket hashes
- AND shared frame projection, admission, receipt, and leak validation are not duplicated in Iroh-specific code.

### Requirement: Agent-comms verification

r[agent_harness_runtime.agent_comms_transport.verification] Agent-comms adoption MUST include positive tests for valid route, frame, projection, and injected transport flow plus negative tests for missing admission, missing policy evidence, scope mismatch, unsupported transport family, oversized frame, raw transport material, raw prompt/provider/credential/tool/model/log material, missing redaction, invalid hash, unavailable transport, no ambient fallback, and receipt leaks.

#### Scenario: Closeout evidence covers generic transport safety

r[agent_harness_runtime.agent_comms_transport.verification.closeout]
- GIVEN maintainers intend to archive the agent-comms transport contract
- WHEN closeout validation runs
- THEN evidence names DTO, admission, projection, injected-port, receipt, migration, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
