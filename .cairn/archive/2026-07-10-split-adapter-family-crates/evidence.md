# Evidence: split-adapter-family-crates

## Baseline

- Initial drain validation passed proposal/design/tasks gates for active changes before implementation began.
- `scripts/cairn/check-roadmap --root .` passed after prior unclassified/blocked changes were archived, with `add-tool-executor-kit` correctly blocked on this change.

## Implementation evidence

- Added focused no-std family crates `animus-adapter-safe` and `animus-adapter-observability` to the workspace and lockfile.
- `animus-adapter-safe` owns reusable safe receipt DTOs, bounded counts, leak classes, BLAKE3 artifact hash validation, review summaries, and positive/negative tests.
- `animus-adapter-observability` owns metric/log DTOs, sink outcomes, delivery policy, adapter traits, and positive/negative tests; it depends on `animus-adapter-safe` instead of the broad adapter crate.
- `animus-adapters` now depends on the family crates and re-exports observability through the compatibility facade while retaining existing safe receipt aliases.
- Added `crates/animus-adapters/src/adapter_family_crates.rs` parity tests for direct observability crate imports versus `animus-adapters` facade imports, plus safe receipt family schema/constant parity and unsafe receipt rejection.
- Added `scripts/cairn/check-adapter-family-crates` and `.rs` checker with positive and negative fixtures for valid family split metadata, missing facade export, dependency cycle, authority leak, and unsafe receipt metadata.
- Updated `cairn/component-catalog.ncl`, `docs/component-catalog.md`, `cairn/adapter-conformance-index.ncl`, `docs/contracts-index.md`, `docs/adapter-family-crates.md`, `README.md`, and the Wasm rail.
- Wired `nix build .#checks.x86_64-linux.adapter-family-crates --no-link -L`.

## Verification

- `cargo test -p animus-adapter-safe --locked` passed.
- `cargo test -p animus-adapter-observability --locked` passed.
- `cargo test -p animus-adapters adapter_family_crates --locked` passed.
- `cargo check -p animus-adapter-safe --target wasm32-unknown-unknown --locked` passed.
- `cargo check -p animus-adapter-observability --target wasm32-unknown-unknown --locked` passed.
- `cargo check -p animus-adapters --target wasm32-unknown-unknown --locked` passed.
- `scripts/cairn/check-adapter-family-crates --root .` passed.
- `scripts/cairn/check-component-catalog --root .` passed.
- `scripts/cairn/check-adapter-conformance-index --root .` passed.
- `scripts/cairn/check-wasm-surface --root .` passed with the new family crates included.
- `cargo fmt --check` passed.
- `cargo clippy -p animus-adapter-safe -p animus-adapter-observability -p animus-adapters --all-targets -- -D warnings` passed.
- `cargo check --workspace --all-targets --locked` passed.
- `nix build .#checks.x86_64-linux.adapter-family-crates --no-link -L` passed after staging rail files so the flake source included them.
- `nix build .#checks.x86_64-linux.wasm-surface --no-link -L` passed after staging the updated Wasm rail.
- `nix run .#cairn -- sync split-adapter-family-crates --root .` dry-run was unblocked.
- `nix run .#cairn -- sync split-adapter-family-crates --root . --execute` completed, but the accepted spec text was not promoted by the sync tool; the requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified with `rg 'adapter_family_crates'`.

## Non-claims

- This change does not add concrete providers, tools, memory stores, workspaces, networks, plugin runtimes, credentials, persistence, or new adapter behavior.
- The safe receipt family crate is independent; the legacy `animus-adapters` safe receipt facade remains available during migration, and this change only proves schema/constant/direct-family behavior parity for the initial wave.
