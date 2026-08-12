# Proposal: Adapt Clankers neutral session ledger for Animus

## Why

Animus needs resume and replay semantics that do not depend on a daemon, TUI, database, concrete JSONL store, provider crate, or Clankers transcript type. Clankers' reusable session-ledger core demonstrates the right pattern: small model-visible DTOs, safe replay metadata, unsupported-entry failures, and pure deterministic replay. Animus should define its own version before adding host persistence.

## What Changes

- Add Animus-owned neutral ledger DTOs for user, assistant, tool, summary, usage, receipt, and unsupported entries.
- Implement pure replay that returns model-visible context and safe metadata counts.
- Add typed replay errors for unsupported entries, missing sessions, missing ledger services, duplicate IDs, missing parents, and cycles where branch queries are in scope.
- Keep persistence, IDs, clocks, filesystem, and host storage in the std shell or host adapters.
- Add positive and negative replay fixtures.

## Impact

- **Files**: possible core/provenance module or crate, tests/fixtures, docs, Cairn artifacts.
- **Testing**: deterministic replay fixtures and fail-closed unsupported/missing-service tests.
- **Non-claims**: this does not import Clankers session storage, Automerge, daemon attach replay, or product transcript compatibility.