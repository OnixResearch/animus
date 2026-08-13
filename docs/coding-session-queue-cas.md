# Coding-session queue compare-and-swap

Animus uses compare-and-swap for queued steering and follow-up changes. A client must identify the exact queue item that it displayed.

## Reviewed baseline

The earlier Animus commands used these fields:

- `EditQueued { item_id, entry_id, input }`
- `RemoveQueued { item_id }`

These commands did not bind the displayed queue generation or expected private text. An edit could also replace the entry identity.

The reviewed Prime Agent source was commit [`0987c1ba7637cbcb99afe9efe1180b838a0aa958`](https://github.com/PrimeIntellect-ai/prime-agent/tree/0987c1ba7637cbcb99afe9efe1180b838a0aa958). Its queue mutation tests bind lane, index, and expected text. Animus keeps the expected-text behavior, but replaces raw expected text with a domain-separated BLAKE3 identity. Animus also binds stable entry identity and checked queue generation.

## Pure contract

`animus_runtime::coding_session_host` owns the pure contract. `EditQueued` and `RemoveQueued` require `CodingSessionQueueMutationPrecondition` with:

- stable entry identity;
- current checked generation;
- expected bounded position;
- private expected-text BLAKE3 identity.

The reducer compares all fields before it changes state. An accepted edit preserves the entry identity. An accepted edit or removal advances the generation once. It does not change the transcript, active run, durable cursor, or unrelated queue entries.

Queue append, dispatch selection, and acknowledgement also advance the checked generation. A command captured before concurrent injection is therefore stale.

## Conflicts

A mismatch returns `CodingSessionIssue::QueueConflict` with `CodingSessionQueueConflict`. Conflict classes are:

- `MissingItem`
- `WrongEntry`
- `MovedPosition`
- `StaleGeneration`
- `StaleText`

The conflict contains the current generation, queue identity, safe entry identities, positions, operation, and redaction marker. It contains no queued body or private expected-text identity. The failed reducer does not mutate session state.

Use `project_coding_session_queue_safe_snapshot` for frontend metadata. An authorized owner can use `coding_session_queue_precondition` to read the private precondition from its current session snapshot.

## Ownership boundaries

`CodingSessionHost::edit_queued` and `CodingSessionHost::remove_queued` require explicit preconditions. SDK, headless, daemon, ACP, and TUI callers must preserve these fields when they add or expose queue mutation routes. A route without these fields must fail before reducer entry.

`edit_queued_exclusive_local` and `remove_queued_exclusive_local` are restricted convenience methods. They derive preconditions while one mutable `CodingSessionHost` borrow owns the current local snapshot. They are not remote or multi-owner compatibility paths.

The current headless, daemon, ACP, and TUI surfaces do not expose an older position-only queue mutation route. The shared `SessionQueueMutationObserved` runtime event gives headless and TUI projections metadata-only success or conflict status.

## Evidence

Accepted transitions emit `SessionQueueMutationObserved` after the normal session-state event. The standard host emits the same event for a conflict without changing authoritative session state.

Queue mutation receipts contain only:

- operation and accepted or conflict status;
- item and stable entry identities;
- bounded position;
- generation before and optional generation after;
- queue BLAKE3 identity;
- optional conflict class;
- redaction marker and receipt BLAKE3 identity.

Receipt projection rejects any supplied current, expected, or replacement text. The error does not echo that text.

## Verification

Run from the repository root:

```sh
scripts/cairn/check-coding-session-queue-cas --root .
cargo test -p animus-runtime sessionhost::tests --locked
cargo test -p animus-std --features coding-session-host coding_session::tests --locked
cargo test -p animus-runtime --features headless headless --locked
cargo test -p animus-tui frontend --locked
```

These checks cover exact edits and removals, generation order, stale text, stale generation, wrong entries, moved positions, concurrent injection, overflow, duplicate commands, unsafe evidence, frontend projection, and compatibility fallback rejection.

They do not prove distributed consensus, replicated queues, automatic merge correctness, exactly-once delivery, availability, or production readiness.
