# Evidence

Date: 2026-08-24

Outcome: pass for the declared bounded advisory cache scope.

## Passing evidence

- `cargo test --workspace --locked` passed.
- `cargo clippy --workspace --all-targets --locked -- -D warnings` passed.
- `cargo fmt --all -- --check` passed.
- `cargo tigerstyle check` passed with zero Octet findings.
- `cargo check -p animus-core --target wasm32-unknown-unknown --locked` passed.
- `scripts/cairn/check-tool-result-cache --root .` passed.
- `nix build .#checks.x86_64-linux.tool-result-cache --no-link -L --option builders ''` passed.
- Component-catalog and runtime-facade inventories passed with positive and negative fixtures.
- Positive fixtures covered stable BLAKE3 projection, bounded retention, LRU, FIFO, watermark trimming, adapter miss routing, and release-after-guard ordering.
- Negative and boundary fixtures covered absent capacity, reversed watermarks, invalid percentages, impossible state, zero capacity, and one-slot eviction.

## Dependency evidence

- No local or sibling-path dependency was added.
- No published immutable `OnixResearch/bounded-cache` revision was available during this change.
- `adanil-code/LRUHashTable` remains an Apache-2.0 design reference only. No source was copied.

## Bounded warnings and non-claims

- The optional local binary cache timed out. Nix disabled it and completed local builds.
- A cache hit does not prove tool correctness, result freshness, authorization, exactly-once execution, durable retention, production readiness, or release eligibility.
