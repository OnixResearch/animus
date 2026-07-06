# Design: TUI tool call and approval panels

## Context

The archived `add-ratatui-tool-host-surface` change established that tools communicate through typed `ToolHostEvent`/`ToolHostCommand` values, approvals are finite and correlated, and the render model is pure. Animus also has hashed-edit-protocol DTOs (`adapt-zerostack-hashed-edit-protocol`), tool-gateway-delivery receipts, confirmation-broker requests, and pi-tool-contract output bounds. A rich tool panel should render all of these without re-implementing dispatch or approval semantics.

## Decisions

### 1. Tool cards render from host bridge events

**Choice:** Tool-call cards are derived from `ToolHostEvent` started/progress/output/result/failure events, composed under the conversation render model, with collapsible bounded output.

**Rationale:** The archived bridge remains the dispatch contract; the card is a pure view over its events.

### 2. Diff views render from hashed-edit DTOs

**Choice:** File-edit and diff views render from hashed-edit-protocol DTOs (BLAKE3 snapshots and tagged spans) and tool-gateway-delivery receipts, host-only. The TUI never reconstructs edits from raw file reads.

**Rationale:** Keeps provenance and integrity in the DTO layer; the TUI only projects it.

### 3. Approvals extend the archived finite/correlated bridge

**Choice:** Interactive approval prompts reuse the archived `ToolHostEvent`/`ToolHostCommand` bridge with finite `ToolCommandChoice` values and `ToolCorrelationId` identities. No new approval contract is added.

**Rationale:** Preserves the archived approval-denial and failure-isolation requirements verbatim.

### 4. Bounded output and no raw payloads

**Choice:** Tool output is bounded per pi-tool-contract output bounds; raw tool payloads and credentials are not retained in render state.

**Rationale:** Keeps render state safe and bounded.

### 5. Downstream-only composition

**Choice:** The panels consume the tool host bridge, hashed-edit DTOs, and confirmation-broker requests as-is. No new field is added to a reusable crate.

**Rationale:** Keeps the TUI strictly downstream.

## Risks / Trade-offs

- Rich diff rendering can tempt direct file reads; the TUI must render only from DTOs.
- Approval UX complexity could expand the command vocabulary; keep finite choices per the archived contract.
- Tool output can be huge; bounding and truncation metadata are mandatory.
