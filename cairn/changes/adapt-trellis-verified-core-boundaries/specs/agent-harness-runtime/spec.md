# agent-harness-runtime Change Spec: Trellis verified-core boundaries

## Requirements

### Requirement: Candidate inventory

r[agent_harness_runtime.trellis_verified_core.candidate_inventory] Animus MUST inventory pure safety-critical decision cores that are candidates for Trellis-style verification, including policy/effect gates, ledger graph validation, branch replay, receipt redaction, schema compatibility, compaction cut points, and roadmap dependency checks when those areas are in scope.

#### Scenario: Only pure cores are candidates

r[agent_harness_runtime.trellis_verified_core.candidate_inventory.pure_only]
- GIVEN maintainers nominate a verification candidate
- WHEN the candidate inventory is reviewed
- THEN the candidate identifies explicit inputs and outputs
- AND filesystem, process, provider, network, credential, terminal, clock, and ambient global effects are excluded from the verified core.

### Requirement: Proof obligations

r[agent_harness_runtime.trellis_verified_core.obligations] Each selected verified-core candidate MUST declare invariants, preconditions, postconditions, rejected-state classes, and equivalence expectations before proof or model code is treated as release evidence.

#### Scenario: Safety property is explicit

r[agent_harness_runtime.trellis_verified_core.obligations.explicit]
- GIVEN a candidate such as graph validation, effect authorization, redaction, or schema compatibility is selected
- WHEN maintainers inspect its verification plan
- THEN the plan states what cannot happen, what must happen, and which invalid inputs are rejected
- AND the proof/model artifact links back to requirement IDs.

### Requirement: Dependency isolation

r[agent_harness_runtime.trellis_verified_core.dependency_isolation] Trellis or verified-helper dependencies MUST be optional, test-only, or isolated in dedicated verified modules until a stable verified-core dependency boundary is accepted.

#### Scenario: Proof dependency does not leak into default runtime

r[agent_harness_runtime.trellis_verified_core.dependency_isolation.default_clean]
- GIVEN default Animus crates are built without verification features
- WHEN dependency checks run
- THEN default reusable runtime/core crates do not require Trellis or proof-tool dependencies
- AND verification-enabled builds remain explicit.

### Requirement: Model or verified logic

r[agent_harness_runtime.trellis_verified_core.model_logic] Animus SHOULD provide model or verified-core implementations for selected pure decision boundaries where proof-oriented hardening materially improves safety claims.

#### Scenario: Model logic handles deterministic input

r[agent_harness_runtime.trellis_verified_core.model_logic.deterministic]
- GIVEN a selected pure decision boundary has modeled inputs
- WHEN model or verified logic evaluates those inputs
- THEN it returns deterministic outputs or typed rejection classes
- AND it does not perform I/O, mutation of external state, logging, or adapter dispatch.

### Requirement: Equivalence tests

r[agent_harness_runtime.trellis_verified_core.equivalence] When model or verified logic and production Rust logic both exist for a decision boundary, Animus MUST include equivalence tests for valid and invalid inputs.

#### Scenario: Divergence is reported

r[agent_harness_runtime.trellis_verified_core.equivalence.divergence]
- GIVEN model/verified logic and production Rust evaluate the same fixture
- WHEN their accepted/rejected status or normalized outputs differ
- THEN the test fails with a diagnostic naming the fixture and boundary
- AND production release evidence cannot claim verified coverage for that boundary.

### Requirement: Negative invariant fixtures

r[agent_harness_runtime.trellis_verified_core.negative_fixtures] Verified-core work MUST include negative fixtures for malformed graphs, authorization bypass attempts, unsafe redaction inputs, unsupported compatibility cases, and invariant-violating counterexamples relevant to the selected candidates.

#### Scenario: Counterexample remains durable

r[agent_harness_runtime.trellis_verified_core.negative_fixtures.counterexample]
- GIVEN a rejected state could break safety claims if accepted
- WHEN negative fixtures run
- THEN the state is rejected with a typed issue code
- AND the fixture can be replayed without live host services.

### Requirement: Verified-core closeout evidence

r[agent_harness_runtime.trellis_verified_core.closeout] Trellis verified-core work MUST document proof scope and run verification/model checks, equivalence tests, negative fixtures, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Verification evidence is reviewable

r[agent_harness_runtime.trellis_verified_core.closeout.evidence]
- GIVEN verified-core work is ready to archive
- WHEN reviewers inspect evidence
- THEN proof scope, model/equivalence checks, negative fixtures, dependency isolation, workspace checks, and Cairn gates are named with status
- AND unsupported proof tooling is recorded as unavailable rather than silently skipped.
