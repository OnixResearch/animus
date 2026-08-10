## Why

Animus durably accepts active work and models steering and follow-up queues. Restart recovery still lacks one operator-visible plan for pending input that was accepted but not consumed.

Without that plan, a host can lose queue provenance, submit recovered input automatically, or replay the same input more than once.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` provides an ISC-licensed recovery-vault reference. Animus will adapt its failure invariants behind existing Rust, BLAKE3, session, authority, and receipt contracts.

## What Changes

- Add a pure pending-input vault model for active accepted input, steering, follow-up, command, and attachment-reference classes. r[agent_harness_runtime.pending_input_vault.boundary]
- Preserve exact ordering, provenance, conversation scope, and domain-separated BLAKE3 identities. r[agent_harness_runtime.pending_input_vault.contents]
- Require explicit inspect, restore, or discard decisions after restart. Never submit recovered input automatically. r[agent_harness_runtime.pending_input_vault.recovery]
- Make restore and disposition idempotent across repeated crashes and stale retries. r[agent_harness_runtime.pending_input_vault.transactions]
- Block ordinary ingress on corrupt or ambiguous vault state while retaining bounded recovery controls. r[agent_harness_runtime.pending_input_vault.corruption]
- Add metadata-only receipts and paired positive and negative restart fixtures. r[agent_harness_runtime.pending_input_vault.evidence] r[agent_harness_runtime.pending_input_vault.verification]

## Success Criteria

- A restart exposes all accepted pending input in its original order.
- Recovered input never reaches a provider without a new explicit admission decision.
- Repeated restore or discard requests cannot duplicate model-visible input.
- Corrupt vault state blocks ordinary input without hiding recovery controls.
- One conversation cannot inspect, restore, discard, or delete another conversation's vault.

## False Completion

The change is incomplete if it stores only transcript messages, automatically resumes queued input, or deletes a vault before recording disposition evidence.

A successful restart smoke is not proof of power-loss durability or exactly-once provider behavior.

## Impact

- **Reusable runtime**: pure vault validation, ordering, transition planning, identities, and receipt projection.
- **Host shell**: explicit durable storage, transactions, input bodies, synchronization, and operator actions.
- **Existing owners**: active-turn recovery, coding-session queues, durable session storage, and messaging receipts remain authoritative.
- **Testing**: restart, repeated-crash, corruption, cross-conversation, leak, and no-auto-submit fixtures.

## Out of Scope

- A default global store, automatic transcript insertion, provider dispatch, or queue policy replacement.
- Distributed consensus, encrypted storage, power-loss guarantees, or exactly-once remote effects.

## Affected Specs

- `agent-harness-runtime`: pending-input vault boundary, contents, recovery, transactions, corruption, evidence, and verification.
