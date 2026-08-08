## Phase 1: Baseline and ownership

- [x] [serial] B1 Block implementation until `add-native-evaluation-core` is accepted or completed in the same reviewed batch, then run current model/tool adapter, Harbor port, artifact, role, cancellation, receipt, no-std, and Wasm baselines. r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [serial] B2 Inventory current service traits and exact ownership for tasks, agents, verifiers, artifacts, model-call capture, result storage, ledger, batch, ATIF, and Harbor. r[agent_harness_runtime.native_evaluation_adapters.boundary] r[agent_harness_runtime.native_evaluation_adapters.compatibility]
- [x] [serial] B3 Record the exact Verifiers dialect/interception source cohort, MIT provenance, adopted ordering/capability patterns, and rejected Python/server/payload/state behavior. r[agent_harness_runtime.native_evaluation_adapters.model_gateway] r[agent_harness_runtime.native_evaluation_adapters.evidence]

## Phase 2: Focused crate and port contracts

- [x] [depends:agent_harness_runtime.native_evaluation_adapters.boundary] C1 Add `animus-adapter-eval` as `#![no_std]` plus `alloc` with only authority-free dependencies and non-default compatibility re-exports. r[agent_harness_runtime.native_evaluation_adapters.boundary]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.ports] C2 Add separate task-source, agent-execution, verifier-execution, artifact-collect, artifact-restore, model-call-capture, result-append, result-replace, and result-reopen traits. r[agent_harness_runtime.native_evaluation_adapters.ports]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.ports] C3 Add service catalog, capability, availability, implementation identity, cancellation, deadline, and bounded resource descriptors without ambient discovery. r[agent_harness_runtime.native_evaluation_adapters.ports]

## Phase 3: Exact binding and failure reduction

- [x] [depends:agent_harness_runtime.native_evaluation_adapters.binding] I1 Add typed service requests that bind the applicable evaluation, plan, task, episode, run, role, model, harness, environment, tool, verifier, rubric, artifact, policy, cancellation, deadline, attempt, and correlation identities. r[agent_harness_runtime.native_evaluation_adapters.binding]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.binding] I2 Add bounded observations that echo exact correlation and implementation identities plus typed success, denied, unavailable, unsupported, cancelled, timed-out, malformed, stale, mismatch, overflow, and infrastructure-failure classes. r[agent_harness_runtime.native_evaluation_adapters.binding]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.binding] I3 Add pure admission and observation validation that rejects stale, cross-plan, cross-task, cross-attempt, widened-capability, wrong-service, and incomplete evidence before state reduction. r[agent_harness_runtime.native_evaluation_adapters.binding]

## Phase 4: Model gateway and compatibility

- [x] [depends:agent_harness_runtime.native_evaluation_adapters.model_gateway] G1 Add pure dialect descriptors and canonical model-call projection for effective model, sampling, endpoint class, usage, finish, error, tool-catalog, and stream-order metadata over caller-supplied bounded values. r[agent_harness_runtime.native_evaluation_adapters.model_gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.model_gateway] G2 Add terminal-after-commit and identical-request coalescing evidence gates bound to exact run, request, attempt, policy, and result identities. r[agent_harness_runtime.native_evaluation_adapters.model_gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.model_gateway] G3 Reject raw provider-body retention, unbounded request or stream material, response rewriting, shared model/state capability, last-write-wins state, missing commit evidence, and content-only retry identity. r[agent_harness_runtime.native_evaluation_adapters.model_gateway]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.compatibility] G4 Bridge current Harbor environment and verifier ports into native evaluation services and keep ATIF, ledger, batch, role, artifact, and receipt ownership unchanged. r[agent_harness_runtime.native_evaluation_adapters.compatibility]

## Phase 5: Evidence and tests

- [x] [depends:agent_harness_runtime.native_evaluation_adapters.evidence] E1 Add metadata-only service admission, request, observation, gateway, artifact, verifier, and storage receipt drafts with BLAKE3 identities. r[agent_harness_runtime.native_evaluation_adapters.evidence]
- [x] [parallel] T1 Add positive deterministic fake-port tests for every service, exact request binding, successful observations, gateway projection, commit ordering, coalescing, and Harbor compatibility. r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [parallel] T2 Add negative tests for missing services, denied capability, stale correlation, wrong plan/task/attempt, verifier sharing, artifact widening, early terminal events, unsafe retry matching, malformed observations, and storage evidence mismatch. r[agent_harness_runtime.native_evaluation_adapters.binding] r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [parallel] T3 Add receipt leak and overclaim tests for prompts, hidden tests, reasoning, provider/tool bodies, credentials, environment values, physical paths, unbounded logs, exactly-once, verifier-soundness, benchmark, completion, production, and release claims. r[agent_harness_runtime.native_evaluation_adapters.evidence] r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [parallel] T4 Add dependency and source-shape guards proving default, no-std, Wasm, and reusable surfaces gain no filesystem, process, socket, HTTP, provider, credential, clock, container, scheduler, persistence, Harbor, Verifiers, or Python runtime authority. r[agent_harness_runtime.native_evaluation_adapters.boundary] r[agent_harness_runtime.native_evaluation_adapters.verification]

## Phase 6: Documentation and closeout

- [x] [depends:agent_harness_runtime.native_evaluation_adapters.evidence] D1 Document port ownership, capability and correlation binding, failure taxonomy, gateway projection, Harbor migration, host obligations, and non-claims. r[agent_harness_runtime.native_evaluation_adapters.boundary] r[agent_harness_runtime.native_evaluation_adapters.evidence]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.verification] D2 Update adapter-family, contracts, component, facade, request-lifecycle, safety, roadmap, README check, and bounded Verifiers reference indexes. r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.verification] D3 Add `scripts/cairn/check-evaluation-adapter-family` and a Nix check covering every port, exact binding, gateway invariants, compatibility, no-std/Wasm, receipts, and dependency isolation. r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.verification] D4 Run focused tests, formatting, Clippy, no-std/Wasm checks, component/facade/composition rails, Cairn validation and gates, and the focused Nix check. r[agent_harness_runtime.native_evaluation_adapters.verification]
- [x] [depends:agent_harness_runtime.native_evaluation_adapters.verification] D5 Review dry-run sync and archive plans, verify requirement promotion, and archive only after current positive and negative evidence passes. r[agent_harness_runtime.native_evaluation_adapters.verification]
