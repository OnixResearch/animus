# Zerostack fresh-context tasks

Animus adapts Zerostack-style fresh-context task and side-question behavior as typed, host-scheduled child-run contracts. Runtime core validates descriptors, isolation, side-question actions, attributed results, and bounded receipts. Hosts own scheduling, provider selection, concurrency, cancellation plumbing, and process execution.

The reusable surface lives in `animus-runtime::fresh_context_tasks` and is re-exported by `animus-runtime`.

## Descriptors and isolation

`FreshTaskDescriptor` declares goal, admitted context references, allowed action profile, model policy, token/tool budgets, timeout label, result schema, and mode. `evaluate_fresh_task_dispatch` fails closed when no scheduler is ready or when parent allowlists, hidden prompts, parent-only secrets, or mutable process-global state are inherited.

## Read-only side questions

`evaluate_side_question_action` permits read/search actions for side questions and denies writes, edits, shell, network mutation, ledger mutation, and other non-read tools with a typed read-only diagnostic.

## Results and receipts

`FreshTaskResult` carries child-run identity, descriptor identity, status, budget usage, citations, uncertainty markers, redacted evidence summaries, and a BLAKE3 output hash. Validation withholds malformed or unsafe results before parent prompt influence. `FreshTaskReceipt` records cancellation reason, scheduler identity, budget usage, output hash, and transcript embedding state without requiring full child transcript storage.

## Focused verification

Run:

```sh
nix develop -c cargo test -p animus-runtime fresh_context_tasks
```

The tests cover missing scheduler fail-closed behavior, isolation violations, read-only side-question enforcement, malformed/unsafe results, and bounded cancellation receipts.
