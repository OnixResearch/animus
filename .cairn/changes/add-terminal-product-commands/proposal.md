## Why

`animus-live-terminal` emits typed descriptors for existing controls. It does not provide exact kernel-clear or state-inspection command classes.

Product hosts need these two commands without giving the terminal kernel, journal, provider, tool, or persistence authority.

## What Changes

- Add dedicated `kernel_clear` and `state_inspection` client command descriptors.
- Add matching explicit control intents and deterministic construction.
- Keep execution and authorization in the worker-owned service.
- Add positive and negative serialization, validation, routing, and authority tests.
- Document the expanded descriptor surface.

## Impact

- **Files**: `animus-live-terminal` core, tests, and terminal host documentation.
- **Testing**: focused terminal tests, strict Clippy, formatting, Octet, Cairn gates, and relevant Nix checks.

## Non-goals

- This change does not execute either command in the terminal.
- This change does not define product-specific authorization or state projection.
- This change does not add endpoint discovery, fallback, or compatibility transport.
