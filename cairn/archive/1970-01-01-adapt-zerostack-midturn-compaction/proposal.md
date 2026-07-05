# Proposal: Adapt Zerostack mid-turn compaction for Animus

## Why

Zerostack distinguishes estimated session tokens from provider-reported prompt pressure and supports opt-in mid-turn compaction when a single long turn approaches the model context window. It also stores long tool outputs separately while preserving bounded head/tail context. Animus already has ledger and compaction work; this Cairn captures the Zerostack-specific mid-turn and artifactization patterns to adapt.

The goal is safe continuation under context pressure without hidden transcript rewrites or unbounded receipts.

## Scope

- Define real prompt pressure calibration from provider usage metadata and cached-token semantics.
- Define mid-turn compaction triggers with context window, reserve budget, threshold, and continuation policy.
- Define stop/compact/resume plans that append explicit ledger summary entries rather than mutating history invisibly.
- Define long tool-output artifactization with bounded model-visible head/tail windows and BLAKE3 artifact hashes.
- Add failure behavior when summarizer, ledger, or artifact services are unavailable.

## Impact

- **Files**: compaction policy DTOs, ledger summary entries, runtime continuation plans, artifact descriptors, tests, and docs.
- **Testing**: positive calibration and mid-turn planning tests; negative missing summarizer, malformed provider usage, orphaned tool output, unsafe artifact receipt, and disabled compaction tests.
- **Non-claims**: this does not copy Zerostack session JSON, require its provider stack, or hide compaction as an in-memory-only transcript rewrite.
