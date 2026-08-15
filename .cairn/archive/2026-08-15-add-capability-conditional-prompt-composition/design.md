## Context

Model-runtime profiles compose exact per-model prompt, skill, memory, and tool fragments. The goose-acp interop already computes the intersection of capability ceilings and descriptor-available tools for a bridge. The composition surface needs the same discipline as an explicit invariant, not a bridge-specific coincidence.

## Decisions

### 1. Availability is an explicit composition input

**Choice:** Prompt composition receives an explicit resolved availability set: tools and skills available for the current turn, derived from extension defaults, model compatibility, policy rules, and the selected role. Composition never infers availability from prompt text.

**Rationale:** Inference from text is fragile. An explicit set makes the decision testable and fail-closed.

### 2. Render fragments conditionally

**Choice:** A fragment renders only when every named tool or skill is in the availability set. A fragment that names a missing capability is dropped, and the composition reports the omission in its receipt.

**Rationale:** Conditional rendering keeps instructions aligned with what can actually run this turn.

### 3. Negative fixtures are mandatory

**Choice:** The invariant includes a negative fixture in which a role names a missing tool and must not produce instructions for it. Missing-tool fragments are a correctness failure, not a style issue.

**Rationale:** The hard-rule baseline requires positive and negative tests together for every validation behavior.

### 4. Fail closed and bounded

**Choice:** Unresolvable capability inputs and dangling references produce a typed diagnostic. Composition is denied before any prompt is emitted. Receipts are metadata-only and reject raw prompt bodies.

**Rationale:** Deterministic admission and bounded receipts match the composition contract culture.

## Risks / Trade-offs

- A missing tool drops its instructions entirely. The receipt records the omission so hosts can decide whether to block the turn.
- Availability computation must stay host-owned. This change defines the contract, not the discovery mechanism.
