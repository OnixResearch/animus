# Design: Add cognitive memory taxonomy

## Context

The current `MemoryRecordEnvelope` is deliberately backend-neutral. It records safe identity, generic kind, scope, evidence, freshness, confidence, redaction, hashes, budget, supersession, and conflicts. That contract is sufficient for safe prompt influence, but it cannot distinguish an event episode from a distilled fact, a reusable procedure, or a self-reflection lesson without overloading labels.

## Decisions

### 1. Taxonomy is safe metadata

**Choice:** Add a bounded category field, or equivalent category descriptor, that classifies records as episodic, semantic, procedural, reflection/pitfall, instruction, fact/preference, summary, candidate, or unsupported.

**Rationale:** Retrieval, compaction, routing, and evaluation can use category metadata while storage and extraction stay host-owned.

### 2. Categories never grant authority

**Choice:** Category labels are evaluated only after existing admission, scope, freshness, redaction, and prompt-budget checks.

**Rationale:** A memory marked procedural or semantic must not bypass policy, guidance, or capability gates.

### 3. Existing records remain compatible

**Choice:** Provide deterministic compatibility mapping from current generic record kinds into the richer taxonomy, with unknown or unsupported categories failing closed when policy requires explicit categories.

**Rationale:** Embedders can adopt taxonomy gradually without breaking current hosts.

### 4. Evidence remains body-free

**Choice:** Receipts and ledgers expose category labels, safe IDs, source labels, counts, policy hashes, and BLAKE3 body hashes only.

**Rationale:** Taxonomy improves explainability without turning evidence into a transcript or secret store.

## Risks / Trade-offs

- Too many categories can make hosts over-classify low-value memories. Keep category names small and policy-driven.
- Misclassification can degrade retrieval quality. Evaluation rails should compare category-aware retrieval against current generic planning.
- Compatibility mapping can hide ambiguity. Unknown category labels should be explicit diagnostics, not silent coercions.
