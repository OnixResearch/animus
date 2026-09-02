# Tasks

## Phase 1: Baseline and prerequisites

- [ ] [serial] Record tool-stage, runtime-authority, and safe-receipt baselines before core changes. r[animus.change_scope_handoff.scope]
- [ ] [serial] Record accepted non-claim boundaries for agent receipts and keep them unchanged. r[animus.change_scope_handoff.nonclaims]

## Phase 2: Core and shell

- [ ] [serial] Define change-scope, handoff-decision, continuation, artifact, and receipt DTOs in a focused pure family. r[animus.change_scope_handoff.scope] r[animus.change_scope_handoff.artifacts]
- [ ] [serial] Implement pure scope-shape validation, handoff-decision recording, continuation admission, and receipt payload construction. r[animus.change_scope_handoff.handoff] r[animus.change_scope_handoff.continuation]
- [ ] [serial] Add the host-side observation that presents scopes for review and records the developer decision without effecting product authority. r[animus.change_scope_handoff.handoff]

## Phase 3: Evidence and isolation

- [ ] [parallel] Add positive scope-shape, recorded-handoff, bounded-continuation, and artifact-identity fixtures. r[animus.change_scope_handoff.fixtures]
- [ ] [parallel] Add negative malformed-scope, missing-decision, unbounded-continuation, artifact-leak, and overclaim fixtures. r[animus.change_scope_handoff.fixtures] r[animus.change_scope_handoff.artifacts] r[animus.change_scope_handoff.nonclaims]
- [ ] [serial] Run focused tests before and after changes, no-std builds, Clippy with warnings denied, octet, Cairn validation and gates, and relevant Nix checks. r[animus.change_scope_handoff.fixtures]
