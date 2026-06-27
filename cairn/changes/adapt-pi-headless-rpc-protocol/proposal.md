# Proposal: Adapt Pi headless RPC protocol for Animus

## Why

Pi's RPC mode demonstrates a useful embedding surface: commands enter over a strict line-framed protocol, lifecycle events stream back asynchronously, and clients can steer, follow up, abort, inspect state, and manage sessions without a terminal UI. Animus needs a similar headless boundary so Rust, non-Rust, CLI, daemon, test, and IDE hosts can drive the same runtime without linking to a TUI.

Animus should adapt the command/event shape while owning its protocol DTOs, preserving Preserves as the preferred portable boundary, and avoiding Pi-specific command names where they do not match Animus runtime concepts.

## What Changes

- Define a headless command/response/event protocol for prompt, steer, follow-up, abort, get-state, get-messages, model selection, session/ledger operations, compaction, and shutdown.
- Require strict record framing and explicit request/response correlation so clients can stream events while matching command acknowledgements.
- Map runtime event DTOs into protocol events without embedding TUI component state.
- Add safe error responses for parse errors, invalid commands, streaming-mode violations, unavailable services, and cancelled operations.
- Provide conformance tests or fixtures for framing, queueing, response IDs, event order, and malformed input.

## Impact

- **Files**: headless protocol DTOs, Preserves and/or JSON compatibility codecs, protocol docs, conformance fixtures, and runtime shell integration.
- **Testing**: positive protocol fixtures for prompt/steer/follow-up/abort/state; negative fixtures for malformed records, missing IDs, invalid streaming behavior, and unavailable services.
- **Non-claims**: this change does not add a network daemon, web server, MCP server, TUI, Pi subprocess compatibility mode, or generic shell command execution.
