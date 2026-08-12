# Proposal: Adapt Claude Code agent-team orchestration for Animus

## Why

Claude Code's subagent and background-agent workflows show a useful orchestration pattern: a lead agent can delegate bounded work to workers, collect reports, and continue while long-running tasks proceed elsewhere. Animus already has background queue and Kanban-oriented changes, but it needs a runtime-level contract for scoped agent workers so delegation does not inherit unlimited context, memory, tools, or mutation authority.

This change adapts the agent-team concept as typed worker tasks, scoped context packets, and evidence-bearing handoff reports.

## What Changes

- Define scoped worker task contracts with objective, allowed resources, context packet, budgets, output schema, authority grants, and parent trace metadata.
- Require worker contexts to be minimized and redacted rather than inheriting the full parent session by default.
- Add typed worker status, cancellation, and completion report DTOs through a host-injected worker service.
- Define coordinator merge/arbitration behavior for worker reports, conflicts, and mutation requests.
- Add positive and negative tests for valid delegation, overbroad scope, missing worker service, leaked parent context, malformed reports, conflicting results, and unauthorized mutation requests.

## Impact

- **Files**: future worker task DTOs, context packet builders, coordinator merge core, worker service adapter contracts, reports, receipts, tests, and docs.
- **Testing**: valid worker delegation and report merge plus negative tests for authority leakage, missing services, malformed outputs, context overexposure, mutation conflicts, and cancellation races.
- **Non-claims**: this change does not require Claude Code cloud sessions, does not spawn OS processes from reusable core crates, does not share full parent memory by default, and does not make workers a bypass around permission or tool policy.
