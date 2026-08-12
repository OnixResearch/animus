# agent-harness-runtime Change Spec: Goose ACP/MCP interoperability adaptation

## Requirements

### Requirement: MCP descriptors are explicit capability claims

r[agent_harness_runtime.goose_acp_mcp_interop.mcp_descriptors] Animus MUST define MCP descriptors for transport kind, command or endpoint labels, tool/resource/prompt identities, schema identities, result contracts, timeout, environment key names, available-tool ceilings, and provenance.

#### Scenario: MCP descriptor validation rejects unsafe claims

r[agent_harness_runtime.goose_acp_mcp_interop.mcp_descriptors.validation]
- GIVEN an MCP descriptor contains an unknown transport, duplicate tool identity, missing timeout, unsafe network claim, or environment value instead of an environment key name
- WHEN descriptor validation runs
- THEN validation returns a typed descriptor error
- AND no MCP server process or network connection is started.

### Requirement: ACP provider descriptors are explicit

r[agent_harness_runtime.goose_acp_mcp_interop.acp_descriptors] Animus MUST define ACP provider or agent descriptors for external agent identity, model/profile preference, permission-mode mapping, session correlation, known limitations, and tool pass-through policy.

#### Scenario: Unsafe ACP permission mapping is rejected

r[agent_harness_runtime.goose_acp_mcp_interop.acp_descriptors.validation]
- GIVEN an ACP descriptor maps a guarded Animus mode to an unrestricted external agent mode without explicit host policy admission
- WHEN ACP descriptor validation runs
- THEN validation rejects the mapping
- AND no external agent session is started from that descriptor.

### Requirement: Interop parsing is data-only

r[agent_harness_runtime.goose_acp_mcp_interop.data_only] MCP and ACP descriptor parsing MUST be data-only and MUST NOT launch processes, discover ambient binaries, open network connections, read environment values, contact external agents, or execute tools.

#### Scenario: Descriptor parsing remains runtime-neutral

r[agent_harness_runtime.goose_acp_mcp_interop.data_only.neutral]
- GIVEN an MCP descriptor names a stdio command label and an ACP descriptor names an external agent adapter
- WHEN reusable descriptor validation runs
- THEN validation inspects only in-memory descriptor data
- AND process launch, HTTP probing, ambient binary discovery, credential lookup, and provider contact are absent.

### Requirement: Tool pass-through is capability intersection

r[agent_harness_runtime.goose_acp_mcp_interop.tool_passthrough] Tools passed through MCP or ACP bridges MUST be limited to the intersection of Animus capability ceilings, active permission mode, host admission, external agent mode, and available-tool declarations.

#### Scenario: Bridge cannot widen tool access

r[agent_harness_runtime.goose_acp_mcp_interop.tool_passthrough.denied]
- GIVEN an ACP bridge requests access to a write-capable tool excluded by active Animus policy
- WHEN tool pass-through resolution runs
- THEN the tool is omitted or denied for that bridge
- AND the external agent cannot invoke it through a fallback tool path.

### Requirement: Interop receipts are safe and bounded

r[agent_harness_runtime.goose_acp_mcp_interop.receipts] MCP and ACP discovery, session start, pass-through, denial, timeout, unavailable-binary, malformed-schema, and network-policy outcomes MUST produce safe receipts with redacted or hashed command, endpoint, argument, environment, and session identities.

#### Scenario: Interop receipt redacts sensitive identities

r[agent_harness_runtime.goose_acp_mcp_interop.receipts.redacted]
- GIVEN an MCP or ACP operation includes command arguments, endpoint URLs, environment key names, external session IDs, or tool arguments
- WHEN an interop receipt is rendered
- THEN the receipt exposes only safe labels, bounded counts, redaction markers, and BLAKE3 identities
- AND raw secret values, full arguments, and unbounded external payloads are omitted.

### Requirement: ACP/MCP interop closeout is evidenced

r[agent_harness_runtime.goose_acp_mcp_interop.verification] ACP/MCP interop adoption MUST include descriptor validation tests, data-only runtime-neutrality tests, tool pass-through denial tests, receipt redaction tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Interop evidence is complete

- GIVEN the Goose ACP/MCP interop change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused MCP descriptor tests, ACP descriptor tests, runtime-neutrality checks, pass-through denial tests, receipt redaction tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
