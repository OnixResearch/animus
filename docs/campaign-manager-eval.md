# Campaign-manager evaluation lane

`crates/animus-campaign-manager-eval` evaluates the [campaign manager](campaign-manager.md) through Animus's Harbor-compatible slice (see [harbor-evaluation-harness.md](harbor-evaluation-harness.md)). A job runs every task against every model route. Each trial is a real Campaign campaign on in-memory Choregraph history. A separate verifier grades each trial from Campaign's replay of that history. Harbor's Python application, Hub, and container backends are not involved.

The crate is the separate non-default `AGPL-3.0-or-later` member of the workspace because it links Campaign.

## Tasks

`config/campaign-manager-eval-tasks.ncl` is a typed Nickel task set. It is exported to `config/generated/campaign-manager-eval-tasks.json`, checked by `scripts/config/check-nickel-configs` with its standalone request and manifest, and revalidated in Rust.

Each task is one exploration campaign:
- It has an objective, a selection budget, and a turn limit.
- Its candidates are admitted when the campaign opens. Its offers are candidates the manager may add.
- Every candidate and offer has a `family/approach` label, a hidden signal from 0 to 9, and a target flag. Exactly one of them per task is the target.

| Task | Target | Budget | What it rewards |
|---|---|---|---|
| `task:json-decoder-hang` | an initial candidate | 4 | exploring |
| `task:config-parser-crash` | an offer | 6 | adding the strong family's offers |
| `task:cache-key-collision` | an offer | 6 | exploiting a strong signal early |

## One trial

1. The environment opens the task's campaign and admits its candidates.
2. Each turn begins with one manager turn through `animus_campaign_manager::advise`, using the trial's route. The menu labels every admitted candidate, appends `[signal N]` once it has run, and offers every offer not yet added. Admitted advice is published, and a reply is recorded as admitted, unchanged, refused, or a model failure.
3. One host planner step follows, through Campaign's ranked planner, which stops after the first finding:
   - **Selection:** the host publishes it, waits for the publication fence to release it, and runs it against the hidden outcome. Running the target records the finding. The host then publishes the result.
   - **Wait:** the host publishes nothing.
   - **Stop:** the host publishes the planner's stop.
4. The trial ends when the campaign stops, or after the task's turns.

The model sees outcomes only through the signals the host writes into labels.

## Grading

The verifier reloads the control projection from the trial's Choregraph graph with `ControlSession::load`. It refuses:
- a missing artifact as `Stale`;
- a history that does not load as `Malformed`;
- a replayed final state that differs from the environment's claim as `Stale`.

| Metric | Source | Value |
|---|---|---|
| `found` | replay | 1 when the task's finding is recorded and the target's candidate was selected, else 0 |
| `efficiency` | replay | `(budget + 1 - selections) / budget` for a found target, else 0 |
| `refusal-rate` | environment ledger, advisory | share of turns with a refused reply or a failed model call |

A reply that asks for no change is neither admitted nor refused.

Every trial also yields a metadata-only ATIF-v1.7 projection. Each manager turn is an agent step calling `campaign.advise`, and each host step is a system step calling `campaign.execute`, `campaign.wait`, or `campaign.stop`. The job receipt binds the projection identities.

## Routes and running

The scripted routes are deterministic and offline:
- `signal-greedy` explores, then exploits a family whose signal is 6 or more.
- `explore-only` never adds offers.
- `malformed` never answers in the schema.

```sh
cargo run -p animus-campaign-manager-eval
```

On the checked task set, the scripted job reports the following (the refusal rate is advisory):

| Route | found | efficiency | refusal rate |
|---|---|---|---|
| `signal-greedy` | 1.000 | 0.556 | 0.000 |
| `explore-only` | 0.333 | 0.083 | 0.000 |
| `malformed` | 0.000 | 0.000 | 1.000 |

The live route is opt-in. It sends each brief as one text-only request through `animus-std-openai` to the reviewed `OpenAI` or `OpenRouter` Responses endpoint:

```sh
cargo run -p animus-campaign-manager-eval --features live -- \
  --live openrouter --model <MODEL> --credential-stdin < /run/credentials/openrouter-api-key
```

The report holds identities, counts, mean rewards, failure classes, and non-claims. It never holds prompts, replies, labels, or credentials.

## Frozen local scorer

The local scorer route answers each brief with a frozen fixed-point linear policy over six declared per-mille features of the listed candidates: `executed`, `signal`, `family_best_signal`, `family_unexplored`, `brief_unexplored` and `family_leads`. Only what the brief shows enters; offers and hidden outcomes never do. The route scores candidates only. It adds no offers and retires, prunes or stops nothing, so `explore-only` is its grant-matched baseline and `signal-greedy`, which adds offers, is a non-comparable reference.

The artifact schema `animus.campaign-manager-local-scorer.v1` binds the weights, feature and action schemas, rank bounds, runtime compatibility (including a per-decision operation budget) and provenance. Admission checks the artifact's `b3:` identity against the protocol's selection before any scoring; a file name is never identity. A malformed brief, an out-of-range score, an exceeded operation budget, or a refused or missing artifact answers with `explore-only`'s reply, recorded with its disposition and charged cost. Each route retains at most 4,096 decision records; replay and reports use them and never re-run inference.

The checked fixture `config/campaign-manager-local-scorer-fixture.ncl` is hand-authored, not trained. The predeclared protocol `config/campaign-manager-local-scorer-eval.ncl` fixes the arms, seeds, primary outcome and adoption rule before any arm runs:

```sh
cargo run -p animus-campaign-manager-eval -- --local-scorer
```

On the checked task set the comparison reports:

| Arm | found | efficiency | refusal rate |
|---|---|---|---|
| `explore-only` (baseline) | 0.333 | 0.083 | 0.000 |
| local scorer (fixture) | 0.333 | 0.167 | 0.000 |
| random ranking, seeds 1 to 8 | mean 0.208 (0.000 to 0.333 by seed) | 0.000 to 0.333 by seed | 0.000 |
| `signal-greedy` (broader grant) | 1.000 | 0.556 | 0.000 |

All 17 scorer decisions were scored without fallback. The adoption rule requires the scorer's mean `found` to exceed the baseline's by 0.100, to reach the random mean, and to have no fallback, so the decision is **no-go**: the fixture matches the baseline's findings with higher efficiency but finds nothing more. Latency is reported per arm as a non-deterministic observation.

## Frozen-policy tree route

The frozen-policy route decides each listed candidate's class through the shared `animus-frozen-policy` leaf; see [frozen-policy.md](frozen-policy.md). It projects the same six brief features and declares the classes `follow-signal`, `explore` and `exploit` (`animus.campaign-manager-candidate-class.v1`). It maps them to ranks: an executed candidate's observed signal, 10, and 20. Like the linear scorer it adds no offers. Any refused candidate answers the whole brief with `explore-only`'s reply and records the disposition: `malformed-brief`, `out-of-range`, `operation-budget`, `incompatible-artifact`, `unavailable`, `stale` or `abstained`. Each record keeps every candidate's `animus.frozen-policy-receipt.v1` identity.

The checked fixture `config/campaign-manager-frozen-policy-fixture.ncl` is a seven-node hand-authored decision tree. It was not trained, and it was written before the comparison ran. The predeclared protocol `config/campaign-manager-frozen-policy-eval.ncl` (`animus.campaign-manager-frozen-policy-eval.v1`) keeps the linear scorer's baseline, reference, seeds and adoption rule. Its random arms draw from the route's own rank range, 0 to 20:

```sh
cargo run -p animus-campaign-manager-eval -- --frozen-policy
```

On the checked task set the comparison reports:

| Arm | found | efficiency | refusal rate |
|---|---|---|---|
| `explore-only` (baseline) | 0.333 | 0.083 | 0.000 |
| frozen-policy tree (fixture) | 0.333 | 0.167 | 0.000 |
| random ranking, seeds 1 to 8 | mean 0.167 (0.000 to 0.333 by seed) | 0.000 to 0.333 by seed | 0.000 |
| `signal-greedy` (broader grant) | 1.000 | 0.556 | 0.000 |

The tree route scored all 17 briefs, 75 candidate decisions with 110 tree splits, without fallback. It ties the baseline on `found` and misses the predeclared 0.100 gain, so the decision is **no-go**. Its per-call latency in a debug build was about ten times the scripted arms', because every candidate decision encodes and hashes a receipt; latency remains a non-deterministic observation.

## Non-claims

- Scores hold only for these synthetic tasks. They do not measure advice quality on real workloads, and they claim no benchmark superiority.
- The verifier is sound only as far as Campaign's history replay goes. `refusal-rate` rests on the environment's ledger.
- A route can exploit the label format without understanding the objective.
- Live scores are not deterministic, and a live run contacts the provider only when invoked.
- The lane establishes no Harbor Hub, cloud, or container parity, and no production or release readiness.
- The local scorer's and the tree route's fixtures encode hand-written heuristics. Their comparisons qualify the route and artifact boundaries; they do not measure a trained model, learning, or consumer improvement.
- Cost units compare equal selection budgets. Equal total compute, memory and model-load amortization are not measured.

## Verification

```sh
nix build .#checks.x86_64-linux.cargo-campaign-manager-eval   # tests with live, Clippy with and without live, scripted job, both scorer comparisons
nix build .#checks.x86_64-linux.campaign-manager-eval-octet   # Octet deny-all with live
nix build .#checks.x86_64-linux.cargo-frozen-policy           # leaf tests, Clippy, and the wasm32 no-std check
nix build .#checks.x86_64-linux.frozen-policy-octet           # Octet deny-all for the leaf
nix build .#checks.x86_64-linux.nickel-config                 # generated task set, fixtures and protocols parity
```
