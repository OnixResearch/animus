## Context

`animus-adapters` already owns queue requests, responses, targets, capabilities, bounds, failure classes, and the `BackgroundQueueService` port. `animus-runtime` owns pure process-job profile resolution.

The missing component is an imperative host for concrete pueue process, namespace, parsing, cancellation, and teardown effects.

## Goals

- Add one narrow reusable pueue host crate.
- Preserve explicit authority and namespace boundaries.
- Execute only reviewed process-job profiles.
- Produce bounded model-visible output and metadata-only receipts.
- Keep positive and negative tests together.

## Decisions

### Decision: Use a separate non-default host crate

**Choice:** Add `animus-std-pueue` as an `AGPL-3.0-or-later` workspace member outside default members.

**Rationale:** Concrete queue process authority must not enter reusable default crates.

### Decision: Inject bounded process execution

**Choice:** The host receives an explicit bounded-process service, absolute pueue programs, cleared environment policy, working root, cancellation, and deadlines.

**Rationale:** The queue host must not invoke a shell or inherit ambient process state.

The production adapter can use the reviewed pinned `bounded-exec` cohort. Deterministic fixtures use a finite fake service.

### Decision: Use typed queue configuration

**Choice:** Nickel declares namespace roots, executable identities, backend capabilities, bounds, and reviewed process-job profiles.

**Rationale:** The host validates all operational authority before it contacts pueue.

### Decision: Keep one capability-scoped namespace

**Choice:** The host receives a caller-opened namespace root and explicit configuration leaf. It does not read home or ambient pueue paths.

**Rationale:** Consumers need strong isolation from unrelated local daemons.

### Decision: Parse bounded backend output

**Choice:** The host accepts only supported pueue JSON or bounded text cohorts. It maps backend values into Animus queue DTOs.

**Rationale:** Raw backend output must not cross the adapter boundary.

### Decision: Supply a tool-executor adapter

**Choice:** The crate exports a pueue tool definition and executor service over `BackgroundQueueService`.

**Rationale:** Model-visible use must retain Animus schema, capability, staged-execution, and receipt gates.

## Functional Core / Imperative Shell

The existing queue DTO validation and process-job profile resolution remain pure. The new crate owns process calls, daemon observations, cancellation, and output parsing.

No vendor type, process handle, filesystem handle, or raw pueue record enters the functional core.

## Risks / Trade-offs

- CLI output changes can require a new reviewed parser cohort.
- Real-daemon tests can expose host timing problems.
- Pueue does not prove command safety or task correctness.
- A capability root limits discovery but does not create a security sandbox.
