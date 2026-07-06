# Design: Add the Supra-Router signal adapter

## Context

`SignalSource` is the pluggable classifier seam introduced by `project-router-request-signals`. A learned classifier can produce better band assignments than static thresholds for ambiguous requests. Supra-Router-51M already runs locally and classifies prompt domain, complexity, math/code content, and route decision. Wrapping it as a `SignalSource` gives Animus an optional learned classifier without making routing depend on it.

## Decisions

### 1. The adapter lives in animus-std behind a non-default feature

**Choice:** `SupraRouterSignalSource` is implemented in `crates/animus-std` behind a `supra-router` Cargo feature, off by default. It depends on a host HTTP client (std-only), not on `animus-router` or `animus-runtime` core.

**Rationale:** Keeps the classifier — a host-owned, network-touching component — out of the no-std router core and the Wasm surface, matching the accepted `baseline.adapters` and `provider_boundary` requirements.

### 2. Classification output maps to SignalDraft bands

**Choice:** The adapter maps Supra-Router's classification fields (domain, complexity, math/code content, route decision) to the eight `SignalDraft` bands via explicit named mapping rules. Task shape derives from domain + code content; risk from complexity; contract/context pressure from complexity and route decision; latency/cost from route decision; privacy from a host-supplied policy label; fallback from route decision.

**Rationale:** The mapping is explicit and auditable; no magic-number thresholds beyond Supra-Router's own output.

### 3. Fail-closed with rule-projector fallback

**Choice:** When Supra-Router is missing, unavailable, timed out, cancelled, or returns malformed output, the adapter falls back to the rule-based `RuleSignalSource` (or denies, per Nickel policy). It never blocks routing on classifier availability.

**Rationale:** Routing correctness must not depend on a classifier. The accepted `model_dispatch` and `clankers_injected_services.fail_closed` requirements demand typed unavailable-service outcomes; this adapter surfaces them and degrades gracefully.

### 4. Policy owns endpoint, timeout, and fallback posture

**Choice:** A Nickel policy hook configures the Supra-Router endpoint (default `127.0.0.1:13306`), timeout class, and fallback posture (rule-projector vs. deny). The adapter reads the resolved policy, not ambient config.

**Rationale:** Matches the repo's Nickel-as-config-source-of-truth rule and keeps endpoint/timeout values named rather than magic numbers.

## Risks / Trade-offs

- A learned classifier can drift. Mitigate by keeping routing behavior reproducible via the rule projector default and by recording which `SignalSource` produced the bands in the receipt.
- Network calls in tests are fragile. Use a deterministic stub `SupraRouterClient` trait so tests never touch the network.
- The adapter could be mistaken for a router. Document clearly that it only produces `SignalDraft` bands; recipe selection stays pure in `animus-router`.
