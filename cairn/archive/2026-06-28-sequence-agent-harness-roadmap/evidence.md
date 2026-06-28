# Evidence: sequence-agent-harness-roadmap

## Implemented artifacts

- `docs/roadmap.md` documents roadmap roles, prerequisite tiers, overlap owners, and closeout rules.
- `cairn/roadmap.ncl` is the typed Nickel source of truth for active-change classification, tier order, dependencies, and overlap owners.
- `scripts/cairn/check-roadmap` exports the Nickel roadmap and runs the Rust validator in `scripts/cairn/check-roadmap.rs`.
- `cairn/roadmap-fixtures/*.ncl` provide negative fixtures for cycles, unknown changes, unknown prerequisites, unclassified active changes, and product surfaces that bypass foundational safety contracts.

## Validation

- `scripts/cairn/check-roadmap --root .`
  - Result: pass.
  - Evidence summary before archive: `roadmap changes: 59`, `active changes: 59`, `blocked dependents: 145`, `roadmap validation: ok`.
  - Evidence summary after archive: `roadmap changes: 58`, `active changes: 58`, `blocked dependents: 139`, `roadmap validation: ok`.
  - Negative fixtures passed for `ROADMAP_DEPENDENCY_CYCLE`, `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE`, `ROADMAP_UNKNOWN_CHANGE_ID`, `ROADMAP_UNKNOWN_PREREQUISITE_ID`, and `ROADMAP_UNSAFE_PRODUCT_SURFACE`.
- `rustc --edition=2021 -D warnings scripts/cairn/check-roadmap.rs`
  - Result: pass.
- `nix run .#cairn -- validate --root .`
  - Result before archive: pass with `valid: true`, `changes: 59`, `specs_validated: 60`, and no issues.
  - Result after archive: pass with `valid: true`, `changes: 58`, `specs_validated: 59`, and no issues.
- `nix run .#cairn -- gate proposal sequence-agent-harness-roadmap --root .`
  - Result: `PASS`.
- `nix run .#cairn -- gate design sequence-agent-harness-roadmap --root .`
  - Result: `PASS`.
- `nix run .#cairn -- gate tasks sequence-agent-harness-roadmap --root .`
  - Result: `PASS`.
- `nix run .#cairn -- sync sequence-agent-harness-roadmap --root .` and `--execute`
  - Result: dry-run and execute receipts passed; the generated sync plan was non-blocking.
  - Manual accepted-spec repair was required because the sync manifest hash did not change the accepted spec content; accepted spec now contains the `agent_harness_runtime.roadmap_sequence.*` requirement IDs.
- `CAIRN_ARCHIVE_DATE=2026-06-28 nix run .#cairn -- archive sequence-agent-harness-roadmap --root . --execute`
  - Result: pass; active change moved to `cairn/archive/2026-06-28-sequence-agent-harness-roadmap/`.

## Non-claims

This evidence does not mark dependent changes archive-ready. The roadmap checker reports blocked dependents while allowing the roadmap structure itself to pass; each dependent change still needs its prerequisite contracts accepted, archived in the same batch, or explicitly waived with a replacement requirement ID.
