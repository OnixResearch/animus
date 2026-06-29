# Design: Adapt Zerostack fresh-context task agents for Animus

## Context

Zerostack's task/subagent UX is useful because it lets the main agent delegate cross-file investigation to a fresh context. Animus should adapt this as a typed product-surface contract that depends on existing runtime events, tool contracts, background scheduling, and permission policy.

## Decisions

### 1. Task requests are bounded descriptors

**Choice:** A fresh-context task request declares goal, admitted context references, tool capability profile, token/tool budgets, timeout/cancel policy, and result schema.

**Rationale:** Subagents should not inherit ambient authority or unrestricted context accidentally.

### 2. Side questions are read-only by construction

**Choice:** A side-question request can use only policy-approved read/search tools and is excluded from the main model-visible ledger unless the host explicitly admits its result.

**Rationale:** `/btw`-style ergonomics should not perturb the main run or mutate state.

### 3. Scheduler is injected

**Choice:** Host adapters own task scheduling, provider selection, concurrency, and cancellation. Runtime core validates descriptors and normalizes outcomes.

**Rationale:** Process-global mutable subagent config would violate Animus' injected service boundary.

### 4. Results are attributable summaries

**Choice:** Task results include child-run identity, context references, safe evidence summaries, budget usage, and uncertainty markers.

**Rationale:** The main agent must know what it can rely on without receiving raw unbounded child transcripts by default.

## Risks

- Fresh-context tasks can hide important evidence. Require result schemas with cited context references and uncertainty markers.
- Parallel tasks can burn budget. Enforce policy budgets and cancellation receipts.
