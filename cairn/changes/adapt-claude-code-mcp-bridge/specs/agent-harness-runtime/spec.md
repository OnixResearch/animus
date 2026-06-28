# agent-harness-runtime Change Spec: Claude Code MCP bridge adaptation

## Requirements

### Requirement: Bounded MCP bridge reference

r[agent_harness_runtime.claude_code_mcp.scope] Animus MUST treat Claude Code MCP support as a bounded interoperability reference, and MUST NOT make MCP a core dependency, auto-discover or auto-trust servers, execute network or process transports from reusable core crates, or replace Animus-owned tool contracts.

#### Scenario: Untrusted MCP server is not advertised

r[agent_harness_runtime.claude_code_mcp.scope.untrusted]
- GIVEN a host discovers or names an MCP server without a trust decision and explicit registration
- WHEN tool registry construction runs
- THEN the server's tools are not advertised to the model
- AND no server process, network connection, or transport probe is started by reusable runtime code.

### Requirement: MCP descriptors map into Animus tools

r[agent_harness_runtime.claude_code_mcp.mapping] MCP server and tool descriptors MUST be converted into Animus tool definitions with name, schema identity, capability labels, source metadata, result bounds, and resource/ability policy references before dispatch.

#### Scenario: Incomplete MCP mapping is rejected

r[agent_harness_runtime.claude_code_mcp.mapping.incomplete]
- GIVEN an MCP tool descriptor lacks strict schema identity, capability labels, result bounds, source metadata, or policy resource mapping
- WHEN bridge mapping validation runs
- THEN validation returns typed diagnostics
- AND the tool is not registered as an Animus tool.

### Requirement: MCP dispatch follows normal permission and capability checks

r[agent_harness_runtime.claude_code_mcp.dispatch] MCP-backed tool dispatch MUST pass Animus argument validation, permission resolution, capability authorization, mutation coordination, and result-bound checks before the response is accepted.

#### Scenario: Denied MCP capability prevents call

r[agent_harness_runtime.claude_code_mcp.dispatch.denied]
- GIVEN an MCP-backed tool maps to a network or filesystem capability denied by current policy
- WHEN the model requests that tool
- THEN dispatch returns a typed denied-capability failure
- AND the MCP server adapter is not called.

### Requirement: MCP transport lifecycle is host-owned

r[agent_harness_runtime.claude_code_mcp.transport] MCP process management, sockets, network access, authentication, server discovery, retries, clocks, and cancellation MUST remain host-owned adapter behavior with typed unavailable, unsupported, denied, timeout, and malformed-response failures.

#### Scenario: Unsupported MCP transport fails safely

r[agent_harness_runtime.claude_code_mcp.transport.unsupported]
- GIVEN a registered MCP tool requires a transport unsupported by the selected host adapter
- WHEN dispatch attempts to prepare the call
- THEN runtime reports an unsupported-transport failure
- AND no fallback shell command, network probe, or provider call is attempted.

### Requirement: Safe MCP bridge receipts

r[agent_harness_runtime.claude_code_mcp.receipts] MCP bridge receipts MUST include safe metadata such as server label, tool name, capability labels, schema identity, status class, redacted resource identity, truncation status, and BLAKE3 hashes, and MUST NOT include raw transport payloads, credentials, full paths, provider payloads, or unbounded external results by default.

#### Scenario: Malformed MCP response is redacted

r[agent_harness_runtime.claude_code_mcp.receipts.malformed]
- GIVEN an MCP server returns a malformed or oversized response
- WHEN the bridge records the failure
- THEN the receipt records status class, schema identity, truncation state, and safe hashes
- AND raw transport payloads and sensitive fields are omitted from receipts and model-visible context.
