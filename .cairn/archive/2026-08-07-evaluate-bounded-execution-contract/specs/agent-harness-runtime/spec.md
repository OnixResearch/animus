# Agent Harness Runtime

## ADDED Requirements

### Requirement: Cross-stack bounded-execution inventory

r[agent_harness_runtime.bounded_execution_evaluation.inventory] Animus MUST record a reviewed inventory of execution request, environment, filesystem, network, resource, timeout, cancellation, output, cleanup, mechanism, observation, receipt, and retained-authority semantics in Animus, Octet, Nickel Export, and Mantle at exact immutable source identities before recommending repository extraction.

#### Scenario: Complete inventory classifies ownership

r[agent_harness_runtime.bounded_execution_evaluation.inventory.complete]
- GIVEN the four reviewed execution surfaces and exact source identities
- WHEN the inventory is evaluated
- THEN each field and outcome MUST be classified as shared candidate semantics, consumer extension, mechanism-specific fact, retained product authority, interoperability metadata, or explicit blocker
- AND similar labels MUST NOT be treated as equivalent without evidence.

#### Scenario: Missing execution semantics block a decision

r[agent_harness_runtime.bounded_execution_evaluation.inventory.incomplete]
- GIVEN an inventory omits a consumer's environment, network, timeout, output, cleanup, or claim semantics
- WHEN decision evaluation runs
- THEN the terminal status MUST NOT be `extract`
- AND the missing consumer and semantic classes MUST appear as bounded blockers.

### Requirement: Capability states remain explicit

r[agent_harness_runtime.bounded_execution_evaluation.capability_states] The evaluation profile MUST distinguish requested, configured, enforced, observed, unavailable, unsupported, and not-applicable execution capability states and MUST NOT collapse mechanism strength into a boolean `sandboxed`, `isolated`, `hermetic`, or `network_denied` label.

#### Scenario: Mechanism observations remain visible

r[agent_harness_runtime.bounded_execution_evaluation.capability_states.visible]
- GIVEN two consumers request the same network or filesystem restriction through different mechanisms
- WHEN their observations are mapped
- THEN the matrix MUST preserve the mechanism, enforcement point, observation source, unavailable behavior, and bounded non-claims
- AND common vocabulary MAY be emitted only where the reviewed mapping is lossless.

#### Scenario: Stronger claim laundering is rejected

r[agent_harness_runtime.bounded_execution_evaluation.capability_states.overclaim]
- GIVEN a mapping would represent a requested, configured, unavailable, or partially observed restriction as enforced
- WHEN mapping validation runs
- THEN the mapping MUST fail closed with an explicit semantic-loss class.

### Requirement: Shared outcome vocabulary is loss aware

r[agent_harness_runtime.bounded_execution_evaluation.outcomes] The evaluation MUST define a candidate shared outcome vocabulary for success, nonzero exit, launch denial, missing executable, policy rejection, timeout, cancellation, signal termination, output overflow, cleanup failure, and unsupported mechanism while preserving consumer-specific detail through explicit extensions or blockers.

#### Scenario: Stable common outcome maps losslessly

r[agent_harness_runtime.bounded_execution_evaluation.outcomes.positive]
- GIVEN consumer-native evidence for an outcome whose cause and terminal semantics match the reviewed shared class
- WHEN mapping validation runs
- THEN the candidate shared outcome and retained native detail MUST round trip without changing success, failure, retry, cleanup, or claim meaning.

#### Scenario: Ambiguous outcome is not coerced

r[agent_harness_runtime.bounded_execution_evaluation.outcomes.negative]
- GIVEN a native result combines timeout, output truncation, cleanup uncertainty, or policy denial in a way the shared vocabulary cannot represent
- WHEN mapping validation runs
- THEN the result MUST be classified as an explicit extension or blocker rather than coerced into a weaker common class.

### Requirement: Cross-product conformance includes failures

r[agent_harness_runtime.bounded_execution_evaluation.conformance] Animus MUST maintain positive and negative cross-product fixtures for execution success, nonzero exit, launch denial, missing executable, environment rejection, path or root escape, timeout, cancellation, signal termination, output overflow, network-policy differences, cleanup failure, unsupported mechanisms, stale source identity, unsafe diagnostics, and weakened non-claims.

#### Scenario: Positive fixtures preserve reviewed meaning

r[agent_harness_runtime.bounded_execution_evaluation.conformance.positive]
- GIVEN valid representative observations from Animus, Octet, Nickel Export, and Mantle
- WHEN conformance evaluation runs
- THEN every mapped field, outcome, capability state, bound, identity, and non-claim MUST match its reviewed profile.

#### Scenario: Negative fixtures fail with stable classes

r[agent_harness_runtime.bounded_execution_evaluation.conformance.negative]
- GIVEN a lossy mapping, stale source, unbounded output, hidden environment authority, unsupported mechanism claim, unavailable enforcement described as active, unsafe diagnostic, or weakened non-claim
- WHEN conformance evaluation runs
- THEN it MUST fail deterministically with the expected stable class and MUST NOT emit extraction-ready status.

### Requirement: Extraction decision is deterministic and terminal

r[agent_harness_runtime.bounded_execution_evaluation.decision] A pure evaluator MUST emit a BLAKE3-bound decision receipt with terminal status `extract`, `incubate`, or `blocked`, reviewed source and profile identities, conformance summary, mapping-loss findings, dependency analysis, retained authorities, blockers, and the next smallest action.

#### Scenario: Extraction criteria are satisfied

r[agent_harness_runtime.bounded_execution_evaluation.decision.extract]
- GIVEN at least two independent consumers need the same mandatory contract, all mandatory mappings are lossless, the candidate core has no ambient authority or product dependency cycle, and independent release and rollback have concrete value
- WHEN decision evaluation runs
- THEN it MAY emit `extract` with the exact admitted shared surface and required successor Cairn scope.

#### Scenario: Evidence supports incubation or blocking

r[agent_harness_runtime.bounded_execution_evaluation.decision.not_extract]
- GIVEN only one committed consumer, unresolved semantic loss, a product dependency cycle, unstable mechanism semantics, missing source evidence, or no independent release value
- WHEN decision evaluation runs
- THEN it MUST emit `incubate` or `blocked` with exact reasons
- AND the result MUST be accepted as an honest terminal outcome rather than converted into a speculative extraction.

### Requirement: Evaluation retains product execution authority

r[agent_harness_runtime.bounded_execution_evaluation.authority] This change MUST keep process launch, filesystem access, environment construction, network configuration, clocks, signals, child reaping, timeout enforcement, cancellation, output capture, cleanup, sandbox setup, proof admission, build policy, and receipt writes in their existing product-owned shells.

#### Scenario: Evaluation consumes observations only

r[agent_harness_runtime.bounded_execution_evaluation.authority.observations]
- GIVEN a consumer executes or supervises a process
- WHEN its facts enter the feasibility matrix
- THEN the evaluation MUST consume already-loaded bounded observations only and MUST NOT invoke the consumer executor or sandbox mechanism.

#### Scenario: No repository is created by evaluation

r[agent_harness_runtime.bounded_execution_evaluation.authority.no_cutover]
- GIVEN the evaluation emits any terminal status
- WHEN this change closes
- THEN it MUST NOT have created a standalone runtime dependency, moved public execution DTO ownership, or cut over a consumer
- AND an `extract` result MUST require a separate successor Cairn before those mutations.

### Requirement: Bounded-execution claims deny universal sandbox equivalence

r[agent_harness_runtime.bounded_execution_evaluation.non_claims] Evaluation profiles, fixtures, decision receipts, and documentation MUST state that mapping success proves only reviewed DTO and classification compatibility for the measured source cohort and does not prove sandbox equivalence, hermeticity, network denial, filesystem isolation, executor correctness, proof soundness, build correctness, future determinism, or release eligibility.

#### Scenario: Required non-claims are present

r[agent_harness_runtime.bounded_execution_evaluation.non_claims.present]
- GIVEN a conformance or decision receipt is produced
- WHEN non-claim validation runs
- THEN every required bounded denial MUST be present before the artifact can contribute to an `extract` result.

#### Scenario: Universal-equivalence wording is rejected

r[agent_harness_runtime.bounded_execution_evaluation.non_claims.overclaim]
- GIVEN a matrix, profile, fixture, or receipt describes different execution mechanisms as universally equivalent or promotes observation compatibility to correctness
- WHEN validation runs
- THEN the artifact MUST be rejected with a stable overclaim issue.
