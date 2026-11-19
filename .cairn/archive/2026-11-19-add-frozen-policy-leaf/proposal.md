## Why

Animus #15 is the primary System One pilot: a small, frozen local policy makes bounded decisions through the existing campaign manager, measured against fixed baselines. `add-frozen-local-scorer` (archived 2026-11-18) qualified a linear scorer whose artifact, scoring and records live inside `animus-campaign-manager-eval`. The issue asks for a simple tree or small MLP first, a `stale` disposition, records that bind input, output and runtime revisions, and a small shared policy contract and runtime leaf once a second real consumer needs it.

Lattice #23 is that consumer. Its optional typed policy operation needs production policy execution that takes finite classification or selection inputs, returns a result chosen from a declared action set with typed failures for malformed, out-of-range, stale and unavailable cases, binds a versioned artifact, and produces bounded receipts that replay without inference. It must not need the whole agent harness, Campaign, or the AGPL evaluation lane.

## What Changes

- Add `crates/animus-frozen-policy`, a pure `no_std` + `alloc` leaf with no ambient authority, depending only on `animus-eval-core`, `serde` and `serde_json`. It never trains or updates weights. r[agent_harness_runtime.frozen_policy.boundary]
- Define the versioned artifact `animus.frozen-policy.v1`. It binds the policy kind and weights, feature schema with raw bounds and `min-max-per-mille-v1` normalization, action schema, score bounds, an optional abstention threshold, an operation budget, runtime compatibility and provenance. Its identity is BLAKE3 over the canonical encoding; a separate model identity digests the weights alone. Admission under a selected identity refuses every contract violation with a typed error before any decision. r[agent_harness_runtime.frozen_policy.artifact]
- Evaluate two frozen policy kinds in checked fixed-point integer arithmetic: `decision-tree-v1` (a bounded binary tree over normalized features) and `mlp-per-mille-v1` (at most four dense layers, ReLU hidden layers, an identity output layer with one score per action). r[agent_harness_runtime.frozen_policy.model]
- Select the best action among the caller's declared eligible subset of the action schema, with ties broken by declared action order and an optional abstention threshold. A selection is a preference among actions the caller declared; it is neither success nor authorization. r[agent_harness_runtime.frozen_policy.selection]
- Return typed failures for unavailable, incompatible-artifact, stale-binding, malformed-input, input-out-of-range, operation-budget (the deterministic deadline), score-out-of-range and abstained cases. A request that declares an eligible fallback action answers with it, and its receipt records that; a request with a malformed shape never falls back. r[agent_harness_runtime.frozen_policy.failure]
- Issue a bounded `animus.frozen-policy-receipt.v1` receipt for every decision. It binds the requested and admitted identities, runtime revision, input and eligible-set identities, the raw features, the per-action scores, the outcome and deterministic cost units. `verify_receipt` re-admits a retained receipt so replay never re-runs inference. r[agent_harness_runtime.frozen_policy.receipt]
- Add a frozen-policy route to `animus-campaign-manager-eval`. It classifies each listed candidate through the leaf, using a hand-authored decision-tree fixture, and maps the classes `follow-signal`, `explore` and `exploit` to ranks. It adds no offers and falls back to `explore-only` with a recorded disposition, including the new `stale` and `abstained` dispositions. r[agent_harness_runtime.frozen_policy.campaign_route]
- Add a second predeclared protocol, `animus.campaign-manager-frozen-policy-eval.v1`, and the `--frozen-policy` comparison, which reuses the existing adoption rule, arms and report. Record its result, including a null or regression result and the go/no-go decision. r[agent_harness_runtime.frozen_policy.evaluation]
- Add positive and negative tests for every boundary, and run the repository rails. r[agent_harness_runtime.frozen_policy.verification]

## Impact

- **Rust**:
  - New workspace member `crates/animus-frozen-policy` (MIT OR Apache-2.0, not a default member).
  - `animus-campaign-manager-eval` depends on it. `DecisionRecord` gains `receipts`, and `Disposition` gains `Stale` and `Abstained`. The comparison shell becomes generic over the scorer arm.
  - The archived linear scorer, its artifact schema and its protocol are unchanged.
- **Config**: `config/frozen-policy-contract.ncl` (the authoring contract), `config/frozen-policy-examples.ncl` (a tree example and an MLP example), and `config/campaign-manager-frozen-policy-{fixture,eval}.ncl`, each with checked JSON projections wired into the Nickel export rails.
- **Flake**: `cargo-frozen-policy` (tests, Clippy, and a `wasm32-unknown-unknown` check that keeps the leaf `no_std`), `frozen-policy-octet`, and the `--frozen-policy` run in `cargo-campaign-manager-eval`.
- **Consumers**: Lattice #23 vendors `animus-frozen-policy`, and depends on neither the evaluation lane nor Campaign. ChaosControl #46 and RSI Lab #80 may target the same artifact schema.
- **Non-claims**:
  - Every artifact here is a hand-authored fixture, not trained. Training, distillation and export stay with RSI Lab.
  - Synthetic-lane results do not establish advice quality on real workloads, consumer improvement or production readiness.
  - The operation budget is a deterministic stand-in for a deadline. Wall-clock timeouts, cancellation, memory measurement and model-load amortization belong to the host.
  - Recurrent state, online updates and a second inference runtime are out of scope.
