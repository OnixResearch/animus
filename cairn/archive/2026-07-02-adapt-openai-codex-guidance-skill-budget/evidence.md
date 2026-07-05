# Evidence: adapt-openai-codex-guidance-skill-budget

## Implementation evidence

- Added pure `front::guidance` DTOs for `GuidanceSource`, `GuidanceBudget`, ordered `GuidanceAssembly` ledgers, skill discovery metadata, deferred full-skill loading decisions, and trust-gated skill package descriptors in `crates/animus-adapters/src/front/guidance.rs`.
- Added positive and negative focused tests for nested guidance precedence, duplicate precedence rejection, over-budget/truncated/redacted ledger records, metadata-only skill discovery, disabled/ambiguous/missing-dependency/implicit-denied loading decisions, and runtime-core script execution denial.
- Added `docs/openai-codex-guidance-skill-budget.md` documenting the contract and non-claims.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters skill::tests` — passed, 15 tests.
- Baseline before edits: `nix develop -c cargo test -p animus-adapters resource::tests` — passed, 10 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters front::guidance::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters skill::tests` — passed, 15 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters resource::tests` — passed, 10 tests.
- After implementation: `nix develop -c cargo check -p animus-adapters --all-targets` — passed.
- Focused clippy: `nix develop -c cargo clippy -p animus-adapters --lib --no-deps -- -D warnings` — globally blocked by pre-existing lint failures outside `front/guidance.rs`; recaptured log had no `front/guidance.rs` diagnostics.
- Cairn proposal/design/tasks gates after task completion — passed.
- Evidence safety: `scripts/cairn/check-evidence-safety --root .` — passed, including negative fixtures.
- Sync dry-run and execute: `nix run path:$PWD#cairn -- sync adapt-openai-codex-guidance-skill-budget --root $PWD`, then `--execute` — unblocked and completed; the accepted spec did not receive the delta automatically, so the requirement block was manually merged and verified by grep.
- Post-sync validation: `nix run path:$PWD#cairn -- validate --root $PWD` — passed.
- Tracey coverage: `nix run path:$PWD#cairn -- tracey coverage --root $PWD` — globally blocked by pre-existing missing IDs; recaptured log had no `openai_codex_guidance_skill_budget` missing IDs.
- Archive dry-run and execute: `nix run path:$PWD#cairn -- archive adapt-openai-codex-guidance-skill-budget --root $PWD`, then `CAIRN_ARCHIVE_DATE=$(date +%F) ... --execute` — unblocked and completed to `cairn/archive/2026-07-02-adapt-openai-codex-guidance-skill-budget/`.
- Final validation: `nix run path:$PWD#cairn -- validate --root $PWD` — passed with 38 active changes.
- Roadmap validation after archive: `scripts/cairn/check-roadmap --root .` — passed with active dependency blockers only.

## Next-best validation for blocked rails

- Fix the existing `animus-adapters` clippy backlog outside `front/guidance.rs` before using crate-wide clippy as closeout evidence for this change.
- Fix the existing global Tracey missing-ID backlog before using full Tracey coverage as closeout evidence.
