# Proposal: Define model-router execution service contracts

## Why

Animus already has pure model-router decisions: cost-aware single-model selection, recipe selection, materialized `RouterExecutionPlan`s, budget/fallback evaluation, and safe router receipts. What is missing is the boundary that tells an agent built from Animus how to execute those plans without moving provider authority into reusable Animus crates.

Without that seam, multi-model behavior is ambiguous: a host might treat router plans as direct permission to discover providers, fan out to arbitrary models, retry outside budget, or persist raw worker outputs. This change makes the intended split explicit: Animus owns deterministic plans and bounded evidence; host agents own concrete provider clients, credentials, concurrency, telemetry, and synthesis execution.

## What Changes

- Define a host-owned model-router execution service contract that consumes validated router plans and returns typed bounded outcomes.
- Define dispatch and outcome envelope rules for role steps, candidate labels, quorum, cancellation, timeout, repair, fallback, and final output validation.
- Require same-request multi-model fan-out to be authorized by a materialized plan, hard parallelism bounds, quorum, and budget state.
- Require router execution receipts to contain safe labels, counts, hashes, redaction markers, and outcome classes only.
- Keep concrete provider clients, credential lookup, model discovery, network I/O, latency telemetry, and live concurrent execution outside reusable Animus defaults.

## Impact

- **Files**: future `animus-runtime` or adapter service DTO modules, `animus-router` outcome/receipt extensions only when data-only, focused tests, docs, facade inventory, and conformance rails.
- **Testing**: positive single-route, confidence, ratings, ReMoM, fusion, fallback, and deterministic fixture-executor cases; negative missing service, ambient provider discovery, over-parallel fan-out, impossible quorum, unsafe receipt, unbounded output, and fallback bypass cases.
- **Non-claims**: this does not implement live OpenAI/Anthropic/local provider clients, provider registry discovery, credential storage, network transport, production latency routing, automatic health checks, or a default multi-provider agent runtime.
