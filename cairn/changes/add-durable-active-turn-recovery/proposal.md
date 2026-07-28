## Why

Animus can repair a dangling tool call after a transcript interruption. It also has bounded session replay and provider retry contracts.

The current contracts do not protect the earlier acceptance window. A host can accept a user message, lose the process before provider output, and lose the accepted work.

A similar gap exists for steering messages and partially prepared journal replacements. Recovery needs one durable, identity-bound protocol across these crash windows.

camelAI demonstrates useful active-turn journal mechanics. Animus will adapt the invariants behind its own Rust, Nickel, BLAKE3, authority, and receipt boundaries.

## What Changes

- Add a pure active-turn recovery core for accepted messages, active markers, journal snapshots, steering queues, recovery plans, and terminal decisions. r[agent_harness_runtime.active_turn_recovery.boundary]
- Add typed Nickel policy for finite journal bounds, replacement rules, recovery modes, corruption posture, retention, and required non-claims. r[agent_harness_runtime.active_turn_recovery.policy]
- Require durable acceptance evidence before provider dispatch and before a steering message becomes eligible for delivery. r[agent_harness_runtime.active_turn_recovery.journal]
- Require full replacement preparation before one atomic journal transaction replaces the prior valid checkpoint. r[agent_harness_runtime.active_turn_recovery.journal]
- Preserve one stable turn and projection identity across recovery without using timestamps or random identifiers as content identity. r[agent_harness_runtime.active_turn_recovery.identity]
- Compose recovery with interrupted tool repair, provider retry, session authority, and cancellation without rerunning interrupted tools. r[agent_harness_runtime.active_turn_recovery.recovery]
- Add metadata-only recovery receipts and deterministic positive and negative crash-window evidence. r[agent_harness_runtime.active_turn_recovery.evidence] r[agent_harness_runtime.active_turn_recovery.verification]

## Success Criteria

- An accepted user message remains recoverable after failure before the first provider event.
- A steering message remains recoverable after durable acceptance and before runtime delivery.
- An interrupted replacement leaves either the old complete checkpoint or the new complete checkpoint.
- Recovery keeps the same turn identity and never reruns an interrupted tool.
- Corrupt, partial, stale, conflicting, or over-budget state blocks continuation with a typed outcome.
- Positive and negative crash-window fixtures pass through production core and host-adapter entrypoints.

## False Completion

The change is not complete if it protects only dangling tool calls, writes after provider dispatch, skips corrupt rows, or relies on timestamps.

A write loop without an explicit transaction is not atomic evidence. A passing model continuation is not proof that the accepted work remained durable.

## Impact

- **Reusable runtime**: pure planning, validation, identity, and receipt projection only.
- **Host shell**: explicit durable store, transactions, synchronization, clock observations, provider calls, cancellation, and cleanup.
- **Configuration**: typed Nickel policy with generated data revalidated by Rust.
- **Existing owners**: interrupted tool repair, provider retry, session authority, ledger replay, tools, and completion remain authoritative.
- **Testing**: deterministic crash-window fixtures, host-store fakes, corruption cases, dependency guards, and focused Cairn and Nix rails.

## Out of Scope

- A default database, global session store, browser history, TUI rendering, provider-native resume, or distributed consensus.
- Automatic tool reruns, provider fallback, authority expansion, silent corruption repair, or transcript deletion.
- Claims for store correctness, power-loss durability, exactly-once remote effects, provider acceptance, or production readiness.

## Affected Specs

- `agent-harness-runtime`: active-turn recovery boundary, policy, journal ordering, identity, recovery, evidence, and verification.
