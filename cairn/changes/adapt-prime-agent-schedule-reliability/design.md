## Context

Prime Agent exposes three scheduling surfaces: `/heartbeat` (one visible recurring instruction owned by the user), `rlm_heartbeat` (multiple agent-managed recurring instructions internal to the session), and `prime-agent schedule` (one-time or cron prompts from user or automation). Jobs persist per session and continue while the UI is detached. Two reliability rules stand out in the docs: due ticks are claimed before delivery so a crash does not replay an uncertain prompt, and missed ticks are coalesced rather than accumulated into an unbounded backlog. The agent-created heartbeat cannot replace or clear the user-owned one.

Animus has `goose-scheduled-recipes` for scheduled workflow jobs and the messaging layer's steer/follow-up delivery modes. Neither owns crash-safety semantics for due ticks, catch-up behavior after downtime, or the user-versus-agent ownership split for recurring prompts.

Completion requires claim-before-delivery and coalescing as pure durable-state transitions, plus the three-surface ownership split. A scheduler that delivers-then-records, accumulates missed ticks, or lets agent schedules mutate user schedules is false completion.

## Decisions

### Decision: claim is a durable transition before delivery

**Choice:** The schedule core consumes a host clock reading and durable job state, and returns a two-step plan: first a claim transition that marks the tick as claimed in durable state, then a delivery instruction. The host applies the claim to durable storage before delivering. Recovery treats a claimed-but-undelivered tick as claimed, not as due.

**Rationale:** Recording after delivery is the classic double-fire bug: a crash between delivery and recording replays the prompt. Claim-first converts the failure mode from duplicate execution to at most one missed prompt, which coalescing then handles.

### Decision: missed ticks coalesce into one catch-up decision

**Choice:** When a job wakes with N missed ticks, the core returns one catch-up decision per policy (fire once, skip, or pause the job) rather than N deliveries. Policy names the catch-up rule per surface. Backlog is bounded by construction.

**Rationale:** A backlog of stale prompts after downtime is noise at best and destructive at worst. One explicit catch-up decision keeps behavior deterministic and reviewable.

### Decision: three surfaces with ownership typed in the job record

**Choice:** Every job record carries its surface (`user_heartbeat`, `agent_recurring`, `scheduled`) and owner identity. Mutation admission checks surface ownership: agent-created surfaces cannot mutate `user_heartbeat` jobs. The user surface admits exactly one visible heartbeat per session.

**Rationale:** The prime-agent ownership split prevents an agent from silently replacing the instruction the user thinks is running. Typing surface and owner in the record makes violations fail closed.

### Decision: schedules are per-session durable state

**Choice:** Job state lives in session-scoped durable state with domain-separated BLAKE3 identity. Recovery after restart replays durable state deterministically and re-derives due ticks from the injected clock. Cross-session schedules are out of scope.

**Rationale:** Session scoping matches the existing ledger boundary and keeps recovery a pure replay problem.

## Alternatives Considered

### Deliver-then-record with idempotency keys

Rejected because prompt delivery is not idempotent: a repeated prompt changes session state. Claim-first is the correct ordering for non-idempotent effects.

### Fire all missed ticks in order after downtime

Rejected because it creates unbounded backlogs and stale instructions. One policy-named catch-up decision is deterministic and bounded.

### One unified schedule surface without ownership

Rejected because it lets agent-created schedules replace user intent. The ownership split is a safety property, not an API nicety.

## Risks / Trade-offs

- Claim-first can lose a prompt on crash (claimed, never delivered). Mitigation: the loss is explicit in durable state and receipts; the alternative is duplicate execution, which is worse for non-idempotent prompts.
- Coalescing hides how many ticks were missed. Mitigation: catch-up receipts record the missed count as metadata.
- Host-injected clocks can be wrong. Mitigation: clock identity appears in claim receipts so hosts can audit time authority.

## Non-Claims

This design does not establish exactly-once execution, real-time guarantees, cross-session scheduling, timer accuracy, prompt outcome correctness, production readiness, or release eligibility.
