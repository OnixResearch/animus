## Why

Animus #15 asks whether a small, frozen local policy can make useful bounded ranking decisions through the existing campaign manager at lower total cost. Its first pilot qualifies the route and its artifact boundary in the existing evaluator, using a fixture artifact and declared schemas, before RSI Lab exports a trained artifact (RSI Lab #80) and before consumers such as Molten #41 and ChaosControl #46 integrate one. The evaluator already supplies the seam: `ModelAdapter`, `animus-campaign-manager::advise`, scripted routes, and independent replay grading.

## What Changes

- Add a frozen local scorer route to `animus-campaign-manager-eval`. It reads only the rendered brief through the existing prompt reader, projects declared integer features for each listed candidate, scores them with a frozen fixed-point linear policy, and replies in the existing manager schema with candidate scores only: it adds no offers and retires nothing. r[agent_harness_runtime.campaign_manager_eval.local_scorer.route]
- Define the versioned artifact `animus.campaign-manager-local-scorer.v1`. It binds the policy kind, weights, feature and action schemas, normalization, rank bounds, runtime compatibility and evaluation provenance; its identity is BLAKE3 over canonical JSON. A missing, mismatched, incompatible or out-of-bounds artifact is a typed failure before any scoring. r[agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
- Declare the feature projection, normalization, observation cutoff and order, candidate order, rank schema, integer conversion and tie policy. Only observations visible in the brief at decision time enter the policy. r[agent_harness_runtime.campaign_manager_eval.local_scorer.schema]
- Retain malformed-brief, out-of-range, operation-budget, incompatible-artifact and unavailable dispositions, each with a deterministic fallback to the declared `explore-only` policy whose cost is charged. r[agent_harness_runtime.campaign_manager_eval.local_scorer.fallback]
- Record each decision's input features, scores, disposition, artifact identity and cost; replay uses retained decisions and never re-runs inference. r[agent_harness_runtime.campaign_manager_eval.local_scorer.record]
- Add a budget-matched random-ranking route with a declared seed under the same grants. r[agent_harness_runtime.campaign_manager_eval.local_scorer.baseline]
- Add a predeclared comparison protocol and a `--local-scorer` report: arms, seeds, tasks, budgets, primary outcome, adoption rule and costs, including null or regression results and the go/no-go decision. r[agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
- Add positive and negative tests for every new boundary and extend the existing evaluator checks. r[agent_harness_runtime.campaign_manager_eval.local_scorer.verification]

## Impact

- **Rust**: new modules in `crates/animus-campaign-manager-eval` only, with no new dependency: the artifact identity uses `animus-eval-core`'s domain-separated `hash_identity`. `animus-adapters` stays implementation-free; no inference runtime, model loader or new crate is added.
- **Config**: a fixture artifact and the predeclared comparison protocol, authored in Nickel with checked JSON projections.
- **Dependencies**: depends on `add-campaign-manager-eval`, whose evaluator this extends.
- **Non-claims**:
  - The fixture artifact is hand-authored, not trained; its results qualify the route boundary only.
  - Synthetic-task results do not establish advice quality on real workloads, consumer improvement or production readiness; ChaosControl #46 owns the real consumer measurement.
  - Model scores and ranks are neither task success nor authorization.
