# Evidence: wire-cairn-policy-validation

## Implemented artifacts

- `cairn-policy/generated/cairn-policy.json` is checked in as the repo-local generated Cairn runtime policy artifact.
- `cairn-policy/README.md` documents the source identity, refresh command, drift-check command, default validation command, and explicit-policy validation command.
- `scripts/cairn/check-policy` compares the checked-in generated policy with a canonical generated source when available and reports a source identity without embedding local-only full paths in durable evidence.

## Validation

- `scripts/cairn/check-policy --root .`
  - Result: pass.
  - Source identity: `local-cairn:24381f7`.
  - Diagnostic: `policy drift: ok`.
- `nix run .#cairn -- validate --root .`
  - Result before archive: pass with `valid: true`, `changes: 58`, `specs_validated: 59`, and no issues.
  - Result after archive: pass with `valid: true`, `changes: 57`, `specs_validated: 58`, and no issues.
- `nix run .#cairn -- validate --root . --policy cairn-policy/generated/cairn-policy.json`
  - Result before archive: pass with `valid: true`, `changes: 58`, `specs_validated: 59`, and no issues.
  - Result after archive: pass with `valid: true`, `changes: 57`, `specs_validated: 58`, and no issues.
- `nix run .#cairn -- gate proposal wire-cairn-policy-validation --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- gate design wire-cairn-policy-validation --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- gate tasks wire-cairn-policy-validation --root .`
  - Result: `PASS`; policy hash `a210ed841d260e0bf026f3c67ec810216b6409cfe6036f9132b51cc4e3176618`.
- `nix run .#cairn -- sync wire-cairn-policy-validation --root .` and `--execute`
  - Result: dry-run and execute receipts passed; the generated sync plan was non-blocking.
  - Manual accepted-spec repair was required because the sync manifest hash did not change accepted spec content; accepted spec now contains the `agent_harness_runtime.cairn_policy.*` requirement IDs.
- `CAIRN_ARCHIVE_DATE=2026-06-28 nix run .#cairn -- archive wire-cairn-policy-validation --root . --execute`
  - Result: pass; active change moved to `cairn/archive/2026-06-28-wire-cairn-policy-validation/`.
- `scripts/cairn/check-roadmap --root .`
  - Result after archive: pass with `roadmap changes: 57`, `active changes: 57`, `blocked dependents: 134`, and `roadmap validation: ok`.

## Non-claims

This change does not alter Cairn policy semantics. Runtime validation consumes generated JSON and does not execute Nickel inside Animus validation.
