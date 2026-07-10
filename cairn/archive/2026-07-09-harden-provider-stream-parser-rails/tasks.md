# Tasks

- [x] [serial] Inventory provider stream parser entrypoints and raw-boundary assertions/panics that can be reached from provider-like chunks. r[agent_harness_runtime.provider_stream_parser_rails.inventory]
- [x] [depends:inventory] Replace parser-facing raw-boundary assertions with typed parse, encoding, ordering, terminal-state, unsupported-event, and budget failures. r[agent_harness_runtime.provider_stream_parser_rails.typed_failures]
- [x] [depends:typed_failures] Enforce frame-byte, record-byte, aggregate-byte, record-count, event-count, and completion budgets before buffering or concatenating stream data. r[agent_harness_runtime.provider_stream_parser_rails.budgets]
- [x] [depends:budgets] Add parser-entrypoint fixtures that feed provider-like raw chunks, SSE lines, JSON records, split boundaries, and terminal markers into parsers. r[agent_harness_runtime.provider_stream_parser_rails.entrypoint_fixtures]
- [x] [depends:entrypoint_fixtures] Project stream parser successes and failures into safe receipts with parser classes, budget kind, counts, provider family, BLAKE3 identities, and redaction markers only. r[agent_harness_runtime.provider_stream_parser_rails.receipts]
- [x] [depends:receipts] Add positive ordered-stream tests and negative invalid-encoding, malformed-record, oversized-frame, record-budget, aggregate-budget, duplicate-terminal, missing-terminal, out-of-order-delta, unsupported-event, hidden-reasoning-leak, and receipt-leak tests. r[agent_harness_runtime.provider_stream_parser_rails.verification]
