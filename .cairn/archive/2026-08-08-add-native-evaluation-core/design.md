## Context

The accepted Harbor evaluation contract already requires pure trial planning, explicit execution ports, finite rewards, verifier isolation, ATIF projection, and safe receipts. Its current Rust implementation places generic planning and reward behavior under `animus-runtime::harbor_eval`. Batch, ledger, role, dataset, evaluation-feedback, and completion modules retain adjacent authority.

Verifiers v1 separates immutable `TaskData` from task behavior, streams tasks through a `Taskset`, lets an `Env` define multi-agent control flow, records each agent run as a `Trace`, and groups traces in an `Episode`. Its built-in environments include single-agent, best-of-N, user simulation, and shared or isolated agentic judging. Its resume path matches saved work by task content rather than mutable row index.

The reviewed cohort is PrimeIntellect-ai/verifiers commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT. Completion requires a native contract with one-way migration from Harbor-owned generic names, no second ledger, explicit identity and bound semantics, and deterministic positive and negative evidence. A facade rename, copied Python object model, or fixture-only API is false completion.

## Decisions

### Decision: create a focused no-std evaluation core

**Choice:** Add `animus-eval-core` as `#![no_std]` plus `alloc`. It owns evaluation values and pure reducers only. `animus-runtime` composes these values with existing runtime and ledger surfaces and can provide compatibility re-exports.

**Rationale:** Evaluation planning is independently useful and large enough to justify a crate barrier. A focused crate prevents Harbor compatibility and host authority from accumulating in the default runtime kernel.

### Decision: represent behavior by immutable identities and pure plans

**Choice:** Task behavior, scorers, verifiers, harnesses, and environment recipes are typed descriptors with exact policy and implementation identities. The core emits requests and plans. It does not retain closures, trait objects with effects, scripts, or dynamic package names.

**Rationale:** Python subclass and decorator behavior is not serializable, no-std, or authority-transparent. Typed descriptors keep planning deterministic and effects injectable.

### Decision: keep the ledger as trace authority

**Choice:** An episode contains ordered agent-run references bound to admitted ledger session, branch, leaf, subject, role, and run identities. It can carry safe summaries and terminal observations. It cannot contain or mutate a parallel transcript body.

**Rationale:** Animus already has canonical replay, branch, redaction, and recovery owners. A second writable trace model would create conflict and repair ambiguity.

### Decision: make taskset generation explicit

**Choice:** A finite taskset declares exact membership. A lazy taskset declares generator identity, generator version, seed, deterministic ordering contract, finiteness class, and a positive run bound. Infinite streams cannot shuffle globally and cannot execute without an explicit selection bound.

**Rationale:** Verifiers permits procedural streams and relies on deterministic regeneration for resume. Animus makes this dependency explicit and identity-bound.

### Decision: ship recipes as pure planners

**Choice:** Built-in recipes produce deterministic episode plans:

- single-agent creates one admitted seat and run;
- best-of-N creates bounded sibling runs and deterministic pass-at-N reduction;
- user simulation creates separate untrainable user and evaluated assistant seats with explicit termination policy;
- isolated agentic judge creates a solver run, allowlisted artifact handoff, and distinct verifier run.

Shared-runtime judging is represented only as an explicit advisory compatibility mode. It cannot satisfy authoritative verifier isolation.

**Rationale:** Recipes provide useful defaults without embedding provider, scheduler, or container behavior in the core.

### Decision: use content-addressed resume with full policy binding

**Choice:** A resume key uses domain-separated BLAKE3 over canonical task data plus taskset, recipe, model, harness, environment, tool, verifier, rubric, reward, sampling, redaction, output, and policy identities. Only complete matching terminal evidence can skip work.

Malformed, torn, failed, missing, stale, duplicated, conflicting, or over-budget evidence remains owed. The core returns keep, rerun, block, or redo-whole-task decisions without reading or rewriting files.

**Rationale:** Task content alone is too weak. Full binding prevents stale evaluation evidence from surviving material policy drift.

### Decision: keep rewards attributed and advisory

**Choice:** Rewards are finite named dimensions with explicit domains, weights, missing-value policy, scorer, verifier, rubric, subject, trajectory, and evidence identities. Reduction occurs in plan order. Judge-produced values remain advisory unless a separate Cairn completion policy admits a narrower claim.

**Rationale:** Numeric scores are evidence, not authorization or proof.

### Decision: author profiles in Nickel and revalidate in Rust

**Choice:** Nickel owns evaluation profile contracts and defaults. Generated runtime data carries schema and BLAKE3 source identities. Rust rejects unknown fields, invalid bounds, identity drift, unsafe defaults, and unsupported recipe combinations.

**Rationale:** This preserves the repository configuration boundary and prevents runtime Nickel authority.

## Migration

1. Add the new crate and compatibility types without changing current Harbor behavior.
2. Move generic task, episode, recipe, reward, and resume semantics behind native names.
3. Make `animus-runtime::harbor_eval` consume native plans and projections.
4. Keep Harbor release, schema, and backend translation only in Harbor-owned modules.
5. Remove duplicate generic reducers after parity and dependency checks pass.

## Adversarial Cases

- Equal task text has different hidden task data or policy identity.
- A lazy generator changes output while keeping the same seed.
- A resume record matches task content but not the rubric or verifier.
- Best-of-N receives a non-finite or missing sibling reward.
- A simulated user is marked trainable or receives assistant-only authority.
- A shared judge requests completion promotion.
- An episode embeds transcript text instead of ledger references.
- A receipt includes prompts, hidden tests, reasoning, tool bodies, or physical paths.

Each case must return one deterministic rejection, rerun, block, or advisory result.

## Alternatives Considered

### Keep generic evaluation inside `harbor_eval`

Rejected because Harbor becomes the apparent owner of independently useful Animus behavior and keeps optional interoperability coupled to native evaluation semantics.

### Add evaluation only under repository tests

Rejected because embedders need a public SDK contract. Test-only models would diverge from production planning and resume behavior.

### Copy the Verifiers Python hierarchy

Rejected because dynamic inheritance, decorators, package discovery, and effectful methods violate no-std, explicit authority, and typed configuration boundaries.

### Store complete traces in episodes

Rejected because the session ledger already owns transcript identity, replay, recovery, and redaction.

## Risks / Trade-offs

- A new crate increases public API and migration work.
- Strict resume identities rerun more work after policy changes. This is safer than accepting stale evidence.
- Descriptor-based behavior is less dynamic than Python subclasses. It remains auditable and portable.
- The initial core has no live host and proves only deterministic contract behavior.

## Non-Claims

This design does not prove task correctness, trace completeness, scorer validity, verifier soundness, model quality, benchmark comparability, training fitness, host execution, sandboxing, production readiness, authoritative completion, or release eligibility.
