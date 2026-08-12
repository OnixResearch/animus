# Evidence: Add a universal-weave branching adapter

## Cohort

- Upstream: `universal-weave` `0.4.1` (crates.io), Unlicense, repository `https://github.com/transkatgirl/universal-weave`.
- Pin: `universal-weave = { version = "=0.4.1", default-features = false }` in `crates/animus-adapter-weave/Cargo.toml`; `Cargo.lock` records `universal-weave 0.4.1`.
- Feature isolation: `Cargo.lock` contains no `rkyv` or `loro` package; the serde lane is disabled. Verified by `scripts/cairn/check-universal-weave-adapter`.

## Baseline

- `cargo check --workspace --all-targets --locked` passed on the worktree before adapter wiring was committed.
- `nix run .#cairn -- validate --root .` reported `"valid": true` with the change package present.

## Implementation

- `crates/animus-adapter-weave`: no_std, unsafe-forbid, workspace lints. Public surface: `WeaveAdapter`, `WeaveAdapterPolicy`, `WeaveNodeContent`, `WeaveOperationKind`, `WeaveOperationReceipt`, `WeaveCounts`, `WeaveSnapshot`, `WeaveSnapshotEntry`, `WeaveAdapterError`.
- Wrapped cohort: `IndependentWeave<u128, WeaveNodeContent, (), DefaultHashBuilder>` only. No upstream type appears in the public API.
- Operations: add, edit, split, merge-with-parent, move, bookmark, activate, remove, active-path, consistency check, redacted snapshot.
- Bounds: `node_limit`, `content_byte_limit`, `parent_fan_in_limit`, validated at construction and before every mutation.
- Evidence: metadata-only receipts with domain-separated BLAKE3 identities; snapshots carry identifiers, counts, flags, content lengths, and content digests only.

## Verification results

- `cargo test -p animus-adapter-weave --locked`: 24 passed, 0 failed. Positive coverage: every operation class, active-path reads, consistency checks, deterministic receipts, stable redacted snapshots. Negative coverage: zero bounds, oversized content, fan-in overflow, node-limit exhaustion, missing nodes, duplicate identifiers, missing parents, out-of-range and non-boundary splits, parentless merges, cyclic moves, unchanged-state proofs after rejection, and evidence leak checks for raw text and credential markers.
- `cargo clippy -p animus-adapter-weave --all-targets --locked -- -D warnings`: clean.
- `cargo fmt -p animus-adapter-weave -- --check`: clean.
- `cargo check -p animus-adapter-weave --target wasm32-unknown-unknown --locked`: passes. This is compile evidence for the opt-in crate only; the supported Wasm surface in `scripts/cairn/check-wasm-surface` is unchanged and the focused rail fails on drift.
- `cargo check -p animus --features weave-adapter --locked`: passes.
- `scripts/cairn/check-universal-weave-adapter --root .`: passes.
- `scripts/cairn/check-component-catalog --root .`: passes with the new `adapters.universal-weave` row; the `weave` family and `weave-adapter` feature gate are registered in the catalog checker allowlists.

## Claim boundaries

- Receipts prove adapter-observed operation facts only: operation class, target identifier, bounded counts, policy identity. They do not claim content authenticity, completeness, privacy, ledger durability, or upstream weave correctness.
- The adapter does not read or write the neutral session ledger and does not persist weave state.
- The `contracts` crate evaluates upstream design-by-contract assertions at runtime. Animus relies only on the explicit `check_consistency` result and its own admission checks.

## Sync and archive

- Accepted-spec repair before sync: the accepted `agent-harness-runtime` spec on `main` carried pre-existing shapes that the Cairn delta merger rejects. Four accepted requirements held multiple bare `r[...]` identities before their first scenario, and one earlier sync had left a duplicate `## Requirements` section plus an orphaned purpose sentence. The repair split the four requirements into one-identity blocks (`Ledger rails`, `Injected service rails`, `Focused inventory rail positive coverage`, `Focused inventory rail negative coverage`, `Safe inventory documentation link`, `Provider-prefix failures fail closed`) and removed the duplicate section. No requirement identity or text changed.
- Sync: `nix run .#cairn -- sync add-universal-weave-adapter --root . --execute` mutated only `cairn/specs/agent-harness-runtime/spec.md`. All five `r[agent_harness_runtime.universal_weave.*]` requirement identities and ten scenario identities are present in the accepted spec. Sync receipt hash `6a59878be26be051e374b1ac7c435c0a2821d61b4205a8ece1a6b9968e2f5a33`.
- Post-sync validation: `nix run .#cairn -- validate --root .` reports `"valid": true`.

## Blockers

None.
