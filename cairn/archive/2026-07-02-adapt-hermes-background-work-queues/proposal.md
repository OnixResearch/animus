# Proposal: Adapt Hermes background work queue semantics for Animus

## Why

Hermes/Pi-era pueue workflows provide a mature pattern for long-running work: queue tasks, poll status, stream bounded logs, separate groups, manage parallelism, and require explicit selectors for destructive actions. Animus should adapt the semantics as a host-managed background work contract so runtime tools can handle builds, tests, downloads, and experiments without blocking the conversation or falling back to ad-hoc shell execution.

## What Changes

- Define background work DTOs for queued command/task identity, group, status, progress, log viewport, exit outcome, cancellation, restart, and stdin/send operations.
- Require explicit target selectors for pause, kill, restart, clean, or group-wide operations.
- Add bounded log/result metadata with truncation status and safe output hashes.
- Keep process supervision and pueue integration in host/std adapters; reusable runtime consumes injected queue services.
- Add positive and negative fixtures for quick waits, long-running tasks, paused/stashed/queued states, group limits, destructive selector rejection, and bounded logs.

## Impact

- **Files**: future background queue DTOs, queue adapter traits, log viewport core, destructive action validation, tool contracts, tests, and docs.
- **Testing**: valid enqueue/status/log flows plus missing target selector, stale task ID, unsupported stdin, unbounded log, denied cancellation, and absent queue service failures.
- **Non-claims**: this change does not require pueue as the only queue backend and does not permit runtime crates to spawn arbitrary shell commands directly.
