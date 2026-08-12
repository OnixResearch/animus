# Proposal: Adapt Claude Code headless automation surfaces for Animus

## Why

Claude Code's CLI, CI, SDK, and pipe-friendly usage show a useful automation surface: agents should run non-interactively with stable inputs, bounded event streams, clear exit outcomes, and embeddable APIs. Animus already has runtime and RPC work underway, but it needs a vendor-neutral headless contract that can serve local CLI, CI, and SDK hosts without depending on Claude-specific providers or cloud surfaces.

This change adapts the headless automation ergonomics as typed requests, event streams, and completion receipts.

## What Changes

- Define headless run requests with objective, workspace/session identity, input messages or stdin payload references, policy profile, allowed capabilities, output contract, and validation plan.
- Define stable run events for lifecycle, prompt assembly, model/tool actions, permissions, background work, validation, receipts, and final response.
- Provide JSONL and Preserves projections for the same event DTOs without making either serialization format the core model.
- Define noninteractive completion status classes and CI-safe failure reporting.
- Add tests for valid headless runs, malformed requests, missing services, unbounded event payloads, denied side effects, serialization compatibility, and final status mapping.

## Impact

- **Files**: future headless request/event DTOs, serialization projections, std CLI/SDK shells, receipt types, tests, and docs.
- **Testing**: valid scripted runs plus negative tests for malformed inputs, unavailable adapters, unsafe event payloads, denied mutations, unsupported serialization, noninteractive prompt requirements, and receipt leak attempts.
- **Non-claims**: this change does not copy Claude Code's CLI flags, does not require Anthropic-managed cloud sessions, does not create pull requests by default, and does not make networked providers mandatory for headless operation.
