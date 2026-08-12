# Proposal: Expose subagent UX hooks

## Why

Subagent orchestration needs operator-visible status, denial reasons, budgets, handoff summaries, and mutation intents. Without UI-neutral projections, TUI and headless hosts will invent incompatible child-run displays and commands, making subagent behavior harder to audit.

This change adds UI-neutral subagent UX hooks that expose safe status and control surfaces without leaking raw logs or granting authority.

## What Changes

- Add a child-run status projection for queued, running, cancelling, completed, cancelled, failed, timed out, and blocked states.
- Expose safe budget usage, denial reasons, handoff summary references, evidence references, and pending mutation intents.
- Add UI/headless command descriptors for cancel, bounded detail import request, acknowledgement, and mutation-intent arbitration request.
- Route every UX action through normal capability, confirmation, and coordinator policy checks.

## Impact

- **Files**: future UI-neutral projection DTOs, headless/TUI adapters, command descriptors, tests, docs, and event fixtures.
- **Testing**: positive status projection and command descriptor tests; negative raw-log exposure, unbounded detail import, unauthorized cancel, mutation bypass, stale handle, and hidden credential tests.
- **Non-claims**: this does not implement a full TUI dashboard or let UI actions bypass runtime policy.
