# Session persistence adapter

Animus models durable session persistence as an injected host service. Reusable logic admits explicit store descriptors, session and branch identity, schema version, redaction policy, freshness policy, and receipt policy before append, replay, restore, compaction, or tombstone work.

The first evidence rung is an in-memory fake store over safe ledger records. Records contain schema labels, session/branch IDs, parent hashes, BLAKE3 identities, counts, status classes, safe adapter labels, and replay receipts only.

Non-claims: filesystem layout, database backend, daemon persistence, encryption, production recovery, lock strategy, clock freshness, and live-provider continuity remain host-owned and unimplemented unless separately indexed.
