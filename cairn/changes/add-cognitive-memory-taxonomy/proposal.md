# Proposal: Add cognitive memory taxonomy

## Why

Animus already has safe, normalized memory records, but current generic kinds collapse several distinct access patterns into broad labels. Agent memory references distinguish episodic, semantic, procedural, and reflection memory because each kind is retrieved, compacted, evaluated, and explained differently.

Animus should add an Animus-owned taxonomy layer so hosts can classify memories precisely without adopting any notebook implementation, framework dependency, hidden memory behavior, or provider-specific store.

## What Changes

- Extend memory descriptors/envelopes with explicit cognitive category metadata for episodic, semantic, procedural, reflection/pitfall, instruction, fact/preference, summary, and candidate records.
- Keep taxonomy metadata non-authoritative: category labels influence planning and evidence only after scope, admission, freshness, redaction, and policy checks pass.
- Add safe projection and compatibility mapping so older generic kinds continue to validate while new category-aware hosts can expose richer records.
- Add focused positive and negative fixtures for category validation, compatibility mapping, prompt planning, and leak rejection.

## Impact

- **Files**: future adapter DTOs, memory planner core, docs, tests, and Cairn artifacts.
- **Testing**: positive category mapping and negative invalid-category, unauthorized-category, stale, redacted, and raw-body fixtures.
- **Non-claims**: this does not add a memory database, automatic extraction, graph store, vector index, or product-specific memory semantics.
