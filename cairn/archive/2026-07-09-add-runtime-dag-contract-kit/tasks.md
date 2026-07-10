# Tasks

- [x] [serial] Define generic DAG node, edge, graph, bounds, validation issue, blocked-node, topological-batch, and graph receipt DTOs. r[agent_harness_runtime.runtime_dag_contracts.graph]
- [x] [depends:graph] Implement pure validation for unique nodes, unique edges, known endpoints, self-edge denial, cycle rejection, bounded graph size, unsafe label rejection, and deterministic diagnostics. r[agent_harness_runtime.runtime_dag_contracts.validation]
- [x] [depends:validation] Add deterministic topological batch planning and blocked-node explanations without host scheduling or execution authority. r[agent_harness_runtime.runtime_dag_contracts.topological_order]
- [x] [depends:topological_order] Add BLAKE3 graph receipt identities and safe receipt projection that records counts, issue codes, sorted graph identities, and redaction markers only. r[agent_harness_runtime.runtime_dag_contracts.receipts]
- [x] [depends:receipts] Document domain integration guidance for Kanban, recipes, ResearchSpec, subagents, workspaces, artifacts, and Cairn roadmap sequencing without replacing domain validation. r[agent_harness_runtime.runtime_dag_contracts.integration]
- [x] [depends:integration] Add positive and negative tests for valid DAGs, topological batches, blocked nodes, duplicate nodes, duplicate edges, missing endpoints, cycles, over-limit graphs, unsafe labels, unstable ordering, and receipt leak rejection. r[agent_harness_runtime.runtime_dag_contracts.verification]
