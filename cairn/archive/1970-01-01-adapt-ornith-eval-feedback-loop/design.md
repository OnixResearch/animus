# Design: Adapt Ornith offline evaluation feedback loop for Animus

## Context

Ornith's asynchronous RL details are training-specific, but the operational lesson applies to Animus: scaffold changes should be evaluated against bounded tasks with enough metadata to know what changed and when evidence became stale. Animus already has batch trajectory, receipt, and dataset-export tracks; this change defines the scaffold-feedback layer that can sit on top of those without mutating runtime policy automatically.

## Decisions

### 1. Evaluation plans are deterministic data

**Choice:** Define offline evaluation plans over task-set identity, scaffold revision, model route, verifier identity, trust-boundary policy hash, tool-surface hash, sampling profile, resource labels, run count, and output format.

**Rationale:** A plan should be reviewable before execution and replayable enough to compare runs without depending on ambient provider or filesystem state.

### 2. Receipts capture comparability, not transcripts

**Choice:** Evaluation receipts record safe comparison metadata: model ID, scaffold hash, verifier hash, policy hashes, context-window class, sampling labels, timeout/resource labels, run count, aggregate status, and redaction state.

**Rationale:** Scaffold evaluation needs reproducible evidence, but storing raw prompts, provider payloads, hidden tests, or tool outputs would create leakage and benchmark-contamination risks.

### 3. Feedback candidates require review

**Choice:** Evaluation may produce feedback candidates such as promote, demote, revise, or investigate, but acceptance requires a host review outcome and a matching scaffold admission path.

**Rationale:** Offline self-improvement should be an evidence pipeline, not autonomous runtime policy mutation.

### 4. Freshness is explicit

**Choice:** Evidence freshness compares current model route, scaffold hash, verifier hash, trust-boundary hash, tool-surface hash, runtime version label, and sampling profile against the receipt. Mismatches mark evidence stale or reduced-weight according to policy.

**Rationale:** Ornith's staleness weighting addresses off-policy training data. Animus can adapt the same principle for operational evidence without implementing RL.

### 5. Benchmark claims stay out of scope

**Choice:** The first contract supports local comparison and adoption review only. Public benchmark claims, model leaderboards, cloud runners, and provider-specific eval APIs require separate changes.

**Rationale:** Animus needs safe internal evidence before it can make external performance claims.

## Risks / Trade-offs

- Too much receipt detail can leak task content. Keep receipts to hashes, labels, counts, statuses, and redaction markers.
- Too little detail makes comparisons meaningless. Include the minimum route, policy, verifier, scaffold, sampling, and resource identities needed for staleness checks.
- Feedback candidates can feel like automatic improvement. Require host review and scaffold admission before any runtime influence.
- Batch runner dependencies can pull in product concerns. Keep this change to DTOs, pure planning, and safe receipts until local execution services exist.
