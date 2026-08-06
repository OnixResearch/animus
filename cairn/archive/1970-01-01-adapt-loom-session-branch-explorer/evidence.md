## Baseline and references

Before implementation, existing branching tests passed (6), and the full `animus-tui` suite passed (147). Session-export filtering selected no tests under its historical module-name filter, with no failures.

The interaction reference is the Loom article. The MIT-licensed data-structure reference is `universal-weave` commit `4e793545dca359f4f1806c74efe395eefa5affbe`. Animus adapted map/read modes, active paths, bookmarks, partial presentation order, and sibling-equivalence concepts. It rejected the reference's mutable store, serialization, persistence, CRDT, destructive editing, and evolving active-state mechanisms. This change adds no reference runtime dependency.

## Ownership

The ledger owns parentage, storage order, replay, model context, tool correlation, active-leaf truth, and completion. Runtime projection owns only deterministic derived facts and typed plans. Annotations are sidecars. Hosts own reads, writes, compare-and-set mutation, isolation, clocks, transport, persistence, and authorization. The TUI reducer owns presentation state only.

## Evidence matrix

- Runtime positive tests cover one root, multiple leaves, active paths, bookmarks, partial sibling order, inspection, leaf activation, append-only internal-node fork, common-ancestor comparison, selected conversation binding, generation identity, and branch preservation.
- Runtime negative tests cover empty trees, duplicate IDs and sequences, missing and multiple parents, cycles, false leaves, stale revisions, unknown targets, repeated and cross-parent order, unsafe labels, bounds, missing external-state facts, missing isolation evidence, differing effects, deletion attempts, receipt leaks, and overclaims.
- Std tests cover strict generated policy parsing, malformed input, exact compare-and-set writes, stale writes, and mutation-free injected failure.
- TUI tests cover deterministic indicators, collapse, bounded focus, blocked activation, two-target comparison, refresh, stale host failure, incompatible projections, and host-port failure.
- The focused rail checks Nickel positive and negative fixtures, runtime/std/TUI suites, no-default root and runtime builds, Wasm runtime build, ambient-authority absence, and rejected dependency absence.

## Non-claims

This evidence does not prove transcript truth, effect rollback, world isolation, semantic quality, UI delivery, renderer correctness, accessibility, production readiness, or release eligibility.
