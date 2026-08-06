# Session branch explorer

Animus derives one bounded branch map from one validated ledger snapshot. The ledger remains the only model-context, replay-order, parentage, tool-correlation, and completion authority.

## Contracts

`animus_runtime::branch_explorer` validates one replay parent, topology, exact revisions, active paths, true leaves, bounds, annotations, external-state posture, comparisons, generation identities, and metadata-only receipts. It performs no I/O.

Bookmarks, labels, and sibling presentation order are sidecars. Partial order lists named siblings first and retains ledger order for omitted siblings. These annotations never change storage sequence or replay.

Inspection can target any admitted node. Activation can target only a current leaf. Fork planning can target any node and emits `SessionBranchOperation::ForkFromMessage`; it never requests edit-in-place, deletion, merge, reparenting, or suffix removal.

A continuation is independent only when the caller supplies `Isolated` posture and a separate isolation-evidence identity. `SharedWorld` and `Unknown` never imply rollback.

Comparison reports one exact common ancestor and bounded ordered suffix metadata. Equivalence requires equal complete generation identities. Both sibling leaves remain present.

## Hosts and TUI

`animus_std::SessionBranchMemoryHost` is a non-default fixture. Real hosts own ledger loading, annotation persistence, compare-and-set active-leaf writes, fork dispatch, durable receipts, clocks, transport, and authorization.

`animus_tui::branch_explorer` provides a pure map/read reducer, deterministic metadata-only text snapshots, and one typed host port. Tile, browser, remote delivery, and accessibility integration remain consumer work.

The selected read view binds the branch session, ledger generation, selected leaf, and canonical conversation projection identity. Render caches have no reverse authority.

## Reference boundary

The interaction reference is the Loom article. The data-structure reference is MIT-licensed `universal-weave` commit `4e793545dca359f4f1806c74efe395eefa5affbe`.

Animus adapts map/read modes, active paths, bookmarks, presentation order, and sibling-equivalence concepts. It does not add a `universal-weave`, Loro, `rkyv`, serialization, CRDT, or mutable-document runtime dependency for this explorer.

## Checks

```sh
scripts/cairn/check-session-branch-explorer --root .
```

## Non-claims

This contract does not prove transcript truth, effect rollback, world isolation, semantic quality, UI delivery, renderer correctness, accessibility, production readiness, or release eligibility.
