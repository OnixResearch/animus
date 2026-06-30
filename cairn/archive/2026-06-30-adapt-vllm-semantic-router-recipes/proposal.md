# Proposal: Adapt vLLM Semantic Router recipes for Animus

## Why

The vLLM Semantic Router micro-agent article frames routers as an active serving layer: one stable model surface can select bounded recipes, fan out to workers, collect quorum evidence, synthesize or repair an answer, and return one normal response. Animus already has typed adapter boundaries, injected services, Steel loop fabric, cost-aware routing work, workflow recipes, and safe receipts. It should adapt the router-side recipe pattern as Animus-owned contracts instead of leaving multi-model collaboration as ad hoc product glue or freeform agent teams.

The goal is a bounded model-router capability: deterministic signal projection chooses an explicit recipe, the recipe runs only through injected model/provider services under policy budgets, output contracts are revalidated before success, and observability exposes safe metadata rather than raw traces.

## What Changes

- Define model-router recipe contracts for single-route, confidence escalation, ratings ensemble, ReMoM-style quorum synthesis, fusion judge/finalizer, and bounded role workflow patterns.
- Define pure request-signal projection for task shape, risk, contract pressure, context pressure, latency band, cost band, privacy/locality, and fallback eligibility.
- Define recipe selection as deterministic policy over signals, model catalog projections, budgets, and allowed backends.
- Define bounded execution plans with max candidates, max parallelism, min-success quorum, max synthesis attempts, timeouts, cancellation, and failure/fallback policy.
- Require final response validation and optional repair before a collaborative route can claim success.
- Emit safe route receipts and audit labels with recipe ID, selected model labels, counts, quorum/fallback classes, policy hash, and redaction markers.

## Impact

- **Files**: future router-recipe DTOs, Nickel policy/contracts, pure signal projection and recipe-selection helpers, execution-plan DTOs, response-contract validation hooks, safe receipts, docs, and tests.
- **Testing**: positive selection/execution-plan fixtures for each recipe family and negative tests for unbounded fan-out, missing quorum, invalid fallback, unsafe model eligibility, malformed output contracts, and receipt leaks.
- **Non-claims**: this does not vendor vLLM, implement an OpenAI-compatible serving proxy, copy vLLM benchmark claims, expose raw multi-agent traces, or let Steel scripts bypass Rust validation, provider policy, budgets, receipts, or injected services.
