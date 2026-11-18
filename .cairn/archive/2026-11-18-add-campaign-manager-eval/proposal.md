## Why

The campaign manager (`add-campaign-manager`) has tests only for its own mechanics: replies, proposals, refusals, and publication with a scripted model. No test measures whether a manager steers a campaign to what it is looking for, or compares one model route against another. Animus already has a Harbor-compatible evaluation slice in `animus_runtime::harbor_eval`, `animus_adapters::harbor`, and `animus_std::harbor`, adapted from Harbor `v0.18.0` with ATIF `v1.7`. It provides trial planning, environment and verifier ports, rewards, ATIF projections, and receipts. The manager should be evaluated through that slice, not through Harbor's Python application.

## What Changes

- Add the optional crate `animus-campaign-manager-eval`, licensed `AGPL-3.0-or-later` because it links Campaign. It stays outside the default members and has an opt-in `live` feature. r[agent_harness_runtime.campaign_manager_eval.boundary]
- Add a typed Nickel task set, `config/campaign-manager-eval-tasks.ncl`, exported and checked through the standalone Nickel export pipeline and revalidated in Rust. Each task is one exploration campaign: candidates and offers hide a signal, and exactly one of them is the target that records a finding. r[agent_harness_runtime.campaign_manager_eval.tasks]
- Add the Harbor environment service. A trial opens a real Campaign campaign on in-memory Choregraph history and runs up to the task's turns. Each turn is one manager turn followed by one host planner step. The host publishes the selection, runs it against the hidden outcome once the publication fence releases it, and publishes the result, so the model learns a signal only from the label the host writes. r[agent_harness_runtime.campaign_manager_eval.trial]
- Add the separate verifier. It reloads the published control projection from the trial's Choregraph history and grades the replayed state: `found` and `efficiency` from Campaign's replay, and the advisory `refusal-rate` from the environment's ledger. It refuses missing, stale, or foreign histories. r[agent_harness_runtime.campaign_manager_eval.verifier]
- Project each trial as a validated ATIF-v1.7 trajectory of metadata only. r[agent_harness_runtime.campaign_manager_eval.trajectory]
- Run jobs through `plan_trials`, `execute_separate_verifier`, `reduce_rewards`, and `receipt`. Per-route summaries compare model routes on the same tasks. r[agent_harness_runtime.campaign_manager_eval.job]
- Add the `animus-campaign-manager-eval` binary. Its report is metadata only: identities, counts, and mean rewards, never prompts, replies, labels, or credentials. r[agent_harness_runtime.campaign_manager_eval.report]
- Add deterministic scripted routes (`signal-greedy`, `explore-only`, `malformed`). Under the `live` feature, add an opt-in route to a reviewed OpenAI-compatible Responses endpoint, with an explicit model and a credential read from standard input. r[agent_harness_runtime.campaign_manager_eval.live]
- Add positive and negative tests and the focused Nix checks `cargo-campaign-manager-eval` and `campaign-manager-eval-octet`. r[agent_harness_runtime.campaign_manager_eval.verification]

## Impact

- **Rust**: new workspace member `crates/animus-campaign-manager-eval`, not in the default members. It uses `animus-runtime` and `animus-std` with only `harbor-eval`, and `animus-std-openai` only under `live`.
- **Config**: the new Nickel source, its generated JSON, its standalone export request and manifest, and their registration in `scripts/config/*` and `config/nickel-export-requests.ncl`.
- **Nix**: the checks `cargo-campaign-manager-eval` (tests with `live`, Clippy with and without `live`, and the scripted job) and `campaign-manager-eval-octet` (Octet deny-all with `live`), plus one `diff` line in `check-nickel-configs`.
- **Non-claims**:
  - Scores hold only for these synthetic tasks. They do not measure advice quality on real workloads or benchmark superiority.
  - The verifier is sound only as far as Campaign's history replay goes.
  - The lane does not establish Harbor Hub, cloud, or container parity, or production or release readiness.
  - A live run contacts the provider only when invoked explicitly, and its scores are not deterministic.
