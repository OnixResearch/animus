# Interrupted tool-call repair

Animus adapts the bounded recovery invariant from [Tau](https://github.com/huggingface/tau): before another provider request, every model-visible assistant tool call on the selected active branch has one compatible terminal result. Animus does not copy Tau's Python session object, infer interruption cause, or rerun interrupted tools.

## Ownership

`animus_runtime::interrupted_tool_repair` is a functional core over already-loaded values. It owns:

- exact call/result diagnosis for one selected active branch;
- evidence-bound interruption-reason reduction;
- ordered append-only tail or historical-branch repair plans;
- BLAKE3 snapshot, action, proposed-leaf, plan, and receipt identities;
- pure validation of host-supplied application evidence;
- provider-continuation gating and canonical synthetic-result projection; and
- metadata-only receipt projection.

The host owns session and branch discovery, durable storage, locking, current-leaf observation, cancellation and shutdown observations, append/replay application, provider dispatch, and all tool executors. The only repair effect DTO is `AppendSyntheticFailures`; no repair type can construct a tool request, approval, mutation permit, retry, fallback, or executor call.

## Binding invariant

Calls are matched by exact run identity, call ID, and BLAKE3 tool-definition identity. The call must precede its result. Duplicate call IDs, duplicate results, result-before-call order, run or tool mismatch, unsupported entry kinds, duplicate entry identities, malformed bounds, and stale active leaves fail closed rather than becoming repair plans.

A clean diagnosis changes nothing. A repairable diagnosis records unmatched calls in transcript order. Matched results are retained and never replaced.

## Cause classification

Synthetic terminal failures use one of:

- `user_cancelled` only for an explicit observation bound to the same run and call;
- `host_cancelled` under the same exact correlation rule;
- `unclean_shutdown` for an explicit correlated host observation; or
- `unknown_interruption` when no correlated cause exists.

A missing result alone never implies user cancellation. Conflicting observations fail closed.

## Append-only application and idempotence

A tail plan appends synthetic results after the active tool-call batch. If later non-result context exists, a historical plan requests a new repaired active branch that preserves the original branch and binds the ordered suffix identities. The application request binds the exact session, branch, old leaf, new leaf, anchor, actions, and suffix. Neither mode edits or deletes committed history.

Hosts return typed durability, replay-validation, active-leaf, committed-action, prior-plan, and executor-non-invocation observations. Provider continuation is admitted only for an originally clean diagnosis or a complete durable application whose replay is clean and whose plan/new-leaf identities match. Partial writes, store failure, stale heads, missing replay validation, and any tool-executor invocation remain blockers. Re-observing an already-applied plan and current repaired leaf yields `already_applied` without another append.

## Receipt safety and non-claims

Repair receipts contain only schema, mode/status/reason classes, old/new leaf hashes, plan hash, bounded action count, redaction marker, non-claim, and receipt BLAKE3 identity. Raw prompts, tool arguments/results, provider payloads, credentials, private paths, exceptions, and transcripts are rejected.

A passing repair receipt establishes only that supplied metadata describes a complete append-only repair and clean replay for the bound branch. It does not prove the interruption cause, that an external side effect did not occur, exactly-once behavior, provider acceptance, store or kernel correctness, arbitrary provider transcript compatibility, or production recovery.

## Verification

```text
cargo test -p animus-runtime --locked session_recovery
scripts/cairn/check-interrupted-tool-repair --root .
scripts/cairn/check-safe-receipts-artifacts --root .
scripts/cairn/check-neutral-ledger --root .
scripts/cairn/check-composition-conformance --root .
```
