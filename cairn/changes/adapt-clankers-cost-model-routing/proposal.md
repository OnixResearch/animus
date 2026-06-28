# Proposal: Adapt Clankers cost-aware model routing for Animus

## Why

Clankers has a practical model-selection pattern: compute task complexity signals, account for per-model cost, enforce soft and hard budgets, and explain why a model role was selected. Animus should adopt that as a pure policy contract so host runtimes can choose providers/models without hiding cost or routing decisions inside provider adapters.

Animus should not copy Clankers' hardcoded role names, thresholds, or provider catalog behavior. Nickel policy and injected model catalogs should own routing roles, prices, budget limits, and fallback behavior.

## What Changes

- Define model-routing DTOs for task signals, available model roles, cost/pricing summaries, budget state, routing decision, fallback reason, and safe receipts.
- Add Nickel-authored routing policy for role names, thresholds, complexity weights, budget behavior, downgrade rules, provider constraints, and missing-price behavior.
- Implement pure routing decision logic over injected model catalogs and current usage/cost summaries.
- Emit safe routing receipts that explain selected role/model, score bucket, budget status, and fallback class without raw prompts, provider payloads, credentials, or full pricing-store dumps.
- Add positive and negative fixtures for simple/complex tasks, explicit user hints, soft-budget pressure, hard-budget downgrade, missing prices, unavailable models, malformed policy, and receipt redaction.

## Impact

- **Files**: future routing policy Nickel, model-routing DTOs, pure selection logic, tests, docs, and Cairn artifacts.
- **Testing**: positive routing/budget fixtures and negative malformed-policy/missing-model/missing-price/leak fixtures.
- **Non-claims**: this does not implement a provider router, OAuth, live provider health checks, or Clankers' exact model role names.
