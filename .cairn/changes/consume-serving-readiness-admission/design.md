## Context

Baseline: Animus `365b0ee6`. `animus_adapters::mesh` already normalizes catalogs, admits one exact model, and projects metadata-only receipts; `animus_std::mesh::execute` sends one admitted request through an injected transport. `animus-router` selects recipes and rotates candidates over a caller-supplied catalog. Lattice (b858d85b) binds compute provenance and `lattice-model-route-v1` route identities but reports model requests as `unknown` (owner `animus#17`).

## Decisions

### Decision: Pure core in `animus_adapters::serving`

**Choice:** A `no_std` module beside `mesh`, reachable from Lattice's existing `animus-adapters` dependency. Submodules `basis`, `facts`, `policy`, `eligibility`, `predispatch`, `ledger`, `version`. No clock, network, credential, or probe; every time value is a host-supplied logical tick.

**Rationale:** Lattice already vendors `animus-adapters`; no new consumer dependency.

### Decision: Eligibility before preference

**Choice:** `screen_routes` filters named route candidates (readiness, exact model and configuration, claimed versus observed identity, privacy, capability, envelope, rollout, owner admission offer, capacity evidence) and returns `eligible_catalog()` for the unchanged `select_recipe` / `select_recipe_with_rotation`. `rank_eligible` applies a learned preference only to the eligible set.

**Rationale:** Existing routing, rotation, budget, and fallback stay authoritative; no ineligible route reaches ranking.

### Decision: Capacity is evidence, never a reservation

**Choice:** Only `request-slots`, `kv-cache-tokens`, `request-quota`, and `token-quota` from the serving owner or provider API count. `free-memory`, `queue-depth`, and host probes never do. Every capacity-aware profile requires serving-owner admission; `gate_dispatch` admits only a request-scoped grant that matches the request, deployment generation, model, configuration, unexpired owner tick, current admission epoch, and replica rule, confirmed by a current recheck.

### Decision: Receipts keyed to the caller's route identity

**Choice:** `RouteCandidateDraft.route_ref` carries an opaque caller-owned `blake3:` identity (Lattice `lattice-model-route-v1`) unchanged onto the receipt with usage provenance (`provider-reported` or `unknown`), deployment, model, admission, replica, outcome class, and the existing Mesh receipt identity. Identity domain `animus-serving-receipt-v1`.

### Decision: Rollout requirements are opt-in

**Choice:** Only candidates that name a `RolloutRequirement` need matching base model, tokenizer, adapter, observed loaded policy generation, probability semantics, operations, and a qualification bound to the served configuration. Grants, rechecks, and responses must carry the same generation; missing or crossed response generations are propagated, never rewritten.

## Risks / Trade-offs

- Usage counters come from the injected transport (`ResponseFacts`); the core cannot verify provider honesty.
- `TransportOutcome` gains a required field; the only in-repo caller is the Mesh test transport.
- Live endpoint qualification was not performed; all evidence is fixture-labelled.
