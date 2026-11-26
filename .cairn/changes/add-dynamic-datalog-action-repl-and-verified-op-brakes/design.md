# Design: Dynamic Datalog Action Contract & Verified Operation Brakes

## 1. Bounded Session Datalog Action Contract (`animus-adapter-rlm-kernel`)
- Define deterministic, bounded `no_std` action primitives (`InsertFact`, `RetractFact`, `QueryRelation`, `Checkpoint`) tagged with `EpistemicLayer::{Certain, Shadow}` and returning explicit 3-way `DeltaStatus::{Active, Dead, Absent}` receipts.
- Enforce strict non-promotion: `UntrustedToolOutput` may populate `Shadow` relations only and can never mutate `Certain` facts without an explicit ratified transition.

## 2. Untrusted Framing & Post-Action Diff Verification (`animus-acp-stdio-host`, `animus-std-agent-guardrail`)
- Wrap all external tool responses in explicit `UntrustedToolOutput` envelopes before planner ingestion (ARTEX boundary pattern).
- Require typed post-action observed-state diffs matching declared operation intent (Protocolis pattern), failing closed on unexpected side effects.

## Verification Plan
- `cargo test -p animus-adapter-rlm-kernel -p animus-acp-stdio-host -p animus-std-agent-guardrail` covering incremental relation reuse across chained turns, rejection of unframed untrusted output entering `Certain`, and fail-closed denial on post-action diff divergence.
