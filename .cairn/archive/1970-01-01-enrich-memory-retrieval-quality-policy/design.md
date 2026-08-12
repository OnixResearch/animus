# Design: Enrich memory retrieval quality policy

## Context

`plan_memory_retrieval` currently normalizes backend labels, validates relevance percentages, sorts candidates deterministically, and applies admission, scope, freshness, redaction, supersession, conflict, and budget filters. That creates a safe baseline. As memory adapters diversify, hosts need quality policy knobs without pushing ranking rules into backend-specific services or model prompts.

## Decisions

### 1. Quality policy is declarative

**Choice:** Add a retrieval quality profile that describes ordering posture and bounded weights for relevance, confidence, freshness, match reason, category priority, manual pins, and diversity groups.

**Rationale:** Hosts can tune retrieval without changing planner code paths or backend contracts.

### 2. Diversity is deterministic

**Choice:** Add diversity or near-duplicate grouping metadata that caps repeated evidence from the same group under a policy-defined posture.

**Rationale:** Retrieval should avoid spending all memory budget on duplicate facts while remaining explainable and testable.

### 3. Rerank evidence is optional and safe

**Choice:** Host-owned rerankers may provide safe rerank scores and reason labels, but reusable logic validates score bounds, freshness, source identity, and leak posture before use.

**Rationale:** Cross-encoder or model-based reranking can improve quality, but it must not introduce provider payloads or nondeterministic authority into the core.

### 4. Backend identity remains non-ranking metadata

**Choice:** Backend labels stay safe evidence and diagnostics; they must not influence model-visible ordering except through validated candidate scores or match reasons.

**Rationale:** Equivalent normalized records should remain equivalent across stores.

## Risks / Trade-offs

- Weight profiles can become hard to reason about. Keep the initial profile small and expose every skip/order reason in safe ledgers.
- Rerank scores can hide backend-specific behavior. Require normalized bounded scores and positive/negative equivalence tests.
- Diversity can suppress genuinely useful repeated evidence. Policies should record skipped duplicate groups explicitly.
