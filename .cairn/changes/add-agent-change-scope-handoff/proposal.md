# Proposal: Agent change-scope handoff with explicit ownership

## Why

Animus runs agents that edit files and run validation. There is no typed boundary where a proposed change set is presented to the developer for explicit ownership, and where continuation is bounded to an approved subset. MoonBit's SeekMoon work shows an ADE whose first priority is review of agent-produced change sets: the developer understands the changes and takes ownership before the task is complete. Animus should make that handoff a first-class, evidence-preserving boundary.

## What Changes

- Record every agent change as a typed change scope binding files, hunks, the driving task, and validation obligations. r[animus.change_scope_handoff.scope]
- Present each scope as an explicit ownership handoff; nothing is complete until the review decision is recorded. r[animus.change_scope_handoff.handoff]
- Bound and hash review artifacts such as diff identities and decisions; exclude raw transcripts. r[animus.change_scope_handoff.artifacts]
- Allow agent continuation only within the approved subset; rejected items form a fresh scope. r[animus.change_scope_handoff.continuation]
- Add positive and negative fixtures for scope shape, handoff, continuation, artifact, and overclaim cases. r[animus.change_scope_handoff.fixtures]

## Impact

- **Core**: a focused family owns scope shape validation, handoff decision recording, continuation admission, and receipt payload construction without host effects.
- **Shell**: the host owns file observations, review presentation, and process orchestration.
- **Existing owners**: tool stages, runtime-authority seals, and safe-receipt contracts keep their claims.

## Lifecycle Prerequisites

None. The boundary composes existing Animus contract and receipt capabilities.

## Out of Scope

- VCS mutation, review UIs, policy enforcement, or product-correctness claims.
- Replacement of tool stages, runtime sealing, or existing safe-receipt rails.

## Affected Specs

- `change-scope-handoff`: scope, handoff, artifacts, continuation, fixtures, and non-claims.
