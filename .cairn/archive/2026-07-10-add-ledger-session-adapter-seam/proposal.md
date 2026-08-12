# Proposal: Add a ledger and session adapter seam

## Why

`docs/request-lifecycle.md` still names receipts and ledger persistence as future host-owned seams. Animus has provenance, session-tree, branching, restore, checkpoint, and compaction contracts, but embedders do not yet have one explicit Lego block for host-owned session storage, replay planning, restore admission, and safe ledger receipts.

A ledger/session adapter seam lets hosts persist or replay sessions without giving reusable crates ambient storage authority or implying a global session store.

## What Changes

- Define host-injected ledger/session service contracts, store descriptors, session identity DTOs, replay plans, restore plans, mutation records, and safe receipts.
- Keep replay and restore planning pure over already-loaded ledger entries and policy; hosts own file/database/network storage, freshness checks, locking, and writes.
- Add positive fixtures for safe replay/restore planning and negative fixtures for missing services, stale snapshots, unsafe raw transcripts, credential leaks, private paths, conflicting branches, and ambient global-store fallback.
- Update request lifecycle docs and dogfood readiness non-claims to identify the seam.

## Impact

- **Surfaces**: runtime provenance/session modules, adapter service traits, std host persistence planning, request lifecycle docs, component catalog, safe receipt checks.
- **Testing**: pure replay/restore tests, missing-service fail-closed tests, receipt leak rejection, catalog checks, and Cairn gates.
- **Non-claims**: this does not add default persistence, a global session directory, database migrations, daemon attach, durable storage backend, native resume behavior, or production restore claims.
