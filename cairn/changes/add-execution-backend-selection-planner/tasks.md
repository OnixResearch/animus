## Phase 1: Selection model

- [ ] [serial] Pin the reviewed agentOS revision, record selected escalation concepts and license, and add the bounded reference to `README.md`. r[agent_harness_runtime.execution_backend_planner.boundary]
- [ ] [depends:pilot-application-host-binding] [depends:seal-runtime-authority-units] Define workload requirement, backend class, candidate, selection policy, denial, and explanation DTOs with nominal identities. r[agent_harness_runtime.execution_backend_planner.model]
- [ ] [serial] Add typed Nickel policy for admitted backend classes, explicit preference order, requirement compatibility, fallback denial, and receipt safety. r[agent_harness_runtime.execution_backend_planner.policy]
- [ ] [depends:evaluate-bounded-execution-contract] Implement the pure weakest-admitted selection and deterministic explanation core. r[agent_harness_runtime.execution_backend_planner.selection]

## Phase 2: Host and consumer boundaries

- [ ] [depends:agent_harness_runtime.execution_backend_planner.selection] Add an implementation-free current-enforcement service and thin std host adapter that rechecks the selected backend before effects. r[agent_harness_runtime.execution_backend_planner.enforcement]
- [ ] [parallel] Add positive fixtures for embedded component, guarded process, isolated host, and external machine selection. r[agent_harness_runtime.execution_backend_planner.verification]
- [ ] [parallel] Add negative fixtures for unknown requirements, ambiguity, unavailable backends, authority drift, resource denial, silent fallback, and sandbox overclaims. r[agent_harness_runtime.execution_backend_planner.verification]
- [ ] [serial] Add metadata-only plan receipts, a frozen Lattice consumer fixture, focused checks, Cairn gates, and documentation. r[agent_harness_runtime.execution_backend_planner.evidence] r[agent_harness_runtime.execution_backend_planner.verification]
