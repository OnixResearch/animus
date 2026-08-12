# Evidence: adapt-hermes-review-report-contracts

## Baseline

- Baseline Cairn gates before implementation: `nix run .#cairn -- gate proposal adapt-hermes-review-report-contracts --root .`, `gate design`, and `gate tasks` — PASS.
- Baseline roadmap validation: `scripts/cairn/check-roadmap --root .` — PASS, with unrelated active prerequisite blockers reported as `BLOCKED` advisories.
- Baseline focused adjacent review tests: `nix develop -c cargo test -p animus-adapters --locked codex::tests` — PASS, 8 tests.

## Implementation

- Added pure no-std `crates/animus-adapters/src/review.rs` with structured review report parsing over in-memory markdown.
- Exported review report DTOs and helpers from `crates/animus-adapters/src/lib.rs`.
- Added parser support for `VERDICT: PASS | WARN | FAIL`, bounded section metadata with BLAKE3 body hashes, and metric findings only under `## Findings`.
- Added finding tag validation for `class`, `scope`, `route`, and `promoted=yes|no`.
- Added explicit append-only metric record construction with safe metadata, report hashes, finding summary hashes, and a BLAKE3 record hash.
- Added read-only metrics summaries and repeated-finding promotion grouping without lifecycle mutation.
- Added `docs/hermes-review-report-contracts.md` and README linkage.

## Checks

- Focused review tests: `nix develop -c cargo test -p animus-adapters --locked review::tests` — PASS, 6 tests.
- Adapter crate tests: `nix develop -c cargo test -p animus-adapters --locked` — PASS, 347 tests.
- Workspace tests: `nix develop -c cargo test --workspace --locked` — PASS.
- Formatting: `nix develop -c cargo fmt --check` — PASS.
- Cairn validation: `nix run .#cairn -- validate --root .` — PASS.
- Evidence safety: `scripts/cairn/check-evidence-safety --root .` — PASS.

## Blocked rails

- `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` remains globally blocked by pre-existing `animus-adapters` lint diagnostics outside this review module. Captured diagnostics: `target/cairn/review-report-clippy.log`.
- The captured clippy log contains no `crates/animus-adapters/src/review.rs` diagnostics after local fixes.

## Non-claims

This evidence does not adopt Hermes metric storage as the only backend, does not auto-dispatch reviewers, does not promote findings automatically, and does not make review metrics a core scoring mechanism.
