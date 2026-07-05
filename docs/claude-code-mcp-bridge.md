# Claude Code MCP bridge boundary

Animus treats Claude Code MCP support as a bounded interoperability reference. MCP servers can be mapped into Animus-owned tool definitions by a host adapter, but MCP is not a core dependency and does not replace Animus tool contracts.

## Scope

The bridge is an optional adapter-layer path:

- hosts explicitly register and trust MCP server descriptors before any tool is advertised;
- reusable runtime code only receives validated Animus tool definitions, capability labels, policy references, schema identities, and result bounds;
- MCP process launch, sockets, network access, authentication, server discovery, retries, clocks, cancellation, and transport lifecycle behavior stay in host/std adapters;
- dispatch uses the normal Animus argument validation, capability authorization, mutation coordination, executor availability, and output-bound checks before any MCP server adapter is called;
- receipts record safe server labels, tool names, schema identities, capability labels, redacted resource identifiers, truncation state, status, and BLAKE3 hashes.

## Non-goals

The bridge does not auto-discover or auto-trust MCP servers, launch fallback shell commands, probe network endpoints from reusable crates, copy raw transport payloads into receipts, expose credentials or full paths, or treat MCP schemas as a substitute for strict Animus validation.

## Verification

The focused adapter tests cover positive registration/mapping/dispatch/receipt paths and negative untrusted-server, incomplete-mapping, denied-capability, unsupported-transport, malformed-response, oversized/leaking-receipt, and invalid-hash cases.
