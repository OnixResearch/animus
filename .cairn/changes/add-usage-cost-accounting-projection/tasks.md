## Phase 1: Accounting contract and pure core

- [ ] [serial] Add typed usage-record admission and per-agent and per-session attribution bindings. r[agent_harness_runtime.usage_accounting.records]
- [ ] [serial] Add per-serving-model pricing with a documented fallback and explicit missing-price diagnostics. r[agent_harness_runtime.usage_accounting.pricing]
- [ ] [serial] Add the per-agent subtree projection with authenticated same-session creator scoping and session reset. r[agent_harness_runtime.usage_accounting.subtree]
- [ ] [serial] Add the explicit non-authoritative label to every projection. r[agent_harness_runtime.usage_accounting.non_authoritative]

## Phase 2: Fail-closed validation and receipts

- [ ] [serial] Add fail-closed validation for malformed records, unknown models, missing required prices, negative totals, overflow, and duplicate summaries. r[agent_harness_runtime.usage_accounting.fail_closed]
- [ ] [depends:contract] Add metadata-only receipts that reject raw usage payloads, provider bodies, credentials, and private paths. r[agent_harness_runtime.usage_accounting.receipts]

## Phase 3: Evidence

- [ ] [parallel] Add positive fixtures for attribution, pricing, subtree projection, fallback pricing, and session reset. r[agent_harness_runtime.usage_accounting.verification]
- [ ] [parallel] Add negative fixtures for malformed records, unknown models, missing required prices, negative totals, overflow, and receipt leak rejection. r[agent_harness_runtime.usage_accounting.fail_closed] r[agent_harness_runtime.usage_accounting.verification]
- [ ] [serial] Verify default builds remain independent of the accounting family. r[agent_harness_runtime.usage_accounting.verification]

## Phase 4: Documentation and closeout

- [ ] [parallel] Document the attribution rule, pricing rule, fallback, subtree scope, non-authoritative label, and non-claims. r[agent_harness_runtime.usage_accounting.pricing] r[agent_harness_runtime.usage_accounting.non_authoritative]
- [ ] [serial] Run focused no-std, Wasm, workspace, formatting, clippy, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.usage_accounting.verification]
