# OpenClaw-inspired gateway control-plane boundary

Animus treats OpenClaw's gateway as a bounded reference for an optional host-owned control-plane shape. The reusable contract is data-only validation in `animus-adapters::gateway`; it is not a server, socket binding, dashboard, provider session, credential store, daemon supervisor, or channel connector.

## Contracts

- `GatewayProfile` names the bounded OpenClaw reference, protocol version, required feature set, and rejected product surfaces.
- `GatewayConnectHandshake` validates the first-frame connect handshake with protocol range negotiation, client identity, role, scopes, advertised features, challenge evidence, and a safe hello snapshot.
- `GatewayRequestFrame`, `GatewayResponseFrame`, and `GatewayEventFrame` validate correlation IDs, methods or event names, bounded payload classes, safe errors, sequence metadata, state-version metadata, and optional snapshot hooks without carrying raw prompts, provider payloads, credentials, private paths, raw tool bodies, or unbounded output.
- `GatewayIdempotency` binds side-effecting request retries to a BLAKE3 key, method, actor, and safe target metadata hash before any host dispatch.
- `validate_gateway_first_frame`, `negotiate_gateway_protocol`, and `ensure_gateway_state_fresh` provide pure admission checks that hosts can call before transport or service effects.

## Non-claims

This contract does not implement an OpenClaw gateway, WebSocket server, HTTP API, dashboard, channel connector, provider session, auth-token store, dedupe cache, remote access policy, daemon lifecycle, or durable session store. Hosts may transport the DTOs over JSONL, Preserves, local IPC, SDK calls, or a future product protocol, but reusable crates do not discover or own those transports.

## Focused checks

```sh
cargo test -p animus-adapters gateway
cargo check --workspace --all-targets
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
```
