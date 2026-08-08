# Sequenced session event replay

Animus provides optional reconnect replay behind the `session-event-replay` feature. The planner consumes already-loaded facts from the admitted session ledger and runtime event order.

## Event classes

- `durable_semantic` is completed semantic content that the session ledger admits.
- `metadata_observation` is safe lifecycle metadata.
- `live_only_delta` is partial streaming data. Reconnect replay omits this class.

The replay planner does not create a second transcript or event store.

## Cursor fences

A cursor binds the session, stream, ledger generation, active leaf, last sequence, policy, and redaction profile. Generation, branch, policy, or redaction drift selects an explicit rebuild or denial.

The planner selects events with a sequence greater than the cursor offset. Duplicate sequences and gaps fail closed. Event-count and byte bounds can return a truncated result.

## Client projections

ACP, headless, and TUI projections consume one replay plan. Each projection preserves the same sequence, event identity, and durability class.

Transports own reconnection, authentication, delivery, timers, retries, and persistence. The core does not use these effects.

## Evidence

Replay receipts contain BLAKE3 identities, outcomes, ranges, counts, omissions, byte counts, and non-claims. They exclude transcript bodies, reasoning, tool arguments, tool results, credentials, environment values, and physical paths.

Run the focused rail:

```sh
scripts/cairn/check-session-event-replay --root .
nix build .#checks.x86_64-linux.session-event-replay --no-link -L
```

This contract does not prove transcript truth, transport delivery, durable storage, or conversation continuity.
