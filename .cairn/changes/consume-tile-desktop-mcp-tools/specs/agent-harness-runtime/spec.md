## ADDED Requirements

### Requirement: Animus uses an exact Tile desktop MCP profile [r[agent_harness_runtime.tile_desktop_mcp.profile]]

Animus MUST bind one published Tile package, MCP cohort, server identity, server generation, tool set, tool schemas, and action-outcome schema before desktop tool exposure.

The profile MUST admit only `tile.observe`, `tile.preview_action`, `tile.execute_action`, and `tile.verify_action`. It MUST NOT expose Cua tools directly.

#### Scenario: Exact Tile tools are admitted [r[agent_harness_runtime.tile_desktop_mcp.profile.discovery]]

- GIVEN the configured Tile server matches every pinned package, protocol, identity, generation, tool, and schema fact
- WHEN bounded MCP discovery runs
- THEN Animus exposes the four declared Tile desktop tools under normal capability filtering
- AND it does not expose any Cua or undeclared Tile tool

#### Scenario: Tool or schema drift fails closed [r[agent_harness_runtime.tile_desktop_mcp.profile.drift]]

- GIVEN a required tool is missing, renamed, duplicated, malformed, extra, or bound to a different schema
- WHEN discovery admission runs
- THEN the Tile desktop profile is unavailable
- AND no mutating Tile tool reaches the model

### Requirement: Tile desktop mutations require current Animus authority [r[agent_harness_runtime.tile_desktop_mcp.authority]]

Each `tile.execute_action` call MUST bind the current session, runtime authority seal, server generation, tool schema, operation class, target, and request identities.

Preview output, annotations, receipts, prior calls, Tile responses, Cua configuration, and connection success MUST NOT grant or widen authority.

#### Scenario: Current exact authority admits execution [r[agent_harness_runtime.tile_desktop_mcp.authority.admitted]]

- GIVEN the current authority seal admits the exact Tile server, tool, operation, target, request, and session
- WHEN staged execution reaches final admission
- THEN the MCP host can send the call to Tile
- AND Tile retains its separate desktop authority decision

#### Scenario: Missing authority stops before the call [r[agent_harness_runtime.tile_desktop_mcp.authority.denied]]

- GIVEN any required current authority or generation binding is missing or stale
- WHEN staged execution evaluates the Tile mutation
- THEN Animus returns a typed denial
- AND no MCP request or native desktop effect occurs

### Requirement: Tile action outcomes retain their meaning [r[agent_harness_runtime.tile_desktop_mcp.outcomes]]

Animus MUST preserve Tile's route, delivery, effect, evidence, and escalation fields through staged execution. It MUST NOT reduce MCP success to desktop success.

#### Scenario: Applied outcome continues normally [r[agent_harness_runtime.tile_desktop_mcp.outcomes.applied]]

- GIVEN Tile returns an applied effect with admitted evidence metadata
- WHEN Animus maps the result
- THEN the staged outcome records applied
- AND continuation follows the existing policy for a completed mutating tool

#### Scenario: Non-success outcome stays explicit [r[agent_harness_runtime.tile_desktop_mcp.outcomes.non-success]]

- GIVEN Tile returns rejected, unsupported, disputed, unavailable, or uncertain
- WHEN Animus maps the result
- THEN the exact non-success class remains visible
- AND Animus does not fabricate successful desktop completion

### Requirement: Uncertain Tile effects create an observation barrier [r[agent_harness_runtime.tile_desktop_mcp.uncertain-barrier]]

An uncertain Tile desktop effect MUST block automatic retry. Animus MAY request a fresh Tile observation under read authority before planning a new mutation.

A later mutation MUST use current authority, target generation, and preview. It MUST NOT reuse the old call as replay authority.

#### Scenario: Timeout cannot replay the action [r[agent_harness_runtime.tile_desktop_mcp.uncertain-barrier.timeout]]

- GIVEN Tile can have delivered a mutation before timeout, disconnect, or process loss
- WHEN Animus receives or reconstructs an uncertain effect
- THEN the continuation state requests observation or explicit resolution
- AND it does not resend the mutation

#### Scenario: Fresh observation permits a new request [r[agent_harness_runtime.tile_desktop_mcp.uncertain-barrier.observed]]

- GIVEN a fresh Tile observation resolves or narrows the prior uncertain effect
- WHEN the controller evaluates the next action
- THEN any mutation is a new staged request with current bindings
- AND foreground escalation requires its own current authority decision

### Requirement: Tile desktop receipts are metadata-only [r[agent_harness_runtime.tile_desktop_mcp.receipts]]

Animus receipts MUST bind safe server, schema, tool, request, target, stage, effect, evidence-role, and escalation identities with BLAKE3.

Receipts MUST omit raw screenshots, trees, OCR text, typed text, key data, clipboard content, arguments, paths, PIDs, handles, and Cua tokens.

#### Scenario: Safe receipt omits private desktop data [r[agent_harness_runtime.tile_desktop_mcp.receipts.redaction]]

- GIVEN a Tile call contains sensitive action or observation data
- WHEN Animus emits its normal receipt
- THEN only admitted metadata and digests appear
- AND prohibited raw fields are absent

### Requirement: Tile desktop MCP conformance covers failures [r[agent_harness_runtime.tile_desktop_mcp.conformance]]

Positive and negative fake-server fixtures MUST cover discovery, authority, outcomes, uncertainty, privacy, timeout, cancellation, malformed responses, process loss, EOF, and teardown.

#### Scenario: Failure matrix is complete [r[agent_harness_runtime.tile_desktop_mcp.conformance.failures]]

- GIVEN the exact fake Tile MCP server and fixture matrix
- WHEN focused conformance tests run
- THEN each admitted, denied, applied, rejected, unsupported, uncertain, malformed, interrupted, and cleanup path has an exact result
- AND the fixture performs no native desktop or Cua effects

#### Scenario: Closeout preserves dependency boundaries [r[agent_harness_runtime.tile_desktop_mcp.conformance.closeout]]

- GIVEN the change is ready to archive
- WHEN focused tests, configuration checks, dependency audits, formatting, and Cairn gates run
- THEN every required result passes
- AND Animus reusable crates contain no Cua runtime or native desktop dependency

### Requirement: Cua remains below Tile [r[agent_harness_runtime.tile_desktop_mcp.boundary]]

Animus MUST treat Cua Driver as an optional Tile adapter implementation. Animus MUST NOT import or directly configure Cua source, SDK, ABI, MCP tools, permissions, history, updater, or telemetry.

#### Scenario: Tile is unavailable [r[agent_harness_runtime.tile_desktop_mcp.boundary.unavailable]]

- GIVEN the configured Tile MCP server cannot start or reach readiness
- WHEN Animus requests the desktop profile
- THEN Animus returns a typed unavailable result and cleans owned MCP resources
- AND it does not discover or start Cua as a fallback
