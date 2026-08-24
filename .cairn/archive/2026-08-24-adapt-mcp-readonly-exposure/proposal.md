## Why

Animus can discover and call tools from one explicitly configured MCP standard-input server. Discovery returns untrusted server declarations, while Animus retains tool exposure and execution authority.

`digitallyinduced/haskell-agent` exposes only MCP tools whose descriptors declare `readOnlyHint: true`. This is a useful default, but the declaration is not proof that the tool is side-effect free. This change adapts the exposure rule while keeping current Animus and Basalt authority gates.

## What Changes

- Expose `readOnlyHint: true` tools through the default read-only MCP surface.
- Hide missing, false, or unknown read-only annotations unless explicit mutation authority admits the exact tool.
- Preserve MCP tool names and reject collisions across built-in and configured catalogs.
- Intersect root-session exposure with child-session authority before sharing tools.
- Record that `readOnlyHint` is untrusted metadata, not behavioral proof.

## Impact

- **Files**: `animus-std-mcp`, tool-catalog admission, Basalt authority composition, child-session projection, fixtures, documentation, and this lifecycle package.
- **Testing**: focused MCP, tool-catalog, Basalt, and child-session rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- Proving that an MCP tool is read-only.
- Executing tools during discovery.
- Granting mutation authority from server annotations.
- Resolving name collisions by renaming tools silently.
