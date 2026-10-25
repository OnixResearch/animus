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

## Non-claims

- Scores hold only for these synthetic tasks. They do not measure advice quality on real workloads, and they claim no benchmark superiority.
- The verifier is sound only as far as Campaign's history replay goes. `refusal-rate` rests on the environment's ledger.
- A route can exploit the label format without understanding the objective.
- Live scores are not deterministic, and a live run contacts the provider only when invoked.
- The lane establishes no Harbor Hub, cloud, or container parity, and no production or release readiness.

## Verification

```sh
nix build .#checks.x86_64-linux.cargo-campaign-manager-eval   # tests with live, Clippy with and without live, scripted job
nix build .#checks.x86_64-linux.campaign-manager-eval-octet   # Octet deny-all with live
nix build .#checks.x86_64-linux.nickel-config                 # generated task set parity
```
