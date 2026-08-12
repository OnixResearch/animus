# agent-harness-runtime Change Spec: Runtime DAG contract kit

## Requirements

### Requirement: Runtime DAGs are reusable structure contracts

r[agent_harness_runtime.runtime_dag_contracts.graph] Animus MUST define reusable DAG structure contracts for safe node IDs, directed edges, graph bounds, optional safe edge labels, validation reports, blocked-node explanations, topological batches, and graph receipt identities without embedding domain payloads.

#### Scenario: Domain graph uses shared structure

r[agent_harness_runtime.runtime_dag_contracts.graph.domain_payload]
- GIVEN a domain module supplies task, recipe, artifact, workspace, roadmap, or experiment identifiers as safe node IDs
- AND domain-specific payloads remain in that module
- WHEN the shared DAG structure is built
- THEN Animus accepts only the graph identity and edge structure into the reusable kit
- AND no domain payload body, raw prompt, raw file body, provider payload, or raw output is copied into the generic graph contract.

### Requirement: DAG validation is pure and fail-closed

r[agent_harness_runtime.runtime_dag_contracts.validation] DAG validation MUST be deterministic pure logic over supplied in-memory DTOs and MUST reject duplicate node IDs, duplicate edges, unknown endpoints, self edges, cycles, unsafe labels, over-limit graphs, and malformed graph metadata before any host effect is requested.

#### Scenario: Cycle is rejected before scheduling

r[agent_harness_runtime.runtime_dag_contracts.validation.cycle]
- GIVEN a graph contains a path from a node back to itself through one or more directed edges
- WHEN DAG validation runs
- THEN validation returns a cycle diagnostic with safe node identity evidence
- AND no scheduler, provider call, tool call, process, workspace mutation, or persistence operation is invoked.

### Requirement: Topological ordering is deterministic and explanatory

r[agent_harness_runtime.runtime_dag_contracts.topological_order] Animus MUST provide deterministic topological batch planning and blocked-node explanation over accepted DAGs, sorting nodes and batches by stable safe identities when multiple nodes are independently ready.

#### Scenario: Independent nodes form a stable batch

r[agent_harness_runtime.runtime_dag_contracts.topological_order.batch]
- GIVEN an accepted DAG has multiple nodes with no unmet incoming dependencies
- WHEN topological batch planning runs
- THEN the ready nodes appear in a deterministic sorted batch
- AND downstream blocked-node explanations name only safe prerequisite node IDs and issue classes.

### Requirement: Graph receipts are redacted and hash-backed

r[agent_harness_runtime.runtime_dag_contracts.receipts] DAG receipt projection MUST use BLAKE3 identities over canonical graph structure and MUST include only schema labels, bounded counts, issue codes, safe graph labels, sorted node and edge identities, topological batch hashes, and redaction markers.

#### Scenario: Receipt leak fixture is rejected

r[agent_harness_runtime.runtime_dag_contracts.receipts.rejects_leaks]
- GIVEN a graph receipt fixture contains a raw prompt, provider payload, credential-looking value, absolute path dump, raw diff, raw tool output, raw model output, raw file body, or unbounded event stream
- WHEN DAG receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND no durable evidence claims the graph is safe.

### Requirement: Domain integration remains opt-in and host-effect free

r[agent_harness_runtime.runtime_dag_contracts.integration] The DAG kit MAY be used by Kanban, recipe workflows, ResearchSpec promotions, nested subagent delegation, workspace planning, artifact lineage, and Cairn roadmap sequencing, but each domain MUST retain its own authorization, payload validation, host service boundaries, and non-claim documentation.

#### Scenario: Domain keeps execution authority outside the kit

r[agent_harness_runtime.runtime_dag_contracts.integration.no_execution]
- GIVEN a recipe workflow or subagent module accepts a DAG structure through the reusable kit
- WHEN the domain prepares an effect request for a ready node
- THEN the domain still applies its own tool, model, workspace, policy, trust, and receipt gates
- AND the generic DAG kit does not execute, enqueue, retry, cancel, persist, or observe that node.

### Requirement: DAG kit verification covers positive and negative cases

r[agent_harness_runtime.runtime_dag_contracts.verification] The DAG contract kit MUST include focused positive and negative tests for graph DTO validation, deterministic topological batching, blocked-node explanations, BLAKE3 receipt projection, duplicate node rejection, duplicate edge rejection, missing endpoint rejection, self-edge rejection, cycle rejection, over-limit rejection, unsafe label rejection, unstable-order prevention, and receipt leak rejection.

#### Scenario: Closeout evidence names graph non-claims

r[agent_harness_runtime.runtime_dag_contracts.verification.closeout]
- GIVEN maintainers intend to archive the DAG contract kit
- WHEN closeout validation runs
- THEN evidence names DTOs, pure validation, topological ordering, receipt projection, domain integration guidance, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records scheduling, persistence, provider calls, process execution, UI rendering, graph databases, and domain payload storage as non-claims.
