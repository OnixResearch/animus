# Design: Adapt vLLM Semantic Router recipes for Animus

## Context

vLLM Semantic Router treats collaboration as a serving primitive: the caller sees one model identity while the router selects a bounded algorithm such as confidence escalation, ratings, ReMoM, fusion, or role workflows. Animus should adapt the concept as typed, policy-governed model-router recipes that run through existing injected model/provider boundaries.

## Decisions

### 1. Router recipes are typed contracts, not prompt conventions

**Choice:** Define Animus-owned recipe contracts for `single`, `confidence`, `ratings`, `remom`, `fusion`, and `workflow` route families. Each recipe declares allowed model roles, output contract, budget limits, concurrency limits, quorum, synthesis/finalizer role, repair behavior, and fallback policy.

**Rationale:** The useful vLLM idea is making collaboration feel like one model while keeping topology and budgets explicit. Typed contracts keep that reviewable and testable.

### 2. Signal projection is pure

**Choice:** Request signals are projected by deterministic functions over bounded, already-available metadata such as task shape labels, risk labels, contract pressure, context pressure, latency/cost bands, locality constraints, user policy hints, and catalog capabilities.

**Rationale:** Recipe selection should not inspect credentials, call providers, run tools, or parse unbounded prompt text in hidden shell logic.

### 3. Selection policy chooses recipes before provider dispatch

**Choice:** A selection core consumes projected signals, validated recipe policy, budget state, and safe model catalog projections, then returns a selected recipe plan or typed denial.

**Rationale:** Routing facts should select the collaboration pattern before any backend call, keeping provider execution subordinate to policy.

### 4. Execution plans are bounded and host-executed

**Choice:** Reusable crates define execution-plan DTOs with max candidates, max parallelism, min-success quorum, timeouts, cancellation classes, max synthesis attempts, and fallback behavior. Hosts execute the plan through injected model/provider services.

**Rationale:** Multi-model collaboration needs concurrency and timeouts, but reusable runtime code must not own async executors, clients, clocks, credentials, or network authority.

### 5. Output contracts gate success

**Choice:** Collaborative routes must validate the final response contract after synthesis, finalization, or fallback. Optional repair may run only when the recipe policy declares a bounded repair role and attempt limit.

**Rationale:** Ensembles are harmful if they break API or automation contracts. Rust validation remains the final gate.

### 6. Receipts collapse traces safely

**Choice:** Receipts expose recipe ID, signal buckets, selected model labels, bounded counts, quorum class, fallback class, policy hash, duration buckets from host summaries, and redaction markers. Raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, and unbounded traces are forbidden.

**Rationale:** Operators need observability while callers still receive one clean response and committed evidence remains safe.

## Risks / Trade-offs

- Router recipes can overlap workflow and subagent features. Keep this change focused on model-router execution behind one model-service surface; product-facing subagents and workflow commands remain separate changes.
- Quality gains are task-shaped and require Animus-owned evaluation. This change records contracts only and does not claim frontier-model benchmark parity.
- Concurrency can become expensive quickly. Hard caps, budget policy, and denial paths are mandatory rather than optional tuning.
