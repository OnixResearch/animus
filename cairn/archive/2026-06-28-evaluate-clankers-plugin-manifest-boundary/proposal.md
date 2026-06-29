# Proposal: Evaluate Clankers plugin manifest boundaries for Animus

## Why

Clankers supports Extism, stdio, and other plugin manifests with launch policies, permissions, environment filtering, sandbox declarations, and live tool registration. Animus should not adopt those runtimes now, but the manifest boundary is useful as a future descriptor pattern. This change evaluates which manifest ideas fit Animus' adapter-first design without adding plugin execution.

## What Changes

- Document Clankers plugin manifest patterns as bounded references for future Animus extension descriptors.
- Define an optional manifest vocabulary for plugin identity, runtime kind, declared tools, capabilities, launch policy metadata, sandbox requirements, environment allowlist names, writable-root descriptors, network policy, and redaction.
- Require manifest parsing to be data-only and not launch processes, load Wasm, inherit environment values, or read secrets.
- Add negative fixtures for path escapes, absolute writable roots, env leakage, missing command, unknown runtime kind, tool collisions, and unsafe network/sandbox combinations.

## Impact

- **Files**: docs, optional manifest schema/fixtures, validation rail, Cairn artifacts.
- **Testing**: data-only manifest validation with positive and negative fixtures.
- **Non-claims**: this does not add plugin execution, stdio supervision, Wasm loading, Zellij, MCP, or sandbox enforcement.