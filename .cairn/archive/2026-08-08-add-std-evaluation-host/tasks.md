## Phase 1: Prerequisites, baseline, and ownership

- [x] [serial] B1 Block implementation until `add-native-evaluation-core` and `add-evaluation-adapter-family` are accepted or complete in the same batch, and coordinate gateway commit ordering with `add-durable-active-turn-recovery`. r[agent_harness_runtime.native_evaluation_host.boundary]
- [x] [serial] B2 Run and record current positive and negative Harbor host, coding-session, durable-session, active-turn, provider retry, streaming, process-job, sandbox, artifact, cancellation, batch, and safe-receipt baselines. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [serial] B3 Inventory exact host owners for policy loading, providers, tools, processes, runtimes, gateways, ledgers, artifacts, verifiers, timers, cancellation, cleanup, and persistence. r[agent_harness_runtime.native_evaluation_host.boundary]
- [x] [serial] B4 Record Verifiers host-source cohort, MIT provenance, adopted lifecycle/ordering mechanisms, and rejected dynamic, permissive, cloud, container, state, and raw-output behavior. r[agent_harness_runtime.native_evaluation_host.evidence]

## Phase 2: Policy and crate boundary

- [x] [depends:agent_harness_runtime.native_evaluation_host.boundary] C1 Add non-default `animus-std-eval` with explicit `evaluation-host` root feature and no default activation. r[agent_harness_runtime.native_evaluation_host.boundary]
- [x] [depends:agent_harness_runtime.native_evaluation_host.boundary] C2 Add explicit-path checked policy loading and pure parsing for host services, runtime, network, concurrency, gateway, verifier, artifact, persistence, cleanup, and evidence settings. r[agent_harness_runtime.native_evaluation_host.boundary]
- [x] [depends:agent_harness_runtime.native_evaluation_host.boundary] C3 Add host authority catalog entries and source-shape guards for every filesystem, process, socket, provider, tool, credential, clock, container, scheduler, and persistence capability. r[agent_harness_runtime.native_evaluation_host.boundary]

## Phase 3: Execution and lifecycle

- [x] [depends:agent_harness_runtime.native_evaluation_host.execution] H1 Add pure preflight over native plans and an explicit service catalog; fail before side effects for missing, denied, incompatible, stale, or over-budget services. r[agent_harness_runtime.native_evaluation_host.execution]
- [x] [depends:agent_harness_runtime.native_evaluation_host.execution] H2 Add separate positive episode and per-episode agent concurrency controls with cancellation-aware permit lifecycle and no permit retention during caller waits. r[agent_harness_runtime.native_evaluation_host.execution]
- [x] [depends:agent_harness_runtime.native_evaluation_host.execution] H3 Add thin orchestration for task materialization, agent runs, ledger-reference capture, pure episode reduction, and typed terminal outcomes through injected services only. r[agent_harness_runtime.native_evaluation_host.execution]
- [x] [depends:agent_harness_runtime.native_evaluation_host.execution] H4 Add exact cancellation, deadline, timeout, service-failure, partial-result, and cleanup-required propagation without ambient retry, provider fallback, or local execution. r[agent_harness_runtime.native_evaluation_host.execution]

## Phase 4: Model gateway

- [x] [depends:agent_harness_runtime.native_evaluation_host.gateway] G1 Add an opt-in local gateway shell for configured loopback or caller-admitted binds and reviewed protocol dialects, with separate scoped model and task-state capabilities. r[agent_harness_runtime.native_evaluation_host.gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_host.gateway] G2 Record canonical effective model, sampling, endpoint, usage, finish, error, tool-catalog, timing-class, request, attempt, and stream-order observations without retaining raw provider bodies in safe records. r[agent_harness_runtime.native_evaluation_host.gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_host.gateway] G3 Gate dispatch on durable accepted-turn evidence, coalesce exact identical non-streaming retries, and withhold terminal events until matching durable turn-commit evidence exists. r[agent_harness_runtime.native_evaluation_host.gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_host.gateway] G4 Add cancellation and teardown that seal concluded runs and reject late model, state, or stream writes. r[agent_harness_runtime.native_evaluation_host.gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_host.gateway] G5 Deny response, tool-result, web-result, or assistant-body rewriting and deny public bind, wildcard network, shared capabilities, content-only coalescing, and unbounded bodies or streams. r[agent_harness_runtime.native_evaluation_host.gateway]

## Phase 5: Verifier, artifacts, and persistence

- [x] [depends:agent_harness_runtime.native_evaluation_host.verifier] V1 Add isolated verifier orchestration with distinct implementation, environment, policy, subject, rubric, and mutation-lineage identities. r[agent_harness_runtime.native_evaluation_host.verifier]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verifier] V2 Add allowlisted bounded artifact collect and restore with lexical admission, collision rejection, no-follow host policy, exact identity evidence, and hidden-test separation. r[agent_harness_runtime.native_evaluation_host.verifier]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verifier] V3 Keep shared-runtime judging explicit and advisory and reject its promotion to verifier isolation or authoritative completion. r[agent_harness_runtime.native_evaluation_host.verifier]
- [x] [depends:agent_harness_runtime.native_evaluation_host.persistence] P1 Add explicit capability-rooted output destinations and safe config, episode, result, resume, and receipt records. r[agent_harness_runtime.native_evaluation_host.persistence]
- [x] [depends:agent_harness_runtime.native_evaluation_host.persistence] P2 Add append, complete-byte preparation, selected synchronization, atomic replacement, reopen, malformed-row classification, and content-addressed resume integration. r[agent_harness_runtime.native_evaluation_host.persistence]
- [x] [depends:agent_harness_runtime.native_evaluation_host.persistence] P3 Rerun torn, malformed, failed, missing, stale, conflicting, or mismatched records and retain cleanup-required state after post-terminal cleanup failure. r[agent_harness_runtime.native_evaluation_host.persistence]

## Phase 6: Vertical slice and adversarial evidence

- [x] [depends:agent_harness_runtime.native_evaluation_host.verification] T1 Add one public-entrypoint vertical slice with fake task, agent, model, tool, verifier, artifact, gateway, timer, cancellation, cleanup, and storage services. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [parallel] T2 Add positive tests for every built-in recipe, nested concurrency, exact gateway retry, terminal-after-commit, isolated verifier, artifact handoff, atomic output, reopen, resume, cancellation, and cleanup. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [parallel] T3 Add negative tests for missing services, ambient executable/provider discovery, concurrency overflow, cancellation races, early terminal events, retry mismatch, late writes, public bind, wildcard network, response rewriting, and over-budget payloads. r[agent_harness_runtime.native_evaluation_host.gateway] r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [parallel] T4 Add verifier and persistence negatives for shared promotion, symlink planting, artifact collision/widening, hidden-test exposure, partial writes, malformed rows, stale resume, failed cleanup, and unsafe paths. r[agent_harness_runtime.native_evaluation_host.verifier] r[agent_harness_runtime.native_evaluation_host.persistence] r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [parallel] T5 Add receipt leak and overclaim tests for raw task or transcript content, reasoning, tool/provider bodies, credentials, environment, paths, logs, sandbox, durability, exactly-once, verifier-soundness, benchmark, completion, production, and release claims. r[agent_harness_runtime.native_evaluation_host.evidence] r[agent_harness_runtime.native_evaluation_host.verification]

## Phase 7: Documentation and closeout

- [x] [depends:agent_harness_runtime.native_evaluation_host.evidence] D1 Document host construction, service catalog, authority map, concurrency, gateway, verifier, artifacts, persistence, failure recovery, cleanup, operator boundaries, and non-claims. r[agent_harness_runtime.native_evaluation_host.boundary] r[agent_harness_runtime.native_evaluation_host.evidence]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verification] D2 Update contracts, std-host, component, facade, request-lifecycle, safety, roadmap, README check, dogfood-readiness, and bounded Verifiers reference indexes. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verification] D3 Add `scripts/cairn/check-std-evaluation-host` and a Nix check covering policy, authority, lifecycle, gateway, verifier, persistence, vertical slice, receipts, and dependency isolation. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verification] D4 Run focused tests, formatting, Clippy, policy and generated-config checks, active-turn/provider/artifact/composition rails, Cairn validation and gates, and the focused Nix check. r[agent_harness_runtime.native_evaluation_host.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_host.verification] D5 Review dry-run sync and archive plans, verify requirement promotion, and archive only after current positive and negative evidence passes. r[agent_harness_runtime.native_evaluation_host.verification]
