# Runtime DAG contract kit

Animus provides a reusable runtime DAG contract kit for graph-shaped runtime data that needs deterministic validation, ordering, blocked-node explanations, and safe receipts. The kit owns structure only: safe node IDs, directed edges, optional edge labels, graph bounds, validation reports, topological batches, blocked-node summaries, and BLAKE3 receipt identities.

Domain payloads stay outside the generic kit. Kanban boards, recipe workflows, ResearchSpec promotions, nested subagents, workspace plans, artifact lineage, and Cairn roadmap sequencing keep their own authorization, payload validation, host service boundaries, and receipt rules.

## Contract

The reusable runtime exports are `RuntimeDagGraph`, `RuntimeDagNode`, `RuntimeDagEdge`, `RuntimeDagBounds`, `RuntimeDagValidationReport`, `RuntimeDagTopologicalPlan`, `RuntimeDagBlockedNode`, and `RuntimeDagReceipt` plus these pure helpers:

- `validate_runtime_dag_graph` checks duplicate nodes, duplicate edges, unknown endpoints, self edges, cycles, unsafe labels, and graph bounds.
- `plan_runtime_dag_topology` returns deterministic sorted topological batches for a valid graph.
- `explain_runtime_dag_blocked_nodes` reports nodes whose incoming prerequisites are not complete.
- `project_runtime_dag_receipt` emits a safe BLAKE3-backed receipt summary.
- `validate_runtime_dag_receipt` rejects unsafe schema, hash, redaction, label, and leak fixtures.

## Non-claims

The DAG kit does not schedule, enqueue, retry, cancel, persist, observe, or execute graph nodes. It does not read files, spawn processes, call providers, open network transports, render UI graphs, store graph databases, or copy raw prompts, provider payloads, raw diffs, file bodies, model/tool output, credentials, private paths, or unbounded logs into receipts.

## Verification

Run the focused in-crate rail:

```sh
cargo test -p animus-runtime runtime_dag --locked
```

Closeout evidence should cite the positive valid-DAG, deterministic-batch, blocked-node, and receipt tests plus the negative duplicate-node, duplicate-edge, missing-endpoint, self-edge, cycle, over-limit, unsafe-label, unstable-order, malformed-hash, and receipt-leak tests.
