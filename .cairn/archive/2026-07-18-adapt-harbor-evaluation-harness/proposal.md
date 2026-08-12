## Why

Animus already has pure batch planning, local evaluation-feedback records, ResearchSpec optimization gates, sandbox and trust-boundary contracts, safe receipts, and supervised/preference/RL dataset projection. Those pieces do not yet compose into one host-executable evaluation model with explicit task, dataset, trial, job, environment, verifier, reward, and trajectory boundaries.

Harbor demonstrates a useful operational decomposition for agent evaluation: a task combines an instruction, environment, and verifier; datasets group tasks; trials bind one agent attempt to one reward outcome; jobs expand and schedule trial matrices; and ATIF provides a versioned trajectory interchange shape. Animus should adapt those mechanisms behind its existing functional-core and imperative-shell boundaries rather than import Harbor's Python application, ambient defaults, cloud registry, or authority model into reusable Rust crates.

## What Changes

- Add an optional Harbor-inspired evaluation boundary that reuses existing Animus batch, evaluation-feedback, ResearchSpec, trust-boundary, sandbox, dataset-export, orchestration, and receipt owners instead of creating parallel lifecycle state. r[agent_harness_runtime.harbor_eval.boundary]
- Define Nickel-authored task, dataset, trial, and job contracts with deterministic generated projections for Harbor-native interoperability and no implicit public-network posture. r[agent_harness_runtime.harbor_eval.configuration]
- Define a pure deterministic task/dataset-to-trial/job planner with stable BLAKE3 identities, finite resource and concurrency policy, deterministic ordering, and explicit resume inputs. r[agent_harness_runtime.harbor_eval.planning]
- Add implementation-free evaluation environment, agent, verifier, artifact-transfer, and trial-execution ports; keep filesystem, process, container, credential, network, clock, scheduler, and persistence effects in host-owned shells. r[agent_harness_runtime.harbor_eval.execution_ports]
- Preserve shared-versus-separate verifier semantics while requiring immutable verifier identity, explicit artifact transfer, and separate-verifier evidence for authoritative promotion profiles. r[agent_harness_runtime.harbor_eval.verifier_isolation]
- Add a version-gated ATIF interchange adapter with strict correlation, redaction, bounds, and safe projection into Animus trajectory and dataset records. r[agent_harness_runtime.harbor_eval.atif]
- Add deterministic multi-dimensional reward records and aggregation while keeping agent/LLM judge scores advisory unless independently admitted by Cairn-owned completion policy. r[agent_harness_runtime.harbor_eval.rewards]
- Emit metadata-only evaluation receipts and explicit Valence/Cairn handoff identities without claiming agent correctness, verifier soundness, sandbox completeness, benchmark superiority, or training suitability. r[agent_harness_runtime.harbor_eval.receipts]
- Add positive and negative conformance fixtures covering config projection, planning, host admission, verifier isolation, trajectory conversion, reward aggregation, receipt redaction, and dependency isolation. r[agent_harness_runtime.harbor_eval.verification]

## Impact

- **Reusable crates**: gain only pure evaluation DTOs, deterministic planning/validation, implementation-free service contracts, safe ATIF translation, reward reduction, and receipt projection. They do not gain Python, Harbor, container, cloud, filesystem, process, credential, network, clock, scheduler, or persistence dependencies.
- **Host shell**: may add a non-default authority-scoped Harbor backend or integration shim that invokes one exact reviewed Harbor cohort through typed arguments and explicit inputs. No ambient backend, credential, task, dataset, agent, or network discovery is allowed.
- **Configuration**: Nickel remains the human-authored source of truth. Harbor TOML/JSON is generated interoperability output, and omitted network policy does not inherit Harbor's public-network default silently.
- **Evidence**: Animus owns bounded trial and trajectory metadata; Valence may wrap identity/linkage; Cairn remains the completion and promotion authority. Numeric rewards and judge outputs remain bounded evidence rather than correctness proof.
- **Claims**: the first evidence rung is a local, explicit, fixture-backed evaluation path. It does not claim Harbor Hub compatibility, cloud-provider parity, arbitrary benchmark compatibility, hosted scheduling, RL training, automatic optimization, verifier soundness, or production readiness.
