# Design: Turnstone-inspired intent validation contracts

## Context

Animus has policy, permission, confirmation, sandbox, and tool-dispatch gates. Turnstone's intent validation suggests a useful review envelope around pending tool calls: a deterministic heuristic verdict is available immediately, optional model review can arrive later, and verdict metadata is persisted for calibration. Animus should adapt the contract without adopting smart approvals.

## Decisions

### 1. Verdicts are evidence, not authority

**Choice:** `IntentVerdict` records a recommendation, confidence, risk level, summary, evidence references, and detector tier. Tool dispatch still requires normal capability, policy, approval, sandbox, mutation-window, and executor checks.

**Rationale:** A judge can tighten review but cannot become a second door into execution.

### 2. Heuristic rules are typed data with positive and negative fixtures

**Choice:** Rule descriptors carry stable IDs, match class, action class, risk, recommendation, safe rationale, and fixture examples. Rule evaluation is pure and deterministic.

**Rationale:** Risk classification must be reviewable and testable without model calls.

### 3. Optional reviewer output can only tighten

**Choice:** Optional model/classifier verdicts may deny, require review, or add evidence. They cannot approve a tool call that policy denies, lower deterministic critical/deny findings, bypass missing authority, or suppress confirmation.

**Rationale:** Learned checks are attackable and must not grant authority.

### 4. Batch admission sees verdict interactions

**Choice:** Verdicts bind to a tool-call batch identity and can request review when individually benign calls combine into an unsafe flow.

**Rationale:** Read-then-exfiltrate risks can appear only at batch level.

## Risks / Trade-offs

- Heuristic rules can grow into an unmaintained blocklist. Keep rules small, typed, and fixture-backed.
- Optional review can increase latency. Runtime should support pending/advisory verdict states without stalling safe deterministic denials.
- Free-text rationales are risky. Durable receipts should store safe reason labels and hashes, not raw judge prose.