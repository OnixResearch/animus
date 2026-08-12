## Why

Prime Agent's scheduled prompts have two reliability mechanisms that Animus lacks. First, due ticks are claimed before delivery, so a crash never replays an uncertain prompt. Second, missed ticks are coalesced rather than accumulated into an unbounded backlog. Prime Agent also separates three scheduling surfaces with distinct ownership: the user's visible heartbeat, agent-created internal heartbeats, and general one-time or cron schedules. The agent-created surface cannot replace or clear the user-owned heartbeat.

Animus has `goose-scheduled-recipes` for scheduled workflow jobs, but no crash-safety semantics for due ticks and no ownership split between user and agent schedules. A naive scheduler can replay a prompt after a crash (double execution) or fire a backlog of stale ticks after downtime. Animus needs both failure modes excluded by contract.

## What Changes

- Add the three scheduling surfaces with distinct ownership: user heartbeat, agent-created recurring prompts, and general one-time or cron jobs; agent surfaces cannot mutate user-owned schedules. r[agent_harness_runtime.schedule_reliability.surfaces]
- Add claim-before-delivery: a due tick is atomically claimed in durable state before its prompt is delivered, so a crash mid-delivery never replays an uncertain prompt. r[agent_harness_runtime.schedule_reliability.claim_before_delivery]
- Add missed-tick coalescing: ticks missed during downtime collapse into one catch-up decision per job instead of an accumulating backlog. r[agent_harness_runtime.schedule_reliability.coalesce]
- Add durable per-session schedule state with deterministic recovery after restart. r[agent_harness_runtime.schedule_reliability.persistence]
- Add a Nickel-authored schedule policy for surfaces, intervals, catch-up rules, limits, and non-claims. r[agent_harness_runtime.schedule_reliability.profile]
- Add metadata-only schedule receipts for claims, deliveries, and coalescing decisions. r[agent_harness_runtime.schedule_reliability.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.schedule_reliability.verification]

## Impact

- **Files**: new focused schedule contract module, Nickel policy, fixtures, focused check script, docs; composition seam in `goose-scheduled-recipes`.
- **Reusable boundary**: schedule decisions are pure over supplied clock readings and durable state; actual timers, persistence, and prompt delivery stay host-owned.
- **Runtime composition**: `goose-scheduled-recipes` consumes claim and coalescing decisions; heartbeat delivery consumes the existing steer/follow-up messaging modes.
- **Testing**: positive fixtures for claim ordering, coalescing, recovery, and surface ownership; negative fixtures for replay after crash, backlog accumulation, and cross-surface mutation; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Real timers, cron daemons, or wall-clock access inside reusable crates; clocks are host-injected.
- Exactly-once execution claims across arbitrary host failures; the claim rule bounds replay, it does not abolish it.
- Cross-session or cross-machine scheduling.
- Proving prompt timeliness, task outcomes, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: schedule boundary, policy, surfaces, claim-before-delivery, coalescing, persistence, evidence, and verification.
