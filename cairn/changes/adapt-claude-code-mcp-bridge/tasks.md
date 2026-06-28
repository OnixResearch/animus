# Tasks

- [ ] [serial] Define optional MCP bridge scope and document that MCP remains an adapter-layer interoperability path rather than a core dependency. r[agent_harness_runtime.claude_code_mcp.scope]
- [ ] [depends:scope] Add tests proving untrusted or unregistered MCP servers are not advertised and do not trigger process, network, or transport probes. r[agent_harness_runtime.claude_code_mcp.scope.untrusted]
- [ ] [serial] Define MCP-to-Animus mapping DTOs for tool name, schema identity, capability labels, source metadata, result bounds, and resource/ability policy references. r[agent_harness_runtime.claude_code_mcp.mapping]
- [ ] [depends:mapping] Add negative mapping tests for missing strict schema, capability labels, result bounds, source metadata, and policy resources. r[agent_harness_runtime.claude_code_mcp.mapping.incomplete]
- [ ] [serial] Wire MCP-backed dispatch through existing Animus argument validation, permission, capability, mutation, and result-bound checks. r[agent_harness_runtime.claude_code_mcp.dispatch]
- [ ] [depends:dispatch] Add denied-capability tests proving MCP server adapters are not called when policy denies mapped capabilities. r[agent_harness_runtime.claude_code_mcp.dispatch.denied]
- [ ] [serial] Add host-owned MCP transport lifecycle adapter contracts with unavailable, unsupported, denied, timeout, and malformed-response failure classes. r[agent_harness_runtime.claude_code_mcp.transport]
- [ ] [depends:transport] Add unsupported-transport tests proving no fallback shell, network, or provider call is attempted. r[agent_harness_runtime.claude_code_mcp.transport.unsupported]
- [ ] [serial] Define safe MCP bridge receipts with server labels, tool names, schema identity, status, redacted resources, truncation metadata, and BLAKE3 hashes. r[agent_harness_runtime.claude_code_mcp.receipts]
- [ ] [depends:receipts] Add malformed/oversized response leak-rejection tests for raw transport payloads, credentials, full paths, provider payloads, and unbounded external results. r[agent_harness_runtime.claude_code_mcp.receipts.malformed]
- [ ] [depends:scope.untrusted,mapping.incomplete,dispatch.denied,transport.unsupported,receipts.malformed] Run focused MCP bridge tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_mcp.scope]
