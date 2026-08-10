# Goose session branching controls

The [session branch explorer](session-branch-explorer.md) uses these controls only for append-only fork plans. It rejects edit-in-place, suffix deletion, merge, and reparent operations.

Animus adapts Goose-style edit, fork, duplicate, and label controls as pure session-branch plans. Reusable runtime code validates already-loaded ledger summaries and policy inputs; hosts keep UI widgets, storage mutation, checkpoints, rollback execution, provider replay, live processes, credentials, and transcript bodies.

## Reusable contract

`animus-runtime::plan_session_branch` accepts:

- a validated `SessionBranchRequest` naming source and target session hashes, source head, cut point, operation, continuation target, safe branch label, provenance label, and requested copy descriptors;
- a validated `SessionBranchPolicy` naming policy hash, admission, suffix-discard posture, checkpoint requirement, and strict unsafe-copy behavior;
- an in-memory slice of `SessionLedgerEntry` summaries;
- host-supplied checkpoint or rollback evidence status.

The planner returns a `SessionBranchPlan` and `SessionBranchReceipt` without reading session stores, calling providers, opening files, using clocks, spawning processes, or mutating host state.

## Fail-closed behavior

Planning denies host mutation when policy denies the request, ambient storage authority is requested, the source head is missing, the cut point is stale, edit-in-place would discard a suffix without policy admission, required checkpoint evidence is missing, target identity duplicates the source identity, strict copy policy sees unsafe descriptor inheritance, or the ledger summary is malformed.

Forks target the cut point and preserve the source branch. Edit-in-place plans model the suffix after the cut point as explicit discard metadata. Duplicate plans copy only safe descriptors such as session labels, recipe references, extension labels, model profile labels, policy labels, and bounded ledger references; credentials, live processes, provider sessions, raw transcripts, raw tool outputs, and host database rows are omitted or rejected by strict policy.

## Safe evidence

Receipts record operation kind, source and target session hashes, source head hash, cut-point hash, suffix count, branch-label status, copied and omitted descriptor classes, policy hash, checkpoint status, continuation target, diagnostics, and BLAKE3 receipt hash.

`validate_session_branch_receipt_text` rejects raw prompt markers, raw transcript markers, provider payloads, credentials, full paths, raw tool output, and host database row material before durable evidence is accepted.

## Verification

Focused coverage lives in `crates/animus-session-core/src/branching.rs` and covers positive fork, duplicate, label update, deterministic receipt hashing, and negative stale cut point, suffix-discard denial, missing checkpoint, unsafe edit body, duplicate target identity, unsafe copy inheritance, ambient authority, and receipt leak fixtures.
