# Tasks

- [ ] [serial] Add Nickel routing policy contracts or checked fixtures for role names, thresholds, complexity weights, user-hint mappings, budget behavior, downgrade rules, provider constraints, and missing-price behavior. r[agent_harness_runtime.clankers_cost_model_routing.policy]
- [ ] [depends:policy] Add invalid-policy tests for duplicate roles, negative weights, unordered thresholds, unknown downgrade targets, and unspecified missing-price behavior. r[agent_harness_runtime.clankers_cost_model_routing.policy.invalid]
- [ ] [serial] Define pure routing decision DTOs over task signals, usage/cost summaries, safe catalog projections, and validated policy. r[agent_harness_runtime.clankers_cost_model_routing.decision]
- [ ] [depends:decision] Add simple, medium, complex, and explicit user-hint decision tests proving routing sends no provider request. r[agent_harness_runtime.clankers_cost_model_routing.decision.complex]
- [ ] [serial] Implement budget-aware decision behavior for soft-budget pressure, hard-budget downgrade, and hard-budget denial. r[agent_harness_runtime.clankers_cost_model_routing.budget]
- [ ] [depends:budget] Add hard-budget tests proving cost upgrades are downgraded or denied with explicit reason metadata. r[agent_harness_runtime.clankers_cost_model_routing.budget.hard_limit]
- [ ] [serial] Define injected safe model catalog projection shape with model labels, capabilities, prices, availability, and policy tags. r[agent_harness_runtime.clankers_cost_model_routing.catalog]
- [ ] [depends:catalog] Add missing-price and unavailable-model tests according to active policy behavior. r[agent_harness_runtime.clankers_cost_model_routing.catalog.missing_price]
- [ ] [serial] Define safe routing receipts with status, selected role/model label, score bucket, budget status, candidate counts, policy hash, fallback class, and BLAKE3 receipt hash. r[agent_harness_runtime.clankers_cost_model_routing.receipts]
- [ ] [depends:receipts] Add leak fixtures for raw prompts, provider JSON, credentials, full pricing stores, and hidden reasoning text. r[agent_harness_runtime.clankers_cost_model_routing.receipts.rejects_leaks]
- [ ] [depends:policy.invalid,decision.complex,budget.hard_limit,catalog.missing_price,receipts.rejects_leaks] Update docs and run focused routing tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_cost_model_routing.verification]
