## Why

Prime Agent lets running agents and retained subagents exchange direct messages with three delivery modes: `auto` steers a busy target and delivers to an idle target, `steer` injects into active work, and `follow_up` waits for the current turn to finish. Every send returns a receipt that is either `delivered` or `queued`. The daemon derives sender identity and enforces size, rate, and pending-queue limits. Animus has an interrupt classifier (`goose-message-queue-interrupts`) and communication seams (Iroh, Syndicate), but no delivery-mode taxonomy and no delivery receipts.

Without the taxonomy, a message that steers active work and a message that waits politely are indistinguishable in contracts. Without receipts, a sender cannot tell accepted-for-later from actually-delivered. Animus needs both as pure decisions with fail-closed limits.

## What Changes

- Add a family roster contract where the host derives sender identity and membership; the model never claims an identity. r[agent_harness_runtime.messaging_receipts.roster]
- Add the three delivery modes (`auto`, `steer`, `follow_up`) as pure routing decisions over target state. r[agent_harness_runtime.messaging_receipts.modes]
- Add typed delivery receipts distinguishing `delivered` (reached an idle target's context) from `queued` (accepted for later delivery). r[agent_harness_runtime.messaging_receipts.receipts]
- Add fail-closed size, rate, and pending-queue limits enforced before routing. r[agent_harness_runtime.messaging_receipts.limits]
- Add a Nickel-authored messaging policy for roster shape, mode defaults, limits, receipt fields, and non-claims. r[agent_harness_runtime.messaging_receipts.profile]
- Add metadata-only messaging receipts. r[agent_harness_runtime.messaging_receipts.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.messaging_receipts.verification]

## Impact

- **Files**: new focused messaging contract module in the orchestration adapter family, Nickel policy, fixtures, focused check script, docs; composition seams in `goose-message-queue-interrupts` and the Iroh/Syndicate seams.
- **Reusable boundary**: routing decisions are pure over supplied roster and target state; transports, delivery, and queues stay host-owned.
- **Runtime composition**: the interrupt classifier consumes delivery mode as a typed input; broadcast stays family-scoped and policy-gated.
- **Testing**: positive fixtures for all three modes, receipt classes, and roster enforcement; negative fixtures for spoofed identity, over-limit messages, and ambient routing; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Transports, network discovery, encryption, or cross-machine delivery; existing Iroh and Syndicate seams keep those.
- Delivery guarantees beyond `delivered` and `queued`; no exactly-once or ordering claims.
- Daemon process management, which the daemon-workers change owns.
- Proving message content correctness, agent cooperation, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: messaging boundary, policy, roster, modes, receipts, limits, evidence, and verification.
