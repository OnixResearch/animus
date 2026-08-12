# Proposal: Add concrete provider streaming adapters

## Why

Animus already has accepted contract-level support for model stream frames, reasoning/thinking controls, stream capability negotiation, runtime event projection, and default thought/redaction policy. That support intentionally stops short of claiming that any concrete network provider adapter streams real tokens or provider-exposed reasoning today.

Without a separate change, future work could blur those two claims: documentation might imply live provider streaming, adapter code might parse helper fixtures but not production chunks, or thought/reasoning channels could be exposed without the accepted visibility policy. This change creates the follow-up Cairn needed before Animus can honestly claim end-to-end provider streaming support.

## What Changes

- Add optional, host-owned concrete provider streaming adapters that translate admitted provider stream chunks into canonical `ModelStreamFrame` values.
- Keep provider clients, credential lookup, network transport, retry policy, and provider-native raw streams outside reusable core/router/runtime defaults.
- Require production parser entrypoints with raw chunk fixtures for content deltas, thought/reasoning deltas, tool-call deltas, usage, finish, cancellation, malformed records, and truncation.
- Bind provider streaming dispatch to existing model capability negotiation and thought/reasoning visibility policy.
- Add safe evidence and documentation that distinguish fixture-backed parser support, optional live-provider evidence, and non-claims.

## Impact

- **Files**: likely `animus-std` provider adapter modules, parser fixtures, provider registry/capability tests, docs/model-streaming.md, release/dogfood evidence docs, and Cairn artifacts.
- **Testing**: positive parser fixtures for supported providers, canonical frame mapping, capability-admitted dispatch, and runtime event projection; negative fixtures for unsupported stream modes, missing credentials/service, malformed/truncated chunks, missing finish, provider error events, cancellation, and thought visibility leaks.
- **Non-claims**: this does not make live provider credentials mandatory, put network clients in reusable no-std crates, expose private chain-of-thought by default, or make raw provider streams public runtime API.
