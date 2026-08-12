## Why

Animus already maps trusted MCP descriptors into Animus tools. It does not yet provide a concrete client host for configuration, server lifecycle, discovery, calls, cancellation, and teardown.

This gap keeps plugin and MCP runtime behavior at the contract rung. A concrete non-default host is needed before any operational MCP claim.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` provides ISC-licensed failure scenarios for configuration layering, credential refresh, bounded discovery, cancellation, and safe receipts.

## What Changes

- Add a separate non-default `animus-std-mcp` host with a pure configuration and lifecycle core plus a thin effect shell. r[agent_harness_runtime.mcp_client_host.boundary]
- Add typed Nickel configuration for exact servers, required or optional posture, explicit standard-input transport, limits, credentials, and policy references. r[agent_harness_runtime.mcp_client_host.configuration]
- Add bounded initialization, tool discovery, calls, cancellation, and owned teardown for one reviewed MCP cohort. r[agent_harness_runtime.mcp_client_host.lifecycle]
- Map discovered tools through the existing MCP bridge and tool executor admission path. r[agent_harness_runtime.mcp_client_host.discovery]
- Require current approval and capability intersection before every MCP effect. r[agent_harness_runtime.mcp_client_host.authority]
- Admit server instructions only as untrusted request-local context contributions. r[agent_harness_runtime.mcp_client_host.context]
- Add metadata-only receipts and deterministic fake-server conformance. r[agent_harness_runtime.mcp_client_host.evidence] r[agent_harness_runtime.mcp_client_host.verification]

## Success Criteria

- The host starts only explicitly configured standard-input servers.
- Required-server failure blocks readiness. Optional-server failure remains visible without silent substitution.
- Discovery and schemas stay within named finite limits.
- Every tool call passes existing Animus argument, capability, approval, mutation, and output gates.
- Cancellation and shutdown reap the owned server process tree.
- No credential, raw transport body, private path, or server instruction enters a receipt.

## False Completion

The change is incomplete if it only adds more MCP DTOs or uses an unbounded mock transport.

A successful server call does not prove server trust, tool safety, sandboxing, network isolation, or production readiness.

## Impact

- **New crate**: non-default `animus-std-mcp`.
- **Configuration**: typed Nickel source with deterministic generated projection.
- **Existing owners**: MCP mapping, tool executor, SecretSpec, authority seals, request context, and receipts remain authoritative.
- **Testing**: fake standard-input servers, malformed frames, limits, cancellation, credential drift, denial, and leak cases.

## Out of Scope

- Automatic server discovery, automatic trust, Streamable HTTP, remote catalogs, OAuth, or ambient credentials.
- Default plugin runtime, server sandbox claims, provider-specific prompt behavior, or MCP parity claims.

## Affected Specs

- `agent-harness-runtime`: MCP client host boundary, configuration, lifecycle, discovery, authority, context, evidence, and verification.
