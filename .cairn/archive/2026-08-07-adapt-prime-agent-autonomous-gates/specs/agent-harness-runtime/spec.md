# Agent Harness Runtime

## ADDED Requirements

### Requirement: Autonomous gates remain an exact optional boundary

r[agent_harness_runtime.autonomous_gates.boundary] Animus MUST provide autonomous continuation with quality gates as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire process, filesystem, clock, network, credential, or persistence authority from gates or change detection.

#### Scenario: Autonomous profile is selected

r[agent_harness_runtime.autonomous_gates.boundary.selected]
- GIVEN an embedder selects the reviewed autonomous profile and supplies gate executors, clocks, and fingerprint inputs
- WHEN autonomous mode is composed
- THEN Animus MAY decide bounded continuations from typed gate results and fingerprints
- AND the existing continuation, ralph-loop, tool-executor, and receipt owners MUST remain authoritative.

#### Scenario: Autonomous mode expects ambient authority

r[agent_harness_runtime.autonomous_gates.boundary.no_ambient]
- GIVEN a caller or the model expects the autonomous layer to run commands, read files, or access clocks directly
- WHEN a decision runs
- THEN Animus MUST return a typed unsupported-authority outcome
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Autonomous policy is Nickel-authored and revalidated

r[agent_harness_runtime.autonomous_gates.profile] Human-authored autonomous policy MUST use typed Nickel contracts for continuation, turn, token, and wall-clock limits, gate descriptors, output bounds, fingerprint input identity, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.autonomous_gates.profile.valid]
- GIVEN a checked profile names the exact cohort, finite limits, gate descriptors, output bounds, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no continuation, gate decision, or receipt may proceed before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.autonomous_gates.profile.invalid]
- GIVEN generated policy is stale, omits a limit, admits reusable-crate execution, or weakens the success boundary
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no autonomous continuation may proceed.

### Requirement: Autonomous continuation is bounded

r[agent_harness_runtime.autonomous_gates.continuation] Autonomous continuation MUST enforce explicit continuation, turn, token, and wall-clock limits as typed inputs to the continuation owner, and reaching any limit MUST stop continuation with a typed limit outcome.

#### Scenario: Continuation proceeds within limits

r[agent_harness_runtime.autonomous_gates.continuation.within_limits]
- GIVEN an autonomous run with all counters below admitted limits
- WHEN the continuation owner evaluates after a turn
- THEN it MAY inject a bounded follow-up continuation
- AND the decision MUST record which counters were consulted.

#### Scenario: A limit is reached

r[agent_harness_runtime.autonomous_gates.continuation.limit_reached]
- GIVEN a continuation, turn, token, or wall-clock counter reaches its admitted limit
- WHEN the continuation owner evaluates
- THEN continuation MUST stop with a typed `limit_reached` outcome naming the exhausted class
- AND the outcome MUST NOT be reported as task success.

### Requirement: Quality gates are host-injected with bounded output

r[agent_harness_runtime.autonomous_gates.gates] Gates MUST be policy-admitted descriptors executed only by the host, results MUST be typed (pass, fail with class, or timeout), and failed-gate output MUST be bounded and classified before it enters the prompt.

#### Scenario: Failed gate returns bounded output for another attempt

r[agent_harness_runtime.autonomous_gates.gates.failed_retry]
- GIVEN a host-executed gate fails and limits remain
- WHEN the gate result is projected
- THEN the agent MUST receive bounded, classified output for another attempt
- AND raw unbounded output MUST NOT enter context.

#### Scenario: Gate execution is attempted inside reusable code

r[agent_harness_runtime.autonomous_gates.gates.no_execution]
- GIVEN any path attempts to spawn, exec, or shell a gate inside reusable crates
- WHEN validation runs
- THEN it MUST fail closed with a typed unsupported-authority outcome
- AND no process, filesystem, or clock effect may occur.

### Requirement: Failed gates do not rerun without workspace change

r[agent_harness_runtime.autonomous_gates.change_detection] The contract MUST record (gate identity, workspace fingerprint, outcome) per attempt and MUST deny rerunning a failed gate whose domain-separated BLAKE3 fingerprint over the admitted file set is unchanged.

#### Scenario: Fingerprint changed since failure

r[agent_harness_runtime.autonomous_gates.change_detection.changed]
- GIVEN a failed gate and a new fingerprint that differs from the recorded failure fingerprint
- WHEN rerun admission runs
- THEN the gate MAY rerun
- AND the new attempt MUST be recorded with its fingerprint.

#### Scenario: Fingerprint unchanged since failure

r[agent_harness_runtime.autonomous_gates.change_detection.unchanged]
- GIVEN a failed gate and a fingerprint identical to the recorded failure fingerprint
- WHEN rerun admission runs
- THEN the rerun MUST be denied with a typed unchanged-workspace outcome
- AND the denial MUST state that coverage outside the admitted file set is unknown.

### Requirement: Gates and limits never imply task success

r[agent_harness_runtime.autonomous_gates.no_success_claim] Gate results MUST carry a typed verified-scope field, continuation outcomes MUST carry explicit `gates_passed` or `limit_reached` classes, and projections MUST NOT map either to task success or objective completion.

#### Scenario: All gates pass

r[agent_harness_runtime.autonomous_gates.no_success_claim.gates_passed]
- GIVEN every admitted gate passes
- WHEN the autonomous outcome is projected
- THEN the outcome MUST carry `gates_passed` with the union of verified scopes
- AND any projection of task success, objective completion, or release eligibility MUST fail validation.

#### Scenario: Outcome is described as success

r[agent_harness_runtime.autonomous_gates.no_success_claim.rejected]
- GIVEN a receipt or projection maps `gates_passed` or `limit_reached` to success or completion
- WHEN validation runs
- THEN it MUST reject the projection with a bounded typed issue
- AND completion authority MUST remain with goal and completion-promise owners.

### Requirement: Autonomous evidence is metadata-only

r[agent_harness_runtime.autonomous_gates.evidence] Autonomous receipts MUST bind only policy identity, gate identity, verified scope, outcome class, fingerprint identity, limit class, counter identities, bounded counts, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude raw gate output and unadmitted workspace paths.

#### Scenario: Safe autonomous receipt is projected

r[agent_harness_runtime.autonomous_gates.evidence.safe]
- GIVEN continuation and gate decisions complete under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, classes, scopes, and required non-claims
- AND it MUST describe only bounded continuation and gate facts.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.autonomous_gates.evidence.rejects]
- GIVEN a receipt draft includes raw gate output, unadmitted paths, or a success, completeness, or release claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Autonomous-gate adoption has positive and negative conformance evidence

r[agent_harness_runtime.autonomous_gates.verification] The autonomous-gate contract MUST include positive and negative tests through production continuation, gate-result, output-bounding, fingerprint, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.autonomous_gates.verification.positive]
- GIVEN fixtures cover gate pass, bounded retry, fingerprint-skip, changed-fingerprint rerun, and clean limit stops
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.autonomous_gates.verification.negative]
- GIVEN a fixture attempts reusable-crate execution, unbounded output, unchanged-fingerprint rerun, limit-as-success confusion, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.autonomous_gates.verification.closeout]
- GIVEN maintainers intend to sync and archive the autonomous-gate change
- WHEN closeout validation runs
- THEN focused autonomous, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND task success, verification completeness, gate sandboxing, production readiness, and release eligibility MUST remain separate non-claims.
