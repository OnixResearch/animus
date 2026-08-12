# Proposal: Evaluate router recipe coverage

## Why

Once the router selects recipes from projected signals, the natural question is: over a representative request corpus, which recipes get selected, do budgets hold, where do denials cluster, and which families are under- or over-selected. This is evaluation-only work that produces an adoption matrix and diagnostic data — it does not change routing behavior. It reuses the batch-trajectory-runner and ResearchSpec lifecycle patterns already adapted into the workspace so the study has explicit baseline, hypothesis, run, and keep/promotion gates.

## What Changes

- Define a ResearchSpec-style study with a baseline (rule-projector signal selection over a fixed corpus), a hypothesis (e.g. "ReMoM/Fusion selection rate is non-trivial and budgets hold"), a run plan using the batch-trajectory-runner with a deterministic test `RouterService` stub, and keep/promotion gates.
- Instrument the router to emit selection telemetry (recipe id, family, denial class, budget consumption) as safe metadata — no raw prompts or payloads.
- Produce an adoption matrix mapping corpus segments to selected families, denial clusters, and budget-exhaustion hotspots.
- Add the study as evaluation-only: it MUST NOT change routing logic, recipe selection, or default behavior.

## Impact

- **Files**: `crates/animus-runtime` batch/eval fixtures or a new eval module, ResearchSpec study artifacts, router telemetry hooks (safe metadata only), docs, and Cairn evidence.
- **Testing**: positive study-run fixtures proving the corpus produces a stable adoption matrix; negative tests proving the study cannot mutate routing state or emit raw payloads.
- **Non-claims**: this does not change routing behavior, add live provider calls, claim benchmark parity with vLLM, or promote any finding into default policy without a separate change.
