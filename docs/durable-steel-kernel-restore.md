# Durable Steel kernel restore

Animus can restore one committed model-facing Steel kernel after a process restart. The host stores a private replay journal, not a Steel virtual-machine image.

## Boundary

`animus-steel` owns the private journal contract and effect-disabled replay. Each snapshot binds these facts:

- session and host-owned kernel generation;
- exact Steel, RLM policy, and bridge cohorts;
- ordered context-binding commits;
- explicit clear epochs;
- committed source cells;
- ordered bridge request and captured-result identities;
- a BLAKE3 chain head.

Source, context values, and captured result bodies stay in the private journal. `Debug` output and restore receipts contain metadata only.

`animus-std::DurableSteelKernelPersistence` stores each complete snapshot as a `steel_kernel_state` record in the existing capability-scoped durable session store. The durable store keeps its existing lock, framing, synchronization, recovery, and retention authority.

## Restore sequence

1. Read the last committed durable store snapshot.
2. Supply the exact current session, kernel generation, journal head, RLM policy, and recovery posture.
3. Validate every private record and BLAKE3 chain link.
4. Build a fresh sandboxed Steel engine.
5. Replay committed cells with captured results supplied directly.
6. Reject any missing, extra, reordered, or changed request.
7. Publish the restored kernel only after the complete replay passes.

No host executor exists in the restore path. Replay cannot read files, run processes, call tools, use the network, or start child sessions.

## Interrupted cells

A kernel head advances only after a complete cell succeeds and its private snapshot commits. If a process exits during the next cell, restore selects the prior committed head.

The host must also supply an accepted active-turn recovery action. Completed external effects remain active-turn observations. Restore does not rerun or reverse them.

## Evidence

Run:

```sh
scripts/cairn/check-durable-steel-kernel-restore --root .
```

The focused rail checks successful restart, clear markers, context, captured results, effect suppression, identity drift, policy drift, corruption, interrupted-cell admission, bounds, receipt leaks, durable-store integration, Clippy, and default dependency isolation.

## Non-claims

This feature does not provide mid-cell continuation restore, Steel heap serialization, effect rollback, exactly-once effects, encrypted storage, power-loss immunity, cross-version migration, production durability, task correctness, or release readiness.
