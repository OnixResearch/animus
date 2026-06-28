# Evidence: reject-clankers-product-surfaces

## Baseline

- `nix run .#cairn -- validate --root .`: PASS before edits; 70 active changes and 71 specs validated.
- `nix run .#cairn -- gate proposal/design/tasks reject-clankers-product-surfaces --root .`: PASS before edits.
- `nix develop -c scripts/cairn/check-roadmap --root . && scripts/cairn/check-evidence-safety --root .`: BLOCKED before edits by pre-existing `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE ensure-wasm-compile-boundary`; the chained evidence-safety command did not run. Next best check is focused Cairn validation plus the new product-surface rail.

## Implementation

- Added `docs/clankers-product-surface-rejection.md` to distinguish bounded Clankers pattern adoption from rejected Clankers product-surface defaults.
- Added `scripts/cairn/check-clankers-product-surfaces` and `scripts/cairn/check-clankers-product-surfaces.rs` as a Rust-backed rail over reusable crate sources and manifests.
- Added positive and negative marker fixtures under `cairn/clankers-product-surface-fixtures/`.
- Added the rail to README/safety-contract verification guidance and to `checks.x86_64-linux.clankers-product-surfaces`.

## Verification

- `nix develop -c scripts/cairn/check-clankers-product-surfaces --root .`: PASS; repo rail ok, positive bounded-doc fixture ok, negative reusable-contract fixture reported `CLANKERS_PRODUCT_SURFACE_MARKER`.
- `nix build .#checks.x86_64-linux.clankers-product-surfaces --no-link -L`: PASS after staging new files for flake visibility and adding a linker input. Remote builder/cache availability warnings were non-blocking.
- `scripts/cairn/check-evidence-safety --root .`: PASS after adding evidence.
- `nix run .#cairn -- validate --root .`: PASS after edits; 70 active changes and 71 specs validated.
- `nix run .#cairn -- gate proposal/design/tasks reject-clankers-product-surfaces --root .`: PASS after edits; tasks gate receipt hash `5aaff3b8a499aef2cfc54533448de556aff28b4e32072db840a23aee4003c1c7`.
- `nix run .#cairn -- sync reject-clankers-product-surfaces --root .`: dry-run PASS with empty reasons, plan hash `22fb48bcb7b4b69388fffe2055295b1e044eb42dc8d8753e1f465c776c6f39ca`, receipt hash `b87e5b13f28a862a7e171272fd3c3d383620845153770f8df105c6873da9ad56`.
- `nix run .#cairn -- sync reject-clankers-product-surfaces --root . --execute`: PASS with empty reasons and receipt hash `47249c11dcebc2a159ff977e182eb807337b9c1790b0471bd118e737a933e5fa`.
- Accepted-spec repair: the sync command succeeded but did not promote `r[agent_harness_runtime.clankers_product_rejection.*]` IDs into `cairn/specs/agent-harness-runtime/spec.md`; the delta was manually merged and verified with a focused requirement-ID search.
- `nix run .#cairn -- validate --root .` after accepted-spec repair: PASS; 70 active changes and 71 specs validated.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive reject-clankers-product-surfaces --root .`: dry-run PASS with `blocked: false`, empty reasons, plan hash `6164e6667b9dabf9a1d1b2186413aa13783498d28f6d2308bf0ebdf5d8fa280e`, and receipt hash `807de245f7f7233ec72b82072b4be8d42d1cbd38c7e4897e83c326333a297e65`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive reject-clankers-product-surfaces --root . --execute`: PASS with empty reasons and receipt hash `855a7a2d8308475f6c99e2c4c705f1694b141519a6c4562a6a36bd667d70bc5f`.
- `nix run .#cairn -- validate --root .` after archive execution: PASS; 69 active changes and 70 specs validated.
- Archive location: `cairn/archive/2026-06-28-reject-clankers-product-surfaces/`.
