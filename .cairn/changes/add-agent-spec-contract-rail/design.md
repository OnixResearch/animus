## Context

Animus separates a functional core from an imperative shell and models tools and adapters behind typed boundaries. Agent actions already carry receipts in parts of the harness. A spec-contract rail extends that pattern: when an agent edits a behavioral model, the loop must run the stacked checking contract and bind the verdict to the action.

The model-checking surface (Choregraph) owns the contract and receipts; Animus consumes them. It does not execute the engine itself.

## Decisions

### Decision: The rail runs before handoff

**Choice:** Add a spec-contract stage that runs after a model edit and before the loop accepts the action result. The update is admitted only with a fresh, matching, passing verdict receipt.

**Rationale:** The rail exists to prevent unverified model drift from silently passing the loop.

### Decision: The checker is a pinned adapter

**Choice:** Consume the external checker through a pinned adapter implementing the bounded checking contract. The adapter translates loop inputs, invokes the checker, and returns verdict and identity facts.

**Rationale:** Animus must not own or guess the engine; the adapter keeps the authority boundary explicit.

### Decision: Receipts bind the edited model exactly

**Choice:** The loop receipt binds the post-edit model hash, checker identity and version, declared bounds, and verdict. A receipt for a different model hash is stale for this action.

**Rationale:** A receipt that does not match the edited artifact cannot vouch for it.

### Decision: Handoff fails closed

**Choice:** Missing, stale, mismatched, or unsupported-checker outcomes deny handoff and produce a deterministic denial receipt for the agent.

**Rationale:** The loop must surface verification gaps, not hide them.

### Decision: Core stays pure

**Choice:** Receipt admission, staleness checks, and verdict classification are pure core logic. Checker invocation and file reads are shell effects.

**Rationale:** Animus core remains testable without processes or files.

## Functional core / imperative shell split

- **Pure core**: receipt admission, model-hash matching, staleness checks, verdict classification, and denial construction.
- **Imperative shell**: invoke the pinned checker adapter, read model artifacts, and persist loop receipts.

## Risks / Trade-offs

- Repeated checking adds loop latency. Cache verdicts by model hash and require refresh only when the model hash changes.
- The checker may be unavailable. Deny handoff with an unsupported-checker class instead of degrading to silent pass.
- Bounds limit what "passing" means. Record the exact bounds in the receipt and keep the bounded-verdict non-claim.

## Non-Goals

- No in-loop model checker, SMT engine, or model authority in Animus.
- No claim that a passing verdict proves model correctness or implementation parity.
- No change to the checking contract; Animus consumes it.
- No ambient checker or tool authority.
