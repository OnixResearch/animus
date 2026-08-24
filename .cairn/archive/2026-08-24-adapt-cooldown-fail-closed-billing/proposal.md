## Why

Animus resolves providers through a typed registry and retries admitted requests under explicit retry-state projection. When a subscription account enters cooldown, the shell must not silently move the live turn onto metered API credit without telling the user.

`digitallyinduced/haskell-agent` makes this a first-class policy: provider transitions preserve the pending turn and durable session state, credential failover understands account cooldowns, and automatic fallback never silently converts subscription usage into API-credit spending. This change binds that policy into Animus provider-retry and usage-accounting surfaces.

## What Changes

- Require cooldown-aware failover that never selects a provider in account cooldown.
- Require that any automatic downgrade from subscription to pay-per-use is visible and recorded before admission.
- Require provider transitions to preserve the pending turn and durable session state.
- Add positive, negative, and boundary fixtures for cooldown and tier-change cases.

## Impact

- **Files**: provider-retry projection, usage-accounting adapter, session evidence types, fixtures, documentation, and this lifecycle package.
- **Testing**: focused provider-retry and usage-accounting rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- A new transport or provider protocol.
- Changes to model or tool semantics.
- Claims that retry preserves byte-level request identity across all providers.
- Permission to spend metered credit merely because a fallback target exists.
