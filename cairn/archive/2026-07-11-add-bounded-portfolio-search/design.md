## Context

Animus has reusable child-run, worker-team, retrieval, and receipt contracts but no shared state that answers which mechanisms were explored, whether workers stayed independent, whether a reduction is genuinely simpler, why a route is blocked, what novelty permits reopening it, or whether adversarial review occurred before validation.

## Decisions

### 1. Keep the state machine in a pure functional core

**Choice:** `animus-runtime::portfolio_search` owns immutable DTOs and pure transition, selection, validation, and receipt functions. Contracts bind both source-policy and validator-policy identities. Hosts own all model, tool, clock, storage, file, process, network, and scheduler effects.

**Rationale:** Every orchestration rule can be tested from values without standing up a provider or runtime.

### 2. Make phases and terminal outcomes explicit

**Choice:** Model framing, independent exploration, synthesis, targeted redirection, adversarial audit, validation, validated result, exhaustion, and blocking as distinct phases. `Validated`, `Exhausted`, and `Blocked` are terminal for this core.

**Rationale:** Honest exhaustion and exact blockers must not be confused with verified success. `Validated` remains narrower than authoritative agent or repository completion.

### 3. Identify mechanisms with domain-separated BLAKE3 fingerprints

**Choice:** Every approach carries an ID, family label, mechanism fingerprint, claim fingerprint, safe references, gap strength, state, optional exact blocker, and next discriminating check. Duplicate mechanism fingerprints are rejected even when wording differs.

**Rationale:** Stable mechanism identity enables deterministic deduplication and bounded receipts without retaining raw prompts or model output.

### 4. Enforce delayed cross-pollination

**Choice:** Independent-exploration dispatches must not disclose prior approach IDs. Synthesis and later phases may cite prior approaches.

**Rationale:** Early anchoring creates correlated portfolios. The core can verify disclosed-reference metadata even though the host remains responsible for actual context isolation.

### 5. Treat equivalent or stronger reductions as blocked

**Choice:** Routes with `Equivalent` or `Stronger` gap strength require matching blocker classes and may not enter audit or validation. `Unknown` routes also remain ineligible until reclassified.

**Rationale:** A theorem-strength restatement is not evidence of proof progress.

### 6. Reopen only with novelty

**Choice:** A blocked route may reopen only when supplied evidence contains a new safe evidence reference or a changed mechanism fingerprint. The transition consumes the novelty and clears the old blocker.

**Rationale:** Rephrasing a known failure should not reset portfolio state.

### 7. Redirect dynamically

**Choice:** Underexplored live families are selected from the minimum observed live-route count, with deterministic lexical ordering and no fixed approach-count requirement.

**Rationale:** The portfolio grows or stops according to evidence and budget, not an arbitrary worker count.

### 8. Require adversarial evidence before validation

**Choice:** Audit findings name a bounded risk class, outcome, safe evidence reference, and residual uncertainty. Only a route with all required risk classes passing can enter validation; a matching pass receipt is then required for `Validated`.

**Rationale:** Synthesis is not validation, and an omitted failure mode must not silently pass.

### 9. Emit safe receipts only

**Choice:** Receipts contain hashes, safe labels, budget counters, phase, selected route, and evidence references. Unsafe path, raw prompt, raw tool output, or secret markers are rejected.

**Rationale:** Orchestration evidence should remain useful without becoming a transcript or authority leak.

## Risks / Trade-offs

- Family labels and gap-strength classes are supplied metadata; the core validates consistency, not natural-language semantics.
- Rejecting duplicate mechanism fingerprints depends on hosts deriving fingerprints from a canonical mechanism description.
- Required audit classes create a stricter path to validation; hosts can still end honestly as exhausted or blocked.
- This capability deliberately does not merge with authoritative completion receipts while that adjacent lifecycle change is active.
