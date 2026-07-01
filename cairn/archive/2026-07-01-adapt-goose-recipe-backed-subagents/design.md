# Design: Adapt Goose recipe-backed subagents for Animus

Goose's subagent design is useful because it combines process/context isolation with reusable recipes and controllable tool access. Animus should adapt those ideas while preserving its existing explicit delegation, isolated ledger, and write-conflict rules.

## Decisions

### 1. Recipe-backed subagents extend explicit delegation

**Choice:** A subagent request may reference an admitted recipe/profile and bind parameters before execution. The request still names role, task, allowed tools, workspace scope, model profile, limits, return mode, and summary schema.

**Rationale:** Recipes make subagents reusable, but delegation must remain explicit and validated.

### 2. Inheritance is constrained by capability intersection

**Choice:** Subagents do not blindly inherit all parent tools or extensions. Effective capabilities are the intersection of parent policy, recipe/profile ceilings, subagent request, workspace scope, and host admission.

**Rationale:** Goose's extension-control pattern is useful, but safer defaults prevent accidental authority widening.

### 3. Parent context receives bounded handoff records

**Choice:** Subagent ledgers remain isolated. Parent context receives status and summary records according to return mode: summary-only, status-plus-summary, or full-detail-with-bounds.

**Rationale:** This preserves context quality while keeping evidence inspectable.

### 4. External subagents are adapters, not ambient CLIs

**Choice:** ACP/MCP-backed external subagents require explicit descriptors and injected host adapters. Reusable runtime never discovers or launches external agent CLIs directly.

**Rationale:** External agent coordination should reuse Animus' adapter-first interop boundary.

### 5. Aggregation handles partial failure explicitly

**Choice:** Parallel aggregation reports successful, failed, timed-out, denied, malformed, and conflicted subagent outcomes separately.

**Rationale:** Goose returns successful subagent results when others fail; Animus should make partial success explicit in reports and receipts.
