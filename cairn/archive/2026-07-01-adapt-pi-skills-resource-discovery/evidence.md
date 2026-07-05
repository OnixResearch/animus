# Evidence: adapt-pi-skills-resource-discovery

## Implementation evidence

- Added pure no-std prompt resource descriptors in `crates/animus-adapters/src/resource.rs` for skills, prompt templates, context guidance, and host resources.
- Added deterministic advertisement and progressive disclosure APIs through `crates/animus-adapters/src/front/guidance.rs` without filesystem scanning, package installation, ambient body reads, or extension execution.
- Added project-trust denial, duplicate-name diagnostics, safe provenance validation, BLAKE3 content identity validation, and selected-body loading guarded by descriptor identity.
- Added positive and negative tests in `crates/animus-adapters/src/resource/tests.rs` and documented the boundary in `docs/pi-resource-discovery.md`.

## Validation evidence

- `nix run path:$PWD#cairn -- gate proposal adapt-pi-skills-resource-discovery --root $PWD` — PASS.
- `nix run path:$PWD#cairn -- gate design adapt-pi-skills-resource-discovery --root $PWD` — PASS.
- `nix run path:$PWD#cairn -- gate tasks adapt-pi-skills-resource-discovery --root $PWD` — PASS.
- Baseline/focused: `cargo test -p animus-adapters skill::tests` — PASS.
- Focused: `cargo test -p animus-adapters resource::tests` — PASS, 10 tests.
- Adapter suite/check: `cargo test -p animus-adapters && cargo check -p animus-adapters --all-targets` — PASS, 213 tests.
- Workspace check: `cargo fmt --check && cargo check --workspace --all-targets` — PASS.
