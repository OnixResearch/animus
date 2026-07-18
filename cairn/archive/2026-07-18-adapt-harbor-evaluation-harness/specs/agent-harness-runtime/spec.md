# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Harbor evaluation adaptation is optional and bounded

r[agent_harness_runtime.harbor_eval.boundary] Animus MUST adapt Harbor task, dataset, trial, job, verifier, reward, and trajectory mechanisms only through Animus-owned reusable contracts and a non-default host boundary pinned to exact reviewed interoperability cohorts, and reusable/default/no-std/Wasm crates MUST NOT acquire Harbor, Python, Pydantic, container, cloud, registry, filesystem, process, credential, network, clock, scheduler, or persistence dependencies.

#### Scenario: Harbor backend is explicitly selected

- GIVEN an embedder selects the reviewed Harbor interoperability profile and supplies every required host service and policy input
- WHEN evaluation composition runs
- THEN Animus MAY generate or consume the admitted Harbor-native artifacts and dispatch through the injected backend
- AND existing Animus runtime, authority, redaction, receipt, and lifecycle owners MUST remain authoritative.

#### Scenario: Default runtime is built

- GIVEN Animus is built without the Harbor interoperability profile
- WHEN dependency and capability checks run
- THEN task, trial, reward, and trajectory contracts that are independently useful MUST remain available under their owning features
- AND no Harbor, Python, container, cloud, or registry runtime MUST be discovered or linked implicitly.

### Requirement: Evaluation configuration is Nickel-authored and explicit

r[agent_harness_runtime.harbor_eval.configuration] Human-authored evaluation tasks, datasets, agents, model routes, environments, verifiers, artifacts, rewards, and jobs MUST be validated by typed Nickel contracts and deterministically projected into exact selected Harbor-native formats with source and projection BLAKE3 identities, while runtime Rust MUST revalidate generated data before host execution.

#### Scenario: Reviewed configuration projects successfully

- GIVEN a typed configuration declares exact task, environment, agent, verifier, artifact, reward, dataset, and job policy including explicit network posture and finite named budgets
- WHEN the selected Harbor projection is generated and reparsed
- THEN source and projection identities and normalized semantic meaning MUST match the checked fixtures
- AND the host MAY receive only the validated generated projection.

#### Scenario: Network posture is omitted

- GIVEN a Harbor-compatible task omits network posture or relies on an ambient public-network default
- WHEN Animus imports or projects the task without an explicit compatibility policy that supplies a reviewed posture
- THEN validation MUST fail closed before environment preparation
- AND omission MUST NOT become public access silently.

#### Scenario: Secret material appears in authored configuration

- GIVEN a configuration contains a credential value, inherited secret, unsafe path, mutable unpinned remote input, unknown field, or unsupported schema cohort
- WHEN Nickel or Rust validation runs
- THEN the configuration MUST be rejected with bounded diagnostics
- AND secret material MUST NOT enter generated config, identity records, receipts, or logs.

### Requirement: Trial and job planning has a deterministic functional core

r[agent_harness_runtime.harbor_eval.planning] Task and dataset validation, trial-matrix expansion, job ordering, stable identity, resource and concurrency admission, resume evaluation, completion-order normalization, and job reduction MUST be pure deterministic functions over explicit loaded inputs and MUST compose existing batch, evaluation-feedback, ResearchSpec, profile, role, sandbox, trust-boundary, orchestration, dataset-export, and receipt identities without replacing their owners.

#### Scenario: Identical evaluation plan repeats

- GIVEN identical validated tasks, dataset membership, agent/model profiles, verifier identities, environment policies, resource limits, ordering policy, and resume evidence
- WHEN planning runs repeatedly
- THEN it MUST produce byte-equivalent ordered trial plans and domain-separated BLAKE3 identities
- AND it MUST perform no filesystem, process, environment, clock, network, provider, container, scheduler, credential, or persistence effect.

#### Scenario: Planning inputs exceed policy

- GIVEN expansion would exceed an admitted trial, concurrency, resource, artifact, or output bound or contains duplicate or conflicting identities
- WHEN planning runs
- THEN it MUST return a typed bounded planning failure before host dispatch
- AND it MUST NOT truncate, reorder, schedule, or substitute inputs silently.

#### Scenario: Resume evidence is stale

- GIVEN prior trial evidence differs in plan, task, agent, model, environment, verifier, reward, redaction, output, or policy identity
- WHEN resume evaluation runs
- THEN the trial MUST remain pending or blocked according to explicit policy
- AND stale evidence MUST NOT be treated as completed work.

### Requirement: Evaluation effects cross implementation-free ports

r[agent_harness_runtime.harbor_eval.execution_ports] Environment preparation and execution, external or installed agent execution, verifier execution, artifact collection and transfer, and trial outcome observation MUST cross implementation-free typed adapter ports, while concrete filesystem, process, container, cloud, credential, network, clock, queue, scheduler, cancellation, deadline, and persistence effects MUST remain in explicit host-owned shells.

#### Scenario: Every required service is admitted

- GIVEN a validated trial plan names exact available environment, agent, verifier, artifact, cancellation, and persistence capabilities
- WHEN the runtime shell dispatches the trial
- THEN it MUST call only the injected services named by the admitted plan
- AND returned host data MUST be reduced to typed bounded observations before entering reusable logic.

#### Scenario: A service is absent or denied

- GIVEN a required backend is missing, disabled, denied, unavailable, cancelled, timed out, malformed, stale, or mismatched
- WHEN trial dispatch is evaluated
- THEN execution MUST stop with the corresponding typed outcome
- AND Animus MUST NOT discover or construct an ambient local, container, cloud, credential, network, or persistence fallback.

### Requirement: Verifier isolation and artifact transfer are explicit

r[agent_harness_runtime.harbor_eval.verifier_isolation] Shared and separate verifier modes MUST bind immutable task, subject, verifier, rubric, environment, and artifact-manifest identities; artifact transfer MUST be allowlisted, bounded, collision-checked, and attributed; and authoritative-promotion profiles MUST require separate verifier execution or independently controlled equivalent evidence plus Cairn-owned completion-policy admission.

#### Scenario: Separate verifier checks admitted artifacts

- GIVEN an agent environment has stopped or reached the declared handoff boundary and an allowlisted collision-free artifact manifest matches the trial plan
- WHEN a separately identified verifier environment receives the artifacts
- THEN only declared artifacts MUST be transferred with exact identity evidence
- AND verifier output MUST remain bound to the immutable subject, verifier, rubric, and environment identities.

#### Scenario: Shared verifier score requests promotion

- GIVEN the verifier ran in an environment the agent could influence
- WHEN its reward requests authoritative completion or promotion
- THEN the result MUST remain advisory unless independently controlled evidence and Cairn policy admit the claim
- AND a numeric score alone MUST NOT establish verifier independence, soundness, or completion.

#### Scenario: Artifact sources collide or widen

- GIVEN artifact entries overlap, name undeclared services, use unsafe paths, exceed bounds, or include undeclared trajectory, log, environment, credential, or verifier material
- WHEN transfer planning runs
- THEN the transfer MUST fail before verifier execution
- AND first-wins collision handling or silent evidence dropping MUST NOT be treated as successful verification.

### Requirement: ATIF interchange is versioned, correlated, and redacted

r[agent_harness_runtime.harbor_eval.atif] The ATIF adapter MUST support only exact reviewed schema versions, validate ordered steps, source roles, tool-call and observation correlation, subagent lineage, metrics, bounded extensions, and terminal state, and project into Animus-owned trajectory and dataset records under explicit redaction and inclusion policy.

#### Scenario: Supported trajectory is translated

- GIVEN a trajectory matches the selected ATIF schema, every observation references an admitted tool call, subagent lineage is valid, metrics are bounded, and terminal state is present
- WHEN pure translation runs
- THEN it MUST produce deterministic Animus trajectory records and a BLAKE3 projection identity
- AND only policy-admitted content MUST be eligible for receipt or dataset projection.

#### Scenario: Trajectory contains private reasoning or arbitrary extensions

- GIVEN ATIF steps contain reasoning content, provider payloads, credentials, environment values, private paths, raw tool arguments or results, or arbitrary extension data
- WHEN a safe receipt or dataset projection is requested
- THEN forbidden material MUST be omitted or replaced only by approved hashes and redaction metadata
- AND complete-history interoperability MUST NOT override Animus privacy and training-export policy.

#### Scenario: Version or correlation is invalid

- GIVEN the trajectory uses an unsupported schema version, duplicate or non-sequential step identity, orphaned observation, unknown tool call, invalid role field, broken subagent lineage, non-finite metric, unbounded extension, or missing terminal outcome
- WHEN validation runs
- THEN translation MUST fail with typed bounded diagnostics
- AND partial output MUST NOT be promoted as a valid trajectory.

### Requirement: Rewards are deterministic evidence, not ambient authority

r[agent_harness_runtime.harbor_eval.rewards] Evaluation rewards MUST use finite typed metric maps with validated names and numeric domains, explicit missing-value and aggregation policy, immutable subject/verifier/rubric identities, deterministic plan-order reduction, and attributed evidence hashes; LLM-as-judge and agent-as-judge results MUST remain advisory unless a separate reviewed Cairn policy admits a narrower claim.

#### Scenario: Multi-dimensional rewards aggregate

- GIVEN every completed trial provides finite validated reward dimensions bound to the expected subject and verifier identities
- WHEN dataset or job reduction runs
- THEN it MUST produce deterministic per-dimension and configured aggregate results in plan order
- AND missing-value policy and contributing trial counts MUST remain visible.

#### Scenario: Reward is invalid or missing

- GIVEN a reward is missing, non-finite, duplicated, outside its domain, bound to the wrong subject or verifier, or uses an unknown dimension
- WHEN reduction runs
- THEN the configured typed failure or missing-value policy MUST apply explicitly
- AND the reducer MUST NOT invent, clamp, discard, or average the value silently.

#### Scenario: Judge awards a passing score

- GIVEN an LLM or agent judge returns a passing score and rationale
- WHEN evaluation evidence is classified
- THEN the result MUST retain judge, model, rubric, subject, trajectory, and policy attribution and remain advisory by default
- AND it MUST NOT authorize effects or prove correctness, verifier soundness, benchmark superiority, or completion by itself.

### Requirement: Evaluation receipts preserve stack claim boundaries

r[agent_harness_runtime.harbor_eval.receipts] Evaluation receipts MUST contain only safe task, dataset, trial, job, agent, model-route, environment, verifier, rubric, artifact-manifest, reward, trajectory-projection, policy, backend, status, freshness, bounded-count, redaction, non-claim, and domain-separated BLAKE3 identity metadata, with explicit optional handoff identities for Valence linkage and Cairn lifecycle gates.

#### Scenario: Trial completes with bounded evidence

- GIVEN an admitted trial reaches a typed terminal outcome and its projected artifacts, rewards, and safe trajectory metadata validate
- WHEN receipt projection runs
- THEN the receipt MUST bind every relevant identity, status, count, freshness, and redaction decision
- AND it MAY be handed to Valence or Cairn without transferring behavioral-correctness authority.

#### Scenario: Receipt draft contains sensitive or overstated material

- GIVEN a receipt draft contains raw prompts, hidden tests or reasoning, trajectories, tool or provider payloads, credentials, environment values, physical paths, unbounded logs, verifier-soundness claims, benchmark-superiority claims, or automatic completion claims
- WHEN receipt validation runs
- THEN projection MUST fail with bounded leak or overclaim diagnostics
- AND no receipt identity or success claim may be emitted.

### Requirement: Harbor evaluation adaptation has positive and negative conformance evidence

r[agent_harness_runtime.harbor_eval.verification] The Harbor evaluation change MUST include positive and negative fixtures through production parser, planner, adapter, verifier, ATIF, reward, and receipt entrypoints; dependency-isolation guards; focused checks; and a fixture-backed local vertical slice before archive.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to sync and archive the Harbor evaluation change
- WHEN closeout validation runs
- THEN Nickel projection parity, planner determinism, adapter failure handling, verifier isolation, ATIF correlation and redaction, reward reduction, safe receipts, dependency isolation, Cairn gates, and relevant Rust/Nix checks MUST pass
- AND any live Harbor, container, cloud, Hub, benchmark, optimization, or training smoke MUST remain separately scoped.

#### Scenario: Negative fixture attempts authority or data widening

- GIVEN a fixture attempts ambient backend discovery, public-network widening, credential inheritance, verifier mutation, unsafe artifact transfer, unsupported ATIF data, judge authority, stale evidence reuse, or receipt leakage
- WHEN the production path evaluates it
- THEN the fixture MUST fail with the expected typed diagnostic before the prohibited effect or claim
- AND failure evidence MUST remain bounded and deterministic.
