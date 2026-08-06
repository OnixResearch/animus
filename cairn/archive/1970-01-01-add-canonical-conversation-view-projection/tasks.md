## Phase 1: Baseline and ownership

- [x] [serial] B1 Run and record current session-ledger, model-streaming, runtime-event, headless, TUI reducer, host-control snapshot, redaction, and receipt baselines. r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [serial] B2 Inventory every model-visible ledger, render cache, active-turn item, steering marker, reconnect cursor, and legacy migration owner. r[agent_harness_runtime.conversation_view_projection.boundary]
- [x] [parallel] B3 Record the exact camelAI reference revision, useful projection invariants, rejected storage details, MIT provenance, and no-runtime-dependency boundary. r[agent_harness_runtime.conversation_view_projection.boundary] r[agent_harness_runtime.conversation_view_projection.evidence]

## Phase 2: Policy and pure projection

- [x] [serial] P1 Add typed Nickel policy for schemas, visible content classes, redaction, tool and reasoning posture, bounds, cursor fields, rebuild rules, legacy healing, receipts, and non-claims. r[agent_harness_runtime.conversation_view_projection.boundary] r[agent_harness_runtime.conversation_view_projection.evidence]
- [x] [serial] P2 Export generated policy data and add strict Rust revalidation with domain-separated BLAKE3 policy identity. r[agent_harness_runtime.conversation_view_projection.identity]
- [x] [serial] C1 Add pure validated source snapshot, cursor, view item, steering marker, projection action, rebuild, heal, and diagnostic types. r[agent_harness_runtime.conversation_view_projection.boundary]
- [x] [serial] C2 Add canonical BLAKE3 stream, cursor, item, action, snapshot, delta, rebuild, heal, and receipt identities over typed source facts. r[agent_harness_runtime.conversation_view_projection.identity]
- [x] [serial] C3 Add pure snapshot and top-up reducers with exact revision fences, strict sequence order, finite bounds, and deterministic diagnostics. r[agent_harness_runtime.conversation_view_projection.stream] r[agent_harness_runtime.conversation_view_projection.rebuild]
- [x] [serial] C4 Add exact active-turn folding and ordered steering-marker projection without content or timestamp deduplication. r[agent_harness_runtime.conversation_view_projection.active_turn]
- [x] [serial] C5 Add conservative legacy-heal planning that changes admitted metadata only after one exact unambiguous join. r[agent_harness_runtime.conversation_view_projection.rebuild]

## Phase 3: Host cache and consumer composition

- [x] [serial] H1 Add implementation-free cache snapshot, delta, rebuild, heal, and evidence ports with compare-and-swap cursor facts. r[agent_harness_runtime.conversation_view_projection.rebuild]
- [x] [serial] H2 Add a non-default std cache shell that applies exact actions idempotently and never writes the source ledger. r[agent_harness_runtime.conversation_view_projection.boundary] r[agent_harness_runtime.conversation_view_projection.rebuild]
- [x] [serial] H3 Map one projection stream into headless and TUI reducers without granting either transcript mutation authority. r[agent_harness_runtime.conversation_view_projection.stream]
- [x] [depends:add-durable-active-turn-recovery] H4 Compose accepted active-turn and steering identities with stable visible items after the recovery contract is available. r[agent_harness_runtime.conversation_view_projection.active_turn]
- [x] [serial] H5 Keep Tile, browser, remote, and framework-specific hosts outside reusable runtime and require separate consumer evidence. r[agent_harness_runtime.conversation_view_projection.boundary]

## Phase 4: Positive and negative evidence

- [x] [parallel] T1 Add positive tests for initial snapshots, matching-cursor top-up, reconnect no-op, active-turn folding, interposed steering, terminal replay, rebuild, and exact metadata healing. r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [parallel] T2 Add identity tests for equal text, equal timestamps, repeated tool labels, multiple provider turns, branch changes, and policy changes. r[agent_harness_runtime.conversation_view_projection.identity] r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [parallel] T3 Add negative stream and cursor tests for incremental-before-snapshot, gaps, duplicates, stale generations, changed leaves, missing anchors, reversed sequences, partial cache application, and divergent writers. r[agent_harness_runtime.conversation_view_projection.stream] r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [parallel] T4 Add negative rebuild and heal tests for ambiguous timestamps, equal content, active-turn races, unknown cache items, visible-content mutation, source-ledger mutation, and partial cache clearing. r[agent_harness_runtime.conversation_view_projection.rebuild] r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [parallel] T5 Add redaction, receipt leak, and overclaim tests for reasoning, tool bodies, provider payloads, credentials, paths, raw UI state, transcript truth, transport delivery, completion, and production claims. r[agent_harness_runtime.conversation_view_projection.evidence] r[agent_harness_runtime.conversation_view_projection.verification]

## Phase 5: Documentation and closeout

- [x] [serial] D1 Document projection ownership, identities, cursor schema, snapshot-first flow, active-turn folding, cache recovery, migration, redaction, and non-claims. r[agent_harness_runtime.conversation_view_projection.boundary] r[agent_harness_runtime.conversation_view_projection.evidence]
- [x] [serial] D2 Update contracts, component, facade, request-lifecycle, ledger, streaming, headless, TUI, host-control, safety, roadmap, and README indexes. r[agent_harness_runtime.conversation_view_projection.boundary]
- [x] [serial] D3 Add a focused projection rail and Nix check for Nickel drift, pure reducers, host fakes, reconnects, collisions, stale cursors, rebuilds, redaction, and dependency isolation. r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [serial] D4 Run focused tests, formatting, Clippy, Wasm and no-std checks where applicable, Cairn validation and gates, and the relevant Nix checks. r[agent_harness_runtime.conversation_view_projection.verification]
- [x] [serial] D5 Review dry-run sync and archive plans, make sure that every requirement reaches the accepted spec, and archive only after current evidence passes. r[agent_harness_runtime.conversation_view_projection.verification]
