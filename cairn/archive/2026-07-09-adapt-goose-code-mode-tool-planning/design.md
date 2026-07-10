# Design: Adapt Goose Code Mode as typed tool planning

Goose Code Mode is valuable because it keeps huge tool catalogs out of every model call and lets a planner batch tool calls. Animus should adapt those concepts through typed data and Steel-compatible fabrics while keeping execution authority behind existing Rust gates.

### 1. Progressive tool discovery is metadata-only

**Choice:** Define discovery DTOs for tool list queries, tool summaries, tool detail requests, schema identities, capability labels, result contracts, source provenance, and discovery receipts. Discovery returns safe metadata only.

**Rationale:** Progressive disclosure reduces prompt/tool overhead without letting a planner invoke unknown tools or inspect plugin runtimes.

### 2. Tool plans are declarative batches

**Choice:** Define `ToolPlan`, `ToolPlanStep`, `ToolPlanDependency`, `IntermediateBinding`, `BatchPolicy`, and `ToolPlanOutcome` data. A plan may sequence steps, express dependencies, and bind bounded prior outputs by reference, but it cannot embed host code.

**Rationale:** The reusable runtime can validate and explain a plan before effects, unlike arbitrary generated JavaScript.

### 3. Steel is an optional planning fabric

**Choice:** A Steel profile may produce or transform tool plans only after Steel policy admits the script hash, budget, host-action allowlist, and receipt posture. Non-Steel typed fabrics can produce the same plan DTOs.

**Rationale:** This preserves Animus' scriptable fabric direction while avoiding Goose's JS/Deno runtime dependency.

### 4. Every step re-enters normal tool admission

**Choice:** Each tool-plan step must satisfy catalog trust, exact capability labels, permission and authority policy, argument validation, mutation-window serialization, output bounds, timeout, cancellation, and receipt policy before dispatch.

**Rationale:** Code Mode must not become a bypass around the existing tool contract and permission kernel.

### 5. Intermediate data is bounded and typed

**Choice:** Intermediate outputs can be referenced only through safe handles, summaries, hashes, counts, and admitted bounded text. Binary, image, secret, or oversized outputs require explicit host-owned handles and cannot be silently converted to model-visible text.

**Rationale:** Chained tool execution must not smuggle raw data into prompts or receipts.

### Risks

- Batching can obscure which step caused a failure. Mitigate with per-step outcome classes and receipts.
- Progressive discovery can hide dangerous tools until late. Mitigate with discovery ceilings and exact tool admission before planning and dispatch.
- Steel planner scripts can grow into an execution runtime. Mitigate with policy budgets, host-action allowlists, and declarative plan output only.
