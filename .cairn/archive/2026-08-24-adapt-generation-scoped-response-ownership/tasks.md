## Phase 1: Generation ownership

- [x] [serial] r[animus.ownership.scope] Add the pure one-generation ownership state and transition contract.
- [x] [parallel] r[animus.ownership.scope.claim] Add a positive fixture that admits one owner before frame handling.
- [x] [parallel] r[animus.ownership.scope.concurrent] Add a negative fixture that rejects a concurrent owner.

## Phase 2: Connection poisoning

- [x] [serial] r[animus.ownership.poison] Bind interruption, malformed framing, ownership mismatch, and early return to poisoning.
- [x] [parallel] r[animus.ownership.poison.interrupted] Add a fixture that poisons an interrupted exchange.
- [x] [parallel] r[animus.ownership.poison.stale_frame] Add a fixture that rejects a late frame from an old generation.

## Phase 3: Terminal release

- [x] [serial] r[animus.ownership.release] Release ownership only after the admitted terminal frame.
- [x] [parallel] r[animus.ownership.release.terminal] Add a positive fixture that permits reuse after terminal completion.
- [x] [parallel] r[animus.ownership.release.early] Add a negative fixture that rejects reuse after early return.

## Phase 4: Verification

- [x] [serial] r[animus.ownership.verification] Add the focused positive, negative, and boundary matrix and run the rails.
