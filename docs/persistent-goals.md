# Persistent goals

The optional `persistent-goals` facade provides one host-created goal per session. It stores only an objective identity, lifecycle status, provenance, token accounting, elapsed accounting, continuation count, and explicit budgets.

Statuses are `active`, `paused`, `completed`, `budget_limited`, `errored`, and `cleared`. Only an explicit admitted completion produces `completed`. Budget exhaustion is a non-success outcome and stops continuation.

Create, pause, resume, and clear actions require explicit user or host provenance. Model and inferred actions fail closed. Active goals raise continuation priority after ordinary turns, but this module does not inject prompts. Continuation policy, Ralph loops, ledger persistence, clocks, and counters remain host-owned.

Receipts contain policy and counter identities, transition, status, budget class, and BLAKE3 identity. They reject objective text and success claims for non-completed goals.

```sh
scripts/cairn/check-persistent-goals --root .
```

This contract does not prove objective achievement, work quality, automatic inference, cross-session consistency, production readiness, or release eligibility.
