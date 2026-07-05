# Evidence: adapt-clankers-controller-continuation-policy

## Implementation evidence

- Added pure `animus-runtime::continuation` state, priority, acknowledgement, and receipt core in `crates/animus-runtime/src/continuation/mod.rs`.
- Added positive and negative focused tests in `crates/animus-runtime/src/continuation/tests.rs` for explicit queued-prompt state, priority ordering, custom priority, exact dispatch/completion acknowledgements, stale/missing/mismatched/duplicate failures, cancellation, shell-owned purity, and receipt leak rejection.
- Added `docs/controller-continuation-policy.md` and linked it from `README.md`.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-runtime` — passed, 56 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-runtime` — passed, 73 tests.
- After implementation: `nix develop -c cargo check -p animus-runtime --all-targets` — passed.
- After implementation: `nix develop -c cargo clippy -p animus-runtime --all-targets --no-deps -- -D warnings` — blocked by pre-existing lint failures outside `continuation`; recaptured log had no `continuation` diagnostics.
- Broader workspace check: `nix develop -c cargo check --workspace --all-targets` — blocked by pre-existing `animus-preserves` non-exhaustive match on `RuntimeEvent` variants.
- Sync dry-run: `nix run path:$PWD#cairn -- sync adapt-clankers-controller-continuation-policy --root $PWD` — unblocked; initial execute produced no accepted-spec content change, so the delta requirements were manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified by grep.
- Post-sync validation: `nix run path:$PWD#cairn -- validate --root $PWD` — passed.
- Tracey coverage: `nix run path:$PWD#cairn -- tracey coverage --root $PWD` — still globally failing for pre-existing uncovered IDs; recaptured log had no `clankers_continuation_policy` missing IDs.
- Final closeout: `scripts/cairn/check-evidence-safety --root .`, Cairn validate, proposal gate, design gate, and tasks gate — passed.
- Archive dry-run and execute: `nix run path:$PWD#cairn -- archive adapt-clankers-controller-continuation-policy --root $PWD`, then `CAIRN_ARCHIVE_DATE=$(date +%F) nix run path:$PWD#cairn -- archive adapt-clankers-controller-continuation-policy --root $PWD --execute` — unblocked and completed; final Cairn validate passed with 39 active changes.

## Next-best validation for blocked rails

- Fix existing `animus-preserves` event projection exhaustiveness before using workspace-wide `cargo check --workspace --all-targets` as closeout evidence for this change.
- Fix existing Clippy failures in `animus-adapters` and pre-existing `animus-runtime` modules before using workspace-wide or crate-wide Clippy as closeout evidence.
