# Evidence: adapt-clankers-request-lifecycle-ownership

## Baseline

- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-request-lifecycle-ownership --root .`: PASS before edits; tasks gate receipt hash `0b46195e18f5f28ac3a6c3bb1d95f8aa4f1be9d3e4c6eb38e31535d8ab0cb8f5`.
- `nix run .#cairn -- validate --root .`: PASS before selecting this change; lifecycle tree was clean.

## Implementation

- Added `docs/request-lifecycle.md` as the maintained ownership map for typed runtime input, deterministic core state, Steel decisions, adapter dispatch, report projection, safe receipt construction, and future ledger seams.
- Linked the lifecycle ownership map from README and `docs/safety-contract.md`.
- Added `scripts/cairn/check-request-lifecycle` and `scripts/cairn/check-request-lifecycle.rs` as a Rust-backed marker rail for required boundary, owner, forbidden-dependency, and validation-command markers.
- Added positive and negative fixtures under `cairn/request-lifecycle-fixtures/`.
- Added `checks.x86_64-linux.request-lifecycle` to keep the rail deterministic under Nix.

## Verification

- `nix develop -c scripts/cairn/check-request-lifecycle --root .`: PASS; doc marker rail passed, positive fixture passed, negative fixture reported `REQUEST_LIFECYCLE_MARKER_MISSING`.
- `nix build .#checks.x86_64-linux.request-lifecycle --no-link -L`: PASS.
- `scripts/cairn/check-evidence-safety --root .`: PASS after adding evidence.
- `nix run .#cairn -- validate --root .`: PASS after edits; 68 active changes and 69 specs validated before sync.
- `nix develop -c scripts/cairn/check-roadmap --root .`: PASS; remaining blocked entries are dependency-order reports, not validation failures.
- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-request-lifecycle-ownership --root .`: PASS after edits; tasks gate receipt hash `311d34c3f3d5da70322e78252139fcd2e9fc3a40da1bf1c23ba12299797d3273`.
- `nix run .#cairn -- sync adapt-clankers-request-lifecycle-ownership --root .`: dry-run PASS with empty reasons, plan hash `c3f691500f1fbe404b8ac488b2459fa450abbd00b9470f1172f0e39ac3b4f1f6`, receipt hash `ec2ea440bcf4f511926465042080803fda281b7bce10db3a501c5e1e8acdbc6e`.
- `nix run .#cairn -- sync adapt-clankers-request-lifecycle-ownership --root . --execute`: PASS with empty reasons and receipt hash `a23a01e9303d525c79a5d2615afccef1e6669e49cf247d1080588e5bb68ce7d3`.
- Accepted-spec repair: the sync command succeeded but did not promote `r[agent_harness_runtime.clankers_lifecycle_doc.*]` IDs into `cairn/specs/agent-harness-runtime/spec.md`; the delta was manually merged and verified with a focused requirement-ID search.
- `nix run .#cairn -- validate --root .` after accepted-spec repair: PASS; 68 active changes and 69 specs validated.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive adapt-clankers-request-lifecycle-ownership --root .`: dry-run PASS with `blocked: false`, empty reasons, plan hash `66681cbeb29d8f17b001ec45df2672959288f27c4574662ff61bd57068539fa7`, and receipt hash `b5a848bab1f22902674a4f3e90eff55a038882b65834750232ae5845fc57a531`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive adapt-clankers-request-lifecycle-ownership --root . --execute`: PASS with empty reasons and receipt hash `cfefdd2b97afd4bde38cb6b3cf71d8709a5d89389ab9a1d0e271f57376868931`.
- `nix run .#cairn -- validate --root .` after archive execution: PASS; 67 active changes and 68 specs validated.
- Archive location: `cairn/archive/2026-06-28-adapt-clankers-request-lifecycle-ownership/`.
- Post-archive roadmap cleanup removed the archived change from active roadmap classification while keeping roadmap validation passing.
