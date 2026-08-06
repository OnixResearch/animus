## Why

The native evaluation core and adapter family define deterministic plans and implementation-free service boundaries. They do not run agents, verifiers, artifact handoffs, model-call gateways, or result stores. Animus needs one non-default standard-library host that composes explicit services without adding authority to reusable crates or making evaluation part of the default agent loop.

Verifiers v1 demonstrates local and remote rollout runtimes, borrowed resources, bounded episode and agent concurrency, model-call interception, retry coalescing, trace persistence, and content-based resume. Animus will adapt the host lifecycle and ordering mechanisms behind its existing process-job, sandbox, provider, durable-session, active-turn, ledger, cancellation, redaction, and receipt contracts. It will not import the Python application, dynamic harness installation, Docker/Modal/Prime backends, permissive network defaults, or raw trace store.

## What Changes

- Add a non-default `animus-std-eval` crate that loads explicit checked evaluation policy and executes native plans only through selected injected services. r[agent_harness_runtime.native_evaluation_host.boundary]
- Add thin orchestration for bounded episode and per-episode agent concurrency, cancellation, deadlines, cleanup, and exact service lifecycle. r[agent_harness_runtime.native_evaluation_host.execution]
- Add an opt-in local model gateway shell over reviewed protocol projections with separate model and state capabilities, exact request identity, retry coalescing, and terminal-after-durable-commit delivery. r[agent_harness_runtime.native_evaluation_host.gateway]
- Add explicit artifact handoff and verifier orchestration with isolated mode as the authoritative-capable default and shared mode restricted to advisory evidence. r[agent_harness_runtime.native_evaluation_host.verifier]
- Add append, atomic replacement, reopen, and content-addressed resume shells over explicit caller-selected storage paths and safe record projections. r[agent_harness_runtime.native_evaluation_host.persistence]
- Add a deterministic fake-backed local vertical slice, metadata-only host receipts, failure-injection tests, a focused rail, and dependency guards. r[agent_harness_runtime.native_evaluation_host.evidence] r[agent_harness_runtime.native_evaluation_host.verification]

## Impact

- **Crates**: new non-default `animus-std-eval`; root compatibility feature `evaluation-host` without default activation.
- **Dependencies**: native evaluation core and adapters plus existing explicit process, provider, ledger, active-turn, cancellation, artifact, role, policy, and safe-receipt services.
- **Authority**: the host owns selected filesystem, process, network, clock, credential, provider, tool, runtime, queue, and persistence calls. Reusable crates retain none.
- **Testing**: deterministic fake services, local gateway protocol fixtures, concurrency bounds, crash windows, resume, verifier isolation, artifact transfer, cleanup, receipt, and dependency checks.

## Out of Scope

- Built-in Docker, Modal, Prime, cloud, Hub, registry, training, scheduler daemon, or remote sandbox implementations.
- Ambient provider, tool, credential, task, runtime, filesystem, or container discovery.
- Public network defaults, arbitrary response rewriting, raw transcript persistence, hidden-test exposure, or live benchmark claims.
- Exactly-once remote effects, complete sandboxing, verifier soundness, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: evaluation host boundary, execution, gateway, verifier, persistence, evidence, and verification.
