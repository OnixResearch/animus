# Frozen-policy tree comparison, 2026-11-19

Change `add-frozen-policy-leaf`, uncommitted, on `origin/main` `365b0ee6`. The protocol `config/campaign-manager-frozen-policy-eval.ncl` and the tree fixture were written before the tree arm ran on any evaluation task.

## Comparison

```sh
cargo run -q -p animus-campaign-manager-eval --locked -- --frozen-policy
```

Exit 0. The report below is the sandboxed run inside `nix build .#checks.x86_64-linux.cargo-campaign-manager-eval`, which printed the same counts, rewards and decision as the earlier dev-shell run. Latency is a non-deterministic observation from a debug build. Prompts, replies and labels are never printed.

```text
FROZEN_POLICY_COMPARISON status=completed
job=job:campaign-manager-frozen-policy-eval protocol=animus.campaign-manager-frozen-policy-eval.v1
artifact=b3:05cdecb7372c81a5aa28d39e5d216d60c75d8747a43b6a09d35b905047b4a9c3 provenance=fixture
planned=33 completed=33 failed=0
arm role=baseline route=model:scripted:explore-only completed=3 found=0.333 efficiency=0.083 refusal-rate=0.000
arm role=reference(broader-grant,non-comparable) route=model:scripted:signal-greedy completed=3 found=1.000 efficiency=0.556 refusal-rate=0.000
arm role=scorer route=model:frozen-policy:05cdecb7372c81a5 completed=3 found=0.333 efficiency=0.167 refusal-rate=0.000
scorer decisions=17 scored=17 malformed_brief=0 out_of_range=0 operation_budget=0 incompatible_artifact=0 unavailable=0 stale=0 abstained=0 brief_bytes=24082 feature_operations=450 inference_operations=110 fallback_candidates=0 leaf_receipts=75
arm role=random route=model:random-rank:seed-1 completed=3 found=0.333 efficiency=0.083 refusal-rate=0.000
arm role=random route=model:random-rank:seed-2 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-3 completed=3 found=0.333 efficiency=0.167 refusal-rate=0.000
arm role=random route=model:random-rank:seed-4 completed=3 found=0.333 efficiency=0.333 refusal-rate=0.000
arm role=random route=model:random-rank:seed-5 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-6 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-7 completed=3 found=0.000 efficiency=0.000 refusal-rate=0.000
arm role=random route=model:random-rank:seed-8 completed=3 found=0.333 efficiency=0.333 refusal-rate=0.000
decision=no-go rule=gain>=100permille,random-parity=true,fallback<=0permille scorer=0.333 baseline=0.333 random_mean=0.167 fallback=0permille
```

The per-arm latency fields are omitted above. In the sandboxed run the tree arm's p50 was 365.7 µs and its p99 662.9 µs per call; the scripted and random arms' p50s were 32.1–38.6 µs. The extra time is per-candidate receipt encoding and hashing, not inference.

The decision is **no-go**:
- The tree route ties the grant-matched baseline on `found` (0.333), with twice its efficiency (0.167 against 0.083). It beats the random mean (0.167), but it does not reach the predeclared gain of 0.100 over the baseline.
- All 17 real manager briefs fit the declared projection and classes, and none fell back.
- The tree route took 110 tree splits against the linear scorer's 450 weight-feature products for the same 75 candidate decisions. That is a cost-unit comparison at equal selection budgets only.
- `signal-greedy` reaches 1.000 only because it may add offers, which the tree route's grant excludes.

The linear scorer's `--local-scorer` comparison still reports its archived decision: no-go, scorer 0.333, baseline 0.333, random mean 0.208. Its report gains the zero-valued `stale`, `abstained` and `leaf_receipts` fields.

## Checks

| Check | Result |
| --- | --- |
| `cargo test -p animus-frozen-policy --locked` | passed: `admission` 6, `decision` 4, `failure` 5, `receipt` 3 |
| `cargo test -p animus-campaign-manager-eval --locked` | passed: `artifact` 3, `catalog` 3, `classifier` 5, `comparison` 3, `grading` 1, `lane` 6, `report` 3, `scorer` 6 |
| `cargo clippy -p animus-frozen-policy -p animus-campaign-manager-eval --all-targets --locked -- -D warnings` (dev shell, rustc 1.96.1) | passed |
| `cargo check --workspace --all-targets --locked` (dev shell) | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` (dev shell) | passed |
| `cargo test --workspace --locked` (dev shell, `ANIMUS_RLM_HOST_PROCESS_FIXTURE` set to coreutils `printf` as in the flake's `cargo-test` check) | passed: 190 test binaries, 2,325 tests, 0 failed |
| `nix build .#checks.x86_64-linux.cargo-frozen-policy` | passed: tests, Clippy, and the `wasm32-unknown-unknown` check |
| `nix build .#checks.x86_64-linux.cargo-campaign-manager-eval` | passed: tests with `live`, Clippy with and without `live`, the scripted job, and both comparisons |
| `nix build .#checks.x86_64-linux.frozen-policy-octet` | `clean` |
| `nix build .#checks.x86_64-linux.campaign-manager-eval-octet` | `warning-only` with the same three findings as before this change, none in it: the binary's `std::env::args` read (two findings) and `unknown lint: ambient_env` in `animus-std` |
| `nix build .#checks.x86_64-linux.nickel-config` | passed: export parity, standalone export parity and negative cases |
| `nix build .#checks.x86_64-linux.cargo-fmt` | passed |
| Cairn `validate --root .` and the proposal, design and tasks gates | passed, 0 issues |

The `nix build` checks ran as `path:` flake references because the new files are untracked.

## Non-claims

- The tree is a hand-authored fixture: its result qualifies the route, the leaf and the artifact boundary, not a trained model or learning.
- Three synthetic tasks do not establish advice quality on real workloads or consumer improvement. ChaosControl #46 owns the real consumer measurement, and RSI Lab #80 the trained export.
- Classes, scores and ranks are neither task success nor authorization.
- Equal total compute, memory and model-load amortization were not measured.
- Production rollout stays conditional on a trained artifact and a consumer pilot.
