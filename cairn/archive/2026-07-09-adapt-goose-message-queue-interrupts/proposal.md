# Proposal: Adapt Goose message queue and interrupt controls

## Why

Goose lets users queue follow-up messages while an agent is busy and interrupt active work with stop, wait, or redirection messages. Animus has continuation policy, headless queueing, cancellation, and host-control event contracts, but it lacks a UI-neutral contract for ordered queued user input, queue edits, and interrupt semantics.

Animus should adapt the control model as pure queue and interrupt reducers that hosts can expose in TUI, headless, SDK, or Desktop surfaces without embedding UI behavior in reusable crates.

## What Changes

- Define queued-message, queue-operation, queue-state, interrupt-signal, redirection, cancellation, and control-event DTOs.
- Add deterministic queue reducers for append, edit, reorder, delete, clear, dispatch-next, and acknowledgement operations.
- Add deterministic interrupt classification and host-mediated cancellation or pause requests with policy-controlled keyword rules and explicit user-control events.
- Require safe receipts for queue operations, interrupts, cancellations, redirections, denied operations, stale active-turn references, and budget limits.

## Impact

- **Files**: future continuation/headless DTOs, pure reducers, interrupt classification helpers, docs, tests, and Cairn artifacts.
- **Testing**: positive queue append/reorder/dispatch tests and negative stale-work, unauthorized queue mutation, unsafe queued body, missing cancellation handle, max-queue, and receipt leak fixtures.
- **Non-claims**: this does not implement Desktop cards, keyboard handling, terminal control, live cancellation transport, provider abort APIs, or storage-backed queue persistence.
