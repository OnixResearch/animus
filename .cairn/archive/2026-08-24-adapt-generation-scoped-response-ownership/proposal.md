## Why

Animus parses streamed provider responses and reuses transport resources through host adapters. A reusable connection can outlive one model exchange. If an exchange is interrupted, malformed, or returned before its terminal frame, late frames can be mistaken for a later response.

`digitallyinduced/haskell-agent` prevents this class of corruption with generation-scoped response ownership. A connection belongs to one exchange generation, and any uncertain exchange poisons the connection instead of returning it to the pool. This change adapts that invariant into Animus streaming contracts.

## What Changes

- Require every reusable streaming exchange to claim one generation-scoped owner.
- Require interruption, malformed framing, or missing terminal frames to poison the connection.
- Require a verified terminal frame before ownership can be released for reuse.
- Add positive, negative, and boundary fixtures for late-frame and concurrent-owner cases.

## Impact

- **Files**: streaming adapter DTOs and reducers, live provider hosts, Iroh transport seams where reusable sessions apply, fixtures, documentation, and this lifecycle package.
- **Testing**: focused streaming and provider-host rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- A new wire protocol or connection pool.
- Claims that a transport delivers frames exactly once.
- Automatic replay after an uncertain exchange.
- Provider-specific semantic validation beyond the admitted terminal-frame contract.
