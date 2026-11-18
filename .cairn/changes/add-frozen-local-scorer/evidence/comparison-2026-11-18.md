# Local scorer comparison, 2026-11-18

Change: `add-frozen-local-scorer` on `origin/main` `7a748b22`.

## Comparison

```sh
cargo run -q -p animus-campaign-manager-eval --locked -- --local-scorer
```

Exit 0. The report (latency is a non-deterministic observation; prompts, replies and labels are never printed):

```text
LOCAL_SCORER_COMPARISON status=completed
job=job:campaign-manager-local-scorer-eval protocol=animus.campaign-manager-local-scorer-eval.v1
artifact=b3:4a2c69d0ee7b5f67c14e553892a2038ea6cf82d39fe29ebbd75602b5abc1d595 provenance=fixture
planned=33 completed=33 failed=0
arm role=baseline route=model:scripted:explore-only completed=3 found=0.333 efficiency=0.083 refusal-rate=0.000
arm role=reference(broader-grant,non-comparable) route=model:scripted:signal-greedy completed=3 found=1.000 efficiency=0.556 refusal-rate=0.000
arm role=scorer route=model:local-scorer:4a2c69d0ee7b5f67 completed=3 found=0.333 efficiency=0.167 refusal-rate=0.000
scorer decisions=17 scored=17 malformed_brief=0 out_of_range=0 operation_budget=0 incompatible_artifact=0 unavailable=0 brief_bytes=23929 feature_operations=450 inference_operations=450 fallback_candidates=0
arm role=random route=model:random-rank:seed-1 completed=3 found=0.333 efficiency=0.167 refusal-rate=0.000
arm role=random route=model:random-rank:seed-2 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-3 completed=3 found=0.333 efficiency=0.333 refusal-rate=0.000
arm role=random route=model:random-rank:seed-4 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-5 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-6 completed=3 found=0.333 efficiency=0.167 refusal-rate=0.000
arm role=random route=model:random-rank:seed-7 completed=3 found=0.333 efficiency=0.083 refusal-rate=0.000
arm role=random route=model:random-rank:seed-8 completed=3 found=0.333 efficiency=0.333 refusal-rate=0.000
decision=no-go rule=gain>=100permille,random-parity=true,fallback<=0permille scorer=0.333 baseline=0.333 random_mean=0.208 fallback=0permille
```

Per-arm latency lines are omitted above; on this run every arm's p50 was 21.6–31.2 µs per call.

The decision is **no-go**. The fixture scorer ties the grant-matched baseline on `found` (0.333) with twice its efficiency (0.167 against 0.083) and beats the random mean (0.208), but it does not reach the predeclared gain of 0.100 over the baseline. All 17 real manager briefs fit the declared projection; none fell back. `signal-greedy` reaches 1.000 only because it may add offers, which the scorer's grant excludes.

## Checks

| Check | Result |
| --- | --- |
| `cargo test -p animus-campaign-manager-eval --features live` | all suites passed; the scorer suites `artifact` (3), `scorer` (6) and `comparison` (3) hold the positive and negative cases |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy -p animus-campaign-manager-eval --all-targets --locked -- -D warnings`, with and without `live` | passed |
| `nix build .#checks.x86_64-linux.{cargo-campaign-manager-eval,campaign-manager-eval-octet,nickel-config}` | all three passed; the sandboxed `--local-scorer` run printed the same no-go decision |
| Octet (`campaign-manager-eval-octet`, `--all-targets --features live`) | `warning-only`, 3 findings, none in this change: the binary's unchanged `std::env::args` read (bin and bin-test targets) and `unknown lint: ambient_env` in `animus-std` |
| Cairn `validate --root .` and the proposal, design and tasks gates | passed, 0 issues |

The binary reads the wall clock in one shell helper under a narrow `tigerstyle::ambient_clock` allowance; latency stays a non-deterministic observation that no decision or record reads. An arm that completes no trial now refuses the report instead of entering the adoption rule as 0.

## Non-claims

The artifact is a hand-authored fixture: its result qualifies the route and artifact boundary, not a trained model or learning. Three synthetic tasks do not establish advice quality on real workloads or consumer improvement; ChaosControl #46 owns the real consumer measurement and RSI Lab #80 the trained export. Scores and ranks are neither task success nor authorization. Production rollout stays conditional on a trained artifact and the consumer pilot.
