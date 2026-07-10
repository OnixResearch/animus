# Evidence: split-runtime-kernel-feature-packs

## Baseline

- `cargo check -p animus-runtime --no-default-features --locked` passed before implementation.
- `nix run .#cairn -- gate proposal split-runtime-kernel-feature-packs --root .`, `gate design`, and `gate tasks` passed before implementation.
- `nix run .#cairn -- validate --root .` and `scripts/cairn/check-roadmap --root .` passed before implementation.

## Implementation evidence

- Added `animus_runtime::kernel` as the direct minimal runtime kernel import path over typed runtime input/config, neutral `LoopFabric`, turn state, runtime events/reports, capability registry, model/tool dispatch planning, and injected adapter orchestration.
- Added a kernel facade parity test proving representative `animus_runtime::kernel::*` imports resolve to the existing flat facade types.
- Added `scripts/cairn/check-runtime-kernel-split` and `scripts/cairn/check-runtime-kernel-split.rs` to check the no-default runtime build, the kernel parity test, the no-default dependency tree, kernel source forbidden markers, and positive/negative fixtures.
- Added positive fixtures for an allowed kernel dependency tree and kernel source, plus negative fixtures for Steel dependency leakage, optional-domain source leakage, and missing kernel exports.
- Updated `cairn/component-catalog.ncl` and `docs/component-catalog.md` with the kernel path plus explicit runtime feature-pack entries for headless, provenance, worker-team, policy, batch, compaction, dataset-export, quality, research, scaffold, status, and orchestration.
- Updated `docs/runtime-composition-preludes.md`, `README.md`, `docs/safety-contract.md`, `cairn/runtime-facade-inventory.ncl`, and `flake.nix`.

## Verification

- `cargo fmt --check` passed after implementation.
- `cargo check -p animus-runtime --no-default-features --locked` passed after implementation.
- `cargo test -p animus-runtime kernel --no-default-features --locked` passed after implementation.
- `scripts/cairn/check-runtime-kernel-split --root .` passed after implementation.
- `scripts/cairn/check-component-catalog --root .` passed after catalog updates.
- `scripts/cairn/check-runtime-facade-inventory --root .` passed after facade updates.
- `nix build .#checks.x86_64-linux.runtime-kernel-split --no-link -L` passed after staging kernel-rail files so the flake source included them.
- `nix run .#cairn -- sync split-runtime-kernel-feature-packs --root .` dry-run was unblocked.
- `nix run .#cairn -- sync split-runtime-kernel-feature-packs --root . --execute` completed, but the accepted spec text was not promoted by the sync tool; the requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified with `rg 'runtime_kernel_split'`.

## Non-claims

- This change does not remove flat compatibility exports, change default `compat` behavior, move code into separate crates, add providers/tools/storage, claim production readiness, or make Steel part of the kernel.
