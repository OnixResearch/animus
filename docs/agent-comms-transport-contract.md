# Agent-comms transport contract

Animus owns a transport-neutral agent communication contract in `animus-adapters`. The shared contract describes safe route admission, frame projection, injected transport effects, and receipt evidence for host transports such as Iroh, local IPC, WebSocket, Matrix, MCP, or future dataspace-backed transports. Concrete transports remain host-owned.

## Neutral DTOs

`AgentCommsRoute` validates a safe route label, transport family, tenant/workspace/session scope, optional endpoint or ticket BLAKE3 hash, required policy hash, admission status, maximum frame bytes, and a stable route hash.

`AgentCommsFrame` validates a safe frame id, frame kind, route hash, payload metadata hash, byte and item counts, redaction evidence, and a stable frame hash. Frame kinds cover dataspace assertions, orchestration events, headless commands, headless events, and subagent lifecycle metadata.

## Admission before effects

`admit_agent_comms_scope` and frame constructors fail closed before a host transport can run. Missing or denied admission, prompt-required admission, missing policy evidence, unsupported transport family, tenant/workspace mismatch, oversized frames, invalid BLAKE3 hashes, missing redaction evidence, or unsafe raw-material markers all produce typed `AgentCommsError` values.

Reusable crates do not open sockets, parse tickets, create relays, start daemons, discover endpoints, read credentials, retry network calls, or persist remote sessions.

## Safe projections

Projection helpers consume existing Animus DTOs and emit hash/count/redaction-only frames:

- `project_assertion_frame` for redacted dataspace assertions;
- `project_orchestration_event_frame` for orchestration events;
- `project_headless_command_frame` and `project_headless_event_frame` for headless host-control metadata;
- `project_subagent_lifecycle_frame` for subagent lifecycle metadata.

The projected frames omit raw prompt text, provider payloads, file bodies, tool payloads, model output, credentials, transport envelopes, and unbounded logs.

## Injected transport port

`AgentCommsTransport` is the host-injected effect port. It receives already validated routes and frames for publish/query/event delivery. Missing or unavailable transports fail closed through typed unavailable-transport diagnostics; hosts may adapt Iroh, local IPC, WebSocket, Matrix, MCP, or dataspace implementations behind the port.

## Safe receipts

`AgentCommsReceipt` records route hash, frame hash, transport family, scope hash, admission class, policy hash, frame kind, byte and item counts, status class, redaction evidence, and a receipt hash. Route-admission and frame receipts can be created through `AgentCommsReceipt::from_route` and `AgentCommsReceipt::from_frame`.

Receipt validation rejects raw Iroh tickets, socket paths, URLs with credentials, authorization headers, prompt bodies, provider payloads, credentials, full host paths, tool bodies, model output, and unbounded logs.

## Iroh migration boundary

The feature-gated Iroh adapter should stay a thin transport-specific layer over the neutral route/frame contract. Iroh tickets, node IDs, relay URLs, keys, endpoint parsing, network clients, and live smoke checks remain outside reusable crates and require separate host evidence.
