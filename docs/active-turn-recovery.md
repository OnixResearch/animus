# Active-turn recovery

Animus protects accepted work before provider dispatch. The pure runtime core plans recovery from supplied facts. The host owns all effects.

## Ownership

`animus_runtime::active_turn_recovery` owns these deterministic operations:

- validation of generated policy;
- BLAKE3 identities for policy, turns, checkpoints, steering, plans, and receipts;
- accepted-message transaction plans;
- complete checkpoint replacement plans;
- exact parked and running control-entry identities;
- provider and steering gates;
- recovery, parked-state, and terminal-cleanup plans;
- metadata-only receipts.

The runtime core does not read storage or clocks. It does not call providers or tools. It does not clean host state.

The `animus-std/active-turn-recovery` feature owns explicit policy loading. It also supplies a deterministic in-memory transaction fixture.

Production hosts must inject durable acceptance, checkpoint, steering, terminal, and cleanup services. They must select their own transaction and storage implementations.

## Required order

1. Validate the accepted message and current authority.
2. Plan one acceptance transaction.
3. Commit the accepted message and active marker together.
4. Validate the exact transaction evidence.
5. Dispatch the provider only after the durable acceptance gate passes.

For steering, commit the steering identity before delivery. A missing, stale, partial, or mismatched append blocks delivery.

For checkpoint replacement, prepare and bound every entry before mutation. Then use one compare-and-swap transaction for the complete checkpoint.

For agent control, prepare an entry from the exact run, revision, and phase. A current parked entry blocks provider continuation after recovery.

A later running entry makes the turn eligible for the other recovery gates. The host owns the checkpoint transaction and durable storage.

## Crash windows

| Crash window | Recovery result |
| --- | --- |
| Before durable acceptance | Provider dispatch stays blocked. |
| After acceptance and before provider output | Recovery keeps the same turn identity and accepted work. |
| During replacement preparation | The prior complete checkpoint remains current. |
| During checkpoint mutation | Exact transaction evidence identifies one complete generation or blocks recovery. |
| After a tool call | The existing interrupted-tool repair core can plan one synthetic failure. |
| After a real tool result | Recovery preserves the real result and does not request another tool effect. |
| After steering acceptance | The durable steering identity remains eligible for one correlated delivery attempt. |
| While control is parked | Recovery returns `Parked` and does not admit provider continuation. |
| After an exact resume record | Recovery applies the other authority, cancellation, repair, and terminal gates. |
| After terminal commit | Recovery does not dispatch providers or tools. Cleanup remains a separate host action. |

## Tool and provider composition

Active-turn recovery does not own a tool-request type. A missing tool result delegates to `animus_runtime::interrupted_tool_repair`.

A real tool result remains in the ordered checkpoint. Recovery never replaces it and never converts provider retry into tool retry.

Provider continuation also needs current authority, cancellation, repair, agent-control, and exact provider-plan gates. Durable acceptance alone does not grant authority.

A cancelling or terminal control phase cannot become a parked or running entry. Existing cancellation and terminal fields own those outcomes.

## Policy and receipts

`config/active-turn-recovery-policy.ncl` is the policy source. Rust revalidates the generated JSON before it plans recovery.

Receipts contain identities, generations, counts, action classes, terminal classes, redaction metadata, and non-claims. They reject transcript, reasoning, tool, provider, credential, environment, path, and store-error bodies.

## Reference boundary

The design uses the active-turn journal ordering pattern from camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2`. The upstream code has an MIT license.

Animus does not use camelAI as a runtime dependency. It does not adopt Cloudflare storage, Python process state, timestamps as identity, or implicit write-loop atomicity.

## Non-claims

This contract does not prove power-loss durability. It does not prove store correctness, control-entry persistence, provider acceptance, or exactly-once effects.

It does not prove tool non-execution, transcript completeness, task completion, production readiness, or release eligibility.

## Validation

```sh
scripts/cairn/check-active-turn-recovery --root .
nix build .#checks.x86_64-linux.active-turn-recovery --no-link -L
```
