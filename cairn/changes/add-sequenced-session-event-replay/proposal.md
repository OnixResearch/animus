## Why

Animus has one authoritative session ledger, ordered runtime events, ACP updates, snapshot-first conversation projections, and durable restore planning. It does not expose one transport-neutral contract for reconnecting from a bounded event offset while distinguishing durable semantic events from live-only deltas.

Sandbox Agent and agentOS provide useful references for monotonic session event sequences, offset replay, and the separation between completed semantic updates and transient streaming deltas. Animus needs the behavior without adopting another event store, JSON schema, remote server, or persistence authority.

## What Changes

- Add typed session stream, generation, sequence, cursor, event-class, and replay-result identities.
- Classify events as durable semantic, live-only delta, or metadata-only observation.
- Add pure replay planning for `sequence > offset`, exact top-up, no-op, gap, stale generation, truncation, and terminal stream outcomes.
- Keep the existing session ledger as the sole transcript, tool-correlation, replay, and model-context owner.
- Project the same admitted order to ACP, headless, TUI, and future transports without transport-specific ordering rules.
- Add metadata-only BLAKE3 receipts and redaction rules.

## Impact

- **Files**: `animus-runtime` session event contracts, adapter DTOs, ACP projection, headless/TUI conformance fixtures, Nickel policy, documentation, and the agent runtime specification.
- **Dependency**: Complete typed ACP session identities before widening ACP cursor use.
- **Testing**: Positive reconnect/top-up/no-op cases and negative duplicate, gap, stale-generation, live-delta replay, oversized range, and redaction cases.
- **Boundary**: Replay proves only ordered projection from admitted ledger facts. It does not prove transcript truth, transport delivery, durable storage, or conversation continuity.
