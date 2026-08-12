# Proposal: Add a universal-weave branching adapter

## Why

Animus owns a neutral append-only session tree ledger, session branching plans, and redacted session-tree export. Those contracts are deliberately immutable: entries are appended, replayed, and projected, but node content is never edited, split, merged, or moved. Hosts that want Loom-style branching interfaces (interactive exploration of alternative continuations, editable branch text, DAG-shaped workspaces) have no reviewed Animus boundary for that mutable model today.

[`universal-weave`](https://github.com/transkatgirl/universal-weave) provides no_std-capable, Unlicense building blocks for exactly this model: tree and DAG weaves with activation, bookmarking, editing, splitting, merging, node moving, stable ordering, and untrusted-input validation. Importing it without a boundary would leak upstream trait surface, unbounded growth, and raw content into reusable Animus crates. Animus needs a pinned, non-default adapter crate that wraps the DAG weave behind Animus-owned DTOs, named bounds, and metadata-only receipts while the neutral session ledger remains the sole owner of replay and persistence semantics.

## What Changes

- Add a non-default `animus-adapter-weave` crate pinned to `universal-weave` `=0.4.1` with default features disabled (no rkyv, serde, or loro lanes). r[agent_harness_runtime.universal_weave.boundary]
- Wrap the DAG-based `IndependentWeave` over an Animus-owned text content type with validated add, edit, split, merge, move, bookmark, activation, and removal operations. r[agent_harness_runtime.universal_weave.adapter_core]
- Enforce a typed adapter policy with named bounds for node count, content size, and fan-in before any mutation runs. r[agent_harness_runtime.universal_weave.bounds]
- Project redacted topology snapshots and metadata-only BLAKE3 receipts that bind operation class, policy identity, and bounded counts without node text. r[agent_harness_runtime.universal_weave.evidence]
- Add positive and negative tests, a focused verification rail, component catalog registration, integration docs, and a bounded README reference. r[agent_harness_runtime.universal_weave.verification]

## Impact

- **New crate**: `crates/animus-adapter-weave` is no_std, unsafe-free, and optional; it is a workspace member but not a default member.
- **Root facade**: the root `animus` crate gains the non-default `weave-adapter` feature and a feature-gated re-export only.
- **Existing owners**: the neutral session ledger, session branching plans, replay, persistence, compaction, and session-tree export remain authoritative and unchanged.
- **Dependencies**: default, reusable, no-std, and Wasm-supported surfaces gain no new dependency; `universal-weave` appears only behind the opt-in crate.
- **Testing**: focused crate tests, a focused Cairn rail, component catalog projection, Wasm compile evidence for the opt-in crate, and workspace-wide checks.

## Out of Scope

- The tree-based `DependentWeave`, the `loro` CRDT collaboration lane, the `DeduplicatedWeave` and action-queue wrappers, and rkyv/serde serialization lanes.
- Replacing or weakening the append-only session ledger, replay, compaction, or branch-summary contracts.
- Host persistence, UI widgets, provider calls, filesystem access, clocks, or network behavior.
- Hash-collision-resistant content identity for weave internals; Animus receipts use domain-separated BLAKE3 over adapter-owned canonical fields instead.
- Proving upstream weave correctness, CRDT convergence, or serialization format stability.

## Affected Specs

- `agent-harness-runtime`: pinned optional boundary, pure adapter core, named bounds, safe evidence, and verification.
