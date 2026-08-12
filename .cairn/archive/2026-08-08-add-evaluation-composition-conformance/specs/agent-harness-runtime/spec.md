## ADDED Requirements

### Requirement: Native evaluation conformance uses a versioned matrix

r[agent_harness_runtime.native_evaluation_conformance.matrix] Animus MUST define a typed versioned conformance matrix that binds the native evaluation schema, crate and feature cohort, fixture families, public entrypoints, fake service catalog, named bounds, expected outcomes, redaction, evidence, and non-claims across core, adapter, host, ledger, batch, role, gateway, verifier, artifact, persistence, Harbor, ATIF, evaluation-feedback, dataset-export, and receipt boundaries.

#### Scenario: Checked matrix is loaded

r[agent_harness_runtime.native_evaluation_conformance.matrix.valid]
- GIVEN generated matrix data matches its Nickel source, selected feature cohort, registered fixture families, and canonical BLAKE3 identity
- WHEN Rust revalidates the matrix
- THEN every required family and public entrypoint is available for deterministic execution
- AND no live service or runtime is discovered.

#### Scenario: Matrix is incomplete or stale

r[agent_harness_runtime.native_evaluation_conformance.matrix.invalid]
- GIVEN the matrix omits a required family, names an unknown or private entrypoint, has stale schema or feature identity, weakens a bound, or requests a live dependency
- WHEN matrix validation runs
- THEN the conformance run is blocked with bounded diagnostics
- AND partial coverage cannot produce a passing summary.

### Requirement: Positive conformance covers every evaluation mechanism

r[agent_harness_runtime.native_evaluation_conformance.positive] The conformance suite MUST exercise public production paths for exact and bounded lazy tasksets, single-agent, best-of-N/pass-at-N, user-simulation, isolated-agentic-judge, ledger-bound episodes, exact service observations, gateway retry and commit ordering, isolated verification, safe artifacts, finite rewards, atomic persistence, exact resume, Harbor and ATIF compatibility, advisory feedback, admitted dataset drafts, and safe receipts.

#### Scenario: Full positive matrix runs

r[agent_harness_runtime.native_evaluation_conformance.positive.complete]
- GIVEN every positive fixture has a checked profile, exact expected identities, and scripted bounded fake services
- WHEN the conformance harness executes the matrix
- THEN every fixture reaches its expected public terminal outcome and safe evidence identity
- AND family-level coverage shows no omitted required mechanism.

#### Scenario: Positive fixture bypasses a production boundary

r[agent_harness_runtime.native_evaluation_conformance.positive.bypass]
- GIVEN a fixture constructs a final episode, reward, resume decision, gateway result, or receipt without invoking its public production entrypoint
- WHEN harness validation runs
- THEN the fixture is rejected as a bypass
- AND it cannot contribute positive coverage.

### Requirement: Negative conformance proves fail-closed behavior

r[agent_harness_runtime.native_evaluation_conformance.negative] Every conformance mechanism family MUST include negative fixtures that assert an exact diagnostic family, no prohibited service call, unchanged reusable state, no success receipt, and bounded evidence for identity drift, authority widening, malformed data, race conditions, verifier or artifact compromise, stale persistence, privacy leaks, and unsupported claims.

#### Scenario: Gateway terminal event arrives early

r[agent_harness_runtime.native_evaluation_conformance.negative.gateway]
- GIVEN a fixture supplies a terminal stream event without matching durable commit evidence
- WHEN the public gateway and host entrypoints evaluate it
- THEN the exact early-terminal diagnostic is returned and no scorer or completion path runs
- AND ledger, episode, reward, resume, and receipt success state remain unchanged.

#### Scenario: Verifier artifact is hostile

r[agent_harness_runtime.native_evaluation_conformance.negative.verifier]
- GIVEN a fixture supplies a shared mutable verifier, planted link, colliding artifact, hidden test, or widened manifest
- WHEN public artifact and verifier entrypoints evaluate it
- THEN the expected isolation or artifact diagnostic is returned before verifier execution
- AND no authoritative or passing receipt is produced.

#### Scenario: Resume evidence is stale

r[agent_harness_runtime.native_evaluation_conformance.negative.resume]
- GIVEN a fixture changes task, model, harness, tool, verifier, rubric, reward, sampling, redaction, output, or policy identity while retaining prior completion data
- WHEN public resume evaluation runs
- THEN work remains owed or blocked with the exact stale reason
- AND stale completion cannot suppress execution.

#### Scenario: Evidence contains private or overstated data

r[agent_harness_runtime.native_evaluation_conformance.negative.evidence]
- GIVEN a fixture includes prompts, hidden tests, reasoning, tool or provider bodies, credentials, environment values, physical paths, unrestricted logs, or unsupported mechanism, benchmark, completion, production, or release claims
- WHEN public receipt validation runs
- THEN the expected leak or overclaim diagnostic is returned
- AND no success identity is emitted.

### Requirement: Conformance calls production entrypoints

r[agent_harness_runtime.native_evaluation_conformance.production_paths] Conformance fixtures MUST construct inputs and deterministic fake services only, while task planning, service admission, host execution, episode and reward reduction, gateway ordering, persistence and resume, compatibility projection, and receipt validation MUST call the same public production entrypoints available to embedders, with no alternate test-only implementation.

#### Scenario: Deterministic fake service is used

r[agent_harness_runtime.native_evaluation_conformance.production_paths.fake]
- GIVEN a fixture needs a model, tool, verifier, artifact, timer, cancellation, cleanup, or storage observation
- WHEN the harness executes it
- THEN a fake implements the public adapter port and returns the scripted typed observation
- AND assertions evaluate public outputs, identities, diagnostics, and unchanged-state properties rather than only call counts.

#### Scenario: Test-only evaluator is introduced

r[agent_harness_runtime.native_evaluation_conformance.production_paths.duplicate]
- GIVEN fixture code reimplements planning, reward reduction, resume matching, gateway admission, artifact policy, or receipt projection
- WHEN source-shape and entrypoint coverage checks run
- THEN the duplicate implementation fails conformance admission
- AND its results cannot satisfy production coverage.

### Requirement: Conformance evidence is bounded and non-authoritative

r[agent_harness_runtime.native_evaluation_conformance.evidence] Conformance summaries MUST bind safe profile, feature, family, fixture, public-entrypoint, dependency, status, expected-diagnostic, count, redaction, non-claim, and BLAKE3 identity metadata and MUST exclude fixture bodies, transcripts, hidden tests, reasoning, provider or tool bodies, credentials, environment values, physical paths, unrestricted logs, and live-behavior claims.

#### Scenario: Matrix completes

r[agent_harness_runtime.native_evaluation_conformance.evidence.complete]
- GIVEN every required fixture has a deterministic terminal pass, expected failure, or exact blocker
- WHEN the focused rail emits its summary
- THEN the summary reports family coverage and bounded evidence without fixture content
- AND it states that live host, provider, tool, verifier, persistence, benchmark, and release behavior remain unproved.

#### Scenario: Summary promotes fixture success

r[agent_harness_runtime.native_evaluation_conformance.evidence.overclaim]
- GIVEN a summary claims provider correctness, sandboxing, network enforcement, storage durability, exactly-once effects, verifier soundness, model quality, benchmark validity, production readiness, completion, or release eligibility
- WHEN evidence validation runs
- THEN the summary is rejected
- AND no passing conformance receipt is emitted.

### Requirement: Evaluation composition conformance is a focused deterministic rail

r[agent_harness_runtime.native_evaluation_conformance.verification] Native evaluation conformance MUST provide a focused script and Nix check that run profile freshness, public-entrypoint guards, every positive and negative family, dependency isolation, component and facade coverage, and safe evidence validation without live providers, tools, processes, sockets, containers, clouds, credentials, networks, schedulers, persistence backends, benchmarks, Verifiers, or Python runtimes. Complete closeout MUST also run Cairn validation and lifecycle gates.

#### Scenario: Conformance change is ready to archive

r[agent_harness_runtime.native_evaluation_conformance.verification.closeout]
- GIVEN maintainers intend to sync and archive the evaluation composition conformance change
- WHEN closeout checks run
- THEN the complete deterministic matrix, production-path guards, dependency checks, safe evidence, Cairn gates, and Nix check pass or record an exact blocker
- AND no skipped live check is represented as successful production evidence.
