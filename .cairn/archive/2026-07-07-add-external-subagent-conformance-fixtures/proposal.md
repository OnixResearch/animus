# Proposal: Add external subagent conformance fixtures

## Why

Animus supports external subagent descriptors for ACP and MCP adapters conceptually, but the safety boundary would be stronger with explicit conformance fixtures. External agent integration is risky because ambient CLI discovery, inherited credentials, undeclared tools, and unsafe output import can accidentally widen authority.

This change adds a fixture-driven conformance rail for external subagent descriptors and outcomes.

## What Changes

- Add positive fixtures for admitted ACP and MCP external subagent descriptors using injected service labels and declared tools.
- Add negative fixtures for missing service labels, undeclared tools, ambient CLI discovery, inherited credentials, malformed protocol labels, unsafe output imports, and over-broad capability claims.
- Add a conformance index or focused check that validates fixture coverage and expected failure classes.
- Document that fixtures exercise pure descriptor validation and do not require live ACP or MCP services.

## Impact

- **Files**: future fixtures, conformance index/check, descriptor validation tests, docs, and closeout evidence.
- **Testing**: fixture matrix covering valid ACP/MCP and negative ambient-discovery, credential, service, tool, protocol, output, and capability failures.
- **Non-claims**: this does not ship a concrete ACP/MCP external-agent runner or perform live network/CLI integration.
