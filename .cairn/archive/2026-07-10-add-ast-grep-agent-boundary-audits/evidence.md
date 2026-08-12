# Evidence: add-ast-grep-agent-boundary-audits

## Baseline

- `nix run .#cairn -- gate proposal add-ast-grep-agent-boundary-audits --root .` passed before implementation.
- `nix run .#cairn -- gate design add-ast-grep-agent-boundary-audits --root .` passed before implementation.
- `nix run .#cairn -- gate tasks add-ast-grep-agent-boundary-audits --root .` passed before implementation.
- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` reported `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE add-ast-grep-agent-boundary-audits` before implementation. This was expected for the untracked active change and is resolved by archiving the change rather than editing the pre-existing user-modified roadmap.

## Implementation evidence

- Added `cairn/ast-grep-agent-boundary-audits.ncl` as the Nickel source of truth for audit profiles, scan scopes, inventory-only posture, rule ids, receipt schema, BLAKE3 rule-bundle identity, and non-claim labels.
- Generated `docs/ast-grep-agent-boundary-audits.md` from Nickel and linked it from `README.md`.
- Added inventory-only ast-grep rules for ambient filesystem calls, ambient process commands, unsafe blocks, panic-style macros, and direct provider-client construction.
- Added positive and negative fixtures for each rule before any warning or blocking promotion.
- Added `scripts/cairn/check-ast-grep-agent-boundary-audits` and `scripts/cairn/check-ast-grep-agent-boundary-audits.rs` to validate profile/doc drift, rule metadata, fixtures, ast-grep version capture, BLAKE3 rule-bundle hash capture, bounded finding counts, and receipt non-claims.
- Wired `nix build .#checks.x86_64-linux.ast-grep-agent-boundary-audits --no-link -L` and updated `docs/safety-contract.md`.

## Verification

- `nickel export --format raw --field docs_markdown cairn/ast-grep-agent-boundary-audits.ncl > docs/ast-grep-agent-boundary-audits.md` passed.
- `scripts/cairn/check-ast-grep-agent-boundary-audits --root .` passed with positive and negative fixture scans, inventory-only source scans, ast-grep version capture, BLAKE3 rule-bundle identity, and counts-only receipt output.
- `nix build .#checks.x86_64-linux.ast-grep-agent-boundary-audits --no-link -L` passed after staging rail files so the flake source included them.
- `nix run .#cairn -- sync add-ast-grep-agent-boundary-audits --root .` dry-run was unblocked.
- `nix run .#cairn -- sync add-ast-grep-agent-boundary-audits --root . --execute` completed, but the accepted spec text was not promoted by the sync tool; the requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified with `rg 'ast_grep_agent_audits'`.

## Non-claims

- ast-grep findings are structural candidates only. This change does not prove harness safety, adapter correctness, provider behavior, Steel policy enforcement, runtime safety, exploitability, release readiness, or production readiness.
- This change does not run codemods, edit generated config artifacts outside their Nickel source of truth, or make reusable crates depend on scan state.
