## Context

`clankers_cost_model_routing` owns cost-aware model selection and receives host-supplied usage summaries. Provider streaming parsers already normalize usage records into canonical frames. Neither surface projects per-agent spend. The accounting projection must stay pure and deliberately non-authoritative.

## Decisions

### 1. Pure attribution and pricing core

**Choice:** A no-std pure core accepts validated usage records, a safe model catalog projection, and policy inputs, then returns per-agent summaries and an approximate subtree figure. It contacts no providers and reads no files.

**Rationale:** Attribution and arithmetic are deterministic decisions. Keeping them pure matches the functional-core rule and makes positive and negative tests direct.

### 2. Price per serving model with a documented fallback

**Choice:** Each accepted usage record is priced against the model that served it. A documented fallback price steps in for models without registry price data. Missing required prices deny the summary entry with a typed diagnostic.

**Rationale:** The estimate must degrade explicitly, never silently. This gives consumers a stable costing rule and an observable gap.

### 3. Subtree scope follows authenticated session creators

**Choice:** A per-agent subtree projection follows only authenticated same-session creator relationships, retains completed descendants, and resets with the active session. Unattributed usage is reported separately, not invented into a subtree.

**Rationale:** An approximate per-agent figure must not manufacture parentage from ambiguous data.

### 4. Explicit non-authoritative label

**Choice:** Every projection carries a non-authoritative marker and documents that it is an eyeballed estimate, not provider billing reconciliation.

**Rationale:** This blocks overclaiming. The claim boundary is part of the contract.

### 5. Fail closed and bounded

**Choice:** Validation rejects malformed records, unknown models, missing required prices, negative totals, overflow, and duplicate agent summaries. Counters are bounded. Receipts are metadata-only.

**Rationale:** Deterministic admission and bounded receipts are the evidence culture baseline.

## Risks / Trade-offs

- Fallback pricing can mislead if the fallback is far from the true price. The documented fallback and missing-price diagnostics make the gap visible.
- Subtree attribution depends on trustworthy creator relationships. Unattributed usage remains separate and explicit.
- The estimate is not reconciliation. Hosts with billing APIs keep reconciliation authority.
