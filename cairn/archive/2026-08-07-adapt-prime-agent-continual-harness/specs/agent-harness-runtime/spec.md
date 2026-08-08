# Agent Harness Runtime

## ADDED Requirements

### Requirement: Continual harness remains an exact optional boundary

r[agent_harness_runtime.continual_harness.boundary] Animus MUST provide harness refinement as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire filesystem, VCS, process, network, credential, or persistence authority from refinement.

#### Scenario: Refinement profile is selected

r[agent_harness_runtime.continual_harness.boundary.selected]
- GIVEN an embedder selects the reviewed refinement profile and supplies snapshot storage
- WHEN refinement is composed
- THEN Animus MAY admit typed proposals against supplemental harness state
- AND the existing prompt composition, skill manifest, memory, checkpoint-rollback, ledger, and receipt owners MUST remain authoritative.

#### Scenario: Refinement expects ambient authority

r[agent_harness_runtime.continual_harness.boundary.no_ambient]
- GIVEN a proposal or caller expects refinement to read files, edit repo state, or persist directly
- WHEN admission runs
- THEN Animus MUST return a typed unsupported-authority outcome before the effect
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Refinement policy is Nickel-authored and revalidated

r[agent_harness_runtime.continual_harness.profile] Human-authored refinement policy MUST use typed Nickel contracts for admitted target classes, size, count, and retention bounds, evidence requirements, conflict rules, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.continual_harness.profile.valid]
- GIVEN a checked profile names the exact cohort, finite bounds, evidence requirements, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no proposal, snapshot, or receipt may proceed before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.continual_harness.profile.invalid]
- GIVEN generated policy is stale, omits a bound, admits a base-prompt target, or weakens evidence or claim boundaries
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no refinement may proceed.

### Requirement: Refinement proposals are typed and evidence-bound

r[agent_harness_runtime.continual_harness.proposals] Refinement proposals MUST name exactly one admitted target class (supplemental prompt, memory, skill description, or subagent specification), exactly one operation (add, update, or retract), and at least one typed evidence reference into the session ledger.

#### Scenario: Well-formed proposal enters admission

r[agent_harness_runtime.continual_harness.proposals.well_formed]
- GIVEN a proposal names an admitted target class, one operation, bounded content, and valid evidence references
- WHEN proposal validation runs
- THEN it MUST return a typed proposal ready for admission
- AND identical proposals under identical policy MUST validate identically.

#### Scenario: Proposal lacks evidence or targets an inadmissible class

r[agent_harness_runtime.continual_harness.proposals.rejected]
- GIVEN a proposal has no evidence reference, a malformed reference, or a target outside the four admitted classes
- WHEN proposal validation runs
- THEN it MUST fail with a bounded typed issue
- AND no state transition, snapshot, or receipt may follow.

### Requirement: Refinement admission is pure and fail-closed

r[agent_harness_runtime.continual_harness.admission] Admission MUST be a pure deterministic function over proposal, current supplemental state, and policy, MUST enforce size, count, conflict, and authority bounds, and MUST reject base system prompt, repo-owned skill package, accepted-spec, and policy-file targets.

#### Scenario: Admitted proposal produces a state plan

r[agent_harness_runtime.continual_harness.admission.admitted]
- GIVEN a validated proposal fits all bounds and conflicts with no admitted state
- WHEN admission runs
- THEN it MUST return a snapshot plan followed by a deterministic state transition
- AND repeated admission of identical inputs MUST return the same plan.

#### Scenario: Immutable or conflicting target is proposed

r[agent_harness_runtime.continual_harness.admission.rejected]
- GIVEN a proposal targets the base prompt, a repo-owned artifact, an accepted spec, or state that conflicts under policy
- WHEN admission runs
- THEN it MUST fail closed with a typed outcome naming the violated rule
- AND current state MUST remain unchanged.

### Requirement: Snapshots precede updates and support exact rollback

r[agent_harness_runtime.continual_harness.snapshots] Every admitted update MUST be preceded by a deterministic snapshot with domain-separated BLAKE3 identity over the pre-update state, retention MUST obey named count and byte bounds, and rollback MUST restore an admitted snapshot exactly.

#### Scenario: Update follows snapshot

r[agent_harness_runtime.continual_harness.snapshots.ordering]
- GIVEN an admitted proposal is ready to apply
- WHEN the update executes
- THEN the snapshot MUST be recorded before the state transition
- AND the snapshot identity MUST appear in the update receipt.

#### Scenario: Rollback restores retained state

r[agent_harness_runtime.continual_harness.snapshots.rollback]
- GIVEN a retained admitted snapshot exists
- WHEN the host requests rollback to its identity
- THEN the supplemental state MUST equal the snapshotted state exactly
- AND rollback to an unknown, evicted, or forged identity MUST fail closed.

### Requirement: Refinement state is session-local by default

r[agent_harness_runtime.continual_harness.scope] Admitted refinement state MUST remain session-local, promotion to durable project or personal state MUST be an explicit host action with separate admission and receipt, and refinement proposals MUST NOT be able to trigger promotion.

#### Scenario: Refinement applies within one session

r[agent_harness_runtime.continual_harness.scope.session_local]
- GIVEN admitted refinements accumulate during a session
- WHEN the session ends or another session starts
- THEN the refinements MUST NOT appear in the other session or in durable state
- AND only an explicit host promotion MAY copy selected refinements to durable state.

#### Scenario: Proposal requests promotion

r[agent_harness_runtime.continual_harness.scope.no_self_promotion]
- GIVEN a proposal or model action attempts durable promotion, cross-session sharing, or ambient persistence
- WHEN admission runs
- THEN it MUST fail closed with a typed outcome
- AND no durable write, VCS mutation, or ambient registration may occur.

### Requirement: Refinement evidence is metadata-only

r[agent_harness_runtime.continual_harness.evidence] Refinement receipts MUST bind only policy identity, target class, operation, snapshot identities, bounded counts, status, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude lesson bodies, prompt text, memory contents, skill bodies, and trajectory spans.

#### Scenario: Safe refinement receipt is projected

r[agent_harness_runtime.continual_harness.evidence.safe]
- GIVEN admission, update, snapshot, or rollback completes under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, classes, counts, and required non-claims
- AND it MUST describe only bounded state transitions, never content.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.continual_harness.evidence.rejects]
- GIVEN a receipt draft includes lesson bodies, prompt text, memory contents, trajectory spans, or an improvement, correctness, or generalization claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Continual harness adoption has positive and negative conformance evidence

r[agent_harness_runtime.continual_harness.verification] The continual harness MUST include positive and negative tests through production proposal, admission, snapshot, rollback, scope, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.continual_harness.verification.positive]
- GIVEN fixtures cover admitted proposals in all four target classes, snapshot-before-update ordering, exact rollback, session-local scope, and explicit promotion
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.continual_harness.verification.negative]
- GIVEN a fixture attempts base-prompt mutation, evidence-free refinement, bound violations, forged snapshots, implicit promotion, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.continual_harness.verification.closeout]
- GIVEN maintainers intend to sync and archive the continual-harness change
- WHEN closeout validation runs
- THEN focused refinement, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND lesson correctness, model improvement, automatic promotion, production readiness, and release eligibility MUST remain separate non-claims.
