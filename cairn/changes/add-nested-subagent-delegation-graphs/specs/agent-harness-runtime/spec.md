# agent-harness-runtime Change Spec: Nested subagent delegation graphs

## Requirements

### Requirement: Nested subagents are explicit graph data

r[agent_harness_runtime.subagent_nested_graphs.graph] Animus MUST represent nested subagent work as optional typed delegation graph data containing safe node IDs, admitted parent/child edges, ancestry proofs, depth policy, graph status, host admission receipts, and safe graph receipt identities.

#### Scenario: Host supplies a two-level graph

r[agent_harness_runtime.subagent_nested_graphs.graph.two_level]
- GIVEN a host supplies a parent run, a child review run, and a grandchild read-only investigation as graph nodes
- AND every edge has a host admission receipt and ancestry proof
- WHEN graph DTO validation runs
- THEN Animus accepts the graph as explicit delegation data
- AND it does not spawn a grandchild, enqueue work, call a provider, or mutate workspace state.

### Requirement: Every nested edge requires admission and acyclic lineage

r[agent_harness_runtime.subagent_nested_graphs.admission] Nested subagent graph validation MUST be pure over supplied graph DTOs and MUST reject missing host admissions, duplicate node IDs, cycle edges, stale ancestry proofs, parent/child identity mismatches, over-depth graphs, unsupported edge kinds, and ambiguous roots.

#### Scenario: Recursive self-spawn is rejected

r[agent_harness_runtime.subagent_nested_graphs.admission.self_spawn]
- GIVEN a graph edge attempts to make a child its own descendant or reuses an ancestor node as a new child
- WHEN graph admission validation runs
- THEN validation returns a cycle or recursive-spawn diagnostic
- AND no child-run admission, service call, or success receipt is produced.

### Requirement: Descendant authority and budgets only narrow

r[agent_harness_runtime.subagent_nested_graphs.monotonicity] Nested delegation validation MUST enforce monotonic authority and budget inheritance so every descendant capability set, workspace scope, mutation authority, context budget, turn budget, tool-call budget, time budget, model/profile ceiling, return mode, and output detail bound is equal to or narrower than all ancestors and host policy.

#### Scenario: Grandchild cannot regain denied mutation authority

r[agent_harness_runtime.subagent_nested_graphs.monotonicity.no_widening]
- GIVEN a child run has read-only workspace access and no mutation authority
- AND a proposed grandchild edge requests write access or mutation intent authority
- WHEN monotonicity validation runs
- THEN validation rejects the edge with an authority-widening diagnostic
- AND the graph cannot be admitted for execution.

### Requirement: Multi-level rollups are safe non-executing reductions

r[agent_harness_runtime.subagent_nested_graphs.rollup] Nested graph cancellation, fan-in, and influence rollup MUST be deterministic reducers over supplied node statuses, terminal reports, safe output hashes, bounded evidence references, and mutation-intent summaries, and MUST NOT execute children, import raw descendant transcripts, apply mutations, or count missing descendants as success.

#### Scenario: Grandchild failure rolls up as attributed evidence

r[agent_harness_runtime.subagent_nested_graphs.rollup.failure]
- GIVEN a grandchild terminal report contains a failed status, safe output hash, and bounded evidence reference
- WHEN graph rollup runs
- THEN the parent-visible result attributes the failure to the child/grandchild path with safe status classes and hashes
- AND raw logs, provider payloads, full paths, credentials, and unbounded output are excluded.

### Requirement: Nested execution and product surfaces remain host-owned

r[agent_harness_runtime.subagent_nested_graphs.host_owned] Scheduling, recursive spawn request admission, `SubagentService` injection, queue selection, provider/model calls, retries, polling, cancellation dispatch, persistence, UI tree rendering, workspace mutation, and lifecycle storage for nested delegation MUST remain host-owned behavior outside reusable runtime and adapter-contract crates.

#### Scenario: Missing graph host service fails closed

r[agent_harness_runtime.subagent_nested_graphs.host_owned.missing_service]
- GIVEN an admitted nested graph contains a pending child-to-grandchild edge
- AND no host graph execution service is injected
- WHEN an embedder attempts to execute or observe the edge through reusable contracts
- THEN Animus returns a service-unavailable or host-required diagnostic
- AND it does not create fallback schedulers, provider clients, shell commands, or background workers.

### Requirement: Nested graph verification covers positive and negative cases

r[agent_harness_runtime.subagent_nested_graphs.verification] The nested subagent graph adaptation MUST include focused positive and negative tests for two-level graphs, sibling-plus-grandchild graphs, acyclic admission, max-depth enforcement, stale lineage, authority monotonicity, budget inheritance, missing host admission, recursive self-spawn denial, cancellation propagation, fan-in rollup, host-owned execution boundaries, no automatic nesting, and receipt leak rejection.

#### Scenario: Closeout evidence states non-claims

r[agent_harness_runtime.subagent_nested_graphs.verification.closeout]
- GIVEN maintainers intend to archive the nested subagent graph adaptation
- WHEN closeout validation runs
- THEN evidence names graph DTOs, admission, monotonicity, rollup, host-owned boundaries, receipts, docs, Cairn validation, and Cairn gate checks
- AND it records automatic recursive spawning, default subagent workers, provider calls, background schedulers, UI controls, raw transcript import, and workspace mutation as non-claims.
