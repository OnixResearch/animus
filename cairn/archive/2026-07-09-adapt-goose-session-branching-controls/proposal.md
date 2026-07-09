# Proposal: Adapt Goose session branching controls

## Why

Goose exposes practical session controls for editing earlier user messages, forking from an edited point, duplicating sessions, and preserving the original branch for comparison. Animus already has neutral session tree, provenance, branch query, checkpoint, and headless control contracts, but it does not yet have a reviewed contract for user-visible session history rewrites and branch creation.

Animus should adapt the useful control shape as ledger-owned branch planning and safe receipts, not as UI behavior or mutable storage shortcuts.

## What Changes

- Define UI-neutral session edit, fork, duplicate, and branch-label request DTOs over existing session ledger identities.
- Add pure planning for branch creation, edit-in-place suffix discard, duplicate-session metadata, continuation target selection, and conflict diagnostics.
- Require explicit policy admission, stale-ledger checks, checkpoint or rollback evidence when destructive edits discard suffix history, and safe branch receipts before hosts mutate durable storage.
- Keep live processes, provider credentials, raw transcripts, tool outputs, and host-specific session databases outside reusable contracts.

## Impact

- **Files**: future session ledger DTOs, pure branch planning helpers, receipt validation, docs, tests, and Cairn artifacts.
- **Testing**: positive fork and duplicate plans plus negative stale-cutpoint, unsafe edit body, suffix-discard denial, duplicate branch identity, and receipt leak fixtures.
- **Non-claims**: this does not implement a Desktop edit button, SQLite migration, process resume, transcript export, provider replay, or any storage mutation from reusable crates.
