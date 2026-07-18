## Why

Animus's host-control boundary has become a reusable protocol product rather than an agent-harness implementation detail. `crates/animus-adapters/src/hostcontrol/` contains about 1,700 lines of UI-neutral, `no_std`-compatible DTO validation for topology, selector lifetime, bounded output, declarative layout, events, waits, typed errors, feature negotiation, and transport envelopes.

Tile independently implements the same architectural contract in product-specific types:

- daemon-owned stable pane IDs plus revision-scoped positional references;
- workspace topology snapshots and snapshot-first event streams;
- bounded pane reads with finite line and byte budgets;
- finite output and agent-state waits;
- semantic agent states with explicit authority and freshness;
- local protocol commands, typed responses, and fail-closed unsupported surfaces.

Neither implementation should depend on the other product. Animus should not become Tile's protocol dependency, and Tile should not become the source of neutral agent-host contracts. A standalone repository can own the shared vocabulary and conformance rules while both products retain their shells, native IDs, policy, transport, PTYs, persistence, and UI behavior.

## What Changes

- Create `OnixResearch/host-control` as the canonical repository for transport-neutral host topology, observation, layout, event, wait, and envelope contracts.
- Extract a `host-control-core` functional core with no filesystem, process, network, clock, terminal, daemon, or persistence authority.
- Add optional wire adapters and a conformance corpus without requiring any socket server or serialization format in the core.
- Define explicit Animus and Tile mapping profiles, including status-vocabulary differences and stable-versus-live selector semantics.
- Pin Animus to one immutable standalone revision and preserve current `animus-adapters` imports through a compatibility facade during migration.
- Require a separate Tile Cairn change for its adapter and cutover; Tile daemon/protocol behavior does not move as an implicit side effect.

## Impact

- **New repository**: `OnixResearch/host-control`, with core DTOs, optional wire adapters, mapping profiles, conformance fixtures, and release evidence.
- **Animus**: host-control semantics leave the broad adapter crate; the existing public path remains a compatibility facade until a reviewed removal.
- **Tile**: becomes a second independent conformance target through a later Tile-owned change, while retaining daemon state, local socket protocol, PTYs, rendering, persistence, and policy.
- **Active Animus changes**: this package does not duplicate std host-adapter splitting or Preserves bridge splitting. Those changes keep authority and serialization adapters separate; this package extracts only the already UI/transport-neutral semantic core.
- **Claims**: conformance proves DTO mapping and bounded protocol behavior for selected fixtures only. It does not prove daemon correctness, transport security, process preservation, terminal truth, policy authorization, or whole-product readiness.
