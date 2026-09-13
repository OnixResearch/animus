# Bound stream-host admission, wakeups, and teardown

## Why

Animus already owns bounded subscriptions, sequenced replay, and typed streaming events. The missing evidence is composition under output saturation, stale wakeups, authorization drift, and cancellation. Baz supplies adversarial cases, not another runtime.

## What Changes

- Add a consumer-owned host conformance fixture with explicit task, byte, queue, and lifetime limits. r[animus.stream_host.admission]
- Prove generation-fenced wakeups and bounded drain behavior. r[animus.stream_host.wakeups]
- Prove fixed deadlines and exactly-once logical cleanup after dependent access ends. r[animus.stream_host.teardown]
- Compose current authorization and existing replay fences without a second event ledger. r[animus.stream_host.replay]
- Separate response publication and evidence reservation from effect completion. r[animus.stream_host.effects]

## Impact

Reuse `animus-session-core`, `animus-runtime`, and explicit std host ports. The fixture exercises the selected local-daemon subscription path and existing provider-stream boundary. Add production fixes only where those cases expose a gap. Do not claim every host satisfies the contract from one fixture.

## Non-goals

No new HTTP server, task scheduler, transcript store, tool-result cache, or universal action receipt family. No provider calls or product activation. Existing active `receipt-live-agent-actions` retains universal receipt ownership.

## Reference

Baz at `aa9723b183d50e419b87af5706e99f4425d1c76e`, `docs/CONTINUATIONS.md` and `docs/SSE.md`: https://github.com/technologylab-ai/baz/tree/aa9723b183d50e419b87af5706e99f4425d1c76e/docs

Local sources: `docs/agent-control-pubsub.md`, `docs/session-event-replay.md`, `docs/model-streaming.md`, `docs/tool-execution-stages.md`, and `docs/runtime-authority-seal.md`.
