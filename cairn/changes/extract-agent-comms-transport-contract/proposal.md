# Proposal: Extract agent-comms transport contract

## Why

The initial Iroh host seam proved that remote agent communication needs route admission, scope checks, bounded frame metadata, redaction evidence, and transport-owned effects. Those concerns are not Iroh-specific. If the reusable contract stays named after Iroh, future WebSocket, local IPC, QUIC, Matrix, MCP, or dataspace transports will either duplicate the same validation logic or depend on misleading Iroh types.

Animus should define a transport-neutral agent-comms contract first, then let Iroh become one thin host transport over that contract.

## What Changes

- Define transport-neutral agent-comms route, frame, frame-kind, admission, redaction, and receipt DTOs.
- Move route admission, tenant/workspace scope checks, maximum frame sizing, payload hashing, and unsafe raw-material rejection into pure reusable helpers.
- Provide projection helpers for dataspace assertions, orchestration events, headless command/event metadata, and subagent lifecycle metadata without embedding transport details.
- Define a host-injected transport port that publishes/queries validated frames without constructing live network clients in reusable crates.
- Document Iroh, WebSocket, local IPC, Matrix, MCP, and future dataspace transports as host-owned implementations of the neutral contract.

## Impact

- **Files**: future `animus-adapters` or shared std-shell agent-comms DTOs, `animus-std` generic shell module, Iroh seam refactor, docs, tests, and facade exports.
- **Testing**: positive route/frame/projection/transport-port tests plus negative missing admission, scope mismatch, oversized frame, raw ticket/URL/credential/prompt/provider payload, missing redaction, invalid hash, and unavailable transport cases.
- **Non-claims**: this does not add a daemon, network server, live Iroh endpoint, WebSocket server, Matrix bridge, MCP server, service discovery, ticket parser, credential store, retry policy, or durable remote-session store.
