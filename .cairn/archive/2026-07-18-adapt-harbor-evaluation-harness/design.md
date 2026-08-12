## Context

Harbor's operational model separates tasks, datasets, agents, environments, verifiers, trials, jobs, rewards, and trajectories. Animus already owns adjacent mechanisms, but they are deliberately split by authority:

- `animus-runtime::batch` owns pure batch plan, resume, ordering, and receipt logic;
- `animus-runtime::eval_feedback` owns local reviewed evaluation-plan and freshness records;
- `animus-runtime::researchspec` owns optimization-study facts and promotion gates;
- `animus-runtime::dataset_export` owns safe supervised, preference, and RL row projection;
- sandbox, trust-boundary, model-profile, agent-role, orchestration, and safe-receipt modules own their existing decisions;
- `animus-adapters` owns implementation-free host ports; and
- `animus-std` and extracted authority-scoped std blocks own concrete effects.

The adaptation must compose these owners without introducing a second agent loop, session ledger, scheduler authority, verifier authority, or evidence model.

## Decisions

### 1. Treat Harbor as a bounded reference and optional backend

**Choice:** Animus owns its evaluation contracts. A Harbor integration targets one exact reviewed Harbor, task-schema, and ATIF cohort through a non-default host boundary. Reusable crates do not depend on Harbor, Python, Pydantic, Docker, a cloud SDK, or Harbor registry clients.

**Rationale:** Harbor is useful as an interoperability and operational reference, but its application stack and release cadence must not become Animus core architecture.

### 2. Compose existing runtime families instead of replacing them

**Choice:** Add a narrow evaluation-harness composition module or feature that references existing batch, evaluation-feedback, ResearchSpec, dataset-export, trust-boundary, sandbox, orchestration, model-profile, role, and receipt identities. It may introduce task/dataset/trial/job DTOs and pure composition functions, but it must not fork their underlying lifecycle or receipt rules.

**Rationale:** Duplicate state owners would permit evaluation runs to bypass existing capability, redaction, replay, freshness, or promotion gates.

### 3. Keep configuration Nickel-authored and projection explicit

**Choice:** Human-authored task, dataset, agent, verifier, environment, reward, and job policy lives under typed Nickel contracts. Deterministic exporters produce the exact Harbor-native TOML/JSON needed by the selected cohort. Rust reparses and revalidates generated data before host execution.

Every environment profile names network posture explicitly. Importing a Harbor task with omitted network policy is rejected unless an explicit compatibility policy supplies a reviewed posture; omission never silently becomes public access. Environment variables are declarations only, secret-like values remain host-owned, and generated artifacts carry source and projection BLAKE3 identities.

**Rationale:** Harbor-native formats are interoperability surfaces, while Nickel provides the reviewable source contract and avoids inheriting permissive ambient defaults.

### 4. Make trial expansion and reduction a pure core

**Choice:** Pure functions validate tasks and datasets, expand agent/model/task combinations into ordered trials, derive stable run identities, apply finite concurrency/resource policy, evaluate resume manifests, normalize completion order, and reduce typed outcomes into job summaries. Inputs include all policy, catalog, freshness, and prior-evidence facts explicitly.

The planner performs no file reads, environment inspection, process/container launch, clock access, provider calls, network access, scheduling, artifact collection, or persistence.

**Rationale:** Matrix expansion, identity, resume, and aggregation are deterministic business logic and must be testable without Harbor or a container runtime.

### 5. Put effects behind evaluation adapter ports

**Choice:** `animus-adapters` exposes implementation-free ports for environment preparation/execution, installed or external agent execution, verifier execution, explicit artifact collection/transfer, and trial outcome observation. The runtime shell calls only injected admitted services.

A non-default host backend may invoke Harbor through a typed argv/process boundary or a minimal Harbor-required integration shim. That backend owns filesystem staging, serialization, child processes, container/cloud APIs, credentials, environment variables, cancellation, deadlines, clocks, queues, and persistence. It must not construct a fallback backend from ambient state.

**Rationale:** This preserves the existing functional-core/imperative-shell split while allowing Harbor interoperability where it is operationally useful.

### 6. Distinguish shared verification from isolated verification

**Choice:** Shared and separate verifier modes are represented explicitly. Both modes bind task, subject, verifier, rubric, environment, and artifact-manifest identities. Artifact transfer is allowlisted, bounded, collision-checked, and records source service plus destination identity.

A profile that can support authoritative promotion requires separate verifier execution or an independently controlled equivalent, immutable verifier/rubric identities, and explicit Cairn completion-policy admission. Shared verifier results remain advisory because the agent may have influenced the verifier environment.

**Rationale:** A numeric score is not evidence of verifier independence, soundness, or tamper resistance.

### 7. Add strict, version-gated ATIF translation

**Choice:** The ATIF adapter accepts one exact reviewed schema version at a time, validates step order, source roles, tool-call/observation correlation, subagent lineage, metrics, terminal state, and bounded extension fields, and maps into Animus-owned trajectory DTOs. Unsupported versions or unknown authority-bearing extensions fail closed.

Raw reasoning, provider payloads, credentials, environment values, private paths, unbounded tool arguments/results, and arbitrary `extra` values are not copied into safe receipts or dataset rows. A host may retain separately authorized private raw artifacts, but reusable projections carry only approved content or hashes and redaction metadata.

**Rationale:** Harbor's ATIF shape is useful for interchange, but its complete-history goal is broader than Animus safe-evidence and training-export defaults.

### 8. Keep rewards typed and judgment bounded

**Choice:** Trial rewards are finite typed maps with validated metric names, numeric domains, missing-value policy, aggregation policy, verifier identity, subject identity, and evidence hash. Aggregation is deterministic over plan order and reports missing, invalid, non-finite, duplicate, or mismatched rewards explicitly.

Programmatic verifier output may become completion evidence only through existing Cairn policy. LLM-as-judge and agent-as-judge results remain attributed advisory evidence with immutable rubric/model/trajectory identities and cannot authorize effects, prove verifier soundness, or claim completion by themselves.

**Rationale:** Harbor's flexible reward model should be adapted without collapsing score, authority, and correctness into one value.

### 9. Emit bounded receipts and preserve stack ownership

**Choice:** Evaluation receipts bind safe task, dataset, trial, job, agent, model-route, environment, verifier, rubric, artifact-manifest, reward, trajectory-projection, policy, and host-backend identities using domain-separated BLAKE3 hashes. They include bounded counts, status/failure classes, redaction markers, freshness, and explicit non-claims, but not raw prompts, hidden tests, trajectories, credentials, environment values, tool payloads, logs, physical paths, or provider bodies.

Valence may wrap canonical identity and linkage around exported evidence. Cairn may consume admitted receipts for lifecycle gates and authoritative completion. Mantle may provide identified build/environment artifacts. None of those handoffs transfer behavioral-correctness claims between layers.

**Rationale:** The stack already has explicit evidence ownership; Harbor adaptation should feed it rather than define a competing evidence authority.

### 10. Start with one local fixture-backed vertical slice

**Choice:** The first implementation proves one explicit local dataset, one admitted agent/model profile, one reviewed local environment backend, separate verification, deterministic reward reduction, safe ATIF projection, and bounded receipts through production parser/planner/adapter entrypoints. Cloud providers, Hub publication, remote registries, hosted scheduling, automatic prompt optimization, SFT/RL execution, and public benchmark claims remain later changes.

**Rationale:** A narrow end-to-end slice tests ownership and authority boundaries before adding provider breadth.

## Risks / Trade-offs

- Harbor task and ATIF schemas can evolve; exact cohort pinning and fixture upgrades make drift visible but require maintenance.
- Generated Harbor configuration can diverge from Nickel meaning; source/projection hashes, Rust revalidation, and semantic parity fixtures are required.
- A Python integration shim may be unavoidable for Harbor's native extension interface; keeping it optional and shell-owned prevents runtime-core contamination.
- Separate verifier environments improve isolation but do not prove verifier correctness or sidecar integrity.
- ATIF can contain sensitive or hidden material; strict default redaction reduces interchange fidelity intentionally.
- Flexible reward aggregation can hide failures; missing/invalid values and aggregation policy must remain visible in receipts.
- A fixture-backed local backend does not establish Docker, cloud, Hub, or benchmark compatibility.

## Non-Goals

- Vendoring or rewriting Harbor, Reward Kit, its Hub, registry client, cloud-provider integrations, or training frameworks.
- Adding Python, Pydantic, container, cloud, or Harbor dependencies to reusable/default/no-std/Wasm crates.
- Making Harbor task configuration, ATIF, or reward files canonical Animus storage formats.
- Default public networking, ambient credential lookup, inherited secret export, or backend discovery.
- Treating shared verifier execution, sidecar evidence, LLM judges, agent judges, or numeric rewards as authoritative completion automatically.
- Persisting hidden reasoning, raw prompts, hidden tests, credentials, provider payloads, private paths, raw tool payloads, or unbounded logs in safe receipts.
- Claiming arbitrary benchmark support, Harbor Hub compatibility, cloud parity, verifier soundness, sandbox completeness, model superiority, automatic optimization, training correctness, or production readiness.
