## Why

`animus-live-terminal` emits typed descriptors for existing controls. It does not provide exact kernel-clear or state-inspection command classes.

Product hosts need these two commands without giving the terminal kernel, journal, provider, tool, or persistence authority.

## What Changes

- Add dedicated `kernel_clear` and `state_inspection` client command descriptors.
- Add matching explicit control intents and deterministic construction.
- Keep execution and authorization in the worker-owned service.
- Return the exact completed runtime-event snapshot to the worker-owned delivery path.
- Add a bounded private-event relay for exact versioned runtime events.
- Add a persistent live Steel session owner with one-effect driving, exact queue guards, cancellation, bounded event cursors, and explicit close.
- Keep private payloads out of supervisor decoding, retention, debug output, and receipts.
- Add positive and negative serialization, validation, routing, rendering, session-control, and authority tests.
- Document the expanded command and event surfaces.

## Impact

- **Files**: `animus-live-steel-agent`, `animus-live-terminal`, `animus-local-daemon`, tests, lifecycle inventory, and host documentation.
- **Testing**: focused terminal tests, strict Clippy, formatting, Octet, Cairn gates, and relevant Nix checks.

## Non-goals

- This change does not execute either command in the terminal.
- This change does not define product-specific authorization. It exposes only worker-invoked live-session effects behind existing Animus policy and authority checks.
- This change does not add terminal persistence or session-store access.
- This change does not add endpoint discovery, fallback, or compatibility transport.
