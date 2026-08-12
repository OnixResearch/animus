# Proposal: Ensure model token and thought streaming for Animus

## Why

Animus already has Cairn work for runtime events, headless event streams, provider metadata, and future provider-stream parser rails, but none of those changes explicitly guarantee that model token streaming and provider-exposed thought/reasoning streaming work end to end.

Without a focused contract, adapters could claim streaming while only returning a final message, thought channels could leak into final answers or receipts, and headless/TUI consumers could observe different partial-output semantics.

## What Changes

- Define a streaming model-adapter contract for ordered visible content token/delta frames, provider-exposed thought/reasoning frames, tool-call deltas, usage, finish, cancellation, and failure.
- Add model capability metadata and request negotiation for content streaming and thought/reasoning streaming so unsupported modes fail or downgrade explicitly.
- Require provider-stream parser tests to feed raw provider bytes or chunks through the real parser entrypoint before any streaming adapter claim is accepted.
- Project stream frames into typed runtime events and protocol/UI-neutral DTOs while preserving correlation, order, bounded bodies, and final-response assembly rules.
- Gate thought/reasoning visibility through policy so hidden or sensitive reasoning never becomes final answer text, receipts, dataset rows, or logs by default.

## Impact

- **Files**: model adapter DTOs, runtime event DTOs, provider registry metadata, parser fixtures, headless/TUI-neutral event mapping, policy/redaction tests, docs, and Cairn artifacts.
- **Testing**: positive tests for content token streaming, thought/reasoning streaming when supported, raw parser fixtures, runtime event order, and final response assembly; negative tests for unsupported thought streaming, malformed streams, missing finish, cancellation/failure, and thought-leak attempts.
- **Non-claims**: this change does not add a concrete network provider, expose private chain-of-thought by default, require every model to support thought streaming, or make provider-native raw streams part of the public runtime API.
