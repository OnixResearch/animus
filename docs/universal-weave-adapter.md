# Universal Weave branching adapter

`animus-adapter-weave` is a non-default `no_std` crate that wraps the pinned `universal-weave` `=0.4.1` DAG weave behind Animus-owned DTOs, named bounds, and metadata-only BLAKE3 receipts. Default Cargo features stay disabled: no `rkyv`, `serde`, `loro`, or `legacy` lane enters the dependency graph.

## Boundary

The adapter covers the mutable branching model: hosts add, edit, split, merge, move, bookmark, activate, and remove text nodes in one DAG weave. The neutral Animus session tree ledger stays append-only and remains the sole owner of replay, compaction, persistence, and export semantics. Committing a weave path into a session is a host decision through the existing ledger admission path.

The wrapped cohort is `IndependentWeave<u128, WeaveNodeContent, (), DefaultHashBuilder>`. The tree weave, the `DeduplicatedWeave` and action-queue wrappers, the `loro` CRDT lane, and the serialization lanes are excluded. Requesting them is a compile-time absence, not a runtime fallback.

## Bounds

`WeaveAdapterPolicy` names three positive bounds: `node_limit`, `content_byte_limit`, and `parent_fan_in_limit`. Construction validates the policy once. Every request is revalidated before any mutation, and every rejection leaves the weave unchanged. Tests pin the unchanged-state property with topology digests.

## Evidence

Every admitted mutation returns a `WeaveOperationReceipt` that binds the operation class, the target identifier, bounded node, root, active, and bookmark counts, the policy identity, and a domain-separated BLAKE3 receipt identity. `WeaveAdapter::snapshot` projects a redacted, deterministically ordered topology view with per-node fan-in, fan-out, flags, content length, and content digests. Raw node text never appears in receipts or snapshots.

## Usage

```rust
use animus_adapter_weave::{WeaveAdapter, WeaveAdapterPolicy};

let policy = WeaveAdapterPolicy::new(1024, 16_384, 4)?;
let mut adapter = WeaveAdapter::new(policy);
let receipt = adapter.add_node(1, &[], "draft root")?;
adapter.split_node(1, 5, 2)?;
let snapshot = adapter.snapshot();
```

The root `animus` crate re-exports the crate as `animus::weave_adapter` behind the non-default `weave-adapter` feature.

## Focused checks

```sh
cargo test -p animus-adapter-weave --locked
cargo check -p animus-adapter-weave --target wasm32-unknown-unknown --locked
scripts/cairn/check-universal-weave-adapter --root .
```

The Wasm compile check is compile evidence for the opt-in crate only. The supported Wasm surface in `scripts/cairn/check-wasm-surface` is unchanged, and the focused rail fails if the adapter crate appears there without separate promotion evidence.

## Non-claims

This adapter does not prove upstream weave correctness, CRDT convergence, or serialization format stability. It does not persist weaves, replay sessions, call providers, read files, inspect the environment, spawn processes, or access clocks. Receipt identities bind adapter-observed metadata; they do not claim content authenticity, completeness, privacy, or ledger durability. The upstream crate evaluates `contracts` design-by-contract assertions at runtime. Animus treats a passed operation as an adapter-local fact. It is not proof of upstream internal correctness beyond the explicit `check_consistency` result.
