## Phase 1: Published contract and baseline

- [ ] [serial] [task:tile-mcp-contract-intake] Record the published Tile cohort, MCP version, package identity, four tool schemas, outcome schema, and source provenance. r[agent_harness_runtime.tile_desktop_mcp.profile]
  - Verify: floating packages, unknown schema identities, direct Cua endpoints, and ambient executable discovery remain rejected.
- [ ] [parallel] [task:tile-mcp-baseline] Run existing MCP client, tool-stage, authority-seal, and interrupted-tool tests before core changes. r[agent_harness_runtime.tile_desktop_mcp.conformance]
  - Verify: record existing failures without changing their meaning or suppressing negative cases.

## Phase 2: Exact profile and authority

- [ ] [serial] [task:tile-mcp-nickel-profile] [after:tile-mcp-contract-intake] Add the typed Nickel Tile MCP profile and regenerate native configuration. r[agent_harness_runtime.tile_desktop_mcp.profile]
  - Verify: only `tile.observe`, `tile.preview_action`, `tile.execute_action`, and `tile.verify_action` are admitted.
- [ ] [serial] [task:tile-mcp-discovery-admission] [after:tile-mcp-nickel-profile] Admit exact server generation, package, protocol, tool names, and schemas through `animus-std-mcp`. r[agent_harness_runtime.tile_desktop_mcp.profile.discovery]
  - Verify: missing, extra, renamed, colliding, malformed, and schema-drifted required tools fail before model exposure.
- [ ] [parallel] [task:tile-mcp-mutation-authority] [after:tile-mcp-discovery-admission] Bind execute calls to current session, runtime seal, operation class, server generation, schema, target, and request identities. r[agent_harness_runtime.tile_desktop_mcp.authority]
  - Verify: preview data, annotations, prior receipts, Tile responses, or Cua configuration cannot widen authority.

## Phase 3: Outcome and continuation

- [ ] [serial] [task:tile-mcp-outcome-mapping] [after:tile-mcp-discovery-admission] Map Tile route, delivery, effect, evidence, and escalation into existing staged outcomes. r[agent_harness_runtime.tile_desktop_mcp.outcomes]
  - Verify: rejected, unsupported, disputed, unavailable, and uncertain states remain non-success outcomes.
- [ ] [serial] [task:tile-mcp-uncertain-barrier] [after:tile-mcp-outcome-mapping] Add observation-first continuation for uncertain desktop effects. r[agent_harness_runtime.tile_desktop_mcp.uncertain-barrier]
  - Verify: no automatic tool retry or foreground action occurs before a fresh observation and newly admitted request.
- [ ] [parallel] [task:tile-mcp-safe-receipts] [after:tile-mcp-outcome-mapping] Add metadata-only call, denial, effect, escalation, and teardown receipts. r[agent_harness_runtime.tile_desktop_mcp.receipts]
  - Verify: raw capture, tree, OCR, typed text, key, clipboard, argument, path, PID, handle, and Cua-token fields are absent.

## Phase 4: Conformance and closeout

- [ ] [serial] [task:tile-mcp-fake-server] [after:tile-mcp-uncertain-barrier] Add an exact fake Tile MCP server with programmable positive and negative outcomes. r[agent_harness_runtime.tile_desktop_mcp.conformance]
  - Verify: fixture behavior cannot perform native desktop, Cua, network, credential, filesystem, or unrelated process effects.
- [ ] [parallel] [task:tile-mcp-host-failures] [after:tile-mcp-fake-server] Test timeout, cancellation, malformed response, late response, process death, EOF, unavailable server, and cleanup. r[agent_harness_runtime.tile_desktop_mcp.conformance.failures]
  - Verify: unknown effects remain visible, no mutation is replayed, and owned processes reach teardown.
- [ ] [parallel] [task:tile-mcp-documentation] [after:tile-mcp-nickel-profile] Document configuration, authority, continuation, privacy, dependency, and support boundaries. r[agent_harness_runtime.tile_desktop_mcp.boundary]
  - Verify: documentation states that Cua remains below Tile and Animus owns no native desktop correctness claim.
- [ ] [serial] [task:tile-mcp-final-gates] [after:tile-mcp-host-failures] [after:tile-mcp-safe-receipts] [after:tile-mcp-documentation] Run MCP, stages, authority, configuration, dependency-isolation, Cairn, and formatting rails. r[agent_harness_runtime.tile_desktop_mcp.conformance.closeout]
  - Verify: positive and negative fixtures pass, and no direct Cua or broad desktop tool surface enters Animus.
