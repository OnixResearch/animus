# OpenClaw-inspired node capability-host boundary

Animus treats OpenClaw-style nodes as bounded references for host-owned peripheral capability hosts. The reusable contract is data-only validation in `animus-adapters::node`; it is not a node daemon, device app, socket transport, pairing store, approval database, platform API, or command executor.

## Contracts

- `NodeAdvertisement` validates node identity, gateway profile association, safe platform metadata, capability claims, command allowlists, permission claims, optional metadata pins, and rejection of gateway-owned authority claims.
- `PairingDescriptor` binds node identity, role, approved scopes, command ceilings, token identity class, and pinned metadata so reconnects with platform or command-ceiling drift require host repair or re-approval.
- `plan_node_invocation` produces deterministic `NodeInvocationPlan` values over injected host services only, rejecting unpaired nodes, unsupported commands, permission mismatches, missing services, unavailable services, and ambient execution fallback before host dispatch.
- `NodeExecPlan` and `NodeApproval` require canonical argv identity, safe cwd/raw-command/session labels, selected node identity, policy hash, and per-node approval evidence before high-impact commands such as `system.run` or `system.which` can dispatch.
- `NodeResultEnvelope` and `NodeInvocationReceipt` preserve safe terminal outcomes and evidence hashes without raw prompts, provider payloads, credentials, environment values, full paths, raw command bodies, or unbounded output.

## Non-claims

This contract does not implement iOS, Android, macOS, WebSocket nodes, camera access, screen recording, location access, SMS delivery, device pairing storage, approval persistence, platform APIs, actual remote command execution, or node daemon supervision. Hosts may map these DTOs onto platform-specific adapters, but reusable Animus crates only validate claims, plans, and safe outcomes.

## Focused checks

```sh
cargo test -p animus-adapters node
cargo check --workspace --all-targets
scripts/cairn/check-runtime-facade-inventory --root .
nix run .#cairn -- validate --root .
```
