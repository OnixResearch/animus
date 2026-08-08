# Prime Agent messaging receipts cohort fixture

- Repository: `https://github.com/PrimeIntellect-ai/prime-agent`
- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Reviewed cohort: `prime-agent-messaging-receipts`
- Reviewed surface: `agent_message` skill and daemon routing for `auto`, `steer`, and `follow_up`.
- Reviewed semantics: host-derived sender identity, family-scoped roster resolution, `delivered` and `queued` receipts, and pre-routing size/rate/pending limits.

Animus adopts only these bounded semantics. It does not adopt transport, encryption, network discovery, queue persistence, exactly-once delivery, ordering, read, acted, or comprehension claims.
