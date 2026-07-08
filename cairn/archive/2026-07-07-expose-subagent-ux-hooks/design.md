# Design: Expose subagent UX hooks

Subagent UX should be a projection layer over safe coordinator and ledger state. It must not become another authority path.

## Decisions

### 1. Status projection is UI-neutral

**Choice:** Runtime exposes safe projection records that TUI, headless, and host-control layers can render without depending on terminal widgets.

**Rationale:** The same status should work for terminal, JSONL, RPC, and logs.

### 2. Detail import is explicit and bounded

**Choice:** UX projections may show that detail exists, but importing detail requires a bounded request that passes handoff mode, size, and policy checks.

**Rationale:** Operators need inspectability without automatic raw-log flooding.

### 3. UX actions are commands, not effects

**Choice:** Cancel, acknowledge, import-detail, and arbitrate-mutation actions are represented as command descriptors that route through the coordinator and normal policy.

**Rationale:** UI controls must not bypass capability, confirmation, or mutation coordination.

### 4. Denials are first-class

**Choice:** Projection includes denial reason, failing boundary, and safe evidence reference when available.

**Rationale:** Users need to understand why subagents did not run or why outputs were withheld.
