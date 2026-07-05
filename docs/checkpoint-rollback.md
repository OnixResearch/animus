# Checkpoint rollback boundary

Animus treats checkpoint and rollback support as a host-owned recoverability boundary. Reusable runtime logic can validate checkpoint metadata, pre-mutation policy decisions, rollback confirmation, freshness checks, and safe receipts, but it does not run `git`, `jj`, shell commands, filesystem mutation, or workspace restore operations directly.

## VCS-neutral checkpoint identity

Checkpoint metadata is recorded as typed safe DTOs:

- backend kind: `jj`, `git`, `in_memory`, `workspace_isolation`, or `unsupported`;
- opaque checkpoint ID supplied by the backend;
- safe workspace label plus BLAKE3 workspace/content identities;
- bounded changed-file count;
- optional session attribution hash and safe tool label;
- stable status and failure-class labels.

Backend-specific command syntax, refs, patch bodies, paths, and file contents stay inside the injected host service.

## Pre-mutation policy

Protected mutating actions are evaluated with an explicit checkpoint policy:

- `disabled` permits dispatch without checkpointing;
- `best_effort` records missing or unavailable checkpoint service failures but does not block dispatch;
- `strict` fails closed before mutation unless the checkpoint service is injected and available.

This decision is a pure function over typed policy inputs and host service state, so tests can prove no provider, tool, filesystem, or VCS operation is performed during planning.

## Rollback execution preconditions

Rollback execution requires both:

1. explicit host/operator approval for the exact target; and
2. a freshness match between the expected current workspace identity and the observed current workspace identity.

If confirmation is missing/denied, or freshness is stale, validation returns a blocked rollback result and the host must not call the checkpoint restore operation.

## Safe receipts

Checkpoint receipts include only bounded safe metadata: operation, status, backend kind, checkpoint ID, workspace/content hashes, changed-file counts, confirmation status, failure class, and a BLAKE3 receipt hash.

Receipt text must not contain raw diffs, raw file bodies, credentials, environment dumps, full path lists, or backend secret locators. Use `validate_checkpoint_receipt_text` before making checkpoint evidence durable.
