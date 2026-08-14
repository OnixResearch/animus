# Design: Durable Steel kernel restore

## Context

The current kernel reconstructs in-memory state by replaying successful source cells. This works during prompt compaction, but the successful cell list is not exported through a durable contract.

A source cell can use bridged host results. Plain source replay would request those effects again.

## Success Contract

After process restart, a host can reconstruct the last committed kernel state for one session. Replay uses committed source plus captured typed results and performs no host effect.

Restore fails closed for wrong cohort, wrong policy, wrong session, missing results, order drift, corruption, or an interrupted cell.

Safe receipts contain no source body, result body, prompt body, credential, or private path.

A VM byte dump, raw source in receipts, or replay that dispatches a host request is false completion.

## Decisions

### 1. Store a private replay journal, not a VM image

Each committed cell record binds:

- the session and kernel identities
- the Steel cohort and policy identities
- the source BLAKE3 identity and private source body
- ordered bridge call identities
- private bounded result bodies and safe result identities
- the prior committed kernel-record identity
- the final value class

Clear operations use explicit records. The safe ledger stores identities and counts only.

### 2. Replay with effects disabled

Restore creates a fresh admitted kernel. The replay driver supplies captured results for each recorded bridge call.

Any unrecorded request, extra request, order mismatch, or result mismatch stops restore. No production executor is available to the replay engine.

### 3. Restore only committed safe points

A durable kernel head advances after the cell reaches a successful final value and its private record commits.

A crash during evaluation leaves the prior committed head authoritative. Completed external effects from the interrupted cell remain active-turn recovery facts.

### 4. Bind restore to exact compatibility facts

Admission compares the session, kernel generation, Steel cohort, RLM policy, bridge schema, journal schema, and committed chain head.

A migration requires an explicit versioned converter with positive and negative fixtures. No implicit best-effort replay occurs.

### 5. Keep private and safe channels separate

Source and result bodies use the replay-private journal. Safe receipts contain only identities, counts, compatibility classes, restore status, and non-claims.

## Restore Flow

```text
durable session head
  -> pure kernel restore admission
  -> private bounded record load
  -> chain and body validation
  -> fresh Steel kernel
  -> effect-disabled replay with captured results
  -> reconstructed committed kernel
  -> safe restore receipt
```

## Alternatives Considered

### Serialize the Steel engine heap

Rejected because engine internals do not form a reviewed stable interchange contract.

### Replay source and run host effects again

Rejected because this can duplicate file edits, commands, messages, and child runs.

### Store only final variable values

Rejected because arbitrary Steel values and closures do not have a complete stable serializer.

## Risks and Controls

- Private records contain sensitive code and results. Existing private-journal policy owns access and retention.
- Replay can drift after a Steel update. Exact cohort admission fails closed.
- Journal corruption can hide the latest state. Hash chaining and committed heads identify the last valid boundary.
- Large histories can make replay slow. Named cell, source, result, and cumulative bounds apply.

## Non-Claims

This design does not establish encrypted storage, power-loss immunity, arbitrary Steel value serialization, mid-cell continuation restore, effect rollback, cross-version compatibility, or production durability.
