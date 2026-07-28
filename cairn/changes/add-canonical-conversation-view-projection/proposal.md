## Why

Animus has a typed runtime event stream, session ledger contracts, model streaming, headless protocols, and a pure TUI reducer. These surfaces do not define one complete conversation-view projection contract.

A host can otherwise let rendered history become a second transcript authority. Reconnects, cache rebuilds, steering, and multiple model turns can then duplicate or overwrite visible messages.

camelAI demonstrates a useful separation between canonical model history and rebuildable browser history. Animus will adapt this invariant without its Cloudflare storage or timestamp-based implementation details.

## What Changes

- Add a pure conversation-view projection core over one selected admitted ledger branch, projection policy, active-turn facts, and cache cursor. r[agent_harness_runtime.conversation_view_projection.boundary]
- Define stable BLAKE3 view-item identities, ledger revision fences, and strictly ordered projection sequences. r[agent_harness_runtime.conversation_view_projection.identity]
- Require one accepted redaction-safe snapshot before incremental updates for each projection stream. r[agent_harness_runtime.conversation_view_projection.stream]
- Fold all model entries for one active turn into the same visible assistant item and preserve ordered steering markers. r[agent_harness_runtime.conversation_view_projection.active_turn]
- Treat render caches as disposable projections with exact cursor validation, deterministic top-up, and explicit full rebuild plans. r[agent_harness_runtime.conversation_view_projection.rebuild]
- Add conservative legacy-heal planning that never guesses across ambiguous identities or mutates visible content. r[agent_harness_runtime.conversation_view_projection.rebuild]
- Add metadata-only projection receipts and positive and negative reconnect, collision, stale-cursor, rebuild, and redaction evidence. r[agent_harness_runtime.conversation_view_projection.evidence] r[agent_harness_runtime.conversation_view_projection.verification]

## Success Criteria

- Reconnect and replay preserve one visible item for one canonical turn identity.
- Repeated top-up is idempotent for the same ledger revision and cursor.
- A stale or contradictory cursor triggers a full rebuild or a blocked outcome.
- Equal timestamps and equal content do not merge distinct ledger entries.
- Multiple assistant entries and steering messages preserve deterministic visible order.
- Render cache loss does not change model-visible ledger history.
- Positive and negative fixtures pass through production projection and host-cache entrypoints.

## False Completion

The change is not complete if the UI cache can become model context, timestamps define identity, or counts alone define progress.

A reconnect that looks correct once is not evidence of idempotence. Silent collision repair, guessed joins, and content-based deduplication are also false completion.

## Impact

- **Reusable runtime**: pure snapshot, delta, cursor, fold, rebuild, heal, and receipt logic.
- **Host shells**: ledger loading, cache reads and writes, stream transport, rendering, clocks, and migration execution.
- **Animus TUI and headless hosts**: shared projection facts without transcript mutation authority.
- **Future consumers**: Tile or browser hosts can consume the contract through separate host-owned changes.
- **Testing**: deterministic projection fixtures, reconnect scenarios, stale and corrupt caches, identity collisions, redaction, and focused Cairn and Nix rails.

## Out of Scope

- A browser framework, Cloudflare Durable Objects, a default database, raw transcript export, or UI layout ownership.
- Treating render history as provider context, persistence evidence, completion evidence, or an authorization source.
- Claims for transport delivery, UI framework correctness, cache durability, transcript truth, accessibility, or production readiness.

## Affected Specs

- `agent-harness-runtime`: conversation-view boundary, identity, stream, active-turn folding, rebuild, evidence, and verification.
