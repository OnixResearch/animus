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

## Risks / Trade-offs

- New public enum variants require exhaustive consumers to update.
- This change provides descriptors only. A consumer must still implement authorized effects and bounded state projection.
- State inspection remains a request. It does not permit private state in terminal receipts.
