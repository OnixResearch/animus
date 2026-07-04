# Design: Decouple runtime loop fabric contracts from Steel

## Context

The runtime crate already separates pure transition planning from the imperative shell that calls fabrics and adapters. The remaining composability issue is that the fabric seam itself names Steel values. This turns one implementation choice into the reusable runtime contract.

## Decisions

### 1. Runtime owns the fabric-neutral DTOs

**Choice:** Add neutral `RuntimeLoopTurn`, `RuntimeLoopDecision`, and related request/response conversion values under `animus-runtime` or a tiny implementation-free contracts module.

**Rationale:** The runtime transition core is the authority for what a loop fabric must decide. It should not inherit the shape of a specific script engine.

### 2. Steel becomes an adapter, not the contract

**Choice:** Keep `animus-steel` as the canonical scriptable fabric and implement a Steel adapter that converts between Steel decisions and neutral runtime decisions before dispatch.

**Rationale:** This preserves the current supported path while allowing embedders to provide Rust, WASM, replay, or test fabrics with the same trait.

### 3. Feature boundaries describe optional Steel authority

**Choice:** Steel convenience runners and policy activation helpers are available only when the Steel integration feature is enabled. The runtime core and fake-fabric tests must compile without requiring Steel types.

**Rationale:** Feature boundaries should reflect composition choices and dependency costs.

### 4. Runtime validation still happens after fabric output

**Choice:** All fabric outputs, including Steel-derived outputs, continue through runtime validation and typed adapter DTOs before model or tool dispatch.

**Rationale:** Decoupling must not let a new fabric bypass adapter contracts, middleware, policy denial, safe failure reports, or max-turn guards.

### 5. Migration is explicit

**Choice:** Existing public Steel runner names are preserved behind compatibility reexports or receive focused deprecation notes while new neutral APIs are documented first.

**Rationale:** Embedders need a clear path without ambiguous duplicate APIs.

## Risks / Trade-offs

- Moving DTOs can churn public names. Keep compatibility shims and facade inventory updates in the same change.
- Neutral DTOs may initially mirror Steel too closely. Tests with a fake non-Steel fabric should force implementation-independent naming and behavior.
- Feature-gating Steel may expose accidental imports. Add compile checks and facade inventory validation so the dependency boundary stays durable.
