# Tasks

## Phase 1: Implementation

- [x] [serial] Add a non-default `animus-dogfood-smoke` binary behind the `animus-std/dogfood-smoke` feature. r[agent_harness_runtime.packaged_headless_dogfood_smoke.cli]
- [x] [depends:agent_harness_runtime.packaged_headless_dogfood_smoke.cli] Parse bounded CLI prompt/config input into typed fake-backed dogfood DTOs before dispatch. r[agent_harness_runtime.packaged_headless_dogfood_smoke.admission]
- [x] [depends:agent_harness_runtime.packaged_headless_dogfood_smoke.admission] Render safe line-oriented packaged smoke evidence with packaged/core rungs, event counts, BLAKE3 identities, ordered events, and omitted-surface non-claims. r[agent_harness_runtime.packaged_headless_dogfood_smoke.receipt]
- [x] [depends:agent_harness_runtime.packaged_headless_dogfood_smoke.receipt] Add positive and negative tests plus a focused rail for missing prompt, missing service, denied capability, malformed output, unsafe labels, and raw-prompt leak checks. r[agent_harness_runtime.packaged_headless_dogfood_smoke.verification]
- [x] [depends:agent_harness_runtime.packaged_headless_dogfood_smoke.verification] Update docs and dogfood claim fixtures to name the packaged headless smoke rung and keep omitted optional surfaces as non-claims. r[agent_harness_runtime.packaged_headless_dogfood_smoke.docs]

## Phase 2: Validation

- [x] [serial] Verify with focused Cargo, dogfood readiness, packaged smoke, and Cairn gates before archive. r[agent_harness_runtime.packaged_headless_dogfood_smoke.verification]
