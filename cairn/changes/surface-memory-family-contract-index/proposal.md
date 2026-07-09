# Proposal: Surface memory family in contract index

## Why

Animus already has memory adapter contracts, tests, conformance archives, and docs, but `docs/contracts-index.md` does not list memory as a maintained public adapter family. That makes the public surface harder to discover and weakens the connection between memory DTOs, docs, validation rails, and reference-codebase decisions.

Animus should surface the memory family in the contracts index and record Agent Memory Techniques as a bounded reference for taxonomy, retrieval, evaluation, and production patterns without adopting its notebooks or framework dependencies.

## What Changes

- Add `family:memory` to the maintained contracts index with owner module, stability posture, authority boundary, safe receipt behavior, focused surface, and checks.
- Add memory rows to the adapter conformance manifest where required so the index rail can verify the family.
- Add bounded reference documentation for Agent Memory Techniques and non-adoption claims.
- Add focused validation that docs, conformance metadata, and README references stay consistent.

## Impact

- **Files**: `README.md`, `docs/contracts-index.md`, `cairn/adapter-conformance-index.ncl`, docs, tests or scripts, and Cairn artifacts.
- **Testing**: positive index/conformance checks and negative missing-family, unsafe-reference, stale-check, and leak fixtures.
- **Non-claims**: this does not implement new memory behavior by itself or adopt external Python notebooks, managed services, APIs, datasets, or licenses beyond bounded reference usage.
