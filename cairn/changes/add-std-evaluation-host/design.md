## Context

Animus already has thin std shells for explicit policy loading, provider retry, durable sessions, coding-session orchestration, Harbor evaluation, and authority-scoped services. The native evaluation core and adapter family will provide effect-free plans and service requests. A host is still required to execute those requests and return bounded observations.

The reviewed Verifiers source is commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT. Useful host mechanisms include distinct episode and in-episode agent concurrency, borrowed-resource lifecycle, per-rollout model clients, interception with separate capabilities, identical-request coalescing, terminal-event withholding until commit, atomic resume-output replacement, and rerun of malformed or failed records.

Animus rejects upstream defaults that permit broad networking, large unbounded request handling, raw trace persistence, dynamic package execution, mutable state replacement, and concrete cloud or container backends inside the evaluation framework. Completion requires an opt-in host, explicit service catalog, bounded lifecycle, durable commit ordering, isolated verifier path, atomic safe persistence, failure injection, and one fake-backed vertical slice. A planner-only facade or tests that bypass public host entrypoints are false completion.

## Decisions

### Decision: create a separate non-default std host crate

**Choice:** Add `animus-std-eval`. The crate consumes validated native plans and `animus-adapter-eval` ports. The root facade exposes it only through `evaluation-host`.

**Rationale:** Evaluation host authority is larger than reusable runtime authority. A separate crate keeps provider, process, network, clock, credential, and persistence dependencies opt-in.

### Decision: require an explicit host service catalog

**Choice:** The host receives selected task, agent, verifier, artifact, gateway, storage, timer, cancellation, and cleanup services as explicit values. It does not inspect environment variables or discover executables, providers, tools, containers, credentials, or storage locations unless a higher shell passes an admitted value.

**Rationale:** Explicit construction makes authority visible and keeps plan identity stable.

### Decision: separate episode and agent concurrency

**Choice:** Policy names a positive outer episode limit and a positive per-episode agent limit. The host obtains permits only around active service calls, propagates cancellation, and releases permits during caller waits. Limits, queue posture, and cancellation identity are part of the plan and receipt.

**Rationale:** Multi-agent recipes need an inner ceiling. One global number either underuses resources or permits unbounded fan-out.

### Decision: integrate the gateway with durable active-turn recovery

**Choice:** The opt-in local gateway accepts only configured loopback or caller-admitted bind descriptors and supported protocol dialects. It mints separate scoped model and state capabilities. Before model dispatch, accepted work must have matching durable evidence. A terminal event is delivered only after matching turn commit. Identical non-streaming retries coalesce only under exact request and attempt-lineage identity.

**Rationale:** Verifiers prevents duplicate graph branches in memory. Animus can strengthen this behavior across crash windows with active-turn recovery.

### Decision: keep protocol mutation deny-by-default

**Choice:** The gateway can project native requests into canonical metadata and apply only exact policy-owned model and sampling selection. Tool-result, web-result, assistant-body, or provider-response rewriting is denied unless a later Cairn defines immutable transformation policy, before/after identities, subject notification, and evaluation non-claims.

**Rationale:** Hidden response rewriting can become grading manipulation and can invalidate trajectory evidence.

### Decision: isolate verifier execution and artifacts

**Choice:** Isolated judging provisions a distinct verifier service identity and policy, transfers only an allowlisted collision-free artifact manifest, reconstructs bounded ledger-derived context, and keeps hidden verifier material outside the solver runtime. Shared judging is explicit and advisory.

**Rationale:** Prompt instructions cannot compensate for mutable verifier or oracle exposure.

### Decision: persist safe records with atomic replacement

**Choice:** The storage shell writes checked configuration, safe episode/result records, and resume metadata through explicit capability-rooted paths. It prepares complete bytes, synchronizes according to selected durability class, and atomically replaces the prior file. Torn or malformed records are ignored as completion evidence and rerun.

**Rationale:** Resume must not crash or silently keep partial output after interruption.

### Decision: keep raw content in separately authorized stores

**Choice:** Default evaluation output contains safe metadata and ledger references. Raw prompts, reasoning, tool bodies, provider payloads, hidden tests, and unrestricted paths require separate explicit storage and redaction policy and never enter safe receipts.

**Rationale:** Evaluation convenience cannot weaken ledger and privacy boundaries.

### Decision: prove composition through one fake-backed vertical slice

**Choice:** A packaged test host runs bounded tasks through fake model, tool, verifier, artifact, gateway, timer, cancellation, and storage services. It uses the same public host entrypoints as embedders.

**Rationale:** Isolated unit contracts do not prove that lifecycle and identity bindings compose.

## Execution Flow

```text
checked Nickel profile + native plan + explicit service catalog
                            |
                            v
                    pure host preflight
                            |
             +--------------+---------------+
             |                              |
             v                              v
      task and episode permits       durable accepted turn
             |                              |
             v                              v
        agent service                 model gateway
             |                              |
             +--------------+---------------+
                            v
                    ledger reference
                            |
                  allowlisted artifacts
                            |
                            v
                isolated verifier service
                            |
                            v
           reward and episode pure reduction
                            |
                            v
            atomic safe output + host receipt
```

## Adversarial Cases

- No service is available but a local executable exists.
- A nested recipe exceeds the per-episode agent limit.
- Cancellation occurs while a model request is in flight.
- A repeated request changes sampling or active-turn identity.
- A terminal stream event arrives before durable commit.
- Solver output plants a symlink at a verifier input path.
- Artifact entries collide or include hidden tests.
- Shared verifier output requests completion.
- Atomic replacement fails after byte preparation.
- A prior output ends with malformed JSON.
- Cleanup fails after terminal persistence.

Each case must return a typed failure or retained cleanup obligation without ambient fallback.

## Alternatives Considered

### Put the host in `animus-std`

Rejected because evaluation authority and dependencies are optional and broad. A focused crate gives embedders a smaller boundary.

### Use the existing Harbor host as the generic host

Rejected because Harbor release, task, ATIF, and backend semantics remain one optional compatibility profile.

### Ship concrete Docker and cloud backends

Rejected because runtime authorization, sandboxing, image trust, credentials, networking, and remote lifecycle require separate reviewed changes.

### Make interception mandatory

Rejected because native Animus adapters already expose direct model calls. The gateway is for external harnesses and remains opt-in.

### Store raw Verifiers-style traces by default

Rejected because Animus ledger, redaction, dataset, and export policy remain authoritative.

## Risks / Trade-offs

- Host integration spans several existing services and has a larger failure surface.
- Durable commit ordering adds latency before model dispatch and terminal delivery.
- Strict artifact isolation can reject external benchmarks with loose file conventions.
- The fake-backed vertical slice does not prove live provider, process, container, or sandbox behavior.

## Non-Claims

This design does not prove host confinement, provider correctness, tool safety, network enforcement, persistence durability, exactly-once remote effects, verifier soundness, artifact authenticity, benchmark comparability, model quality, production readiness, authoritative completion, or release eligibility.
