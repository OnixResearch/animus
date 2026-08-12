# Proposal: Add the Supra-Router signal adapter

## Why

The `project-router-request-signals` change defines a `SignalSource` adapter trait and a deterministic rule-based default projector. The local Supra-Router-51M model (per the `supra-router` skill) classifies a prompt's domain, complexity, math/code content, and route decision before choosing how to handle a task — a natural learned classifier for the eight router signal bands. Animus should offer an optional, non-default `SignalSource` adapter that consults Supra-Router, while keeping the rule projector as the routing-correctness default so routing never depends on the classifier.

## What Changes

- Add an optional `SupraRouterSignalSource` in `animus-std` (behind a non-default `supra-router` feature) that implements the `SignalSource` trait by calling the Supra-Router endpoint (default `127.0.0.1:13306`) and mapping its classification output to a `SignalDraft`.
- Keep the adapter host-owned and fail-closed: a missing, unavailable, timed-out, cancelled, or malformed Supra-Router response falls back to the rule projector (or denies, per policy) rather than blocking routing.
- Add a Nickel policy hook for the Supra-Router endpoint, timeout class, and fallback posture.
- Add positive tests with a deterministic Supra-Router stub and negative tests for unavailable endpoint, malformed classification, timeout, and policy-deny fallback.

## Impact

- **Files**: `crates/animus-std` supra-router adapter module and feature, Nickel policy hook, std tests, facade inventory, and Cairn evidence.
- **Testing**: positive mapping fixtures via a stub; negative tests for unavailable/malformed/timeout/cancelled responses and policy-deny fallback.
- **Non-claims**: this does not make routing correctness depend on Supra-Router, vendor a serving stack, expose raw classifier traces, or enable the adapter by default; the rule projector remains the default `SignalSource`.
