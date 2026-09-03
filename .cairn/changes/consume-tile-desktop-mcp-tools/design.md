## Context

`animus-std-mcp` starts one explicit MCP standard-input server from typed Nickel configuration. It bounds discovery, checks current authority, owns process teardown, and emits metadata-only receipts.

Tile's MCP bridge already keeps desktop actions on the same daemon policy path as local actions. The Cua adapter change keeps Cua below that path.

The missing consumer behavior is exact tool-profile admission and continuation after a mutation with unknown effect.

## Goals / Non-Goals

**Goals:**

- Consume one exact published Tile MCP cohort.
- Expose only four reviewed Tile desktop tools.
- Preserve Tile action outcome meaning through Animus stages.
- Block blind retry after uncertain desktop mutation.
- Keep receipts bounded and private data absent.

**Non-Goals:**

- Do not start Cua directly.
- Do not move desktop policy into Animus.
- Do not make an MCP descriptor, annotation, or server response into authority.
- Do not persist raw desktop observations in normal receipts.

## Decisions

### 1. Connect to Tile, not Cua

**Choice:** The product composition launches one exact Tile MCP server through `animus-std-mcp`. Cua remains an optional leaf selected by Tile.

**Rationale:** Tile owns desktop consent, capabilities, targets, action planning, and verification. Direct Cua access would create a parallel policy path.

### 2. Use a closed Tile tool profile

**Choice:** Typed Nickel configuration names the exact server identity, package identity, MCP cohort, schema identities, and four allowed tool names.

The allowed set contains `tile.observe`, `tile.preview_action`, `tile.execute_action`, and `tile.verify_action`. Discovery rejects missing, extra, renamed, or drifted required tools.

**Rationale:** A flat external tool catalog is too broad. Exact schema and name admission makes review and authority stable.

### 3. Bind each mutation to current authority

**Choice:** `tile.execute_action` requires the current Animus session, runtime authority seal, server generation, tool schema, target, request, and operation class.

Preview output, MCP annotations, Tile receipts, Cua settings, and prior success cannot widen this binding.

**Rationale:** Tool discovery and preview describe possible behavior. They do not authorize effects.

### 4. Reuse staged partial-effect handling

**Choice:** Map Tile's route, delivery, effect, evidence, and escalation fields into existing staged tool outcomes and partial-effect observations.

Applied can continue under normal policy. Rejected or unsupported returns a typed non-success. Uncertain enters an observation barrier.

**Rationale:** Animus already owns staged execution and interrupted-tool repair. A second desktop retry engine would duplicate authority and state transitions.

### 5. Make uncertainty observation-first

**Choice:** An uncertain result blocks automatic tool retry. The host can call `tile.observe` or the returned Tile verification path under read authority.

A later desktop mutation is a new request with current authority, target generation, and action preview. It is not a replay of the old tool call.

**Rationale:** The first action can have landed before timeout or disconnect. Blind retry can duplicate a submission or destructive effect.

### 6. Keep evidence metadata-only

**Choice:** Animus receipts include BLAKE3-bound server, schema, tool, request, target, stage, effect, evidence-role, and escalation identities.

They omit raw screenshots, accessibility trees, OCR text, typed text, key data, clipboard content, arguments, paths, PIDs, handles, and Cua tokens.

**Rationale:** Action accountability does not require a second durable desktop capture store.

### 7. Validate with an exact fake server

**Choice:** A fixture Tile MCP server implements the accepted schema and programmable outcomes. Host tests cover process lifecycle and protocol errors without native desktop access.

Live Tile and Cua evidence remains separate and belongs to Tile and OnixOS.

**Rationale:** Animus owns client admission and continuation behavior, not native desktop correctness.

## Risks / Trade-offs

**Tile schema changes** → Bind exact schema identities and fail closed on drift.

**Observation lacks decisive evidence** → Keep the action uncertain and require user or policy resolution.

**Four tools still permit mutation** → Apply existing exact mutation authority and runtime seal checks.

**Receipts become too detailed** → Enforce field allowlists and negative privacy fixtures.

**Tile is unavailable** → Return a typed unavailable result and clean the owned MCP process. Do not start Cua directly.

## Validation Plan

- Run current `animus-std-mcp`, tool-stage, authority-seal, and interrupted-tool tests before implementation.
- Validate the Nickel profile and generated native configuration.
- Test exact four-tool discovery and rejection of extra, missing, renamed, colliding, or drifted tools.
- Test current authority and seal bindings before `tile.execute_action`.
- Test applied, rejected, unsupported, uncertain, disputed, and unavailable-verifier outcomes.
- Test timeout, cancellation, malformed response, late response, process death, EOF, and teardown.
- Assert that uncertain outcomes cannot enter automatic retry.
- Assert that observations and receipts omit prohibited private fields.
- Run focused MCP, stages, authority, configuration, Cairn, and dependency-isolation rails.
