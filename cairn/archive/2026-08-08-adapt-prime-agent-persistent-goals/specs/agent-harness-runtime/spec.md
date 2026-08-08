# Agent Harness Runtime

## ADDED Requirements

### Requirement: Persistent goals remain an exact optional boundary

r[agent_harness_runtime.persistent_goals.boundary] Animus MUST provide persistent goals as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire clock, filesystem, process, network, credential, or persistence authority from goal state.

#### Scenario: Goal profile is selected

r[agent_harness_runtime.persistent_goals.boundary.selected]
- GIVEN an embedder selects the reviewed goal profile and supplies injected clocks, counters, and persistence
- WHEN goal tracking is composed
- THEN Animus MAY maintain one durable goal per session with accounting
- AND the existing continuation, ralph-loop, ledger, and receipt owners MUST remain authoritative.

#### Scenario: Goal state expects ambient authority

r[agent_harness_runtime.persistent_goals.boundary.no_ambient]
- GIVEN a caller or the model expects goal state to read clocks, write files, or persist directly
- WHEN a transition runs
- THEN Animus MUST return a typed unsupported-authority outcome
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Goal policy is Nickel-authored and revalidated

r[agent_harness_runtime.persistent_goals.profile] Human-authored goal policy MUST use typed Nickel contracts for budget classes, accounting fields, admitted transitions, the one-active-goal rule, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.persistent_goals.profile.valid]
- GIVEN a checked profile names the exact cohort, budget classes, admitted transitions, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no goal transition or receipt may proceed before admission.

#### Scenario: Policy drifts or weakens success semantics

r[agent_harness_runtime.persistent_goals.profile.invalid]
- GIVEN generated policy is stale, admits model-created goals, or lets a non-completion transition yield `completed`
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no goal transition may proceed.

### Requirement: Goal state is durable, typed, and accounted

r[agent_harness_runtime.persistent_goals.state] The goal record MUST carry objective identity, status, creation provenance, token and elapsed accounting, continuation count, and optional explicit budget, transitions MUST be pure functions over host-injected clocks and counters, and at most one goal per session MAY be active.

#### Scenario: Goal lifecycle advances under accounting

r[agent_harness_runtime.persistent_goals.state.transitions]
- GIVEN an active goal and host-injected counter updates
- WHEN turns complete
- THEN the goal MUST accumulate token, elapsed, and continuation accounting within named bounds
- AND identical inputs MUST produce identical transitions.

#### Scenario: Second active goal is requested

r[agent_harness_runtime.persistent_goals.state.one_active]
- GIVEN a session already holds an active goal
- WHEN a create request arrives
- THEN the new goal MUST be admitted only as paused or rejected with a typed outcome
- AND the existing active goal MUST remain unchanged.

### Requirement: Only explicit completion marks success

r[agent_harness_runtime.persistent_goals.continuation] An active goal MUST keep presenting its objective to continuation policy after ordinary assistant turns, and only an admitted explicit completion transition MUST produce the `completed` status.

#### Scenario: Active goal continues after an ordinary turn

r[agent_harness_runtime.persistent_goals.continuation.after_turn]
- GIVEN an active goal and an ordinary assistant turn that did not complete the goal
- WHEN continuation policy evaluates
- THEN the active goal MUST raise continuation priority
- AND the goal status MUST remain `active`.

#### Scenario: Non-completion events attempt success

r[agent_harness_runtime.persistent_goals.continuation.no_implicit_success]
- GIVEN session detach, compaction, turn exhaustion, or an ordinary stop
- WHEN goal status is read
- THEN the status MUST NOT be `completed`
- AND any projection that reports these events as completion MUST fail validation.

### Requirement: Budget exhaustion is a typed non-success outcome

r[agent_harness_runtime.persistent_goals.budget] Token, time, and continuation budgets MUST be explicit, exhaustion MUST stop continuation with the `budget_limited` status, and downstream owners MUST NOT report budget exhaustion as task success.

#### Scenario: Budget exhausts during continuation

r[agent_harness_runtime.persistent_goals.budget.exhausted]
- GIVEN an active goal reaches its token, time, or continuation budget
- WHEN the next continuation is evaluated
- THEN continuation MUST stop and the goal MUST transition to `budget_limited`
- AND the receipt MUST name the exhausted budget class.

#### Scenario: Budget-limited goal is presented as success

r[agent_harness_runtime.persistent_goals.budget.not_success]
- GIVEN a goal in `budget_limited`, `errored`, or `cleared` status
- WHEN a projection or receipt describes the outcome
- THEN it MUST NOT claim task success or objective achievement
- AND validation MUST reject any success claim for non-`completed` goals.

### Requirement: Goal lifecycle ownership is explicit

r[agent_harness_runtime.persistent_goals.ownership] Create, pause, resume, and clear MUST be explicit user or host actions with provenance, the model surface MUST be limited to goal inspection and completion requests, and the agent MUST NOT infer goals from ordinary tasks.

#### Scenario: Host creates a goal explicitly

r[agent_harness_runtime.persistent_goals.ownership.explicit]
- GIVEN a user or host issues a create, pause, resume, or clear action with provenance
- WHEN admission runs
- THEN the transition MUST apply with a receipt
- AND the provenance MUST appear in the goal record.

#### Scenario: Model attempts lifecycle control or goal inference

r[agent_harness_runtime.persistent_goals.ownership.no_inference]
- GIVEN the model attempts to create, pause, resume, or clear a goal, or a task prompt is treated as an implicit goal
- WHEN admission runs
- THEN it MUST fail closed with a typed outcome
- AND the model MAY only inspect the active goal or request completion.

### Requirement: Goal evidence is metadata-only

r[agent_harness_runtime.persistent_goals.evidence] Goal receipts MUST bind only policy identity, lifecycle transition, accounting counter identities and values, budget class, status, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude objective text, trajectory spans, and success overclaims.

#### Scenario: Safe goal receipt is projected

r[agent_harness_runtime.persistent_goals.evidence.safe]
- GIVEN a goal transition completes under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, transition class, accounting, and required non-claims
- AND it MUST describe only bounded state transitions.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.persistent_goals.evidence.rejects]
- GIVEN a receipt draft includes objective text, trajectory spans, or a success claim for a non-`completed` goal
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Persistent-goal adoption has positive and negative conformance evidence

r[agent_harness_runtime.persistent_goals.verification] The persistent-goal contract MUST include positive and negative tests through production state, transition, continuation, budget, ownership, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.persistent_goals.verification.positive]
- GIVEN fixtures cover explicit lifecycle actions, accounting, admitted completion, budget exhaustion, and goal-aware continuation
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.persistent_goals.verification.negative]
- GIVEN a fixture attempts model-inferred goals, model lifecycle control, invalid transitions, second active goals, budget-as-success confusion, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.persistent_goals.verification.closeout]
- GIVEN maintainers intend to sync and archive the persistent-goal change
- WHEN closeout validation runs
- THEN focused goal, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND objective achievement, work quality, automatic inference, production readiness, and release eligibility MUST remain separate non-claims.
