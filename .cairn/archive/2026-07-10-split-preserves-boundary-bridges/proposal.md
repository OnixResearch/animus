# Proposal: Split Preserves boundary bridges by payload family

## Why

`animus-preserves` is a useful serialization bridge, but it currently depends on both adapter and runtime surfaces. Embedders that only need adapter DTO serialization should not have to pull runtime event/report concepts, and runtime-event consumers should not inherit provenance/session or policy-visible bridge commitments unless they opt in.

Splitting Preserves bridges by payload family makes serialization a set of small wire-format Lego blocks with explicit coverage tiers and validation boundaries.

## What Changes

- Define family-specific Preserves bridge crates or modules for adapter boundary DTOs, runtime events/reports, provenance/session records, and policy-visible DTOs.
- Keep `animus-preserves` as a compatibility facade that re-exports family bridges during migration.
- Move schema catalog ownership to family entries with explicit coverage tier, dependency set, safe compatibility metadata rules, and focused fixtures.
- Add positive and negative round-trip, malformed input, unsupported schema, unsafe metadata, unknown-field, and leak-rejection tests per bridge family.

## Impact

- **Surfaces**: `animus-preserves`, schema catalog, Preserves docs, component catalog, runtime facade inventory, optional root `preserves` feature.
- **Testing**: family-specific text/binary round trips, facade parity, metadata leak rejection, schema mismatch rejection, and Cairn gates.
- **Non-claims**: this does not change the Preserves wire format silently, publish unstable review-only fixtures as stable contracts, or add runtime/provider/tool authority.
