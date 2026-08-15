## Phase 1: Accounting contract and pure core

- [x] [serial] Add typed usage-record admission and per-agent and per-session attribution bindings. r[agent_harness_runtime.usage_accounting.records]
- [x] [serial] Add per-serving-model pricing with a documented fallback and explicit missing-price diagnostics. r[agent_harness_runtime.usage_accounting.pricing]
- [x] [serial] Add the per-agent subtree projection with authenticated same-session creator scoping and session reset. r[agent_harness_runtime.usage_accounting.subtree]
- [x] [serial] Add the explicit non-authoritative label to every projection. r[agent_harness_runtime.usage_accounting.non_authoritative]

## Phase 2: Fail-closed validation and receipts

- [x] [serial] Add fail-closed validation for malformed records, unknown models, missing required prices, negative totals, overflow, and duplicate summaries. r[agent_harness_runtime.usage_accounting.fail_closed]
- [x] [depends:contract] Add metadata-only receipts that reject raw usage payloads, provider bodies, credentials, and private paths. r[agent_harness_runtime.usage_accounting.receipts]

## Phase 3: Evidence

- [x] [parallel] Add positive fixtures for attribution, pricing, subtree projection, fallback pricing, and session reset. r[agent_harness_runtime.usage_accounting.verification]
- [x] [parallel] Add negative fixtures for malformed records, unknown models, missing required prices, negative totals, overflow, and receipt leak rejection. r[agent_harness_runtime.usage_accounting.fail_closed] r[agent_harness_runtime.usage_accounting.verification]
- [x] [serial] Verify default builds remain independent of the accounting family. r[agent_harness_runtime.usage_accounting.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document the attribution rule, pricing rule, fallback, subtree scope, non-authoritative label, and non-claims. r[agent_harness_runtime.usage_accounting.pricing] r[agent_harness_runtime.usage_accounting.non_authoritative]
- [x] [serial] Run focused no-std, Wasm, workspace, formatting, clippy, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.usage_accounting.verification]
