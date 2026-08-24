## Why

Animus owns durable sessions, daemon workers, background-work queue contracts, and restart recovery. A remote message gateway adds a stricter boundary: an incoming update or pending reply can be lost if processing begins before durable admission.

`digitallyinduced/haskell-agent` demonstrates a useful Telegram gateway discipline. Incoming updates and pending replies are persisted before processing, each conversation runs in order, separate conversations can run concurrently, and pending work resumes after restart. This change adapts those generic semantics without adding a Telegram product surface.

## What Changes

- Persist incoming updates before agent processing begins.
- Persist pending reply intents before delivery begins.
- Preserve ordered processing within one conversation while permitting bounded concurrency across conversations.
- Resume admitted pending work after restart without duplicate processing or delivery.
- Deny mutating tools by default for remote gateway sessions unless exact authority admits them.

## Impact

- **Files**: session-core gateway DTOs and reducers, background-work queues, daemon-worker recovery, authority profiles, fixtures, documentation, and this lifecycle package.
- **Testing**: focused queue, daemon-worker, active-turn-recovery, session-replay, and authority rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- A Telegram adapter, bot setup flow, or remote transport implementation.
- Remote user identity proof or message-delivery proof.
- Unlimited gateway concurrency.
- Mutation authority from transport authentication alone.
