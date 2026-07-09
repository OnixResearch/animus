# Design: Adapt Iroh as an agent-comms host transport

The Iroh adapter should be a concrete host seam over the neutral agent-comms contract. It validates Iroh-specific route descriptors and then delegates shared safety to transport-neutral route/frame helpers.

### 1. Iroh descriptor is metadata, not a ticket parser

**Choice:** Define `IrohAgentCommsRoute` or equivalent descriptor with safe node label, topic label, protocol/ALPN label, scope, optional BLAKE3 ticket or node-address hash, remote-admission status, and max frame bytes. Raw tickets, relay URLs, node-address bodies, auth headers, keys, and credential material are rejected.

**Rationale:** Hosts may use Iroh tickets internally, but reusable crates should persist or route only safe labels and hashes.

### 2. Iroh maps into neutral agent-comms

**Choice:** The Iroh adapter converts its descriptor into `AgentCommsRoute` and uses neutral projection helpers for dataspace assertions, orchestration events, headless metadata, and subagent lifecycle metadata.

**Rationale:** Iroh should not own generic frame kinds, redaction classes, admission logic, or receipt validation.

### 3. Live endpoint ownership stays outside reusable crates

**Choice:** Define a caller-supplied `IrohAgentTransport` port for publish, query/subscribe, and event delivery. The port receives validated neutral frames and Iroh route metadata. It owns endpoint construction, dialing, ticket parsing, relays, auth, retries, clocks, persistence, and live smoke tests.

**Rationale:** This preserves the functional-core / imperative-shell split and avoids default network dependencies.

### 4. Feature-gated exports and docs are explicit

**Choice:** Keep the adapter behind non-default `iroh-host` feature exports and document it as optional host-owned infrastructure. Root `animus` may re-export the seam only when `iroh-host` is enabled.

**Rationale:** Users should opt into remote communication seams intentionally.

### Risks

- Hosts may confuse ticket hashes with usable tickets. Mitigate by naming fields `*_hash` and documenting that live tickets remain host-owned.
- Iroh-specific validation could drift from neutral validation. Mitigate by routing all shared projection/admission/receipt behavior through the neutral agent-comms helpers.
- A fake-backed adapter may be overclaimed as production remote comms. Mitigate with explicit non-claims and optional live smoke evidence owned by host products.
