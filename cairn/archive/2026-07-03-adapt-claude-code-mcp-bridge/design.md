# Design: Adapt Claude Code MCP bridge patterns for Animus

## Context

MCP is a useful tool/data-source interoperability protocol supported by Claude Code. Animus should support MCP at the adapter boundary because hosts may want to reuse MCP servers. The core contract, however, should remain Animus-owned: tools have typed definitions, capability labels, permission checks, result bounds, and receipts.

This change specifies how MCP descriptors and calls enter that existing model without turning MCP transport or discovery into reusable core behavior.

## Decisions

### 1. MCP is an optional adapter bridge

**Choice:** MCP server descriptors and tools are converted into Animus tool definitions by a std/host adapter before the runtime sees them.

**Rationale:** The runtime should reason about Animus tool contracts, not protocol-specific transport details.

### 2. Capability mapping is explicit

**Choice:** Every MCP tool mapping declares Animus capability labels, resource/ability policy references, argument schema identity, result bounds, and source metadata.

**Rationale:** External tools need the same permission and receipt path as native tools.

### 3. Transport and server lifecycle stay host-owned

**Choice:** Process launch, sockets, network, authentication, server discovery, retries, and clocks belong to host adapters, not `no_std` core or adapter-contract crates.

**Rationale:** MCP deployments vary widely and can require sensitive credentials or network access.

### 4. Bridge receipts expose safe metadata only

**Choice:** Receipts include server label, tool name, capability labels, status class, schema identity, redacted resource IDs, truncation metadata, and BLAKE3 hashes, excluding raw transport payloads by default.

**Rationale:** Operators need auditability without copying external server secrets or large responses into the session ledger.

## Risks / Trade-offs

- MCP schemas can be weaker than Animus needs. Reject or wrap tools that cannot provide strict validation.
- Auto-discovery is convenient but unsafe. Require explicit host registration and trust decisions.
- Transport errors can leak credentials. Keep error classes typed and sanitize messages before receipts.
