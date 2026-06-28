# Evidence: dogfood-cairn-lifecycle-evidence

## Implemented artifacts

- `docs/cairn-lifecycle.md` documents lifecycle layout, change thresholds, validation/gate workflow, sync/archive closeout, and bounded evidence conventions.
- `scripts/cairn/check-evidence-safety` scans committed lifecycle evidence summaries and negative fixtures without printing matched private material.
- `cairn/evidence-safety-fixtures/*.md` cover raw prompt, credential, provider payload, full private path, and unbounded output leak classes.

## Validation

- `scripts/cairn/check-evidence-safety --root .`
  - Result: pass.
  - Negative fixtures passed for `EVIDENCE_CREDENTIAL`, `EVIDENCE_FULL_PRIVATE_PATH`, `EVIDENCE_PROVIDER_PAYLOAD`, `EVIDENCE_RAW_PROMPT`, and `EVIDENCE_UNBOUNDED_OUTPUT`.
- `nix run .#cairn -- validate --root .`
  - Result before archive: pass with `valid: true`, `changes: 57`, `specs_validated: 58`, and no issues.
  - Result after archive: pass with `valid: true`, `changes: 56`, `specs_validated: 57`, and no issues.
- `scripts/cairn/check-roadmap --root .`
  - Result before dogfood archive: pass with `roadmap changes: 57`, `active changes: 57`, `blocked dependents: 134`, and `roadmap validation: ok`.
  - Result after dogfood archive: pass with `roadmap changes: 56`, `active changes: 56`, `blocked dependents: 128`, and `roadmap validation: ok`.
- `nix run .#cairn -- gate proposal dogfood-cairn-lifecycle-evidence --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- gate design dogfood-cairn-lifecycle-evidence --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- gate tasks dogfood-cairn-lifecycle-evidence --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- sync dogfood-cairn-lifecycle-evidence --root .` and `--execute`
  - Result: dry-run and execute receipts passed; the generated sync plan was non-blocking.
  - Manual accepted-spec repair was required because the sync manifest hash did not change accepted spec content; accepted spec now contains the `agent_harness_runtime.cairn_dogfood.*` requirement IDs.
- `CAIRN_ARCHIVE_DATE=2026-06-28 nix run .#cairn -- archive dogfood-cairn-lifecycle-evidence --root . --execute`
  - Result: pass; active change moved to `cairn/archive/2026-06-28-dogfood-cairn-lifecycle-evidence/`.

## Non-claims

This change does not require every typo fix to open a Cairn change and does not replace normal Rust, Nix, formatting, linting, or feature checks. It defines when Cairn lifecycle evidence is required and how durable summaries stay bounded.
