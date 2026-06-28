# Design: Adapt Clankers cost-aware model routing for Animus

## Context

Cost-aware routing helps agents choose cheaper or stronger models while preserving operator budgets. Clankers implements this in product code with concrete provider catalogs and role names. Animus should adapt the reusable parts: typed signals, Nickel policy, pure decisions, injected catalogs, and safe receipts.

## Decisions

### 1. Routing policy is configuration

**Choice:** Role names, score thresholds, complexity weights, keyword/user-hint mappings, budget behavior, downgrade rules, provider constraints, and missing-price behavior live in Nickel-authored policy or checked fixtures.

**Rationale:** Routing thresholds and role names are not universal. They should be reviewable config, not hardcoded runtime constants.

### 2. Routing decisions are pure

**Choice:** A routing core accepts task signals, current usage/cost summaries, a model catalog projection, and routing policy, then returns a decision or typed failure.

**Rationale:** Model selection should be testable without providers, network calls, credentials, or clocks.

### 3. Provider catalogs are injected projections

**Choice:** Hosts provide model IDs, capabilities, price summaries, availability labels, and policy tags as safe catalog projections. Routing does not own provider discovery or health checks.

**Rationale:** Provider/router behavior belongs to host adapters and provider registry shells.

### 4. Budget status can force downgrade or denial

**Choice:** Policy chooses whether soft budgets bias toward cheaper roles, hard budgets force downgrade, or hard budgets deny model calls when no compliant model exists.

**Rationale:** Operators need predictable spend controls, and embedders may prefer denial over automatic downgrade.

### 5. Receipts explain without leaking prompts

**Choice:** Routing receipts record decision status, selected role/model label, score bucket, budget status, candidate counts, policy hash, and fallback class. Raw prompts, provider payloads, credentials, full pricing files, and hidden chain-of-thought are excluded.

**Rationale:** Routing is operationally important, but receipts should not become prompt transcripts or provider logs.

## Risks / Trade-offs

- Complexity scoring can become opaque. Keep explanations compact and bucketed.
- Missing prices can hide cost. Policy must explicitly choose deny, zero-cost-with-warning, or host-provided fallback.
- Routing overlaps provider registry work. Keep selection pure and let provider adapters execute selected model calls.
