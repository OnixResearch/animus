# Proposal: Adapt Goose ACP/MCP interoperability for Animus

## Why

Goose leans into open agent standards: MCP extensions expose tools, resources, and prompts, while ACP providers let Goose coordinate with other coding agents and subscription-backed CLIs. Animus already has adapter-first boundaries, plugin/tool manifest work, and planned headless surfaces, but it needs a standards-oriented interop contract that can admit MCP/ACP behavior without giving reusable core ambient process, network, or secret authority.

This change adapts Goose's standards posture into Animus-owned descriptors, adapter contracts, and safety receipts.

## What Changes

- Define MCP capability descriptors for tools, resources, prompts, schema identity, result contracts, transport kind, timeout, environment key names, and available-tool ceilings.
- Define ACP agent/provider descriptors for external agent identity, session mode, model/profile preference, permission mode mapping, tool pass-through policy, and session correlation.
- Keep descriptor parsing data-only; concrete stdio, HTTP, process, network, and credential handling remains in host adapters.
- Add extension secret requirements as key names and admission prompts without storing or exporting secret values.
- Add interop receipts for MCP/ACP discovery, session start, tool pass-through, denial, timeout, malformed schema, and unavailable binary outcomes.
- Add negative tests for env value leakage, unknown transports, tool collisions, unsafe network claims, missing executors, and permission-mode downgrades.

## Impact

- **Files**: future MCP/ACP descriptor DTOs, validation helpers, provider/session mapping contracts, host adapter traits, safe receipts, docs, tests, and optional importers.
- **Testing**: positive descriptor and session-mapping tests; negative tests for launch bypass, credential leakage, duplicate tools, unavailable binaries, network policy violations, and unsafe provider permission mappings.
- **Non-claims**: this change does not run arbitrary MCP servers in reusable core, does not make ACP agents a provider fallback without explicit configuration, does not grant network access by extension declaration, and does not implement experimental MCP Apps UI behavior.
