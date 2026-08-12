# Design: Type subagent capabilities

Subagent capability decisions should be explicit and machine-checkable in reusable runtime code, with string labels confined to adapter and policy interchange boundaries.

## Decisions

### 1. Typed capabilities are the runtime core form

**Choice:** Runtime capability resolution operates on typed capability variants such as read context, search, named tool use, named mutation intent, workspace read/write scope, model use, and external service use.

**Rationale:** Types make it harder to accidentally equate unrelated labels or accept wildcard authority.

### 2. Catalog validation bridges string labels

**Choice:** Adapter strings convert to typed capabilities only after normalization and catalog validation.

**Rationale:** Existing policy and receipt surfaces keep stable text labels while runtime logic receives validated structured values.

### 3. Denials identify the ceiling source

**Choice:** Denial diagnostics identify whether parent policy, recipe/profile ceiling, request ceiling, workspace scope, or host admission rejected a capability.

**Rationale:** Operators and tests need actionable denial reasons, not a generic intersection miss.

### 4. Compatibility remains explicit

**Choice:** Existing adapter DTOs continue to expose stable string labels, and conversion shims preserve old fixture behavior where safe.

**Rationale:** Embedders should not be forced to rewrite policy files before the typed runtime path is ready.
