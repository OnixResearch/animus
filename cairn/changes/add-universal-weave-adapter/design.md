# Design: Add a universal-weave branching adapter

## Context

`universal-weave` `0.4.1` ships two weave implementations. `DependentWeave` is a tree where each node depends on its parent's contents. `IndependentWeave` is a DAG where nodes are meaningful on their own and can be moved between parents. Both support caller-supplied `u128`-style keys, activation, bookmarking, stable ordering, editing, splitting, and merging. Serialization (`rkyv`, `serde`) and CRDT collaboration (`loro`) live behind default-on or opt-in Cargo features. With default features disabled the crate depends only on `contracts`, `hashbrown`, and `indexmap`, all no_std-compatible with `alloc`.

Animus already owns the append-only session tree ledger (`animus-adapters/src/ledger/`), Goose-style session branch plans, compaction, and redacted session-tree export. Those surfaces model history as immutable entries. The weave adapter covers the complementary mutable model: a host-managed branching workspace where node text can be edited, split, merged, and moved before anything is committed to a ledger.

A bounded search considered three routes: reimplementing weave semantics inside `animus-adapters`, wrapping `universal-weave` inside the existing `animus-adapters` facade, and a focused optional crate. The focused crate is the only route that keeps upstream traits out of the default dependency graph, keeps the Wasm-supported surface unchanged, and matches the adapter-family migration direction.

## Decisions

### Decision: pin one exact upstream crate with default features disabled

**Choice:** `animus-adapter-weave` depends on `universal-weave = "=0.4.1"` with `default-features = false`. The rkyv, serde, legacy, and loro lanes stay disabled. The crate is Unlicense, which is compatible with the workspace `MIT OR Apache-2.0` posture.

**Rationale:** An exact version pin plus `Cargo.lock` prevents a fast-moving young crate from silently changing adapter behavior. Disabled default features keep `rkyv` out of the dependency graph, which avoids its soundness and format-stability claim surface.

### Decision: wrap the DAG weave only

**Choice:** The adapter wraps `IndependentWeave<u128, WeaveNodeContent, (), DefaultHashBuilder>`. It exposes add, edit, split, merge-with-parent, move, bookmark, activation, removal, active-path, and consistency-check operations. The tree weave, dedup wrapper, undo wrapper, and CRDT lane are excluded.

**Rationale:** The DAG weave is the superset model (tree weaves convert into it) and supports node moving, which session exploration interfaces need. One wrapped implementation keeps the admission and receipt surface small enough to test exhaustively.

### Decision: own the content type and every DTO

**Choice:** `WeaveNodeContent` is an Animus-owned text record. It implements `IndependentContents` and `DiscreteContents` (split at a UTF-8 boundary, merge by concatenation). All request, result, snapshot, and receipt types are Animus-owned. No upstream type appears in the public API.

**Rationale:** Upstream trait or layout changes must surface as compile errors inside the adapter crate, never as breaking changes for embedders.

### Decision: enforce named bounds before mutation

**Choice:** `WeaveAdapterPolicy` names `max_nodes`, `max_content_bytes`, and `max_parents_per_node`. The adapter validates the policy once at construction and revalidates every request against it before touching the weave. Violations fail closed with typed errors and leave the weave unchanged.

**Rationale:** An unbounded weave is an unbounded memory surface. Bounds are policy data, not literals buried in logic.

### Decision: receipts and snapshots carry metadata only

**Choice:** Operation receipts bind operation class, target identifier, bounded node/root/active/bookmark counts, policy identity, and a domain-separated BLAKE3 receipt hash. Snapshots expose per-node identifiers, fan-in/fan-out counts, flags, content length, and a BLAKE3 content digest. Raw node text never appears in receipts or snapshots.

**Rationale:** Branching workspaces routinely hold drafts, prompts, and provider text. Evidence must prove adapter facts without copying that content.

### Decision: keep the session ledger authoritative

**Choice:** The adapter does not read, write, or replace the neutral session ledger, and it does not project weave state into ledger entries. Committing a weave path into a session remains a host decision through the existing ledger admission path.

**Rationale:** Two owners for conversation history would fork replay, compaction, and export semantics.

## Alternatives Considered

### Reimplement weave semantics in `animus-adapters`

Rejected because mutable DAG editing with split/merge/move is exactly the subtle bug class the upstream crate exists to solve, and a reimplementation would carry the defect risk without the upstream test corpus.

### Add `universal-weave` as an optional dependency of `animus-adapters`

Rejected because the facade crate is part of the default build and the Wasm-supported surface; even an optional upstream dependency there widens review scope for every embedder and conflicts with the adapter-family direction of focused crates.

### Enable the serde or rkyv lanes for host persistence

Rejected for the initial adapter because serialization format stability and rkyv soundness are separate claim surfaces. Hosts can persist snapshots through their own stores today.
