# Proposal: Adapt Claude Code MCP bridge patterns for Animus

## Why

Claude Code's MCP support demonstrates a practical interoperability pattern: external data sources and tools can be registered without baking every integration into the agent. Animus already has tool contracts and plugin-manifest work, but it needs a bounded MCP bridge contract so MCP servers can map into Animus capabilities without becoming core dependencies or bypassing policy.

This change adapts MCP as an optional adapter-layer bridge with strict schema validation, capability mapping, and safe transport metadata.

## What Changes

- Define MCP server and tool descriptors as optional adapter inputs that map into Animus tool definitions and capability labels.
- Require strict argument/result schemas, resource/ability mapping, and permission checks before MCP tool dispatch.
- Keep MCP transport, process management, network access, and server discovery in std/host adapters.
- Add safe MCP bridge receipts with server identity labels, tool names, redacted resource IDs, status classes, and hashes.
- Add positive and negative tests for valid tool mapping, malformed schemas, unsupported transports, denied capabilities, untrusted servers, missing executors, and leak attempts.

## Impact

- **Files**: future MCP bridge DTOs, adapter contracts, schema translators, policy fixtures, receipts, tests, and docs.
- **Testing**: valid MCP-to-Animus tool mapping plus negative tests for schema mismatch, unknown capabilities, denied resources, missing server adapters, unsafe transport labels, malformed results, and receipt leaks.
- **Non-claims**: this change does not make MCP a core dependency, does not auto-discover or auto-trust servers, does not permit network/process execution from reusable core crates, and does not replace Animus-owned tool contracts.
