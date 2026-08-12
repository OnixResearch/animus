# Proposal: Add a host-injected subagent coordinator API

## Why

Animus validates subagent, fresh-context, and worker-team data, but embedders still need to build their own orchestration loop for enqueue, status, cancellation, collection, and failure classification. Without a common coordinator contract, hosts can diverge on fail-closed semantics and lifecycle reporting.

This change adds a reusable coordinator API that owns pure state transitions while keeping actual execution behind injected host services.

## What Changes

- Add a host-injected `SubagentService` contract for enqueue, status, cancel, and collect operations across supported child-run kinds.
- Add a pure coordinator state machine for admitted, queued, running, cancelling, completed, failed, timed-out, and collected child-run states.
- Map service absence, denial, timeout, cancellation, malformed report, and unavailable outcomes into typed diagnostics and safe lifecycle events.
- Keep provider selection, process spawning, external adapter execution, persistence, and queue backends outside reusable runtime code.

## Impact

- **Files**: future runtime coordinator module, service trait, state-transition tests, lifecycle event projection, docs, and facade integration.
- **Testing**: positive enqueue/status/cancel/collect flows with fake injected services; negative missing-service, denied, timeout, malformed output, stale handle, and accidental ambient fallback tests.
- **Non-claims**: this does not implement a concrete queue backend, run OS processes, discover external agents, or bypass lower-level child-run validation.
