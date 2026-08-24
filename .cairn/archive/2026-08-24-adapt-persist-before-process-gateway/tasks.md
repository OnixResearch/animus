## Phase 1: Durable admission

- [x] [serial] r[animus.gateway.persist] Add pure inbound-update and pending-reply admission plans with stable identities.
- [x] [parallel] r[animus.gateway.persist.incoming] Add a fixture that requires durable inbound admission before processing.
- [x] [parallel] r[animus.gateway.persist.reply] Add a fixture that requires durable reply intent before delivery.

## Phase 2: Ordered bounded work

- [x] [serial] r[animus.gateway.order] Add per-conversation ordering and explicit cross-conversation capacity plans.
- [x] [parallel] r[animus.gateway.order.same] Add a fixture that rejects same-conversation reordering.
- [x] [parallel] r[animus.gateway.order.different] Add a fixture that permits bounded progress for separate conversations.

## Phase 3: Restart and authority

- [x] [serial] r[animus.gateway.resume] Add idempotent recovery for pending updates and uncertain replies.
- [x] [parallel] r[animus.gateway.resume.restart] Add a positive restart-resume fixture.
- [x] [parallel] r[animus.gateway.resume.duplicate] Add negative duplicate-processing and false-delivery fixtures.
- [x] [serial] r[animus.gateway.authority] Deny remote mutation by default and intersect exact current authority.
- [x] [parallel] r[animus.gateway.authority.mutation] Add positive admitted-mutation and negative default-denial fixtures.

## Phase 4: Verification

- [x] [serial] r[animus.gateway.verification] Add the focused positive, negative, and boundary matrix and run the rails.
