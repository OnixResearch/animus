# Schedule reliability

Animus provides an optional pure schedule contract behind the `schedule-reliability` feature. Hosts own clocks, timers, durable storage, prompt queues, and delivery.

## Surfaces

- `user_heartbeat` is the one visible recurring instruction that the user owns.
- `agent_recurring` contains session-local recurring instructions that the agent owns.
- `scheduled` contains one-time or recurring jobs that a user or host owns.

An agent cannot replace, pause, or clear the user heartbeat. Each durable session admits no more than one user heartbeat.

## Claim order

`evaluate_schedule_tick` returns a claim transition for a due tick. The host must write this transition to durable state before it calls `admit_delivery_after_claim`.

If a crash occurs after the claim, recovery classifies the tick as `claimed_undelivered`. Recovery does not deliver the same tick again.

This rule can lose one prompt after a crash. It does not establish exactly-once execution.

## Missed ticks

The core calculates the missed count from a host-provided logical tick. It returns one policy decision for the complete backlog:

- `fire_once` requires one durable claim and one catch-up delivery.
- `skip` records one skipped catch-up decision.
- `pause` records one paused catch-up decision.

The core rejects missed counts above the policy bound. It never creates one delivery for each missed tick.

## Recovery and evidence

`ScheduleState::recover` accepts caller-loaded jobs for one session. It rejects duplicate jobs, cross-session jobs, invalid ownership, invalid intervals, and a second user heartbeat.

Schedule receipts contain policy, state, job, surface, decision, clock, count, and non-claim metadata. They exclude prompt bodies, tool output, environment values, physical paths, and exactly-once claims.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-schedule-reliability --root .
nix build .#checks.x86_64-linux.schedule-reliability --no-link -L
```

This contract does not establish timer accuracy, real-time delivery, prompt outcome correctness, production readiness, or release eligibility.
