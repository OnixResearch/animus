# Tasks

## Phase 1: Contracts

- [x] [serial] Author the `animus.frozen-policy.v1` contract, a tree example and an MLP example, the campaign tree fixture and the predeclared comparison protocol in Nickel, with checked JSON projections wired into the export rails. r[agent_harness_runtime.frozen_policy.artifact] r[agent_harness_runtime.frozen_policy.evaluation]

## Phase 2: Pure leaf

- [x] [serial] Add the `no_std` `animus-frozen-policy` crate with no Animus dependency and no ambient authority. r[agent_harness_runtime.frozen_policy.boundary]
- [x] [serial] Implement artifact admission with typed errors before any decision, a canonical artifact identity and a model identity. r[agent_harness_runtime.frozen_policy.artifact]
- [x] [serial] Implement `decision-tree-v1` and `mlp-per-mille-v1` admission and checked fixed-point evaluation with worst-case operation counts. r[agent_harness_runtime.frozen_policy.model]
- [x] [serial] Implement binding, input and budget checks, eligible selection with stable ties and abstention, typed failures, and the declared fallback rule. r[agent_harness_runtime.frozen_policy.selection] r[agent_harness_runtime.frozen_policy.failure]
- [x] [serial] Implement bounded receipts, their identities and `verify_receipt`. r[agent_harness_runtime.frozen_policy.receipt]

## Phase 3: Campaign route and comparison

- [x] [serial] Add the frozen-policy classifier route, the `stale` and `abstained` dispositions, and the per-candidate receipt identities to the evaluation lane. r[agent_harness_runtime.frozen_policy.campaign_route]
- [x] [serial] Generalize the comparison shell over the scorer arm and add `--frozen-policy`. r[agent_harness_runtime.frozen_policy.evaluation]
- [x] [serial] Run the predeclared comparison and record the result, including null or regression outcomes and the go/no-go decision. r[agent_harness_runtime.frozen_policy.evaluation]
  - No-go on the synthetic lane; see [evidence](evidence/comparison-2026-11-19.md).

## Phase 4: Verification

- [x] [serial] Add positive and negative tests for admission, both model kinds, selection, every failure, receipts, the route and the protocol. Run formatting, Clippy, the crate tests, the wasm32 check, both scorer comparisons, Octet, Nickel parity, and Cairn validation and gates. r[agent_harness_runtime.frozen_policy.verification]
  - See [checks](evidence/comparison-2026-11-19.md#checks).
