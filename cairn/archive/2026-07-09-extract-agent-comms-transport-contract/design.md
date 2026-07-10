# Design: Extract agent-comms transport contract

Agent communication crosses several existing Animus surfaces: headless RPC commands/events, dataspace assertions, orchestration events, subagent lifecycle state, host-control envelopes, and remote admission. The reusable contract should describe what may cross a remote boundary, while transport adapters own how bytes move.

### 1. Transport-neutral DTOs own shared safety

**Choice:** Define `AgentCommsRoute`, `AgentCommsFrame`, `AgentCommsFrameKind`, `AgentCommsAdmission`, `AgentCommsRedaction`, and `AgentCommsReceipt` as transport-neutral DTOs. The route records a safe route label, transport family label, scope, optional endpoint/ticket hash, admission status, policy hash, and max frame bytes. The frame records kind, route hash, payload metadata hash, byte/count bounds, redaction evidence, and safe correlation IDs.

**Rationale:** Route and frame validation is identical across Iroh, local IPC, WebSocket, Matrix, MCP, and future transports. Keeping it neutral avoids coupling every caller to Iroh names.

### 2. Projection helpers consume existing contracts

**Choice:** Add pure projection helpers that convert dataspace assertions, orchestration events, headless command/event metadata, and subagent lifecycle metadata into agent-comms frames. Projections include only safe labels, hashes, counts, status classes, and redaction markers.

**Rationale:** Existing runtime/adapter DTOs stay authoritative. Agent-comms should not invent raw payload formats or duplicate protocol state.

### 3. Host transports are injected ports

**Choice:** Define a host-injected `AgentCommsTransport`-style port for publish, query/subscribe, and event delivery over validated routes and frames. Reusable crates never instantiate sockets, endpoints, relays, clients, daemons, or credential stores.

**Rationale:** Animus already uses injected services for effects. Remote communication must follow the same fail-closed pattern.

### 4. Admission and scope checks run before effects

**Choice:** Route admission, policy hash presence, tenant/workspace scope, frame-size ceilings, BLAKE3 payload identities, and redaction evidence are validated before calling a host transport.

**Rationale:** Remote communication expands authority if unchecked. The neutral seam must make the safe path boring and the unsafe path impossible to accidentally dispatch.

### Risks

- Over-generalization could hide transport-specific constraints. Mitigate by keeping transport-specific descriptors as thin adapters into the neutral route.
- Generic frames could become raw payload envelopes. Mitigate with hash/count/redaction-only DTOs and explicit negative leak fixtures.
- Migration from Iroh-named types can churn callers. Mitigate with a short compatibility alias window or internal refactor before public adoption.
