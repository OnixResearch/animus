# agent-harness-runtime Change Spec: Orchestration adapter family

## Requirements

### Requirement: Orchestration remains runtime-owned with public adapters

r[agent_harness_runtime.orchestration_adapters.family] Animus MUST satisfy host-backed orchestration needs by maintaining a public orchestration adapter family before adding a dedicated `animus-orchestration` crate, and reusable orchestration decisions MUST remain owned by `animus-runtime` unless a future accepted change defines a separate crate barrier.

#### Scenario: Adapter family is preferred over a new crate

- GIVEN maintainers need a host-backed seam for durable task boards, handoffs, scheduling admission, cancellation, or lifecycle evidence
- WHEN they choose the next architecture boundary
- THEN they expose implementation-free adapter contracts for host effects
- AND they do not add a new orchestration crate without a separate accepted crate-boundary change.

### Requirement: Orchestration service contract

r[agent_harness_runtime.orchestration_adapters.service_contract] The orchestration adapter family MUST define no-std implementation-free contracts for host-owned board or task graph loading, handoff append, scheduling admission, cancellation, bounded lifecycle evidence, and typed host failure outcomes, while concrete storage, process supervision, clocks, VCS commands, filesystem authority, network transport, and persistence remain host-owned.

#### Scenario: Host service supplies board data without ambient authority

- GIVEN runtime logic needs a board or task graph snapshot
- WHEN it calls an admitted orchestration adapter service
- THEN the service receives typed safe labels, operation metadata, and bounded DTOs
- AND reusable runtime code does not discover files, open databases, spawn workers, read clocks, call VCS commands, open networks, or construct persistence backends.

### Requirement: Runtime bridge preserves compatibility

r[agent_harness_runtime.orchestration_adapters.runtime_bridge] Runtime-facing orchestration helpers MUST consume the public adapter-family contract or a documented compatibility bridge, and existing `KanbanBoardService`-style runtime exports SHOULD remain available through aliases, bridge implementations, or compatibility modules until a later accepted removal change.

#### Scenario: Existing runtime embedder keeps compiling

- GIVEN an embedder already implements the runtime-local Kanban board service seam
- WHEN the orchestration adapter family is introduced
- THEN the embedder can use a documented bridge or compatibility export
- AND new implementations can target the `animus-adapters` family path without importing unrelated runtime internals.

### Requirement: Orchestration adapters fail closed

r[agent_harness_runtime.orchestration_adapters.fail_closed] Orchestration adapter dispatch MUST fail closed for missing services, unavailable backends, denied capabilities, invalid graph snapshots, unsafe handoffs, cross-tenant leaks, out-of-scope mutations, unsupported scheduling operations, and schema mismatches, and MUST NOT attempt fallback discovery through ambient process, filesystem, network, provider, or persistence authority.

#### Scenario: Missing service does not create a scheduler

- GIVEN runtime logic has a valid task graph operation but no admitted orchestration adapter service
- WHEN dispatch is attempted
- THEN the operation returns a typed missing-service or denied-service outcome
- AND no board database, scheduler, worker process, VCS command, filesystem path, network client, or persistence backend is created.

### Requirement: Safe orchestration evidence

r[agent_harness_runtime.orchestration_adapters.safe_evidence] Orchestration adapter outcomes and receipts MUST record only safe family labels, operation kinds, task or board labels, status classes, counts, redaction markers, policy identities, and BLAKE3 hashes of bounded DTOs, and MUST reject raw prompts, provider payloads, credentials, full paths, hidden reasoning, unbounded logs, host database records, and raw task bodies from durable evidence.

#### Scenario: Unsafe orchestration evidence is rejected

- GIVEN an orchestration receipt fixture contains raw prompt text, provider JSON, credential-looking material, an absolute path dump, hidden reasoning, unbounded logs, host database rows, or raw task body content
- WHEN receipt validation runs
- THEN validation rejects the receipt with a typed leak diagnostic
- AND the receipt is not eligible for durable evidence or archive closeout.

### Requirement: Orchestration adapter verification

r[agent_harness_runtime.orchestration_adapters.verification] Orchestration adapter family work MUST include focused positive and negative tests for service injection, board or graph loading, handoff append, scheduling admission, cancellation, safe evidence, missing service, unavailable backend, denied capability, invalid graph, unsafe handoff, cross-tenant leak, out-of-scope mutation, unsupported operation, compatibility bridge behavior, contracts-index coverage, facade inventory coverage, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers adapter-family boundaries

- GIVEN maintainers intend to archive the orchestration adapter family change
- WHEN closeout validation runs
- THEN evidence names focused adapter service tests, runtime bridge tests, fail-closed negative tests, safe-evidence leak tests, fixture or no-op host adapter tests, contracts-index updates, facade-inventory updates, documentation, workspace checks, Cairn validation, and Cairn gate checks
- AND concrete board databases, production schedulers, process supervisors, VCS automations, filesystem persistence layouts, network transports, and live multi-agent orchestration are listed as non-claims unless optional host implementations separately prove them.
