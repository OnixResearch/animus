# Design: Adapt Clankers runtime patterns for Animus

## Context

Clankers is a large application agent with many product-edge surfaces. Animus is smaller and should stay embeddable. The useful transfer is not Clankers' TUI, daemon, Matrix, plugin, browser, or self-evolution surface. The useful transfer is the set of boundary patterns that keep an agent runtime reviewable: policy in Nickel, explicit request lifecycle ownership, safe receipt metadata, neutral replay history, injected provider/tool services, and small drift rails.

This design adapts those patterns into Animus terms while preserving functional-core / imperative-shell boundaries.

## Decisions

### 1. Nickel owns Steel runtime policy

**Choice:** Add an Animus-owned Nickel policy under `policy/steel-runtime/` for Steel profile definitions, script bindings, runtime budgets, allowed host actions, fallback modes, and receipt redaction.

**Rationale:** Animus already made Nickel the source of truth for repo-owned config. Steel policy is configuration with contracts, so it belongs in Nickel rather than hard-coded Rust literals or untyped JSON.

**Boundary:** Rust may consume checked generated policy data or typed test fixtures. Runtime code must not evaluate Nickel at turn execution time.

### 2. Runtime lifecycle is documented before widening behavior

**Choice:** Add a request-lifecycle document for the Animus runtime path before expanding provider, persistence, or session behavior.

**Rationale:** The current runtime surface is intentionally small. A lifecycle document names which type owns each transition so future work does not fork state machines across `animus-std`, `animus-steel`, adapter crates, and future runtime crates.

### 3. Receipts are metadata, not transcripts

**Choice:** Introduce typed receipt DTOs for runtime and adapter observations. Receipts expose status, issue code, labels, counts, and BLAKE3 content hashes while redacting raw prompts, provider payloads, credentials, tool bodies, full paths, and oversized outputs.

**Rationale:** Receipts give embedders deterministic evidence without turning observability into a data leak.

### 4. Session ledger stays neutral

**Choice:** Define a neutral session ledger/resume contract over model-visible messages, tool results, summaries, and safe replay metadata. Product-owned storage remains outside the reusable core.

**Rationale:** Animus should support resume and context rebuild without importing a daemon, UI, database, provider, or concrete JSONL session store.

### 5. Provider and tool dispatch are injected services

**Choice:** Runtime dispatch must call injected provider/model/tool services through adapter contracts and must fail closed when a required service is absent, unavailable, denied by capability policy, or returns malformed output.

**Rationale:** Concrete providers and tools belong at product edges. The reusable runtime should own the decision-to-dispatch state transition, not client construction or credential lookup.

### 6. Verification rails are small and copyable

**Choice:** Add focused checks for policy drift, Steel profile validity, receipt redaction, session-ledger replay, and dispatch missing-service behavior. These checks should emit reviewable evidence and be included in flake checks once stable.

**Rationale:** Clankers' checked-brick pattern is valuable, but Animus should keep smaller rails that match its current scope.

## Risks / Trade-offs

- Adapting too much from Clankers can pull application concerns into Animus. Keep product-edge features out of scope.
- Receipt fields can ossify too early. Start with a minimal DTO and add fields only when a test or embedder contract needs them.
- Nickel-generated policy JSON can drift from Nickel source. Add a focused drift check before consuming generated policy from runtime tests.
- Session ledger design can accidentally become a storage API. Keep persistence as an injected shell concern and test replay over in-memory fixtures first.
- Provider/tool dispatch can become too generic. Begin with the existing model/tool adapter contracts and add richer provider-neutral DTOs only after concrete gaps appear.
