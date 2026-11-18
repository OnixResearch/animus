## Context

`animus-campaign-manager-eval` runs Harbor-compatible trials of the campaign manager. A route is a `ComponentAdapter<ModelRequest, ModelResponse>`; scripted routes read the rendered brief through `routes::reading` and reply in the manager schema. The independent verifier reloads Campaign's published history and grades `found`, `efficiency` and the advisory `refusal-rate`. This change adds a frozen local policy to that seam without changing the manager, Campaign, the verifier or `animus-adapters`.

## Decisions

### Route and grant

`LocalScorerRoute` (identity `model:local-scorer:<artifact-identity-prefix>`) is a pure adapter over an admitted artifact. Each call reads the brief, scores every listed candidate and replies `{"scores":[…],"add":[]}`. It never adds offers, retires, prunes or stops: candidate generation, pruning, stopping, eligibility, publication and execution stay with Campaign and the host. `explore-only` has the same grant and is the comparison baseline; `signal-greedy` adds offers, so it is reported as a broader-grant reference, not a comparable arm.

### Feature projection (`animus.campaign-manager-brief-features.v1`)

Only the brief presented at the decision enters the policy: listed candidates in brief order, their families and observed signals. Offers, labels beyond the family prefix, hidden targets and earlier briefs never enter. Six integer features per candidate, each per mille in `[0, 1000]`, with integer division rounding down:

| Index | Feature | Value |
| --- | --- | --- |
| 0 | `executed` | 1000 if the brief shows a signal, else 0 |
| 1 | `signal` | `signal * 1000 / 9`, 0 if unexecuted |
| 2 | `family_best_signal` | the family's best observed signal `* 1000 / 9`, 0 if none |
| 3 | `family_unexplored` | unexecuted family members `* 1000 /` family size |
| 4 | `brief_unexplored` | unexecuted candidates `* 1000 /` listed candidates |
| 5 | `family_leads` | 1000 if the family holds the brief's best observed signal, else 0 |

Nine is the task schema's signal bound. A signal above it, more than 32 candidates, or a brief without a candidate list is a malformed brief.

### Policy and actions (`linear-per-mille-v1`, `animus.campaign-manager-rank-action.v1`)

`score = bias + Σ weight[i] * feature[i] / 1000` in checked `i64`, with each weight and the bias in `[-1_000_000, 1_000_000]`. The score becomes the first rank dimension; later dimensions are 0, matching the scripted routes. The artifact declares `[min_rank, max_rank]`; a score outside it is out of range, never clamped. Its bounds must lie within the magnitude the brief presents. Equal scores give equal ranks, and candidates are emitted in brief order; the route never breaks ties with randomness, and Campaign's ordering of equal ranks is unchanged.

### Artifact (`animus.campaign-manager-local-scorer.v1`)

A JSON object with exactly: `schema`, `policy`, `feature_schema`, `action_schema`, `features` (ordered names), `weights`, `bias`, `rank_bounds`, `runtime` (`{"crate": "animus-campaign-manager-eval", "artifact_api": 1, "operation_budget": N}`), `provenance` (kind, producer, evaluation summary, non-claims). Its identity is `animus_eval_core::hash_identity` (domain-separated, framed BLAKE3 with the `b3:` prefix) under the artifact schema over the compact encoding of the typed document, whose keys are declared in sorted order, so input spacing and key order never change it. Admission takes the artifact bytes and the expected identity; it returns a typed `ArtifactError` before any scoring for missing or oversized bytes, unknown fields, another schema, policy or feature/action schema, a feature list differing from the declared projection, out-of-bounds weights or rank bounds, an unsupported runtime crate, `artifact_api` or operation budget, or an identity mismatch. A file name is never identity.

The first artifact is a hand-authored fixture under `config/`, authored in Nickel with a checked JSON projection. Its provenance says `fixture`, not trained.

### Dispositions and fallback

Each call records one disposition:

- `scored`;
- `malformed-brief`;
- `out-of-range` (a score or the declared bounds outside the presented domain);
- `operation-budget` (the decision's weight-feature products, listed candidates × 6, exceed the artifact's declared operation budget; the deterministic stand-in for a timeout);
- `incompatible-artifact` and `unavailable` (a route built from a refused or missing artifact).

Every disposition other than `scored` replies with `explore-only`'s reply for the same brief and records the fallback and its cost. Fallback is deterministic, never silent, and never re-tries the scorer.

### Records and cost

Each decision record holds the call index, the artifact identity (if admitted), the per-candidate features and scores, the disposition, the fallback, and deterministic cost units: brief bytes read, feature operations, inference multiply-adds and fallback candidates. The route keeps at most 4,096 records and refuses further calls past that bound. Replay and reports use these records; nothing re-runs inference. Wall-clock latency is measured by the binary around each call and reported separately as non-deterministic.

### Random baseline

`RandomRankRoute` replies with one uniform rank in the declared action bounds per listed candidate, from SplitMix64 seeded by the declared seed and the call index, and no offers. It runs on the same tasks, turns and selection budgets as every other arm.

### Comparison protocol

`config/campaign-manager-local-scorer-eval.ncl` declares the arms (`explore-only`, `random-rank` for seeds 1 to 8, `local-scorer` for the selected fixture identity, and `signal-greedy` as a broader-grant reference), the task set (all three evaluator tasks; the fixture used none of them, so none are in-sample), primary outcome `found`, secondaries `efficiency`, `refusal-rate` and fallback rate, and the adoption rule. The rule says go only if the scorer's mean `found` is at least `explore-only`'s plus 0.10, at least the random arm's mean, and its fallback rate is 0. `animus-campaign-manager-eval --local-scorer` runs the protocol and prints per-arm completion, rewards, fallback counts, cost units, latency, and the decision with its non-claims. The decision is about this synthetic lane only; production rollout stays conditional on a trained artifact and the consumer pilot.

## Risks

- A hand-authored fixture can encode the scripted heuristic; the report must say so and must not present its result as learning.
- Three synthetic tasks are a small sample; the report includes null and regression results unchanged.
