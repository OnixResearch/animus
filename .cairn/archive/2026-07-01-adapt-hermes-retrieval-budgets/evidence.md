# Evidence: adapt-hermes-retrieval-budgets

## Implementation evidence

- Added `crates/animus-adapters/src/retrieval.rs` with typed retrieval budget descriptors, retrieval-capable resource policy admission, repeat/stop planning over evidence summaries, and bounded receipt/report metadata with BLAKE3 content hash support.
- Exported retrieval budget DTOs and pure functions from `crates/animus-adapters/src/lib.rs`.
- Added `docs/retrieval-budgets.md` and README linkage for first/repeat/stop policy, missing-evidence behavior, safe metadata, and non-claims.

## Validation evidence

- Focused retrieval tests: `nix develop -c cargo fmt --check && nix develop -c cargo test -p animus-adapters retrieval` — PASS, 6 tests.
- Focused clippy probe: `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` — BLOCKED by pre-existing unrelated active-worktree adapter warnings; grep of the clippy log found no remaining diagnostics for `crates/animus-adapters/src/retrieval.rs` after local fixes.
- Cairn gates: `nix run path:$PWD#cairn -- gate proposal adapt-hermes-retrieval-budgets --root .`, `gate design`, and `gate tasks` — PASS.
- Cairn validation: `nix run path:$PWD#cairn -- validate --root .` — PASS.

## Safety notes

- Retrieval evidence metadata rejects raw fetched bodies by default.
- Safe references reject full host paths and traversal markers.
- The change does not add web search, browser automation, a documentation provider, live retrieval, or production-readiness claims.
