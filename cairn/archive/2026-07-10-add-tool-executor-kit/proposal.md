# Proposal: Add an explicit tool executor kit

## Why

Animus has strong tool DTOs, planning surfaces, catalog concepts, and permission/capability checks, but tool execution remains scattered between contracts, runtime admission, plugin manifests, and future host adapter notes. A Lego-style harness needs one explicit kit that says: this is how a host advertises tools, validates arguments, admits mutations, invokes an injected executor, bounds output, and records safe receipts.

The kit should not ship product tools by default. It should make tool execution pluggable, policy-checked, and testable.

## What Changes

- Define a focused tool executor kit with `ToolCatalog`, `ToolDefinition`, `ToolExecutorService`, execution plans, failure classes, and safe execution receipts.
- Keep executor effects host-owned and injected; reusable crates only validate metadata, arguments, capability labels, mutation targets, output policies, and receipts.
- Add deterministic fake executor fixtures for composition tests and negative fixtures for missing executors, denied capabilities, malformed arguments, unsafe outputs, mutation conflicts, cancellation, timeout, and receipt leaks.
- Register the kit in the component catalog and adapter/runtime conformance rails.

## Impact

- **Surfaces**: adapter tool contracts, runtime tool admission, host executor traits, component catalog, composition conformance, dogfood readiness docs.
- **Testing**: positive fake executor path, negative fail-closed executor cases, receipt leak rejection, catalog/conformance checks, Cairn gates.
- **Non-claims**: this does not add default product tools, shell command execution, network tools, MCP transport, plugin runtime, filesystem mutation, or production tool packs.
