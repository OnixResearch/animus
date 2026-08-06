## Context

`animus-adapters` currently exposes broad model and tool ports plus a Harbor evaluation module with environment and verifier services. The native evaluation core will define effect-free plans and decisions. A focused family is needed so evaluation hosts can implement only the services they own and so reusable crates can check exact request and observation bindings.

The reviewed Verifiers cohort is commit `1a997814e85a2e4f409add960aa852737388dbd8`. Its interception server separates model and state capabilities, records effective model-call metadata, coalesces identical non-streaming retries, and withholds terminal stream events until a turn is committed. Its dialect layer isolates native protocol parsing from generic trace construction. Animus adopts the separation and ordering invariants, not the Python server, raw payload retention, BLAKE2b identities, one-gibibyte body allowance, or mutable state endpoint.

Completion requires a focused no-std adapter crate, exact plan binding, fail-closed absence behavior, current-owner compatibility, fake-backed positive and negative tests, and no host dependencies. Trait declarations without production request validation or failure mapping are false completion.

## Decisions

### Decision: create one focused evaluation adapter family

**Choice:** Add `animus-adapter-eval` as `#![no_std]` plus `alloc`. It owns implementation-free service traits, typed requests and observations, availability descriptors, capability declarations, failure classes, and receipt drafts.

**Rationale:** A separate family keeps generic evaluation services out of the broad compatibility facade and lets embedders select the smallest contract set.

### Decision: split services by authority and lifecycle

**Choice:** Define separate ports for:

- task-source selection and bounded task materialization;
- admitted agent-run execution;
- independently identified verifier execution;
- artifact collect and restore;
- model-call capture and protocol projection;
- result append, replace, and reopen evidence.

No composite service receives ambient authority that another port owns.

**Rationale:** Fine-grained ports preserve least authority and make unavailable, denied, cancelled, timed-out, and malformed paths testable without mocks.

### Decision: bind every request and observation to the plan

**Choice:** Each request carries evaluation, plan, task, episode, run, role, model, harness, environment, tool, verifier, rubric, artifact, policy, cancellation, deadline, and correlation identities relevant to that service. Observations echo exact correlation and implementation identity plus bounded status and evidence.

**Rationale:** A service result without exact binding can be replayed across tasks or policies and can corrupt evaluation evidence.

### Decision: keep protocol normalization pure and payload-bounded

**Choice:** The adapter family represents supported protocol dialect identities and canonical model-call projections over already-loaded bounded typed values. It records effective model, sampling, endpoint class, usage, finish, error, and stream-order metadata. It does not open sockets, parse unbounded streams, retain raw provider bodies, or mutate responses.

**Rationale:** Protocol meaning belongs in a pure boundary. Transport, credentials, retries, and persistence belong in the host shell.

### Decision: expose commit-order requirements as evidence gates

**Choice:** A terminal stream observation is admissible only when it references matching durable or policy-admitted turn-commit evidence. An identical-request coalescing observation is admissible only when request identity, active run, attempt, and first-result identity match exactly.

**Rationale:** This captures Verifiers' useful no-race invariants while allowing Animus active-turn recovery to provide stronger durable evidence.

### Decision: retain existing owners through bridges

**Choice:** Harbor environment and verifier ports bridge into native service traits through explicit conversion. ATIF owns external trajectory shape, ledger owns transcript, batch owns ordering, roles own separation, and safe receipts own redaction.

**Rationale:** Migration must reduce duplicate contracts, not create parallel evaluation stacks.

### Decision: fail closed without service discovery

**Choice:** Pure admission returns unavailable or unsupported when a required service is absent, incompatible, denied, stale, or over budget. It never discovers a local backend, constructs a provider, chooses a container, or downgrades verifier isolation.

**Rationale:** Ambient fallback obscures authority and invalidates plan identity.

## Adversarial Cases

- An agent result uses the right run ID but the wrong plan or task identity.
- A verifier observation comes from the solver service or mutable shared workspace.
- An artifact restore widens the allowlisted manifest.
- A terminal event arrives before matching turn-commit evidence.
- Two requests have equal bodies but different model, sampling, policy, or attempt identity.
- A task source returns more rows than the requested bound.
- A result store reports success without exact write evidence.
- An adapter receipt includes raw prompts, provider bodies, tool output, credentials, or paths.

Each case must fail before reusable state accepts the observation.

## Alternatives Considered

### Add all ports to `animus-adapters`

Rejected because it enlarges a broad facade and prevents smallest-crate selection. Compatibility re-exports remain possible.

### Keep Harbor ports as the generic evaluation API

Rejected because Harbor version and backend semantics remain optional interoperability concerns.

### Put live interception in the adapter crate

Rejected because sockets, HTTP, streaming, credentials, clocks, and provider traffic are host authority.

### Use one composite evaluation backend trait

Rejected because a broad backend can hide verifier, artifact, storage, provider, and runtime authority behind one object.

## Risks / Trade-offs

- More traits and identities increase integration work.
- Strict binding rejects loosely implemented external harnesses until they provide sufficient evidence.
- A focused crate adds another compatibility layer during Harbor migration.
- Adapter checks cannot prove host behavior or remote exactly-once effects.

## Non-Claims

This design does not implement or prove provider transport, tool execution, container isolation, verifier independence beyond checked identities, artifact authenticity, persistence durability, complete model-call capture, model correctness, benchmark validity, production readiness, or release eligibility.
