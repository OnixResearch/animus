## Phase 1: Cooldown-aware failover

- [x] [serial] r[animus.billing.failover] Implement pure cooldown-aware automatic provider selection over supplied account facts.
- [x] [parallel] r[animus.billing.failover.cooldown_skip] Add a positive fixture that skips a cooldown provider and records the reason.
- [x] [parallel] r[animus.billing.failover.cooldown_selected] Add a negative fixture that rejects automatic cooldown-provider selection.

## Phase 2: Billing-tier visibility

- [x] [serial] r[animus.billing.tier] Bind billing-tier changes into the pre-admission transition record.
- [x] [parallel] r[animus.billing.tier.recorded] Add a positive fixture that records a subscription-to-credit transition.
- [x] [parallel] r[animus.billing.tier.silent] Add a negative fixture that rejects a silent tier conversion.

## Phase 3: Pending-turn preservation

- [x] [serial] r[animus.billing.pending] Bind one pending turn and durable session to the provider transition plan.
- [x] [parallel] r[animus.billing.pending.survives] Add a positive fixture that preserves the turn across transition.
- [x] [parallel] r[animus.billing.pending.loss] Add a negative fixture that rejects turn loss or duplication.

## Phase 4: Verification

- [x] [serial] r[animus.billing.verification] Add the focused positive, negative, and boundary matrix and run the rails.
