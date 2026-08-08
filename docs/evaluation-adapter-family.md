# Native evaluation adapter family

Animus provides native evaluation effect boundaries through the non-default `animus-adapter-eval` crate. The crate is `#![no_std]` plus `alloc`. It validates host-supplied service facts. It does not perform an effect.

## Ownership

`animus-eval-core` owns task, taskset, recipe, episode, reward, resume, policy, and evaluation receipt meaning. `animus-adapter-eval` owns only these contracts:

- explicit service catalogs and capability ceilings;
- exact request bindings;
- bounded service observations and failure classes;
- pure model-call metadata projections;
- terminal-after-commit and retry-coalescing gates;
- metadata-only adapter receipt drafts.

The session ledger remains the transcript and replay authority. Batch retains ordering. Agent roles retain capability and separation policy. Artifact owners retain manifest and authenticity policy. Safe receipts retain redaction. ATIF and Harbor retain their external compatibility meaning.

Hosts own task loading, agent and verifier execution, artifact transfer, provider and tool calls, cancellation observation, deadlines, clocks, persistence, and result lifecycle effects.

## Separate ports

The family defines one trait for each authority:

1. `TaskSourcePort`
2. `AgentExecutionPort`
3. `VerifierExecutionPort`
4. `ArtifactCollectPort`
5. `ArtifactRestorePort`
6. `ModelCallCapturePort`
7. `ResultAppendPort`
8. `ResultReplacePort`
9. `ResultReopenPort`

A host can implement a subset. A service descriptor can claim only the capability for its port. Ambient discovery, provider transport, and shared mutable state are outside every reusable port ceiling.

## Admission and exact binding

A host supplies `ServiceCatalog`. `admit_service` checks these facts before dispatch:

- service kind;
- exact implementation identity;
- availability;
- exact policy identity;
- capability ceiling;
- positive item and byte bounds.

`EvaluationBinding` binds all material evaluation facts. It includes evaluation, plan, task, episode, run, role, model, harness, solver and target environments, tools, verifier, rubric, artifacts, policy, cancellation, deadline, attempt, attempt lineage, and correlation identities.

`build_service_request` rejects a stale admission, a different implementation, a different policy, a widened bound, or a shared verifier environment. Agent execution must use the selected solver environment. Verifier execution must use a different environment.

A host returns `ServiceObservation`. `validate_service_observation` checks the request identity, full binding identity, correlation, implementation, attempt, item count, and byte count. A successful observation requires a bounded result identity. A failed observation cannot carry success evidence.

The failure classes are `Denied`, `Unavailable`, `Unsupported`, `Cancelled`, `TimedOut`, `Malformed`, `Stale`, `Mismatch`, `Overflow`, and `InfrastructureFailure`. Reusable code never selects an ambient fallback.

## Model gateway projection

`project_model_call` accepts bounded metadata for these reviewed dialect classes:

- OpenAI-compatible Chat Completions;
- OpenAI Responses;
- Anthropic Messages.

The projection records dialect, endpoint class, effective model, sampling, tool catalog, request, run, policy, attempt, usage, finish, error, and streaming mode. It does not retain a provider body. It rejects response rewriting, shared model and state capabilities, and unbounded request or stream material.

`admit_terminal_after_commit` blocks a terminal stream event until exact matching turn-commit evidence is complete. The gate checks run, request, turn, policy, attempt, result, and commit identities.

`admit_identical_request_coalescing` applies only to bounded non-streaming retries. It checks canonical request, run, model, sampling, policy, active turn, attempt lineage, first result, and both attempt numbers. Equal body bytes alone do not authorize coalescing.

## Harbor compatibility

`animus_adapters::harbor::bridge_harbor_request` converts only the existing Harbor environment and verifier calls. It rejects conversion when task, run, role, model, solver environment, verifier, or ordered artifact-manifest identity is lost.

`bridge_harbor_observation` converts bounded Harbor outcome metadata. ATIF projection stays in `animus-runtime`. The ledger, batch, role, artifact, and receipt owners do not change.

## Reviewed Verifiers cohort

The reviewed reference is `PrimeIntellect-ai/verifiers` commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT. The repository, revision, and ordered source-path cohort identity is `b3:facd1eab5ae08491dbab9ae51a85aac73b90415e61030b551d1f7f3f3ffc5918`. The adapter review uses these files:

- `verifiers/v1/dialects/base.py`
- `verifiers/v1/dialects/chat.py`
- `verifiers/v1/dialects/responses.py`
- `verifiers/v1/dialects/anthropic.py`
- `verifiers/v1/interception/base.py`
- `verifiers/v1/interception/pool.py`
- `verifiers/v1/interception/server.py`
- `verifiers/v1/state.py`

Animus adopts separate model and state capability identities, protocol-specific metadata projection, bounded stream ordering, and exact retry-coalescing evidence.

Animus rejects the Python server, sockets, HTTP, SSE transport, secrets, tunnels, provider payload retention, dynamic server pools, BLAKE2b request identity, the one-gibibyte body allowance, task payload service, mutable shared state, and last-write-wins state replacement.

## Receipts

`project_adapter_receipt` emits safe identities, status, counts, labels, redaction counts, and explicit non-claims. It rejects these content classes:

- prompts and hidden tests;
- reasoning;
- provider and tool bodies;
- credentials and environment values;
- physical paths;
- unbounded logs.

It also rejects claims of exactly-once remote effects, verifier soundness, artifact authenticity, complete capture, benchmark validity, authoritative completion, production readiness, and release eligibility.

## Checks

Run `scripts/cairn/check-evaluation-adapter-family --root .`. The Nix check is `checks.x86_64-linux.evaluation-adapter-family`.

## Non-claims

These contracts do not prove host behavior, provider transport, tool execution, verifier independence beyond checked identities, artifact authenticity, persistence durability, exactly-once effects, complete model-call capture, benchmark validity, authoritative completion, production readiness, or release eligibility.
