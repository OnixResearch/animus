# Design: Add memory routing contracts

## Context

The current memory adapter family separates source import, storage, retrieval/search, extraction, embedding/search support, admission, update/delete, and instruction-source behavior. The pure retrieval planner selects from already-loaded candidates. It does not decide whether a query should search episodic records, semantic facts, procedural records, reflection memories, a graph adapter, a vector adapter, or a safe local file adapter.

## Decisions

### 1. Routing is a pure plan over safe signals

**Choice:** Add route request and route candidate DTOs that contain only safe labels, category hints, operation intent, allowed scopes, required adapter roles, declared adapter availability, confidence, and policy identity.

**Rationale:** Hosts can use any classifier or rule engine, but reusable logic sees only validated typed inputs.

### 2. Read and write routes are explicit

**Choice:** Route plans distinguish read/search, source/import, extraction, durable store, update/delete, and prompt-influence intents.

**Rationale:** Query routing and write routing have different authority and admission requirements; conflating them risks accidental persistence or prompt influence.

### 3. No ambient fallback

**Choice:** If a selected route requires an unavailable, unsupported, denied, or missing adapter role, the route outcome records a fail-closed diagnostic and no alternate backend is discovered implicitly.

**Rationale:** Memory must remain host-injected and policy-bound.

### 4. Route receipts are safe

**Choice:** Route receipts include route IDs, operation intent, selected or skipped adapter roles, safe reason labels, policy hash, and redaction state, never classifier prompts, raw memory bodies, credentials, provider payloads, or full paths.

**Rationale:** Operators need to debug route selection without leaking hidden memory or provider inputs.

## Risks / Trade-offs

- Route planning can duplicate retrieval policy if boundaries are unclear. Keep routing about adapter selection and retrieval planning about candidate inclusion.
- Classifier confidence may be unreliable. Treat confidence as an input to deterministic policy, not as authority.
- Multi-route plans can increase latency. Budgets and adapter availability should be visible in the route plan.
