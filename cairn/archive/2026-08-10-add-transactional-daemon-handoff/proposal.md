## Why

Animus has daemon role contracts, detach-safe subscriptions, session restore plans, and durable active-turn recovery. It lacks a transactional replacement handoff for one live session worker.

A replacement can otherwise start from stale state, claim an unreleased session, or survive after a cancelled timeout.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` provides an ISC-licensed authenticated handoff reference. Animus will adapt its ordering and cancellation invariants without adopting its Bash supervisor or Lisp process model.

## What Changes

- Add a pure strict-idle decision over active work, queues, children, pending commits, repairs, confirmations, and cancellation state. r[agent_harness_runtime.daemon_handoff.idle]
- Add a monotonic handoff transaction with prepared, paused, released, claimed, ready, cancelled, and restored outcomes. r[agent_harness_runtime.daemon_handoff.transaction]
- Bind replacement claims to one current handoff identity, lease generation, session identity, and authenticated local endpoint. r[agent_harness_runtime.daemon_handoff.claim]
- Require owned replacement teardown before the old worker can reclaim a released lease. r[agent_harness_runtime.daemon_handoff.cancellation]
- Preserve the composer draft and pending-input vault generation without transferring PTYs or raw terminal state. r[agent_harness_runtime.daemon_handoff.session_state]
- Add safe receipts, deterministic state-machine tests, and a non-default daemon smoke. r[agent_harness_runtime.daemon_handoff.evidence] r[agent_harness_runtime.daemon_handoff.verification]

## Success Criteria

- Handoff starts only from an exact strict-idle snapshot.
- At most one worker owns the session lease at each admitted generation.
- A stale or late replacement cannot claim a cancelled handoff.
- The old worker exits only after authenticated replacement readiness.
- Timeout rollback occurs only after the full replacement process tree terminates.
- Tile integration remains a separate Tile-owned change after the Animus contract stabilizes.

## False Completion

The change is incomplete if handoff uses a Boolean takeover flag without identity, generation, readiness, and cancellation evidence.

Process separation, local authentication, or a passing smoke does not prove sandboxing, remote security, or lossless process migration.

## Impact

- **Reusable runtime**: strict-idle and transaction reducers, canonical identities, and receipt projection.
- **Daemon adapter**: versioned handoff DTOs and service ports.
- **Host shell**: process spawning, local transport, authentication, leases, deadlines, teardown, and exit ordering.
- **Downstream**: a later Tile change can map its managed-agent surfaces without moving session authority into Tile.

## Out of Scope

- PTY transfer, remote handoff, cross-machine migration, transparent provider-session transfer, or automatic process upgrade.
- Security-sandbox claims, global daemon defaults, or whole-product readiness.

## Affected Specs

- `agent-harness-runtime`: daemon handoff idle, transaction, claim, cancellation, session state, evidence, and verification.
