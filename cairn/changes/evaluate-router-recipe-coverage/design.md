# Design: Evaluate router recipe coverage

## Context

Routing logic is being completed across selection, materialization, aggregation, receipts, and runtime wiring. Before promoting any recipe-policy tuning into defaults, Animus needs a disciplined evaluation: what does the router actually do on a representative corpus? The workspace already adapts ResearchSpec lifecycle patterns (baseline, hypothesis, run, keep/promotion, archive) and a batch-trajectory-runner. This change composes them into an evaluation-only study of router recipe coverage.

## Decisions

### 1. ResearchSpec lifecycle frames the study

**Choice:** Structure the evaluation as a ResearchSpec study: a recorded baseline (current rule-projector selection over a fixed corpus), an explicit hypothesis, a run plan, and keep/promotion/archive gates. The study artifacts live under the ResearchSpec lifecycle location used elsewhere in the repo.

**Rationale:** Reuses the accepted `hermes_researchspec_lifecycle` adaptation so the study has disciplined gates rather than ad hoc benchmarking.

### 2. The batch-trajectory-runner drives the corpus

**Choice:** The run plan uses the batch-trajectory-runner with a deterministic test `RouterService` stub so the study is reproducible without live providers. The corpus is a checked-in fixture set of typed `RequestMetadata` values, not raw prompts.

**Rationale:** Keeps the study provider-free and reproducible, matching the accepted `clankers_batch_trajectory_runner` boundary.

### 3. Telemetry is safe metadata only

**Choice:** The router emits selection telemetry as safe metadata: recipe id, family, denial class, budget consumption counters, and signal bucket labels. No raw prompts, provider payloads, worker outputs, or credentials. Telemetry is feature-gated and off by default.

**Rationale:** Connects to the accepted receipt-redaction requirements; evaluation must not leak payloads.

### 4. Findings are non-promoting by default

**Choice:** The study produces an adoption matrix and diagnostic report. Promoting any finding into default recipe policy or thresholds requires a separate change with its own Cairn gates. This change MUST NOT mutate routing logic.

**Rationale:** Evaluation and policy change are separate concerns; mixing them would let a study silently change routing behavior.

## Risks / Trade-offs

- A small corpus can mislead. Document the corpus's representativeness limits and treat findings as diagnostic, not authoritative.
- Telemetry hooks can accidental leak. Reuse the `validate_receipt` rail for any emitted metadata and add a negative test asserting no raw payload appears.
- The study can rot if the corpus is not maintained. Tag the corpus as a maintained fixture and gate the study run in CI as a non-blocking check.
