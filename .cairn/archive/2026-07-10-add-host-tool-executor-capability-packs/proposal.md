# Proposal: Add host tool executor capability packs

## Why

Animus has strong tool DTOs, argument validation, dispatch gates, output bounds, and manifest-derived metadata, but the dogfood roadmap still lists tool execution as an actionable blocker. Tool contracts need a host-owned executor pack path that proves known tools can be admitted, executed through injected services, bounded, and receipted without shipping product tools as reusable defaults.

Capability packs should make executor availability and authority explicit, with deterministic fake executors first and packaged host executors later.

## What Changes

- Define host-owned tool executor pack descriptors that bind tool definitions, exact capability labels, schema identities, executor service labels, output policies, and mutation policies.
- Add deterministic fake executor pack fixtures for positive and negative dispatch evidence.
- Require executor injection and capability-pack admission before prompt exposure or tool execution.
- Produce safe executor receipts for completed, denied, unavailable, timed-out, cancelled, malformed-output, mutation-conflict, and leak-rejected outcomes.
- Document that reusable crates expose contracts only; packaged tools remain non-default host assets.

## Impact

- **Files**: tool executor pack DTOs, admission logic, fake executor fixtures, std-shell examples, docs, tests, and Cairn evidence.
- **Testing**: positive read-only and bounded-output executor fixtures plus negative missing executor, capability gap, denied approval, malformed args, unsafe output reference, unbounded output, mutation conflict, timeout, cancellation, and receipt leak cases.
- **Non-claims**: this does not add default product tools, shell/process/network/filesystem authority, plugin execution, global tool discovery, or production tool catalogs.
