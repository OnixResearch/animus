## Why

Animus already owns a typed append-only session tree and active-path replay. It does not expose one complete branch-explorer contract across all admitted leaves.

The active `add-canonical-conversation-view-projection` change covers one selected branch. Hosts still need safe topology, annotation, comparison, and branch-selection facts around that view.

Loom demonstrates useful read and map views for branchable model sessions. `universal-weave` demonstrates dependent trees, active paths, bookmarks, stable ordering, and duplicate-sibling detection.

Animus will adapt these interaction patterns through existing ledger identities. It will not add `universal-weave` as a dependency or create another mutable transcript store.

## What Changes

- Add a bounded redaction-first session branch projection over one validated Animus ledger snapshot. r[agent_harness_runtime.session_branch_explorer.projection]
- Keep conversation replay as a strict single-parent tree. Keep evidence and synthesis references in separate sidecars. r[agent_harness_runtime.session_branch_explorer.boundary]
- Add typed bookmarks, safe labels, and sibling presentation order without changing ledger or replay order. r[agent_harness_runtime.session_branch_explorer.annotations]
- Add freshness-fenced inspect, activate-leaf, and fork-from-node plans. r[agent_harness_runtime.session_branch_explorer.selection]
- Add deterministic branch comparison over common ancestors, suffix identities, safe counts, usage summaries, and receipt references. r[agent_harness_runtime.session_branch_explorer.comparison]
- Add exact sibling-equivalence facts that never delete, merge, or rewrite source entries. r[agent_harness_runtime.session_branch_explorer.equivalence]
- Add an `animus-tui` reference explorer with read and map views over pure reducer state. r[agent_harness_runtime.session_branch_explorer.tui]
- Add metadata-only receipts and positive and negative conformance evidence. r[agent_harness_runtime.session_branch_explorer.evidence] r[agent_harness_runtime.session_branch_explorer.verification]

## Success Criteria

- The map view and selected read view derive from the same admitted ledger revision.
- Bookmarks and presentation order never change replay context or canonical storage order.
- Activating a stale, unknown, or non-leaf target fails before host mutation.
- Forking from an internal node appends a new lineage and preserves the original branch.
- Branch comparison reports exact common-ancestor and suffix facts without claiming semantic quality.
- Equal responses can share an equivalence annotation without deleting either branch.
- Branch selection never claims rollback of prior filesystem, network, provider, or tool effects.

## False Completion

A tree widget over unvalidated rows is not a branch explorer. A second mutable transcript store is also false completion.

Content-based deletion, timestamp identity, silent reparenting, destructive split or merge, and raw transcript receipts do not satisfy this change.

A transcript fork does not prove an external-world fork. Mutating continuations need separate admitted isolation and effect evidence.

## Impact

- **Reusable contracts**: pure branch projection, annotations, selection, comparison, equivalence, diagnostics, and receipt logic.
- **Host shells**: ledger loading, annotation persistence, active-leaf mutation, continuation dispatch, clocks, transport, and durable writes.
- **Animus TUI**: reference read and map views without ledger or tool authority.
- **Future consumers**: Tile and browser hosts can consume the stable projection through separate consumer changes.
- **Testing**: deterministic topology, stale selection, ordering, annotation, comparison, equivalence, redaction, receipt, and TUI fixtures.

## Out of Scope

- A `universal-weave`, `rkyv`, or Loro runtime dependency.
- Destructive edit-in-place, node removal, reparenting, split, merge-with-parent, or CRDT synchronization.
- Weighted stochastic walks, automated branch generation campaigns, branch scoring policy, or winner selection.
- Tool-effect rollback, filesystem cloning, provider-session cloning, or credential copying.
- Tile integration, browser layout, remote delivery, accessibility, or production-readiness claims.

## Affected Specs

- `agent-harness-runtime`: branch-explorer boundary, projection, annotations, selection, comparison, equivalence, TUI, evidence, and verification.
