## Context

`animus-std-mcp` owns explicit MCP process startup, bounded discovery, current authority gates, calls, cancellation, and teardown. MCP descriptors are untrusted request-local input. The tool catalog needs a deterministic rule for default exposure without treating server claims as authority.

`digitallyinduced/haskell-agent` exposes only tools annotated `readOnlyHint: true`. Animus can use this as a default-admission input, then intersect it with existing authority seals and Basalt UCAN checks.

## Goals

- Expose declared read-only tools through the default MCP surface.
- Require explicit authority for every other tool.
- Reject tool-name collisions before model exposure.
- Apply the same or narrower policy to child sessions.
- Keep annotations classified as untrusted metadata.

## Non-goals

- Behavioral proof that a declared read-only tool has no side effects.
- Ambient discovery of MCP servers.
- Automatic approval of mutating tools.
- Server-controlled tool renaming or namespace rewriting.

## Decisions

### Decision: Read-only annotation enables only default exposure

**Choice:** A complete descriptor with `readOnlyHint: true` can enter the default read-only catalog after normal schema and authority checks. Missing, false, malformed, or unknown values do not qualify.

**Rationale:** Default denial prevents incomplete descriptors from becoming mutation authority.

### Decision: Mutation needs exact authority

**Choice:** A non-read-only tool can be exposed only when current host and Basalt authority admit the exact server identity, tool name, schema identity, operation class, and session.

**Rationale:** A server declaration cannot grant authority to the model.

### Decision: Tool names do not collide

**Choice:** MCP names are preserved. Any collision with a built-in tool or another configured server blocks the conflicting entries and names every source.

**Rationale:** Silent renaming changes the model-facing action language and can route calls to the wrong capability.

### Decision: Child sessions receive an authority intersection

**Choice:** A child receives the intersection of the root exposed set, the child authority profile, and the current MCP server generation. Child setup cannot broaden exposure.

**Rationale:** Shared process lifetime does not imply shared mutation authority.

## Dependency Direction

```text
MCP process host and Basalt authority services
                    ^
                    |
tool descriptors -- pure exposure reducer -- child projection
```

The reducer consumes supplied descriptors and authority facts. It does not spawn servers, call tools, read credentials, or inspect effects.

## Adversarial Audit

The audit must try to show that Animus:

- exposes a missing, false, or malformed read-only annotation by default;
- lets `readOnlyHint` grant mutation authority;
- silently renames a colliding tool;
- exposes two tools with one model-facing name;
- gives a child a broader MCP set than its parent or profile;
- claims that a passing descriptor proves no side effects.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Strict default exposure can hide useful tools until policy admits them. That is the intended safety posture.
- Servers can lie about `readOnlyHint`. Existing runtime authority and sandbox controls remain necessary.
- Name preservation makes some server combinations unusable until the operator resolves the collision.

## Non-Claims

This change does not prove MCP server integrity, tool behavior, side-effect freedom, sandboxing, credential safety, remote trust, or successful execution.
