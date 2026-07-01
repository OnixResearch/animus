# Proposal: Adapt Goose scheduled recipe workflows for Animus

## Why

Goose can schedule recipes with cron-like timing, background or foreground execution, run-now controls, pause/edit actions, and session records for each scheduled run. Animus already plans background work queues, process job profiles, continuation policy, and workflow commands, but recurring agent workflows need their own admission, isolation, and receipt contract.

This change adapts scheduled recipes as policy-admitted workflow jobs that run through injected queue and host services.

## What Changes

- Define scheduled workflow descriptors with recipe identity, parameter values, cadence, timezone or calendar policy, execution mode, concurrency policy, workspace scope, and retention policy.
- Validate schedules as data-only plans; reusable core never starts timers, daemon processes, cron jobs, or shell commands.
- Require recipe trust, capability policy, sandbox policy, queue admission, and background-work receipts before a scheduled run can execute.
- Add run records that link schedule identity, recipe version, parameter hashes, queue/job IDs, session ledger IDs, outcome, retries, and safe evidence.
- Add pause, resume, edit, run-now, and cancellation semantics as host-mediated control requests.
- Add negative tests for untrusted recipes, secret leakage in parameters, overlapping writes, missing queue service, runaway retry loops, and schedule mutation without confirmation.

## Impact

- **Files**: future scheduled workflow DTOs, pure schedule validation helpers, queue-control contracts, run record receipts, docs, tests, and optional std host adapters.
- **Testing**: positive recurring read-only workflow schedules and run-now controls; negative tests for untrusted recipes, unsafe parameters, missing queue service, concurrency conflicts, invalid cadence, and unauthorized schedule mutation.
- **Non-claims**: this change does not implement an ambient daemon, does not edit system cron, does not run recipes without host queue admission, and does not store secrets or raw prompts in schedule records.
