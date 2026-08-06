## ADDED Requirements

### Requirement: Native evaluation core is focused and authority-free

r[agent_harness_runtime.native_evaluation_core.boundary] Animus MUST provide independently useful task, taskset, environment-plan, episode, recipe, reward, resume, and evaluation-receipt behavior through a focused non-default `#![no_std]` core, and that core MUST NOT read files, inspect environment variables, access clocks, call providers or tools, spawn processes, use networks or credentials, schedule work, persist state, evaluate Nickel, or depend on Harbor, Verifiers, Python, container, cloud, or registry runtimes.

#### Scenario: Native evaluation feature is selected

r[agent_harness_runtime.native_evaluation_core.boundary.selected]
- GIVEN an embedder supplies validated in-memory evaluation descriptors and selects the native evaluation feature
- WHEN evaluation planning and reduction run
- THEN Animus produces deterministic typed plans, decisions, and safe receipt drafts
- AND all host effects remain absent or represented only by explicit requests.

#### Scenario: Default runtime is built

r[agent_harness_runtime.native_evaluation_core.boundary.default]
- GIVEN Animus is built without native evaluation, Harbor, or host-evaluation features
- WHEN dependency and facade checks run
- THEN the default runtime loop retains its current authority and dependency posture
- AND no evaluation host, Python package, container backend, provider, or persistence service is discovered or linked.

### Requirement: Evaluation policy is Nickel-authored and revalidated

r[agent_harness_runtime.native_evaluation_core.policy] Human-authored taskset, recipe, reward, resume, redaction, resource, and evidence policy MUST use typed Nickel contracts with named positive bounds and deny-by-default postures, and generated runtime data MUST carry exact schema and source BLAKE3 identities that Rust revalidates before use.

#### Scenario: Checked evaluation profile is loaded

r[agent_harness_runtime.native_evaluation_core.policy.valid]
- GIVEN a checked generated profile matches its Nickel source, schema, bounds, and canonical identity
- WHEN Rust revalidates the in-memory profile
- THEN the normalized policy is accepted with the same deterministic BLAKE3 identity
- AND no runtime Nickel evaluation occurs.

#### Scenario: Profile contains unsafe or stale data

r[agent_harness_runtime.native_evaluation_core.policy.invalid]
- GIVEN a profile contains unknown fields, non-positive bounds, public-network defaults, secret material, unsupported recipes, stale source identity, or malformed hashes
- WHEN Nickel or Rust validation runs
- THEN the profile is rejected with bounded diagnostics
- AND no plan, dispatch request, or success receipt is produced.

### Requirement: Tasks and tasksets have explicit deterministic identity

r[agent_harness_runtime.native_evaluation_core.tasks] Evaluation tasks MUST separate immutable bounded task data from behavior, scorer, and policy identities, while tasksets MUST declare exact finite membership or a generator identity, version, seed, ordering contract, finiteness class, and positive selection bound sufficient for deterministic planning and resume.

#### Scenario: Lazy taskset is bounded

r[agent_harness_runtime.native_evaluation_core.tasks.lazy]
- GIVEN a lazy taskset declares its generator, version, seed, deterministic order, finiteness class, and requested selection bound
- WHEN taskset planning runs
- THEN it produces an ordered bounded task plan and canonical BLAKE3 identity
- AND the planner does not enumerate beyond the selected bound.

#### Scenario: Infinite or unstable taskset is unsafe

r[agent_harness_runtime.native_evaluation_core.tasks.rejected]
- GIVEN an infinite taskset lacks a selection bound or requests global shuffle, or a generator lacks stable identity, version, seed, or ordering semantics
- WHEN taskset validation runs
- THEN validation fails before episode planning
- AND no implicit seed, truncation, shuffle, or regeneration claim is invented.

### Requirement: Episodes reference the canonical ledger

r[agent_harness_runtime.native_evaluation_core.episodes] An evaluation episode MUST represent global multi-agent structure through ordered seat and run identities plus exact references to admitted Animus ledger sessions, branches, leaves, subjects, and terminal observations, and it MUST NOT create a second writable transcript or use copied trace bodies as replay authority.

#### Scenario: Multi-agent episode completes

r[agent_harness_runtime.native_evaluation_core.episodes.complete]
- GIVEN every planned seat returns a bounded terminal observation and an exact admitted ledger reference
- WHEN episode reduction runs
- THEN the episode binds each run in deterministic plan order
- AND transcript history remains owned by the referenced ledger.

#### Scenario: Episode carries an untrusted trace body

r[agent_harness_runtime.native_evaluation_core.episodes.duplicate_trace]
- GIVEN an episode draft tries to replace its ledger reference with copied messages, reasoning, tool bodies, or provider payloads
- WHEN episode validation runs
- THEN the draft is rejected or reduced only to admitted safe metadata
- AND copied content cannot become canonical replay or completion evidence.

### Requirement: Built-in evaluation recipes are deterministic plans

r[agent_harness_runtime.native_evaluation_core.recipes] Animus MUST provide pure deterministic planners for single-agent, bounded best-of-N/pass-at-N, user-simulation, and isolated-agentic-judge recipes, with exact role, model, harness, tool, environment, verifier, rubric, resource, cancellation, redaction, and policy bindings and no effect dispatch.

#### Scenario: Isolated judge recipe is planned

r[agent_harness_runtime.native_evaluation_core.recipes.isolated_judge]
- GIVEN a solver, distinct verifier, immutable rubric, bounded artifact manifest, and separate environment policy are admitted
- WHEN isolated-judge planning runs
- THEN the plan orders solver execution, bounded handoff, and separate verifier execution
- AND the plan does not claim that the verifier is sound or that a passing score completes the task.

#### Scenario: Shared judge requests authoritative status

r[agent_harness_runtime.native_evaluation_core.recipes.shared_judge]
- GIVEN a judge shares an agent-controlled runtime or mutable workspace
- WHEN recipe planning or reduction evaluates its result
- THEN the result is marked advisory and shared
- AND it cannot satisfy verifier isolation or authoritative completion requirements.

#### Scenario: User simulation widens authority

r[agent_harness_runtime.native_evaluation_core.recipes.user_sim]
- GIVEN a user-simulation seat requests trainable status, assistant-only tools, mutation capability, or hidden evaluator authority
- WHEN recipe validation runs
- THEN the unsafe capability or recipe is rejected
- AND the evaluated assistant receives only the admitted scenario interaction.

### Requirement: Rewards are finite attributed evidence

r[agent_harness_runtime.native_evaluation_core.rewards] Evaluation rewards MUST use finite named dimensions with explicit domains, weights, missing-value policy, deterministic plan-order reduction, contribution counts, and immutable scorer, verifier, rubric, subject, trajectory, and evidence identities, and judge-produced values MUST remain advisory unless separate accepted policy admits a narrower claim.

#### Scenario: Reward dimensions reduce successfully

r[agent_harness_runtime.native_evaluation_core.rewards.valid]
- GIVEN every reward dimension is finite, in-domain, uniquely named, and bound to expected identities
- WHEN reward reduction runs
- THEN per-dimension and configured aggregate results are produced in plan order
- AND missing policy and contributor counts remain visible.

#### Scenario: Reward value is malformed or overclaims

r[agent_harness_runtime.native_evaluation_core.rewards.invalid]
- GIVEN a reward is NaN, infinite, duplicated, unknown, out of domain, mismatched, silently clamped, or presented as authorization or correctness proof
- WHEN reward validation runs
- THEN reduction fails or returns the explicit configured missing outcome
- AND no authority, verifier-soundness, benchmark, or completion claim is emitted.

### Requirement: Resume is content-addressed across all material policy

r[agent_harness_runtime.native_evaluation_core.resume] Evaluation resume decisions MUST bind canonical task data plus taskset, recipe, model, harness, environment, tool, verifier, rubric, reward, sampling, redaction, output, and policy identities with domain-separated BLAKE3, and only complete matching terminal evidence MAY skip work.

#### Scenario: Completed evidence matches exactly

r[agent_harness_runtime.native_evaluation_core.resume.match]
- GIVEN a prior record is complete, terminal, safe, and matches every current resume identity
- WHEN pure resume evaluation runs
- THEN the corresponding planned work is marked keep
- AND the decision identifies the exact evidence without reading or mutating persistence.

#### Scenario: Resume evidence is unsafe or stale

r[agent_harness_runtime.native_evaluation_core.resume.stale]
- GIVEN prior evidence is malformed, torn, failed, missing, duplicated, conflicting, over-budget, or differs in any material identity
- WHEN resume evaluation runs
- THEN the work remains rerun, blocked, or redo-whole-task according to explicit policy
- AND stale evidence cannot suppress execution silently.

### Requirement: Evaluation evidence is metadata-only

r[agent_harness_runtime.native_evaluation_core.evidence] Native evaluation receipts MUST bind safe taskset, task, recipe, plan, episode, ledger-reference, reward, resume, policy, status, count, redaction, and non-claim metadata with domain-separated BLAKE3 and MUST reject raw prompts, hidden tests, reasoning, tool bodies, provider payloads, credentials, environment values, physical paths, unbounded logs, transcript bodies, and unsupported claims.

#### Scenario: Safe evaluation receipt is projected

r[agent_harness_runtime.native_evaluation_core.evidence.safe]
- GIVEN an evaluation result has validated identities, bounded counts, redaction state, and terminal classification
- WHEN receipt projection runs
- THEN it emits only the admitted metadata and required non-claims
- AND the receipt does not transfer behavioral-correctness or completion authority.

#### Scenario: Receipt draft leaks content

r[agent_harness_runtime.native_evaluation_core.evidence.leak]
- GIVEN a receipt draft contains forbidden content, private locators, unapproved content hashes, or verifier-soundness, benchmark, training, production, completion, or release claims
- WHEN receipt validation runs
- THEN validation fails with bounded leak or overclaim diagnostics
- AND no success identity is emitted.

### Requirement: Native evaluation core has positive and negative verification

r[agent_harness_runtime.native_evaluation_core.verification] Native evaluation adoption MUST include positive and negative production-path fixtures for policy, exact and lazy tasksets, every built-in recipe, episode-to-ledger references, rewards, resume, migration, safe receipts, no-std, Wasm, dependency isolation, and claim boundaries before archive.

#### Scenario: Native evaluation core is ready to archive

r[agent_harness_runtime.native_evaluation_core.verification.closeout]
- GIVEN maintainers intend to sync and archive the native evaluation core change
- WHEN closeout checks run
- THEN focused Rust, Nickel, no-std, Wasm, migration, dependency, receipt, Cairn, and Nix checks pass or record an exact blocker
- AND live providers, tools, containers, benchmarks, optimization, training, and production claims remain out of scope.
