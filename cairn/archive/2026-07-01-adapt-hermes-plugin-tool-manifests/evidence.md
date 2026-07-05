# Evidence: adapt-hermes-plugin-tool-manifests

## Implementation evidence

- Extended `crates/animus-adapters/src/manifest.rs` with toolset version/kind, manifest provenance, manifest trust status, required tool descriptions, parameter schema identities, result contract identities, and per-tool capability labels.
- Added safe projection from plugin manifests into `crate::tool::Definition` metadata. Definitions are active only for trusted manifests with host-injected executor names.
- Reused the pure tool dispatch gate so missing executors return `MissingExecutor` with `fallback_attempted == false`.
- Documented the boundary in `docs/hermes-plugin-tool-manifests.md`.

## Validation evidence

- `nix run path:$PWD#cairn -- gate proposal adapt-hermes-plugin-tool-manifests --root $PWD` — PASS.
- `nix run path:$PWD#cairn -- gate design adapt-hermes-plugin-tool-manifests --root $PWD` — PASS.
- `nix run path:$PWD#cairn -- gate tasks adapt-hermes-plugin-tool-manifests --root $PWD` — PASS baseline before task completion.
- Baseline/focused: `cargo test -p animus-adapters manifest::tests` — PASS, 12 tests before changes.
- Focused after implementation: `cargo test -p animus-adapters manifest::tests` — PASS, 18 tests.
- Adapter suite/check: `cargo test -p animus-adapters && cargo check -p animus-adapters --all-targets` — PASS, 219 tests.
