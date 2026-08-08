# Messaging modes and receipts

Animus provides an optional `messaging-receipts` facade over pure family-scoped messaging contracts. The reviewed Prime Agent cohort is pinned to revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`.

## Roster and identity

The host supplies the family roster and sender identity. Message content cannot claim a sender. Target resolution accepts only roster members. Broadcast resolves to other members of the same family. Unknown and cross-family targets fail before routing.

## Modes

- `auto` delivers to an idle target and steers a busy target.
- `steer` injects into active work. Policy may deny this mode for a role.
- `follow_up` queues delivery until the current turn finishes.

Routing is pure. The host owns the interrupt classifier, transport, delivery effect, and queue.

## Limits

Size, rate, and pending-queue checks run before target routing. Each decision binds the supplied counter identities. A failed limit produces no delivery or queue effect.

## Receipts

Only two success variants exist:

- `delivered` requires host attestation that the message entered an idle context.
- `queued` requires host attestation that pending storage accepted the message.

Receipts contain policy, roster, sender, target count, mode, variant, counter identities, and BLAKE3 identities. They exclude message bodies and reject read, acted, ordered, exactly-once, transport-security, and comprehension claims.

Run the focused rail:

```sh
scripts/cairn/check-messaging-receipts --root .
```

This contract does not prove transport security, exactly-once delivery, ordering, message correctness, comprehension, agent cooperation, production readiness, or release eligibility.
