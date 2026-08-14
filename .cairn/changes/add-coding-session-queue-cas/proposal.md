# Proposal: Add compare-and-swap coding-session queue edits

## Why

The coding-session facade supports queued steering and follow-up mutation. Current Prime Agent now rejects queue edits when the expected text no longer matches.

A live headless, daemon, or terminal client can race another client or an active queue transition. Position-only edits can change the wrong item.

## What Changes

- Add queue mutation commands that bind entry identity, expected generation, and expected text identity. r[agent_harness_runtime.coding_session_queue_cas.commands]
- Apply queue edits and removals through a pure compare-and-swap reducer. r[agent_harness_runtime.coding_session_queue_cas.reducer]
- Return explicit conflict snapshots without mutating queue or transcript state. r[agent_harness_runtime.coding_session_queue_cas.conflicts]
- Project safe conflict events and receipts without queued text bodies. r[agent_harness_runtime.coding_session_queue_cas.evidence]
- Preserve a local convenience API only when one exclusive owner supplies a current snapshot. r[agent_harness_runtime.coding_session_queue_cas.compatibility]
- Add positive and negative race, stale, ordering, and leak tests. r[agent_harness_runtime.coding_session_queue_cas.verification]

## Prerequisites

- `agent_harness_runtime.coding_session_host.queues`
- `agent_harness_runtime.coding_session_host.functional_core`
- `agent_harness_runtime.pi_headless_protocol.queueing`
- `agent_harness_runtime.goose_message_queue.reducer`

## Impact

- **Crates**: session-core queue DTOs and reducers, coding-session facade, headless protocol, and frontend commands.
- **Compatibility**: external mutable clients must provide preconditions. Single-owner local callers can use a current-snapshot wrapper.
- **Evidence**: queue bodies remain private. Safe output uses identities, generations, positions, and status classes.

## Out of Scope

- Distributed consensus, multi-host queue replication, automatic conflict merging, last-writer-wins behavior, or production availability.

## Affected Specs

- `agent-harness-runtime`: queue commands, reducer, conflicts, evidence, compatibility, and verification.
