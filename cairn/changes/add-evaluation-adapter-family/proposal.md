## Why

The native evaluation core will plan tasks, episodes, recipes, rewards, and resume without effects. It still needs stable boundaries for task loading, agent runs, verifier runs, artifact handoff, model-call capture, and result persistence. Reusing broad runtime traits or Harbor-specific ports would either widen dependencies or keep generic evaluation tied to one interoperability backend.

Verifiers v1 demonstrates useful separation between harness programs, runtimes, model interception, task state, and evaluation orchestration. Its concrete Python server, native provider payloads, dynamic harness installation, and shared mutable state do not fit reusable Animus crates. Animus needs a focused implementation-free adapter family that translates native evaluation requests into injected host observations without owning any effect.

## What Changes

- Add a focused `animus-adapter-eval` `#![no_std]` plus `alloc` crate that depends on native evaluation and existing Animus DTO contracts without adding host authority. r[agent_harness_runtime.native_evaluation_adapters.boundary]
- Define typed ports for task sources, agent execution, verifier execution, artifact collection and restore, model-call capture, and evaluation result storage. r[agent_harness_runtime.native_evaluation_adapters.ports]
- Define exact request, capability, availability, cancellation, deadline, observation, and failure envelopes that bind every call to an admitted evaluation plan. r[agent_harness_runtime.native_evaluation_adapters.binding]
- Add pure protocol-projection contracts for endpoint dialect identity, effective model and sampling metadata, stream terminal ordering, and redacted model-call observations over caller-supplied bounded values. r[agent_harness_runtime.native_evaluation_adapters.model_gateway]
- Add compatibility bridges from current Harbor environment and verifier ports plus ATIF, batch, ledger, role, and artifact owners without duplicating their semantics. r[agent_harness_runtime.native_evaluation_adapters.compatibility]
- Add safe adapter receipts, positive and negative contract fixtures, fake implementations, no-std and Wasm checks, and dependency-isolation rails. r[agent_harness_runtime.native_evaluation_adapters.evidence] r[agent_harness_runtime.native_evaluation_adapters.verification]

## Impact

- **Crates**: new `animus-adapter-eval`; compatibility re-exports through `animus-adapters` and root `animus` under non-default features.
- **Dependencies**: native evaluation core plus existing authority-free model, tool, artifact, ledger-reference, role, cancellation, and receipt DTO crates only.
- **Host boundary**: concrete provider, process, container, file, network, clock, credential, queue, and persistence implementations remain outside the adapter family.
- **Testing**: deterministic fake ports, request/observation binding, failure mapping, gateway ordering, Harbor bridge, safe receipt, no-std, Wasm, and dependency checks.

## Out of Scope

- A live proxy server, sockets, HTTP, SSE transport, provider clients, tool execution, task discovery, harness installation, containers, cloud sandboxes, files, or databases.
- Dynamic Python protocols, raw provider-body retention, ambient local fallback, last-write-wins shared state, or response rewriting.
- Verifier soundness, exactly-once remote effects, complete trace capture, benchmark validity, or production readiness.

## Affected Specs

- `agent-harness-runtime`: evaluation adapter boundary, ports, binding, model gateway, compatibility, evidence, and verification.
