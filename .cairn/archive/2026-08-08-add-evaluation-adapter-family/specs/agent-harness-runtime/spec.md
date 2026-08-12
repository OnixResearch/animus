## ADDED Requirements

### Requirement: Evaluation adapters form a focused authority-free family

r[agent_harness_runtime.native_evaluation_adapters.boundary] Animus MUST expose native evaluation effect boundaries through a focused non-default `#![no_std]` adapter-family crate, and the crate MUST NOT read files, inspect environment variables, access clocks, open sockets, call providers or tools, spawn processes, use credentials, schedule work, persist state, launch containers, discover services, or depend on Harbor, Verifiers, Python, HTTP, cloud, or registry runtimes.

#### Scenario: Embedder selects evaluation contracts

r[agent_harness_runtime.native_evaluation_adapters.boundary.selected]
- GIVEN an embedder selects the evaluation adapter feature and supplies explicit implementations
- WHEN adapter admission runs
- THEN only implementation-free typed contracts and pure validation enter the reusable dependency graph
- AND every concrete effect remains owned by the supplied host.

#### Scenario: Required service is absent

r[agent_harness_runtime.native_evaluation_adapters.boundary.absent]
- GIVEN an admitted evaluation plan requires a service that is absent, incompatible, denied, stale, or unsupported
- WHEN adapter admission runs
- THEN it returns a typed blocked outcome before dispatch
- AND it does not discover or construct an ambient fallback.

### Requirement: Evaluation effects use separate least-authority ports

r[agent_harness_runtime.native_evaluation_adapters.ports] Task materialization, agent execution, verifier execution, artifact collection, artifact restore, model-call capture, result append, result replacement, and result reopen MUST use separate typed ports with explicit capability, availability, implementation, cancellation, deadline, and resource descriptors.

#### Scenario: Host implements a subset of ports

r[agent_harness_runtime.native_evaluation_adapters.ports.subset]
- GIVEN a host implements task and agent execution but no verifier or persistence service
- WHEN a plan requires only the implemented services
- THEN admission can produce requests for those services
- AND no verifier or persistence authority is implied.

#### Scenario: Composite service requests ambient authority

r[agent_harness_runtime.native_evaluation_adapters.ports.widening]
- GIVEN a service implementation claims undeclared provider, filesystem, verifier, artifact, storage, or runtime capability
- WHEN the service catalog is validated
- THEN validation rejects the widened implementation
- AND no broad backend trait can bypass the separate port ceilings.

### Requirement: Adapter requests and observations are exactly bound

r[agent_harness_runtime.native_evaluation_adapters.binding] Every evaluation service request and observation MUST bind all applicable evaluation, plan, task, episode, run, role, model, harness, environment, tool, verifier, rubric, artifact, policy, cancellation, deadline, attempt, implementation, and correlation identities, and reusable logic MUST reject stale, mismatched, cross-plan, cross-task, cross-attempt, wrong-service, or incomplete evidence.

#### Scenario: Service returns matching observation

r[agent_harness_runtime.native_evaluation_adapters.binding.match]
- GIVEN a service receives an admitted request and returns bounded evidence with every required matching identity
- WHEN observation validation runs
- THEN the result is eligible for native evaluation reduction
- AND the observation retains its exact implementation and correlation attribution.

#### Scenario: Observation is replayed across plans

r[agent_harness_runtime.native_evaluation_adapters.binding.replay]
- GIVEN an observation has a matching run label but a different plan, task, attempt, policy, or implementation identity
- WHEN reusable logic validates it
- THEN validation returns a typed mismatch or stale outcome
- AND the observation cannot mutate episode, reward, resume, or receipt state.

### Requirement: Model-call projection preserves protocol and commit ordering

r[agent_harness_runtime.native_evaluation_adapters.model_gateway] Evaluation model-call capture MUST use pure bounded protocol projections that identify the selected dialect, effective model and sampling, endpoint class, usage, finish, error, tool-catalog, request, attempt, and stream-order facts, and terminal stream evidence MUST NOT be accepted before matching turn-commit evidence.

#### Scenario: Terminal event follows commit

r[agent_harness_runtime.native_evaluation_adapters.model_gateway.commit_order]
- GIVEN a bounded model stream reaches its protocol terminal event and matching active-turn commit evidence is complete
- WHEN gateway observation validation runs
- THEN the terminal event can be admitted for host delivery
- AND the corresponding model-call metadata remains bound to the committed turn.

#### Scenario: Terminal event races the commit

r[agent_harness_runtime.native_evaluation_adapters.model_gateway.early_terminal]
- GIVEN a terminal event is observed without matching commit evidence or with stale run, request, attempt, or turn identity
- WHEN gateway validation runs
- THEN delivery remains blocked
- AND scoring, episode reduction, and completion cannot observe the turn as committed.

#### Scenario: Identical request retry is coalesced

r[agent_harness_runtime.native_evaluation_adapters.model_gateway.coalescing]
- GIVEN two non-streaming attempts have exact matching canonical request, run, model, sampling, policy, active-turn, and attempt-lineage identities
- WHEN coalescing admission runs
- THEN the later attempt can await or reuse the first bounded result according to policy
- AND equal body content alone cannot authorize coalescing.

#### Scenario: Gateway requests unsafe behavior

r[agent_harness_runtime.native_evaluation_adapters.model_gateway.unsafe]
- GIVEN a gateway draft retains raw provider bodies, accepts unbounded streams, rewrites responses, shares model and state capabilities, uses last-write-wins task state, or lacks commit evidence
- WHEN adapter validation runs
- THEN the draft is rejected before host transport
- AND no safe receipt or successful capture claim is emitted.

### Requirement: Native adapters preserve existing owners

r[agent_harness_runtime.native_evaluation_adapters.compatibility] Native evaluation adapters MUST provide explicit compatibility conversion for existing Harbor environment and verifier ports while ATIF retains external trajectory semantics, the ledger retains transcript authority, batch retains ordering, roles retain separation, artifacts retain manifest policy, and safe receipts retain evidence redaction.

#### Scenario: Harbor backend is selected

r[agent_harness_runtime.native_evaluation_adapters.compatibility.harbor]
- GIVEN a native evaluation plan selects the reviewed Harbor compatibility profile
- WHEN port conversion runs
- THEN native requests are translated only into the exact supported Harbor service cohort
- AND Harbor-specific release, schema, backend, and artifact semantics remain in Harbor-owned modules.

#### Scenario: Compatibility conversion loses identity

r[agent_harness_runtime.native_evaluation_adapters.compatibility.loss]
- GIVEN conversion cannot preserve a required task, plan, verifier, rubric, artifact, trajectory, policy, or correlation identity
- WHEN compatibility admission runs
- THEN conversion is blocked with a typed loss reason
- AND partial output is not accepted as equivalent native evidence.

### Requirement: Adapter evidence is bounded and secret-free

r[agent_harness_runtime.native_evaluation_adapters.evidence] Evaluation adapter receipts MUST bind only safe service, capability, availability, request, observation, gateway, verifier, artifact, storage, policy, status, count, redaction, and BLAKE3 identity metadata and MUST reject prompts, hidden tests, reasoning, provider or tool bodies, credentials, environment values, physical paths, unbounded logs, and unsupported host-behavior claims.

#### Scenario: Adapter call has safe evidence

r[agent_harness_runtime.native_evaluation_adapters.evidence.safe]
- GIVEN a service call has validated request and observation identities plus bounded status metadata
- WHEN receipt projection runs
- THEN it emits a metadata-only receipt draft
- AND the receipt states that host behavior and external effects remain outside the adapter claim.

#### Scenario: Adapter receipt overclaims

r[agent_harness_runtime.native_evaluation_adapters.evidence.overclaim]
- GIVEN a receipt draft claims exactly-once remote effects, verifier soundness, artifact authenticity, complete capture, benchmark validity, completion, production readiness, or release eligibility
- WHEN receipt validation runs
- THEN validation rejects the unsupported claim
- AND no success identity is emitted.

### Requirement: Evaluation adapter family has positive and negative verification

r[agent_harness_runtime.native_evaluation_adapters.verification] Evaluation adapter adoption MUST include deterministic positive and negative fake-backed tests for every port, capability and availability admission, exact request and observation binding, failure reduction, gateway commit ordering, retry coalescing, Harbor compatibility, safe receipts, no-std, Wasm, dependency isolation, and claim boundaries.

#### Scenario: Adapter family is ready to archive

r[agent_harness_runtime.native_evaluation_adapters.verification.closeout]
- GIVEN maintainers intend to sync and archive the evaluation adapter change
- WHEN closeout checks run
- THEN focused Rust, no-std, Wasm, compatibility, dependency, receipt, Cairn, and Nix checks pass or record an exact blocker
- AND no live provider, tool, process, socket, container, cloud, persistence, benchmark, or production behavior is claimed.
