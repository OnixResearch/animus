# Adapt Mcp Readonly Exposure Specification

## Purpose

Defines the `adapt-mcp-readonly-exposure` capability.

## Requirements

### Requirement: Default MCP exposure requires a read-only declaration

r[animus.mcp.exposure] The default MCP tool surface MUST expose only complete descriptors with `readOnlyHint: true` after normal schema and authority checks. Missing, false, malformed, or unknown values MUST NOT qualify for default exposure.

#### Scenario: Declared read-only tool is exposed

r[animus.mcp.exposure.readonly]
- GIVEN a complete MCP descriptor declares `readOnlyHint: true`
- WHEN the default exposure reducer admits the catalog
- THEN the tool MAY enter the default read-only surface

#### Scenario: Mutating or unknown tool stays hidden

r[animus.mcp.exposure.mutating]
- GIVEN a descriptor has a missing, false, malformed, or unknown read-only value
- WHEN default exposure is computed
- THEN the tool MUST remain hidden unless separate mutation authority admits it

### Requirement: Mutating MCP exposure requires exact authority

r[animus.mcp.authority] A non-read-only MCP tool MUST be exposed only when current host and Basalt authority admit the exact server identity, tool name, schema identity, operation class, and session. Server annotations MUST NOT grant authority.

#### Scenario: Exact mutation authority admits one tool

r[animus.mcp.authority.exact]
- GIVEN current authority admits one exact non-read-only MCP tool
- WHEN exposure is computed for the bound session
- THEN only that exact tool MAY enter the exposed catalog

#### Scenario: Read-only metadata proves no behavior

r[animus.mcp.authority.non_claim]
- GIVEN a descriptor declares `readOnlyHint: true`
- WHEN an exposure receipt is produced
- THEN the receipt MUST NOT claim that the tool has no side effects

### Requirement: MCP tool-name collisions fail closed

r[animus.mcp.collision] MCP tool names MUST be preserved. A collision with a built-in tool or another configured server MUST block every conflicting entry and MUST NOT silently rename a tool.

#### Scenario: Duplicate name is rejected

r[animus.mcp.collision.duplicate]
- GIVEN two tool sources declare the same model-facing name
- WHEN the catalog is admitted
- THEN the conflicting entries MUST be rejected
- AND the diagnostic MUST name every conflicting source

### Requirement: Child MCP exposure is an authority intersection

r[animus.mcp.child] A child session MUST receive only the intersection of the root exposed set, the child authority profile, and the current MCP server generation. Child setup MUST NOT broaden the tool set.

#### Scenario: Child broadening fails

r[animus.mcp.child.intersection]
- GIVEN a child profile lacks authority for one root-exposed MCP tool
- WHEN the child tool catalog is projected
- THEN that tool MUST remain hidden from the child

### Requirement: MCP exposure verification is complete

r[animus.mcp.verification] Positive, negative, and boundary fixtures MUST cover descriptor admission, exact mutation authority, collision denial, child intersection, and annotation non-claims.

#### Scenario: MCP exposure matrix passes

r[animus.mcp.verification.matrix]
- GIVEN the MCP exposure rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming descriptors and authority facts MUST behave as declared
- AND every annotation, collision, or child-broadening violation MUST fail as declared
