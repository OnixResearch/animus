## Why

Animus already owns a bounded MCP standard-input client, staged tool execution, runtime authority seals, interrupted-tool repair, and metadata-only receipts.

Tile exposes four desktop tools through its own MCP bridge. A future Cua Driver adapter belongs below that Tile bridge, not beside it.

Animus needs one exact consumer profile that preserves Tile's action outcomes. An uncertain desktop mutation must create an observation barrier instead of a blind tool retry.

## What Changes

- Add a typed Nickel profile for one exact Tile MCP server cohort.
- Admit only `tile.observe`, `tile.preview_action`, `tile.execute_action`, and `tile.verify_action` for this profile.
- Bind server, package, schema, tool, session, authority-seal, target, and request identities before each mutation.
- Map Tile delivery, effect, evidence, and escalation results into existing staged tool outcomes.
- Require a fresh Tile observation after an uncertain effect before any new desktop action.
- Add bounded receipts that omit raw screenshots, trees, typed text, clipboard data, arguments, and native handles.
- Add fake-server positive and negative conformance fixtures.

## Capabilities

### New Capabilities

- `tile-desktop-mcp-profile`: Exact Tile desktop tool discovery and call admission.
- `tile-desktop-uncertain-effect-barrier`: Observation-first continuation after uncertain desktop mutation.

### Modified Capabilities

- `mcp-client-host`: Adds one reviewed Tile server profile without broadening generic MCP authority.
- `tool-execution-stages`: Preserves Tile effect and escalation states through existing staged outcomes.

## Impact

- **Configuration**: `config/repo.ncl` owns the reviewed Tile MCP profile and native export.
- **Host**: `animus-std-mcp` remains the only process and protocol owner.
- **Runtime**: Existing authority seals and stage transitions govern Tile mutations.
- **Evidence**: Safe receipts distinguish call delivery from desktop effect.
- **Testing**: A fake Tile server covers exact success, refusal, uncertainty, drift, timeout, and cleanup.

## Dependencies

- A published Tile cohort that implements `harden-desktop-action-outcomes`
- The optional Tile Cua adapter only when Cua-backed host access is selected

Cua Driver remains an adapter-private Tile dependency. Animus does not import Cua source, SDK, ABI, configuration, or tool schemas.

## Non-Goals

- No direct `cua-driver mcp` process in Animus.
- No Cua tool catalog, permission mode, manifest, session, target token, history, updater, or telemetry surface.
- No new desktop domain model in Animus core.
- No automatic retry or foreground escalation.
- No raw desktop capture in receipts or durable session metadata.
- No claim that MCP success proves desktop effect, Cua support, platform support, or application correctness.
