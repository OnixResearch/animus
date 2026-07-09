# Tasks

- [ ] [serial] Define nested subagent graph node, edge, ancestry proof, depth policy, host admission receipt, and safe graph receipt DTOs. r[agent_harness_runtime.subagent_nested_graphs.graph]
- [ ] [depends:graph] Add pure graph validation for acyclicity, unique node IDs, parent/child edge consistency, stale lineage, max depth, and host admission on every edge. r[agent_harness_runtime.subagent_nested_graphs.admission]
- [ ] [depends:admission] Add monotonic authority and budget inheritance checks for capabilities, workspace scope, context, turns, tool calls, time, model/profile ceilings, and mutation authority. r[agent_harness_runtime.subagent_nested_graphs.monotonicity]
- [ ] [depends:monotonicity] Add cancellation, fan-in, and influence-rollup reducers over multi-level terminal results without executing children or importing raw descendant output. r[agent_harness_runtime.subagent_nested_graphs.rollup]
- [ ] [depends:rollup] Add host-owned boundaries for scheduling, recursive spawn requests, service injection, persistence, UI tree rendering, provider calls, retries, and workspace mutation. r[agent_harness_runtime.subagent_nested_graphs.host_owned]
- [ ] [depends:host_owned] Add positive and negative tests for two-level graphs, sibling-plus-grandchild graphs, cycle rejection, over-depth rejection, authority widening, budget inflation, missing host admission, recursive self-spawn, cancellation propagation, fan-in rollup, and receipt leak rejection. r[agent_harness_runtime.subagent_nested_graphs.verification]
