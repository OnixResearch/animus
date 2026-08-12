## Phase 1: Lifecycle and contract

- [x] [serial] Add the roadmap classification and exact claim-binding requirements. r[agent_harness_runtime.dogfood_readiness.current_head_binding]

## Phase 2: Pure validation core

- [x] [depends:agent_harness_runtime.dogfood_readiness.current_head_binding] Extend the release evidence checker, generator, and Nickel schema with the bounded packaged-headless dogfood claim. r[agent_harness_runtime.dogfood_readiness.current_head_binding]
- [x] [depends:agent_harness_runtime.dogfood_readiness.current_head_binding] Add positive, missing, stale, duplicate, conflicting, and dirty fixtures for the claim-to-receipt binding. r[agent_harness_runtime.dogfood_readiness.current_head_binding.fixtures]

## Phase 3: Imperative shell and docs

- [x] [depends:agent_harness_runtime.dogfood_readiness.current_head_binding.fixtures] Compose release-index validation into the dogfood readiness shell and Nix rail. r[agent_harness_runtime.dogfood_readiness.current_head_binding.composed_rail]
- [x] [depends:agent_harness_runtime.dogfood_readiness.current_head_binding.composed_rail] Update readiness, release-evidence, focused-rail, roadmap, and verification docs with the new bounded claim. r[agent_harness_runtime.dogfood_readiness.current_head_binding.docs]

## Phase 4: Validation and closeout

- [x] [depends:agent_harness_runtime.dogfood_readiness.current_head_binding.docs] Run focused positive and negative rails, Cairn validation and gates, sync the accepted spec, and archive the change. r[agent_harness_runtime.dogfood_readiness.current_head_binding.verification]
