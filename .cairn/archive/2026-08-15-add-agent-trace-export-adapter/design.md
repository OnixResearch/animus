## Context

The admitted session ledger already records a monotonic journal sequence per agent, and the evaluation core already references ledger traces, but no product surface exports a bounded semantic timeline. Tau-style trace exports demonstrate the pattern: dense causal timelines, lite content bounds, sequence authority, and content-free accounting with explicit non-claims.

## Decisions

### 1. Own the projection in a non-default adapter family

**Choice:** Add `agent_harness_runtime.trace_export.*` DTOs and pure validation in a new non-default no-std adapter family. Consumers feed already-loaded ledger projections and validated item records; the family never reads stores or contacts providers.

**Rationale:** This matches the adapter-first and functional-core posture. Completing projection DTOs without live discovery keeps the surface bounded and testable.

### 2. Split lite and full content modes

**Choice:** Lite mode bounds each semantic text or output item to a fixed non-zero byte ceiling and retains exact complete-content metrics. Full mode retains complete content. Mode is a validated export parameter, never a heuristic.

**Rationale:** An explicit fixed ceiling keeps the export bounded without losing measurement facts.

### 3. Journal sequence is the only in-agent causality

**Choice:** Exported items carry authoritative journal sequence and qualified absolute append time within an agent. Cross-agent wall-clock time is emitted as a readable overview and never used for cross-agent causality.

**Rationale:** Wall clocks are not a causality basis across independently journaled agents. This claim boundary mirrors the ledger's own authority.

### 4. Keep accounting content-free

**Choice:** The performance accounting projection emits provider-prompt accounting, fixed-point cache ratios, and missing-evidence coverage markers only, with one summary per included agent. No semantic text, reasoning, tool output, or provider payload bodies.

**Rationale:** A content-free report supports cost and cache review without duplicating the transcript.

### 5. Fail closed before projection acceptance

**Choice:** Validation rejects unknown kinds, malformed records, non-monotonic journal sequences, unbounded items, duplicate summaries, and unsupported shapes before any projection is accepted. Receipts are metadata-only and reject leak sentinels.

**Rationale:** Deterministic admission and bounded receipts are the evidence culture requirements for every projection surface.

## Risks / Trade-offs

- Snapshots move forward over time; consumers must not infer cross-agent causality from wall time. The sequence requirement and docs make this explicit.
- A fixed lite ceiling can truncate long tool outputs. Metrics retain the exact lengths so consumers can detect truncation.
- This export is not provider billing reconciliation or a full OTLP-aware trace pipeline.
