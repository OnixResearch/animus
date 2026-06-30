# vLLM Semantic Router recipe adaptation

Animus adapts the vLLM Semantic Router micro-agent recipe pattern as typed, policy-governed model-router contracts. The adaptation keeps vLLM as a bounded reference only: Animus does not vendor vLLM, expose a serving proxy, copy benchmark claims, or let router collaboration bypass Rust adapter validation, provider policy, budgets, safe receipts, or injected model services.

## Boundary

`animus-adapters` owns reusable no-std DTOs for:

- route families: `single`, `confidence`, `ratings`, `remom`, `fusion`, and `workflow`;
- bounded request-signal projection over task shape, risk, contract pressure, context pressure, latency, cost, privacy/locality, and fallback eligibility;
- deterministic recipe selection before provider dispatch;
- hard execution bounds for candidates, parallelism, quorum, timeout class, synthesis, repair, and fallback policy;
- final output-contract result states for success, malformed output, and best-valid-evidence fallback;
- safe router receipts with recipe ID, signal buckets, safe model labels, counts, quorum/fallback classes, policy hash, duration bucket, redaction markers, and deterministic BLAKE3 identity.

Hosts remain responsible for concrete provider calls, concurrency, clocks, credentials, network clients, cancellation plumbing, and live router policy loading. Reusable runtime/core code should consume selected plans through injected model/provider services.

## Safety

Router receipts and leak checks reject raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, and unbounded traces. Signal projection accepts only bounded labels and does not require raw prompt bodies or provider payloads.

## Verification

Run focused checks:

```sh
cargo test -p animus-adapters --locked model_router::
cargo test -p animus-adapters --locked
cargo clippy -p animus-adapters --all-targets --locked -- -D warnings
```
