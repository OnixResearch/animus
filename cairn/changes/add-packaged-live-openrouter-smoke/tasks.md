## Phase 1: Exact OpenRouter cohort

- [x] [serial] Admit the exact OpenRouter Responses endpoint pair while retaining the official OpenAI pair. r[agent_harness_runtime.packaged_live_openrouter_smoke.endpoint]
- [x] [depends:endpoint] Add cohort-driven pure planning and metadata-only evidence projection. r[agent_harness_runtime.packaged_live_openrouter_smoke.contract]
- [x] [depends:contract] Add the non-default binary with exact model input and bounded credential standard input. r[agent_harness_runtime.packaged_live_openrouter_smoke.binary]
- [x] [depends:binary] Add positive tests and negative endpoint-pair, input, and leak tests. r[agent_harness_runtime.packaged_live_openrouter_smoke.verification]

## Phase 2: Evidence and closeout

- [x] [depends:verification] Add docs and focused Rust and Nix rail coverage. r[agent_harness_runtime.packaged_live_openrouter_smoke.docs]
- [ ] [depends:docs] Run deterministic checks, sync the accepted spec, archive the change, commit, and run current-head live evidence with the Pi OpenRouter key. r[agent_harness_runtime.packaged_live_openrouter_smoke.closeout]
