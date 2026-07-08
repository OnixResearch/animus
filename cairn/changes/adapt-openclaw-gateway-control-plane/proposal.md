# Proposal: Adapt OpenClaw gateway control-plane contracts

## Why

Animus has reusable headless and host-control DTOs, but it does not yet name the optional product-host gateway shape that a daemon or attach shell would expose. OpenClaw provides a useful bounded reference for a single control plane that unifies operators, UIs, nodes, events, snapshots, and side-effect retries without making the gateway itself part of reusable runtime core.

This change adapts the protocol lessons as Animus-owned DTOs and validation rules before any server, WebSocket transport, dashboard, or daemon is claimed.

## What Changes

- Define an OpenClaw-inspired gateway protocol profile for first-frame connect, challenge binding, version negotiation, client role, scopes, feature lists, and safe hello snapshots.
- Define transport-neutral request, response, and event frame DTOs with correlation IDs, safe errors, sequence metadata, and state-version/snapshot hooks.
- Require idempotency metadata for side-effecting gateway methods so host transports can retry without duplicate effects.
- Keep WebSocket servers, HTTP endpoints, sockets, dashboards, auth token storage, provider sessions, and daemon supervision outside reusable Animus crates.

## Impact

- **Files**: future adapter/runtime DTO modules, focused protocol fixtures, docs, facade inventory updates, and host-shell examples.
- **Testing**: positive connect/frame/idempotency/snapshot fixtures; negative non-connect first frame, protocol mismatch, missing idempotency, unsafe error, stale state-version, and ambient transport fallback tests.
- **Non-claims**: this does not implement a gateway daemon, WebSocket server, HTTP server, Control UI, channel connector, provider session, credential store, or production remote access story.
