# Proposal: Plan model/provider dispatch before execution

## Why

Tool dispatch already has a strong pure planning boundary: definitions, arguments, output policy, mutation windows, and fail-closed readiness are evaluated before executor calls. Model/provider dispatch should have the same shape before live provider adapters, routers, streaming backends, or local model services execute.

This change adds a pure model/provider dispatch plan so runtime can validate provider capability, request shape, prompt/output budgets, stream policy, receipt policy, and fallback denial before adapter execution.

## What Changes

- Define a pure `ModelDispatchPlan` or equivalent DTO that admits or denies model/provider dispatch before calling a concrete adapter.
- Include provider/model capability identity, request contract identity, prompt/content budget, streaming/thought visibility policy, receipt policy, cancellation/timeout posture, and no-fallback evidence.
- Add runtime shell wiring so model dispatch executes only after an admitted plan.
- Add fixture tests for valid model dispatch and fail-closed missing provider, denied capability, budget overflow, unsupported stream mode, malformed request shape, cancellation, timeout, and fallback attempts.

## Impact

- **Files**: `animus-adapters` model/provider planning DTOs, runtime shell dispatch, std fixtures, streaming tests, docs, facade inventory, and Cairn evidence.
- **Testing**: pure planning positive/negative tests plus runtime no-dispatch-on-denial tests.
- **Non-claims**: this does not add live provider clients, OAuth, credential lookup, routers, network transport, or provider discovery.
