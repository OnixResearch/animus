# Evidence: adapt-clankers-runtime-facade-inventory

## Baseline

- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-runtime-facade-inventory --root .`: PASS before edits; tasks gate receipt hash `6541cc52c799cca7281a93d205a56f17eb77e9ddca97ac71907f454f067c0966`.
- `nix run .#cairn -- validate --root .`: PASS before selecting this change; lifecycle tree was clean.

## Implementation

- Added `docs/runtime-facade-inventory.md` documenting bounded Clankers inventory adoption, Animus-owned classes, forbidden reusable authority, evidence safety, and explicit non-adoption of Clankers product surfaces.
- Added `cairn/runtime-facade-inventory.ncl` as the typed Nickel source of truth for runtime-facing public export records.
- Added `scripts/cairn/check-runtime-facade-inventory` and `scripts/cairn/check-runtime-facade-inventory.rs` to validate the Nickel inventory against discovered crate-root public exports.
- Added positive and negative Nickel fixtures under `cairn/runtime-facade-inventory-fixtures/` for valid classification, unclassified exports, and reusable ambient-authority rejection.
- Added `checks.x86_64-linux.runtime-facade-inventory` and linked the inventory docs from README and the safety contract.

## Verification

- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root .`: PASS; 66 inventory entries matched 66 runtime-facing public exports, class counts were 6 host-injection surfaces, 40 reusable contracts, and 20 shell adapters. Positive and negative fixtures passed; local `b3sum` was unavailable so local hash output was skipped.
- `nix build .#checks.x86_64-linux.runtime-facade-inventory --no-link -L --rebuild`: PASS; inventory BLAKE3 hash `2801408408e446ffeda236d3f6e76e0e59dec6decfbd015f3898b6951610e06d`, same class counts, and all fixtures passed.
- `scripts/cairn/check-evidence-safety --root .`: PASS after adding evidence.
- `nix run .#cairn -- validate --root .`: PASS after edits; 67 active changes and 68 specs validated before sync.
- `nix develop -c scripts/cairn/check-roadmap --root .`: PASS; remaining blocked entries are dependency-order reports, not validation failures.
- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-runtime-facade-inventory --root .`: PASS after edits; tasks gate receipt hash `a3ea62fd677dacc451fc55ba7bda1b8725f8c26f177373184f2146204b56b996`.
- `nix run .#cairn -- sync adapt-clankers-runtime-facade-inventory --root .`: dry-run PASS with empty reasons, plan hash `2257044d2bf9bdb245e1243e9ca3d6e939309a278d289eeb9642bc56bc604854`, receipt hash `6cf058e14bb1894c317837252109aa4064e5b826df7a43cda4e872f6aea994e6`.
- `nix run .#cairn -- sync adapt-clankers-runtime-facade-inventory --root . --execute`: PASS with empty reasons and receipt hash `8a879ce01db837524016689f83e4fb8289ecafbe0c787ee1084e7e0ee4e18518`.
- Accepted-spec repair: the sync command succeeded but did not promote `r[agent_harness_runtime.clankers_runtime_facade_inventory.*]` IDs into `cairn/specs/agent-harness-runtime/spec.md`; the delta was manually merged and verified with a focused requirement-ID search.
- `nix run .#cairn -- validate --root .` after accepted-spec repair: PASS; 67 active changes and 68 specs validated.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive adapt-clankers-runtime-facade-inventory --root .`: dry-run PASS with `blocked: false`, empty reasons, plan hash `3ff5fe34b4c3da0721a565495ef7b5d2994425296d3456b28dd8b7051b76b317`, and receipt hash `245855360043b181fbd841c0656ff601df12ff8a9e99ff65bb6bb3d206482ece`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive adapt-clankers-runtime-facade-inventory --root . --execute`: PASS with empty reasons and receipt hash `d38399705cde1d7ed6c4a513546d56570f4ac0d1548f369a6a0cbfea50b78b89`.
- `nix run .#cairn -- validate --root .` after archive execution: PASS; 66 active changes and 67 specs validated.
- Archive location: `cairn/archive/2026-06-28-adapt-clankers-runtime-facade-inventory/`.
- Post-archive roadmap cleanup removed the archived change from active classification and replaced dependent references with accepted requirement ID `agent_harness_runtime.clankers_runtime_facade_inventory.closeout`.
