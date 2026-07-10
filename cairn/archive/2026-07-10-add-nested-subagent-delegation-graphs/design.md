# Design: Add nested subagent delegation graph contracts

## Context

The current subagent runtime facade models child-run admission and coordination for a single parent/child edge. Fan-out/fan-in handles sibling sets, and lineage records explain how child output may influence a parent. Nested work needs one more layer: a graph-level view that validates multiple admitted edges together.

## Decisions

### 1. Nested agents are a delegation graph, not recursion

**Choice:** Represent nested work as an explicit directed acyclic graph of safe node IDs and admitted edges. Each edge references a parent admission, child admission, ancestry proof, depth, and host admission receipt.

**Rationale:** A graph contract makes nesting reviewable and prevents a running child from acquiring ambient authority to spawn descendants.

### 2. Authority and budgets are monotonic down the graph

**Choice:** A descendant edge is valid only when capabilities, workspace scope, mutation authority, context budget, turn budget, tool-call budget, time budget, and model/profile ceilings are equal to or narrower than every ancestor ceiling.

**Rationale:** Nesting must not widen authority or launder capabilities through a child.

### 3. Graph validation is pure and execution-free

**Choice:** Reusable logic validates in-memory graph data and returns safe diagnostics/receipts. It does not enqueue, spawn, retry, poll, cancel, persist, or mutate child runs.

**Rationale:** Hosts own actual execution topology and service injection.

### 4. Multi-level fan-in remains bounded evidence

**Choice:** Aggregation can roll up terminal statuses, failure classes, mutation intents, and safe output hashes across depths. Raw transcripts, provider payloads, full paths, and unbounded logs remain excluded.

**Rationale:** Parent context should receive attributed evidence, not descendant internals by default.

## Risks / Trade-offs

- Strict max-depth and monotonicity may reject useful product workflows. Hosts can choose larger explicit limits, but reusable defaults should be conservative.
- Graph data may duplicate existing child-run state. This duplication is acceptable because graph validation requires a whole-tree view.
- Cancellation propagation can be host-specific. The SDK should validate propagation plans and receipts, not execute cancellation.
