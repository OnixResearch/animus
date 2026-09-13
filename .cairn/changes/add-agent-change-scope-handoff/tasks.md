# Tasks

## Phase 1: Baseline and prerequisites

- [x] [serial] Record tool-stage, runtime-authority, and safe-receipt baselines before core changes. r[animus.change_scope_handoff.scope]
- [x] [serial] Record accepted non-claim boundaries for agent receipts and keep them unchanged. r[animus.change_scope_handoff.nonclaims]

## Phase 2: Core and shell

- [x] [serial] Define change-scope, handoff-decision, continuation, artifact, and receipt DTOs in a focused pure family. r[animus.change_scope_handoff.scope] r[animus.change_scope_handoff.artifacts]
- [x] [serial] Implement pure scope-shape validation, handoff-decision recording, continuation admission, and receipt payload construction. r[animus.change_scope_handoff.handoff] r[animus.change_scope_handoff.continuation]
- [x] [serial] Add the host-side observation that presents scopes for review and records the developer decision without effecting product authority. r[animus.change_scope_handoff.handoff]

## Phase 3: Scope identity correction

- [x] [serial] Preserve the imported baseline and run maintained public-API controls before the identity repair. r[animus.change_scope_handoff.fixtures]
- [x] [serial] Count-frame each scope collection under the v2 digest domain and document legacy-identity refusal. r[animus.change_scope_handoff.scope]
- [x] [serial] Verify exact replay, cross-field refusal, collection changes, all hunk fields, legacy refusal, and pending-state preservation through public APIs. r[animus.change_scope_handoff.scope] r[animus.change_scope_handoff.handoff] r[animus.change_scope_handoff.fixtures]

## Phase 4: Evidence and isolation

- [x] [parallel] Add positive scope-shape, recorded-handoff, bounded-continuation, and artifact-identity fixtures. r[animus.change_scope_handoff.fixtures]
- [x] [parallel] Add negative malformed-scope, missing-decision, unbounded-continuation, artifact-leak, and overclaim fixtures. r[animus.change_scope_handoff.fixtures] r[animus.change_scope_handoff.artifacts] r[animus.change_scope_handoff.nonclaims]
- [ ] [serial] Run focused tests before and after changes, no-std builds, Clippy with warnings denied, octet, Cairn validation and gates, and relevant Nix checks. r[animus.change_scope_handoff.fixtures]

## Qualification status (2026-09-13)

The encoding repair at `71d7eb63547d1042fd2f71f31f446906dbcddb98` passes its six maintained controls and package/caller checks.
The full flake gate remains blocked by 214 runtime Octet findings, including preserved public `usize` signatures.
All three strict host selections pass with zero findings, warnings, and errors.
The primary comparison passes. No push, archive, or primary integration occurred.
The stale safe-receipt wrapper selects zero tests, while the direct current-owner command passes five cases.
The final task stays open. A default-off build is not proof that the runtime crate is `no_std`.
See `evidence/qualification-2026-09-13.md` for exact subjects, results, and the required scope decision.
