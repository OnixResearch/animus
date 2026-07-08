# Design: Adapt OpenClaw gateway control-plane contracts

Animus should adapt the protocol shape, not the product server. The reusable boundary is data-only validation and event projection; host shells decide whether those DTOs travel over WebSocket, JSONL, Preserves, IPC, or an in-process SDK.

## Decisions

### 1. Gateway profile is optional host data

**Choice:** Represent the gateway as a protocol profile and frame contract supplied by an optional host shell.

**Rationale:** This keeps daemon and transport ownership out of `animus-core`, `animus-adapters`, `animus-steel`, and default runtime contracts while still giving future host shells a common shape.

### 2. Connect must establish role and scope before requests

**Choice:** The first accepted request is a connect handshake with protocol range, client identity, role, scopes, advertised capabilities, and challenge evidence.

**Rationale:** OpenClaw's first-frame rule is a strong fail-closed boundary. Animus should preserve that before any prompt, attach, node, session, or tool operation is admitted.

### 3. Events are safe state projections

**Choice:** Events carry sequence and optional state-version/snapshot metadata, but model prompts, provider bodies, credentials, and raw tool output remain excluded.

**Rationale:** Operators and UIs need refresh and gap detection without turning the gateway stream into a transcript leak.

### 4. Idempotency is method metadata

**Choice:** Side-effecting gateway methods declare idempotency requirements as metadata that validation can enforce before host dispatch.

**Rationale:** Retry safety belongs in the protocol contract, but dedupe cache storage and expiry are host-owned implementation details.

### 5. Transport remains host-owned

**Choice:** WebSocket, TLS, local sockets, HTTP routes, dashboards, daemon lifecycle, auth token storage, and remote exposure remain non-default host adapters.

**Rationale:** Animus should keep reusable crates embeddable and deterministic while leaving product topology to hosts.
