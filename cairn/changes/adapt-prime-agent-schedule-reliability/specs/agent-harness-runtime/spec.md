# Agent Harness Runtime

## ADDED Requirements

### Requirement: Scheduled prompts remain an exact optional boundary

r[agent_harness_runtime.schedule_reliability.boundary] Animus MUST provide scheduled-prompt reliability as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire clock, timer, filesystem, process, network, or persistence authority from scheduling.

#### Scenario: Schedule profile is selected

r[agent_harness_runtime.schedule_reliability.boundary.selected]
- GIVEN an embedder selects the reviewed schedule profile and supplies injected clocks, durable storage, and a delivery host
- WHEN scheduling is composed
- THEN Animus MAY produce claim, delivery, coalescing, and recovery decisions for session jobs
- AND the existing scheduled-recipe, messaging, ledger, and receipt owners MUST remain authoritative.

#### Scenario: Scheduling expects ambient authority

r[agent_harness_runtime.schedule_reliability.boundary.no_ambient]
- GIVEN a caller or the model expects the schedule layer to read clocks, set timers, or persist directly
- WHEN a decision runs
- THEN Animus MUST return a typed unsupported-authority outcome
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Schedule policy is Nickel-authored and revalidated

r[agent_harness_runtime.schedule_reliability.profile] Human-authored schedule policy MUST use typed Nickel contracts for surfaces, interval bounds, catch-up rules, pending limits, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.schedule_reliability.profile.valid]
- GIVEN a checked profile names the exact cohort, surfaces, finite interval and pending bounds, catch-up rules, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no claim, delivery, or receipt may proceed before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.schedule_reliability.profile.invalid]
- GIVEN generated policy is stale, omits a bound, permits cross-surface mutation, or weakens claim boundaries
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no schedule decision may proceed.

### Requirement: Scheduling surfaces have typed ownership

r[agent_harness_runtime.schedule_reliability.surfaces] Every job record MUST carry its surface (`user_heartbeat`, `agent_recurring`, or `scheduled`) and owner identity, agent-created surfaces MUST NOT mutate user-owned jobs, and each session MUST admit at most one visible user heartbeat.

#### Scenario: Agent manages its own recurring jobs

r[agent_harness_runtime.schedule_reliability.surfaces.agent_owned]
- GIVEN the agent creates, updates, or pauses `agent_recurring` jobs within admitted bounds
- WHEN mutation admission runs
- THEN the mutation MUST apply with a receipt
- AND the user-owned heartbeat MUST remain unchanged.

#### Scenario: Agent attempts to mutate user-owned jobs

r[agent_harness_runtime.schedule_reliability.surfaces.cross_surface]
- GIVEN the agent attempts to replace, clear, or pause the `user_heartbeat` job, or to create a second visible user heartbeat
- WHEN mutation admission runs
- THEN it MUST fail closed with a typed outcome
- AND no mutation, delivery, or receipt may follow.

### Requirement: Due ticks are claimed before delivery

r[agent_harness_runtime.schedule_reliability.claim_before_delivery] A due tick MUST be claimed in durable state before its prompt is delivered, recovery MUST treat a claimed-but-undelivered tick as claimed rather than due, and no tick MAY be delivered before its claim is durable.

#### Scenario: Due tick claims then delivers

r[agent_harness_runtime.schedule_reliability.claim_before_delivery.ordered]
- GIVEN a job with a due tick and a host clock reading
- WHEN the schedule core evaluates
- THEN it MUST return a claim transition followed by a delivery instruction
- AND the host MUST apply the claim to durable state before delivering.

#### Scenario: Crash interrupts delivery after claim

r[agent_harness_runtime.schedule_reliability.claim_before_delivery.crash]
- GIVEN a tick was claimed but the host crashed before delivery completed
- WHEN recovery replays durable state
- THEN the tick MUST NOT be re-delivered
- AND the recovery receipt MUST record the claimed-undelivered outcome.

### Requirement: Missed ticks coalesce into one catch-up decision

r[agent_harness_runtime.schedule_reliability.coalesce] When a job wakes with missed ticks, the core MUST return one policy-named catch-up decision (fire once, skip, or pause) per job, MUST record the missed count, and MUST NOT accumulate an unbounded backlog.

#### Scenario: Downtime produces one catch-up decision

r[agent_harness_runtime.schedule_reliability.coalesce.catch_up]
- GIVEN a job with multiple missed ticks after downtime
- WHEN the schedule core evaluates with a current clock reading
- THEN it MUST return exactly one catch-up decision per the policy rule for that surface
- AND the decision receipt MUST record the missed count.

#### Scenario: Backlog accumulation is attempted

r[agent_harness_runtime.schedule_reliability.coalesce.no_backlog]
- GIVEN any path attempts to queue every missed tick for delivery
- WHEN validation runs
- THEN it MUST fail closed with a typed outcome
- AND pending state MUST remain within admitted bounds.

### Requirement: Schedule state is per-session and deterministically recoverable

r[agent_harness_runtime.schedule_reliability.persistence] Job state MUST live in session-scoped durable storage with domain-separated BLAKE3 identity, recovery after restart MUST replay durable state deterministically, and due ticks MUST be re-derived from the injected clock.

#### Scenario: Restart recovers schedules

r[agent_harness_runtime.schedule_reliability.persistence.recovery]
- GIVEN durable job state and a host restart
- WHEN recovery runs with an injected clock reading
- THEN the recovered job set MUST equal the durable set
- AND due evaluation MUST be identical for identical durable state and clock input.

#### Scenario: Cross-session schedule is requested

r[agent_harness_runtime.schedule_reliability.persistence.session_scoped]
- GIVEN a caller requests a job targeting another session or a global schedule
- WHEN admission runs
- THEN it MUST fail closed with a typed unsupported-scope outcome
- AND no cross-session mutation or ambient registration may occur.

### Requirement: Schedule evidence is metadata-only

r[agent_harness_runtime.schedule_reliability.evidence] Schedule receipts MUST bind only policy identity, job identity, surface, claim, delivery, and coalesce classes, clock identity, missed counts, bounded counts, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude prompt bodies.

#### Scenario: Safe schedule receipt is projected

r[agent_harness_runtime.schedule_reliability.evidence.safe]
- GIVEN claim, delivery, coalescing, or recovery completes under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, classes, counts, and required non-claims
- AND it MUST describe only bounded scheduling facts.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.schedule_reliability.evidence.rejects]
- GIVEN a receipt draft includes prompt bodies, or an exactly-once, real-time, or outcome-correctness claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Schedule adoption has positive and negative conformance evidence

r[agent_harness_runtime.schedule_reliability.verification] The schedule contract MUST include positive and negative tests through production surface, claim, coalescing, recovery, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.schedule_reliability.verification.positive]
- GIVEN fixtures cover on-time claim and delivery, crash recovery without replay, downtime coalescing, surface ownership, and deterministic restart recovery
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.schedule_reliability.verification.negative]
- GIVEN a fixture attempts deliver-then-record ordering, backlog accumulation, cross-surface mutation, second user heartbeats, cross-session schedules, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.schedule_reliability.verification.closeout]
- GIVEN maintainers intend to sync and archive the schedule change
- WHEN closeout validation runs
- THEN focused schedule, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND exactly-once execution, real-time guarantees, timer accuracy, production readiness, and release eligibility MUST remain separate non-claims.
