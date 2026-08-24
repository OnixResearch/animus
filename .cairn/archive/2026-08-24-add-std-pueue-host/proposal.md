## Why

Animus defines a backend-neutral `BackgroundQueueService`, but it has no concrete pueue host. The pi-steel-repl migration needs a published replacement for its Pi-owned TypeScript adapter.

A non-default std component can own pueue process effects without granting process authority to reusable Animus cores.

## What Changes

- Add a separate `animus-std-pueue` crate for one explicit local pueue namespace.
- Implement `BackgroundQueueService` for enqueue, status, bounded logs, groups, explicit controls, poll, wait, and supported interactions.
- Resolve enqueue work through reviewed named process-job profiles instead of raw shell text.
- Add a tool-executor adapter for the existing Animus tool and staged-execution contracts.
- Add typed Nickel configuration, component-catalog entries, focused rails, and metadata-only receipts.
- Add positive and negative fake-process and isolated real-daemon tests.

## Impact

- **Files**: new host crate, workspace manifest, Nickel configuration, generated projections, component catalog, Nix checks, docs, fixtures, and release evidence.
- **Testing**: crate tests, real-daemon isolation tests, Clippy, formatting, Cairn validation and gates, Octet, and focused Nix checks.

## Consumer

Initial consumer: `git@github.com:OnixResearch/pi-steel-repl.git`.

The consumer will pin the published Animus revision. It will not use an ambient sibling worktree or copy source.

## Non-goals

- Do not add pueue authority to `animus-core`, `animus-runtime`, or `animus-adapters`.
- Do not accept raw shell text as the durable enqueue contract.
- Do not discover ambient pueue daemons or configuration.
- Do not claim sandboxing, command correctness, or scheduler reliability.
