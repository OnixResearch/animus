# Evidence: evaluate-clankers-release-evidence-index

## Baseline

- `nix run .#cairn -- validate --root .`: PASS before edits; 66 active changes and 67 specs validated.
- `nix run .#cairn -- gate proposal evaluate-clankers-release-evidence-index --root .`: PASS before edits; receipt hash `88db9108daf93b386be5718194f99f41d210f5e1f3d88728aee5141cee6a869c`.
- `nix run .#cairn -- gate design evaluate-clankers-release-evidence-index --root .`: PASS before edits; receipt hash `daf7aa18c9a79353e809e0ce34fe8516173b43a23cf265b595b3035c2c17e14f`.
- `nix run .#cairn -- gate tasks evaluate-clankers-release-evidence-index --root .`: PASS before edits; receipt hash `4ed10120ce5dfd69cdfbdaafdab928f9eb3eee0ad5a819600ceaa23c3d975ceb`.
- `scripts/cairn/check-evidence-safety --root .`: PASS before evidence-index implementation.

## Implementation

- Added `docs/release-evidence-index.md` documenting the bounded Clankers evidence-index pattern, current-head requirements, pure-vs-optional separation, and Animus non-claims.
- Added `cairn/release-evidence-index-schema.ncl` as the typed Nickel fixture schema for future evidence-index head, check, claim, non-claim, and missing-mode records.
- Added positive and negative Nickel fixtures under `cairn/release-evidence-index-fixtures/` for valid current-head pure evidence, stale receipts, dirty worktrees, missing modes, and optional-surface overclaims.
- Added `scripts/cairn/check-release-evidence-index` and `scripts/cairn/check-release-evidence-index.rs` with a pure parser/validator core and a thin shell wrapper that compiles the checker, exports Nickel fixture lines, and verifies expected diagnostics.
- Added `checks.x86_64-linux.release-evidence-index` and linked the release evidence evaluation from the README verification surface.

## Verification

- `scripts/cairn/check-release-evidence-index --root .`: PASS; local `b3sum` was unavailable, the schema sample passed, one positive fixture passed, and four negative fixtures reported the expected stale, dirty, missing-mode, and overclaim diagnostics.
- `nix build .#checks.x86_64-linux.release-evidence-index --no-link -L`: PASS; schema sample BLAKE3 hash `4cf1772a00ada0603d2f0f5b7110840b3f7590891f8373297643443b57611386`, one positive fixture passed, and four negative fixtures reported expected diagnostics. Remote builder and cache connection warnings were non-blocking because the derivation built locally.
- `rustfmt --edition 2021 scripts/cairn/check-release-evidence-index.rs`: PASS, followed by a focused rail rerun.
- `nix run .#cairn -- validate --root .`: PASS after implementation; 66 active changes and 67 specs validated.
- `nix run .#cairn -- gate proposal/design/tasks evaluate-clankers-release-evidence-index --root .`: PASS after implementation; tasks gate receipt hash `085b47813b77461521cce0bd3cb6ff1aaa6465c90d6ff2ad9b640f5cc2ba7255`.
- `nix run .#cairn -- sync evaluate-clankers-release-evidence-index --root .`: dry-run PASS with empty reasons, plan hash `9ef28955a5badae65fd636bcce70fb39699fb7699cb637618d6c4da4c8714013`, and receipt hash `ee07fdab161011e97b10270fb7992558ff5d4243473050e9b34a0d754aa250ca`.
- `nix run .#cairn -- sync evaluate-clankers-release-evidence-index --root . --execute`: PASS with empty reasons, plan hash `dfed42c1e9b21ae71874b6e0e731d9112fcc49898828355d6b596b11b981eba1`, and receipt hash `8cbbea5f04ad4a514f5f96a2644af0e3974115bd90d1c832977582988506be12`.
- Accepted-spec repair: sync succeeded but did not promote `r[agent_harness_runtime.clankers_evidence_index.*]` IDs into `cairn/specs/agent-harness-runtime/spec.md`; the delta was manually merged and verified with a focused requirement-ID search.
- `nix run .#cairn -- validate --root .` after accepted-spec repair: PASS; 66 active changes and 67 specs validated.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive evaluate-clankers-release-evidence-index --root .`: dry-run PASS with `blocked: false`, empty reasons, plan hash `0d5c945c6db867b7e2448b6a04d200ac6d242671c55048d09091272ebb9686f4`, and receipt hash `60d0bb918e988307aa29f69bad03443295f0ef152efb5379b37e9cbc4bbfc26c`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive evaluate-clankers-release-evidence-index --root . --execute`: PASS with empty reasons and receipt hash `b8b75755aaae5bedb112811110ca17b4d380948c8f4571cc28f6eed56b714782`.
- `nix run .#cairn -- validate --root .` after archive execution: PASS; 65 active changes and 66 specs validated.
- Archive location: `cairn/archive/2026-06-28-evaluate-clankers-release-evidence-index/`.

## Non-claims

This evaluation does not add Clankers' full harness, dogfood suites, live-provider checks, VM gates, release tagging, production-readiness proof, plugin runtime validation, daemon/TUI dogfood, remote service validation, or performance claims. It defines the future evidence-index shape and fixture rail only.
