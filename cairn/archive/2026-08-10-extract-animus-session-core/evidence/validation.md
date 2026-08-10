# Validation evidence

Date: 2026-08-10

## Identities

- Published baseline commit: `526519b8b66a68a62aece40f0d5b6c6ed4e6a08e`.
- Pre-closeout implementation patch BLAKE3: `17bfafa5e6ef434ba8415e79bc324d058d79bd4abbfba8c822b84bad8b6f7177`.
- The patch identity excludes this Cairn change package. It binds the implementation, tests, checked catalogs, documentation, and Nix source filter before sync and archive.
- The 44 preserved `animus.*` schema and identity literals had BLAKE3 `460b5b257c94bcb4c737598f28d9f82ad8682cd7a6648079d48ce57c2d99e161` before and after extraction.
- Component catalog BLAKE3: `0da2fd84764e5cf333700e91561ce0026e78991672e8ecdddd2f2fc4eb4c83f1`.
- Adapter conformance family BLAKE3: `8d968fff6a8462509746ee441bb885afc1291c5f1101e8c937df5c8bd8fcb94e`.
- Adapter conformance case BLAKE3: `6f1a3c0bf276c92f332a543067c41821fcdc5e4483ee5ec808108b24a77a9436`.
- Runtime facade inventory BLAKE3: `f9a4ddd81a464df88ba2445239307c704153638f8af3f72abd39f8623867dc48`.
- Cairn proposal receipt: `81ceaa15f896c01d305bf974a65197a7b8e8a7c1ee05ce672f5c2c8976b2a77f`.
- Cairn design receipt: `f156ec5881879566f5ce4250fa89ede9e39b41d78d047d53b981106d6e6ea806`.
- Cairn sync dry-run receipt: `19657c360059d0ac9ac79230f0628b35a0046b0af1d3a04d91b8bfbf30308f87`.
- Cairn sync execution receipt: `659ede7c9d703285615fcdd68dc8ec1e205254cca0b8513d87422f4cbb3cd360`.
- Synced accepted specification BLAKE3: `d9cd96df91379d3ce19784b89a1134120ef30f4e4252ae911484e3daa46c5d77`.

## Passed checks

All Cargo commands used the pinned Nix development shell and the external Cargo target directory.

- `cargo test -p animus-session-core --features session-event-replay --locked`: 100 unit tests and four dependency-boundary tests passed.
- The positive, forbidden-host, and malformed manifest fixtures passed their intended checks.
- The runtime facade test passed with and without `session-event-replay`. It checked type, error, root-export, feature, and content identity compatibility.
- `cargo test --workspace --locked`: passed. Environment-dependent live tests remained explicitly ignored.
- Scoped and workspace `cargo clippy ... --no-deps -- -D warnings`: passed.
- `cargo tigerstyle check`: passed with Octet config BLAKE3 `8f23014663e8ff59702c121f65f3cb912c6a368d32d84948ba655d08535b7dad` and profile BLAKE3 `ee57050fa909d35877a684fbed0e3c4e67fabe625dd9d009b26b487d82f73f2c`.
- `scripts/config/check-nickel-configs`: passed.
- `scripts/cairn/check-wasm-surface --root .`: passed, including the direct `animus-session-core` Wasm build and positive and negative dependency fixtures.
- Agent control, session authority profile, runtime authority seal, session branch explorer, conversation projection, active-turn recovery, interrupted-tool repair, session-tree export, and session-event replay rails passed.
- Component catalog, adapter conformance index, runtime facade inventory, and evidence-safety rails passed with their positive and negative fixtures.
- Focused Cairn validation passed. Proposal and design gates passed. Cairn sync added the accepted requirement.
- `nix build path:$PWD#checks.x86_64-linux.cargo-test --no-link -L`: passed after the filtered source admitted the malformed fixture.
- `nix flake check path:$PWD -L --option secret-key-files ''`: passed for `x86_64-linux`. The flake reported `aarch64-linux` as incompatible and omitted it.

## Bounded failures and non-claims

- The configured SSH Nix builder was unavailable. Nix used the local builder and completed the checks.
- The development shell did not put `check-nickel-configs` on `PATH`. The repository-owned `scripts/config/check-nickel-configs` command passed.
- `octet-precommit-check` was unavailable. The repository-authoritative `cargo tigerstyle check` command passed.
- The first full Nix run found that `manifest.toml` was absent from the filtered source. The filter now admits that fixture. The focused Cargo Nix check and the full flake check then passed.
- Global Tracey coverage remains inherited debt. Receipt `783a6a7bfc77b109e67ac44257aa2f9edbf732467c9fd3706c5040725722b26e` reported 923 referenced IDs from 3,260 requirements. No missing or dangling ID contained `session_core_crate`.
- These checks establish the declared extraction, compatibility, dependency, and build facts only. They do not grant host authority or prove semantic correctness, release eligibility, live-provider behavior, or whole-system correctness.
