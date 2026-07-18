# Goose ACP/MCP interoperability adaptation

Animus adapts Goose's standards-oriented ACP/MCP posture as data-only descriptors, bridge planning, and safe receipts. Reusable validation does not start MCP servers, probe HTTP endpoints, discover binaries, read credentials, contact external agents, or execute tools.

## Runtime boundary

- `McpDescriptor` validates MCP transport kind, command/endpoint labels, tools, resources, prompts, schema IDs, result contracts, timeout class, environment key names, available-tool ceilings, and provenance hash.
- MCP command and endpoint fields are labels only. Raw URLs, command lines, and environment values fail validation.
- `AcpDescriptor` validates external agent identity, model/profile preference, permission-mode mapping, session correlation, known limitations, and tool pass-through policy.
- ACP mappings cannot downgrade a guarded Animus permission mode to unrestricted external authority without explicit host admission.
- `resolve_tool_pass_through` limits bridge tools to the intersection of Animus capability ceilings, active permission-mode tools, host admission, external agent mode, and descriptor-available tools.
- `InteropReceipt` records operation/status, BLAKE3 identities, bounded counts, and redaction markers only.

## Bounded local ACP host

The descriptor layer remains reusable data-only policy. The optional production adapter is documented in [`acp-stdio-host.md`](acp-stdio-host.md). It pins `agent-client-protocol` 0.10.4 / schema 0.11.4 under Apache-2.0, uses only bounded local newline-delimited stdio, and delegates transcript/run/durable ownership to `animus_std::coding_session_host`. ACP requests carrying MCP servers fail closed; this adapter does not start MCP servers or widen remote authority.

## Verification

Focused verification for this surface:

```sh
cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal adapt-goose-acp-mcp-interop --root .
nix run .#cairn -- gate design adapt-goose-acp-mcp-interop --root .
nix run .#cairn -- gate tasks adapt-goose-acp-mcp-interop --root .
```

The adapter tests include positive stdio and streamable-HTTP MCP descriptors, explicit ACP descriptors, data-only neutrality, tool pass-through intersection, and safe receipt cases plus negative unknown transport, duplicate tools, missing timeout, env-value leakage, unsafe endpoint, permission downgrade, missing session correlation, bridge widening, and receipt leak cases.
