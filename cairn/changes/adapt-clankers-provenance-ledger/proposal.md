# Proposal: Adapt Clankers provenance ledger patterns for Animus

## Why

Animus needs provenance and resume evidence that fits its own crate boundaries rather than importing a large application runtime or adopting an external interchange format as the internal source of truth. Clankers has already split reusable session-ledger, artifact-hash, redacted-receipt, tool-host, and effect-contract patterns away from its TUI/daemon/product edge. Toolpath offers a useful public provenance document shape, but its crates and CLI are better treated as optional interchange than as Animus core architecture.

This change makes the decision explicit: build an Animus-owned provenance ledger and safe receipt model, adapt only bounded Clankers patterns that match Animus' functional-core / imperative-shell split, and keep Toolpath compatibility behind an optional adapter.

## What Changes

- Define Animus-owned provenance/session-ledger DTOs for model-visible messages, summaries, usage, safe receipts, unsupported entries, typed facts, artifact hashes, redaction classes, actor labels, trace IDs, and parent/branch relationships.
- Adapt Clankers' neutral ledger replay, append-only safe fact ledger, BLAKE3 artifact identity, redaction classes, tool-host outcomes, effect requests, and safe receipt patterns without importing Clankers desktop storage, Automerge, daemon, TUI, plugin, Matrix, or self-evolution surfaces.
- Treat Toolpath as an optional std-only interchange adapter that can export/import redacted Animus provenance as Toolpath Graph/Path/Step documents when explicitly requested.
- Add positive and negative tests for deterministic replay, branch/dead-end queries, safe receipt construction, redaction leak rejection, unsupported-entry failures, optional Toolpath export boundaries, and dependency isolation.
- Update docs so maintainers know which parts of Clankers and Toolpath are bounded references and which are out of scope.

## Impact

- **Files**: possible new provenance/ledger modules or crates, docs under `docs/`, optional Toolpath adapter module or feature, tests/fixtures, README reference note, and Cairn evidence.
- **Testing**: focused unit tests for ledger replay, typed fact sanitization, BLAKE3 artifact hashes, branch queries, redaction failures, optional Toolpath export, and dependency-boundary checks.
- **Non-claims**: this change does not implement a daemon, TUI, Automerge session store, Pathbase upload, Toolpath cache/resume CLI, GitHub/Git derivation, Matrix/P2P routing, browser tooling, or self-evolution workflow.
