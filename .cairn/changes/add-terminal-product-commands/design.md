## Context

The live terminal core already owns typed command descriptors and pure control reduction. The selected product requires explicit kernel clear and bounded state inspection.

Using the generic `session` class would hide distinct authorization and audit meaning. Product workers need exact classes before they decode private payloads.

## Decisions

### Decision: Add dedicated command variants

**Choice:** Add `ClientCommand::KernelClear` and `ClientCommand::StateInspection`. Each variant contains only a BLAKE3 correlation identity.

**Rationale:** Exact classes let a worker admit, deny, audit, and route each operation without inferring meaning from a generic session payload.

### Decision: Keep construction in the pure core

**Choice:** Add matching `ControlIntent` variants and construct descriptors through `command_from_control`.

**Rationale:** The terminal remains a deterministic descriptor producer. It gains no kernel, journal, provider, tool, session, or persistence effect.

### Decision: Preserve the private routing boundary

**Choice:** Serialize the typed descriptor through the existing daemon or direct connection. The supervisor continues to route bytes unchanged.

**Rationale:** The worker remains the only owner of authorization and execution.

### Decision: Fail closed on malformed descriptors

**Choice:** Reuse exact serde tags, deny unknown variants through enum decoding, and require a valid BLAKE3 correlation identity before routing.

**Rationale:** Malformed or forged commands must not reach the worker-owned effect boundary.

### Decision: Return inspection through the event stream

**Choice:** Add a dedicated `WorkerEvent::StateInspection` variant. It carries only request and observation identities, worker and queue generations, restore status, BLAKE3 heads, and `u32` counts.

**Rationale:** The existing command result is identity-only. The event stream already supplies ordered, bounded, snapshot-first delivery without granting terminal effect authority.

### Decision: Keep state events exact across the wire

**Choice:** Add a tagged daemon event variant and reject unknown outer or nested fields. The supervisor and terminal bind each result to its request, payload, generation, restore status, root, and ordered sequence.

**Rationale:** A typed variant prevents private prompt, source, result, credential, or path fields from hiding inside a generic event class.

### Decision: Render but do not receipt private state

**Choice:** Store the latest validated inspection in terminal view state and render its safe fields. Terminal completion receipts remain unchanged.

**Rationale:** Operators can inspect bounded state without expanding durable evidence or exposing session content.

## Risks / Trade-offs

- New public enum variants require exhaustive consumers to update.
- The product worker must still authorize inspection and construct truthful metadata.
- State inspection remains a request. It does not permit private state in terminal receipts.
