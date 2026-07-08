# Design: Add external subagent conformance fixtures

External subagent descriptors need a reusable conformance rail that is independent from any live ACP or MCP implementation.

## Decisions

### 1. Fixtures cover descriptor admission, not live execution

**Choice:** Fixtures validate descriptor shape, service injection labels, declared tools, credential references, protocol labels, capability ceilings, and output-import safety.

**Rationale:** Pure validation can run in normal CI without external agents.

### 2. Negative cases are first-class

**Choice:** Each known unsafe pattern gets an expected failure fixture: missing service, undeclared tool, ambient CLI discovery, inherited credentials, malformed protocol, unsafe output import, and overbroad capability claim.

**Rationale:** External subagents are a trust boundary, so failure behavior should be documented and regression-tested.

### 3. Conformance index is machine-checkable

**Choice:** A small index records fixture name, expected status, failure class, and requirement coverage.

**Rationale:** Maintainers can see which external-subagent boundaries are covered and avoid silent fixture drift.

### 4. Live adapters remain optional

**Choice:** The rail does not require an ACP/MCP server, CLI, network access, or credentials.

**Rationale:** Descriptor safety must be testable without standing up the world.
