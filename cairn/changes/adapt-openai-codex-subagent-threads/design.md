# Design: Adapt OpenAI Codex subagent thread patterns for Animus

## Context

Codex recommends using subagents for explicitly requested parallel work and warns that noisy intermediate logs can degrade the main conversation. Animus can encode that pattern as subagent thread records and summary handoff contracts while leaving scheduling and concrete model selection to runtime policy and adapters.

## Decisions

### 1. Subagents require explicit delegation

**Choice:** Runtime starts subagents only from an explicit delegation request supplied by the user, a trusted skill, or an approved automation policy.

**Rationale:** Parallel agents consume budget and increase coordination complexity, so implicit spawning should not surprise users or embedders.

### 2. Subagent transcripts are isolated by default

**Choice:** A subagent has its own thread ledger. The parent receives bounded status and final summary records unless policy explicitly imports more detail.

**Rationale:** This preserves main-thread context quality while keeping subagent evidence inspectable.

### 3. Summary shape is typed

**Choice:** Subagent completion produces a typed summary containing findings, evidence references, confidence, unresolved questions, and recommended next actions.

**Rationale:** Aggregation should not depend on parsing arbitrary assistant prose.

### 4. Profiles constrain capabilities

**Choice:** Subagent role profiles name model preferences, reasoning effort, tool permissions, workspace scope, and output schema.

**Rationale:** Reviewers, test runners, triagers, and explorers need different capabilities and budgets.

### 5. Parallel writes require isolation or serialization

**Choice:** Subagents that may write shared resources require isolated workspaces, mutation target serialization, or parent-approved handoff before changes are merged.

**Rationale:** Read-heavy subagents are low-risk; parallel writes can lose updates without explicit coordination.

## Risks / Trade-offs

- Summaries can omit useful detail. Evidence references and optional inspectable transcripts provide a recovery path.
- Explicit-only spawning may be less automatic, but it keeps budget and side effects under control.
- Role profiles can become another config surface. Nickel contracts and validation should keep profiles understandable.
