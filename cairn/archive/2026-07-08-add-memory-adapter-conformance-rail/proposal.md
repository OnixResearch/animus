# Proposal: Add memory adapter conformance rail

## Why

A generic memory adapter architecture is only useful if concrete backends can be swapped without changing reusable behavior. Existing agents demonstrate many backend shapes: no-op/disabled memory, local markdown directories, SQLite or Postgres state, vector stores, MCP memory tools, and remote memory services. Animus needs a conformance rail that proves those backend families preserve the same safety and prompt-influence semantics after normalization.

## What Changes

- Define a memory adapter conformance matrix with required positive and negative scenarios for each backend family.
- Add fixture-backed conformance cases for null/no-op, local file/markdown, SQL/document, vector or hybrid search, MCP/remote, extractor, and admission adapters.
- Require explicit unavailable, denied, malformed, stale, redacted, over-budget, conflict, and leak outcomes.
- Verify file paths, provider payloads, remote IDs, vector internals, and raw bodies do not leak into receipts or prompt influence ledgers.
- Keep concrete backend I/O in host shells; reusable conformance tests use in-memory DTOs or test-owned fixtures.

## Impact

- **Files**: future conformance matrix policy, fixtures, fake adapters, focused tests, docs, and Cairn artifacts.
- **Testing**: positive backend-equivalence cases and negative no-fallback, unsafe-receipt, malformed-record, missing-negative-coverage, and backend-unavailable fixtures.
- **Non-claims**: this does not require every backend to exist before the rail lands; unavailable or not-applicable entries are acceptable only with explicit safe evidence and reason.
