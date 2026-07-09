# Proposal: Add nested subagent delegation graph contracts

## Why

Animus already has a unified subagent facade, context packets, coordinator services, fan-out/fan-in reducers, and safe lineage records. Those contracts cover one parent delegating to children, but host products will need deeper agent trees: a child may need to request a narrower grandchild investigation or verification task.

Animus should support that as explicit delegation graph data, not as ambient recursive spawning. Each edge must be admitted by the host, every descendant must receive no more authority than its ancestors, and reusable runtime logic must stay pure and non-executing.

## What Changes

- Define nested delegation graph DTOs for nodes, edges, ancestry, depth policy, authority ceilings, budget inheritance, cancellation propagation, and graph receipts.
- Validate graph admission with cycle detection, max-depth enforcement, monotonic capability/budget checks, and safe parent/child lineage identity.
- Extend fan-in and lineage concepts to multi-level aggregation without importing raw child or grandchild transcripts.
- Keep scheduling, spawning, provider calls, process execution, workspace mutation, persistence, UI tree rendering, and retry policy host-owned.

## Impact

- **Files**: future subagent graph DTOs/reducers, docs, tests, conformance fixtures, and facade inventory updates.
- **Testing**: positive two-level delegation graph and cancellation/fan-in cases plus negative cycle, over-depth, capability widening, budget inflation, missing host admission, recursive self-spawn, stale lineage, and receipt leak fixtures.
- **Non-claims**: this does not add automatic recursive spawning, default subagent services, built-in agent workers, provider calls, background schedulers, UI controls, or raw transcript sharing.
