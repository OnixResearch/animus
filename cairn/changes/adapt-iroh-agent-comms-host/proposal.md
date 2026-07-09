# Proposal: Adapt Iroh as an agent-comms host transport

## Why

Iroh is a good fit for optional peer-to-peer agent communication, but the reusable runtime must not become an Iroh client or expose raw tickets, relays, private keys, or endpoint bodies. After extracting the neutral agent-comms contract, Animus should keep Iroh as a thin host-owned adapter that maps safe Iroh route metadata into neutral frames and delegates all live networking to caller-owned transport code.

## What Changes

- Define an `iroh-host` route descriptor that validates safe node/topic labels, ALPN/protocol label, scope, optional ticket or node-address hash, max frame bounds, and prior remote admission.
- Map Iroh routes into neutral agent-comms routes and frames rather than keeping shared frame logic inside Iroh-named types.
- Provide a host-injected Iroh transport port for publish, query/subscribe, and orchestration-event delivery over already validated frames.
- Reject raw Iroh tickets, node-address bodies, relay URLs, private keys, auth headers, credentials, prompts, provider payloads, file bodies, tool payloads, model outputs, and unbounded logs before live transport effects.
- Document live endpoint setup, ticket parsing, auth, persistence, retry, relay, and smoke tests as host-owned responsibilities.

## Impact

- **Files**: future `animus-std` Iroh adapter module, feature exports, docs, tests, and root facade exports.
- **Testing**: positive fake-transport publish/query/event tests plus negative raw-ticket, invalid protocol, missing admission, cross-scope, oversized frame, unavailable transport, and receipt leak tests.
- **Non-claims**: this does not add an Iroh endpoint, relay client, ticket parser, key store, daemon, service discovery, production remote-session UX, or network reliability claim.
