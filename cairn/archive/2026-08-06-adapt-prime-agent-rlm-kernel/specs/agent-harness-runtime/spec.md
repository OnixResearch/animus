# Agent Harness Runtime

## ADDED Requirements

### Requirement: RLM kernel remains an exact optional boundary

r[agent_harness_runtime.rlm_kernel.boundary] Animus MUST provide the model-facing persistent kernel as a focused non-default profile over `animus-steel` pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire filesystem, process, network, environment, credential, clock, scheduler, or persistence authority from the kernel.

#### Scenario: Model-facing Steel profile is selected

r[agent_harness_runtime.rlm_kernel.boundary.selected]
- GIVEN an embedder selects the reviewed RLM kernel profile and supplies a host executor
- WHEN the kernel is composed
- THEN Animus MAY expose a persistent Steel environment, typed host requests, and bounded child spawn to the model
- AND the existing Steel fabric, subagent, compaction, tool-executor, and receipt owners MUST remain authoritative.

#### Scenario: Ambient authority is requested

r[agent_harness_runtime.rlm_kernel.boundary.no_ambient]
- GIVEN model code or a caller expects the kernel to read files, spawn processes, open sockets, or persist state directly
- WHEN admission runs
- THEN Animus MUST return a typed unsupported-authority outcome before the effect
- AND it MUST NOT substitute an ambient implementation or widen the admitted operation set.

### Requirement: RLM kernel policy is Nickel-authored and revalidated

r[agent_harness_runtime.rlm_kernel.profile] Human-authored kernel policy MUST use typed Nickel contracts for exact cohort identity, fuel, depth, allocation, environment-size, child-spawn budgets, admitted host-operation classes, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.rlm_kernel.profile.valid]
- GIVEN a checked profile names the exact cohort, finite bounds, explicit operation classes, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no evaluation, host request, or receipt emission may occur before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.rlm_kernel.profile.invalid]
- GIVEN generated policy is stale, omits a bound, admits an ambient operation class, or weakens claim boundaries
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no kernel evaluation or host request may proceed.

### Requirement: Kernel environment persists prompt context as variables

r[agent_harness_runtime.rlm_kernel.environment] The kernel MUST bind admitted prompt context to named variables, preserve variables, imports, and helper definitions across turns, and enforce named bounds on environment size, evaluation fuel, recursion depth, and allocation with pure deterministic evaluation results.

#### Scenario: Context persists across turns

r[agent_harness_runtime.rlm_kernel.environment.persists]
- GIVEN an admitted environment holds variables and helper definitions from earlier turns
- WHEN a new model evaluation runs against the same kernel
- THEN the evaluation MUST observe the persisted bindings within admitted bounds
- AND repeated evaluation of identical code, environment, and policy MUST return the same result or the same bounded diagnostics.

#### Scenario: Evaluation exceeds a named bound

r[agent_harness_runtime.rlm_kernel.environment.bounded]
- GIVEN model code exceeds fuel, depth, allocation, or environment-size bounds
- WHEN evaluation runs
- THEN it MUST stop with a bounded typed outcome naming the violated bound
- AND it MUST NOT partially mutate environment state, emit host requests, or consume unbounded resources.

### Requirement: Typed host requests are the only effect channel

r[agent_harness_runtime.rlm_kernel.host_requests] Kernel code MUST perform effects only by returning typed host requests, the runtime MUST revalidate every request against the admitted policy before host execution, and reusable crates MUST NOT execute filesystem, process, network, credential, or persistence effects on behalf of model code.

#### Scenario: Admitted host request executes through the adapter boundary

r[agent_harness_runtime.rlm_kernel.host_requests.admitted]
- GIVEN model code returns a host request whose operation class is admitted by policy
- WHEN runtime admission runs
- THEN it MUST revalidate the request fields and forward it to the host-injected executor
- AND the bounded result MUST return to the kernel as a typed value.

#### Scenario: Unadmitted or malformed host request is returned

r[agent_harness_runtime.rlm_kernel.host_requests.rejected]
- GIVEN model code returns an unadmitted operation class, a malformed request, or a request exceeding named bounds
- WHEN admission runs
- THEN it MUST fail closed with a typed denial receipt
- AND no effect, partial effect, or ambient fallback may occur.

### Requirement: Child spawn is bounded and typed

r[agent_harness_runtime.rlm_kernel.children] The `rlm(...)` operation MUST name a child profile, an explicit context slice, and result expectations, and the runtime MUST enforce depth, fan-out, token, wall-clock, and result-size budgets before spawn and return typed child results with lineage into the existing subagent contracts.

#### Scenario: Bounded child spawn succeeds

r[agent_harness_runtime.rlm_kernel.children.spawn]
- GIVEN a child request fits admitted depth, fan-out, and budget limits and names an admitted child profile
- WHEN the runtime processes the request
- THEN it MUST delegate lifecycle to the existing subagent owners
- AND the child result MUST return to the parent kernel as a typed value with lineage metadata.

#### Scenario: Child spawn exceeds budgets or requests ambient authority

r[agent_harness_runtime.rlm_kernel.children.rejected]
- GIVEN a child request exceeds depth, fan-out, budget, or result-size limits, or expects ambient discovery or cross-session spawn
- WHEN admission runs
- THEN it MUST return a typed denial before spawn
- AND no child session, message, or ambient registration may be created.

### Requirement: Kernel state survives compaction

r[agent_harness_runtime.rlm_kernel.compaction_survival] Compaction MUST NOT clear kernel variables, imports, or definitions, compaction plans MUST record kernel-held context so summaries reference rather than repeat it, and kernel clear MUST remain an explicit host action with its own receipt.

#### Scenario: Compaction runs during a long task

r[agent_harness_runtime.rlm_kernel.compaction_survival.preserved]
- GIVEN an active kernel holds task state and compaction triggers on context pressure
- WHEN the compaction plan executes
- THEN prompt context MUST shrink while kernel bindings remain observable to later evaluations
- AND the compaction receipt MUST record that kernel state was preserved.

#### Scenario: Implicit kernel clear is attempted

r[agent_harness_runtime.rlm_kernel.compaction_survival.no_implicit_clear]
- GIVEN a compaction, goal change, or child completion attempts to clear the kernel implicitly
- WHEN the transition is validated
- THEN it MUST be rejected with a typed outcome
- AND only an explicit host-initiated clear with its own receipt MAY reset the environment.

### Requirement: RLM kernel evidence is metadata-only

r[agent_harness_runtime.rlm_kernel.evidence] Kernel receipts MUST bind only policy identity, operation classes, bounded counts, child lineage, compaction-survival status, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude variable values, model-written code bodies, child transcripts, credentials, and raw paths.

#### Scenario: Safe kernel receipt is projected

r[agent_harness_runtime.rlm_kernel.evidence.safe]
- GIVEN evaluation, host requests, and child spawns complete under the admitted profile
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, counts, statuses, and required non-claims
- AND it MUST describe only bounded processing of admitted operations.

#### Scenario: Receipt contains kernel contents or overclaims

r[agent_harness_runtime.rlm_kernel.evidence.rejects]
- GIVEN a receipt draft includes variable values, code bodies, transcripts, credentials, raw paths, or a sandbox, correctness, or task-success claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: RLM kernel adoption has positive and negative conformance evidence

r[agent_harness_runtime.rlm_kernel.verification] The RLM kernel MUST include positive and negative tests through production environment, admission, child-spawn, compaction, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.rlm_kernel.verification.positive]
- GIVEN fixtures cover variable persistence, admitted host requests, bounded child spawn, typed results, and kernel survival across compaction
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.rlm_kernel.verification.negative]
- GIVEN a fixture attempts direct effects, unadmitted operations, bound violations, ambient or over-limit child spawn, implicit kernel clear, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.rlm_kernel.verification.closeout]
- GIVEN maintainers intend to sync and archive the RLM kernel change
- WHEN closeout validation runs
- THEN focused kernel, Steel, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND security sandboxing, model correctness, task completion, production readiness, and release eligibility MUST remain separate non-claims.
