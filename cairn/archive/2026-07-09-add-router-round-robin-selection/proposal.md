# Proposal: Add router round-robin selection

## Why

Animus already has policy-bound router fallback and denies provider-side or ambient model-registry fallback. The missing behavior is explicit round-robin selection across equivalent eligible router candidates. Today `select_recipe` and execution-plan materialization pick the first eligible catalog entry in deterministic catalog order, so hosts that inject multiple equivalent local or remote models cannot distribute load without mutating catalog order outside the router contract and outside receipts.

Round-robin must be represented as a pure, auditable router policy feature rather than provider health fallback or live load balancing. Reusable crates should consume safe host-supplied cursor state, rotate only among already eligible candidates, return next-cursor metadata, and leave persistence, provider calls, health checks, credentials, and scheduling host-owned.

## What Changes

- Add typed router rotation policy and cursor DTOs for role candidate selection and fallback-chain materialization.
- Add a pure round-robin helper that rotates eligible catalog projections after role, backend, privacy, and budget filtering, while preserving candidate caps, quorum, recipe role order, and selected plan determinism.
- Materialize fallback chains from rotated eligible fallback entries only when policy explicitly enables fallback-role rotation; `evaluate_fallback` still consumes the materialized chain in order and continues to deny provider bypasses.
- Add safe rotation receipts containing policy labels, safe role labels, cursor class, candidate counts, catalog hash, and BLAKE3 identity without raw prompts, provider payloads, health data, credentials, or endpoints.
- Add positive and negative tests covering deterministic rotation, cursor wraparound, per-role isolation, fallback-chain rotation, invalid/stale cursor handling, backend/privacy filtering, and provider-bypass denial.

## Impact

- **Files**: `crates/animus-router` selection/execution-plan core, `animus-adapters::router::execution` receipt/envelope metadata, router docs, tests, and Cairn artifacts.
- **Testing**: focused `animus-router` round-robin tests, router execution adapter receipt tests, router policy conformance if policy labels change, Cairn validation, and Cairn gates.
- **Non-claims**: this does not add live provider clients, provider health checks, weighted load balancing, latency routing, credential lookup, persistent cursor storage, network fallback, or a default multi-provider runtime.
