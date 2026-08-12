# Evidence: add-component-catalog

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed before implementation and reported unrelated active-prerequisite blockers for `split-std-host-adapter-crates`.
- `scripts/cairn/check-runtime-facade-inventory --root .` passed before implementation.

## Implementation evidence

- Added `cairn/component-catalog.ncl` as the Nickel source of truth for component ids, owners, classes, feature gates, authority posture, dependency links, capability families, focused checks, safe evidence labels, and smallest public imports.
- Added `docs/component-catalog.md` as the checked generated projection.
- Added `scripts/cairn/check-component-catalog` and `scripts/cairn/check-component-catalog.rs` with pure in-memory catalog validation and a thin shell wrapper for Nickel export, docs drift checks, fixture execution, and BLAKE3 line hashing when `b3sum` is available.
- Added positive fixtures for valid reusable-contract, host-injection-surface, shell-adapter, optional-bridge, and generated-doc cases.
- Added negative fixtures for unknown dependencies, unknown composition links, unknown classes, invalid authority labels, missing focused checks, reusable ambient authority, unsafe evidence labels, dependency cycles, and generated-doc drift.
- Updated README, `docs/contracts-index.md`, `docs/runtime-facade-inventory.md`, and `docs/safety-contract.md` to point at the catalog and focused rail.
- Wired `nix build .#checks.x86_64-linux.component-catalog --no-link -L` into `flake.nix`.

## Verification

- `scripts/cairn/check-component-catalog --root .` passed after implementation.
- `nix build .#checks.x86_64-linux.component-catalog --no-link -L` passed after staging catalog-related files so the flake source included them.
- `nix run .#cairn -- sync add-component-catalog --root .` dry-run was unblocked.
- `nix run .#cairn -- sync add-component-catalog --root . --execute` completed, but the accepted spec text was not promoted by the sync tool; the requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified with `rg 'component_catalog'`.
- `nix run .#cairn -- validate --root .` passed after accepted-spec merge.

## Non-claims

- This change does not split crates, change public Rust APIs, add concrete providers, add filesystem/process/network/terminal/credential/persistence authority to reusable crates, or claim production/dogfood readiness.
