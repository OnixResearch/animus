# Durable local session store

Animus provides a non-default `animus-std/durable-session-store` host block for capability-scoped local persistence. The reusable `animus_runtime::durable_session_store` module owns pure policy validation, transaction planning, bounded binary framing, recovery classification, and metadata-only receipt projection. `animus-std` alone owns file handles, restrictive permissions, exclusive lease files, bounded reads/appends, `sync_all`, and reopen orchestration.

## Admission and policy

The host receives an already opened `cap_std::fs::Dir`; relative journal and lock names; a validated store descriptor; and an explicit lease ID. There is no home-directory, current-directory, environment, or global-session fallback. The Nickel source of truth is `config/durable-session-store-policy.ncl`; checked JSON is loaded and revalidated without runtime Nickel evaluation.

The durable profile is `sync_all_each_commit`. A commit is reported successful only after the complete frame group and commit record have been appended and the injected synchronizer succeeds. The descriptor also binds session, branch, policy hash, exclusive-lock posture, replay-private posture, encryption posture, and metadata-only receipt policy.

## Private journal and safe evidence

Private frame payloads may contain bounded model-visible user, assistant, tool-result, summary, repair, or branch-navigation bodies, plus metadata-only session authority-profile bindings. Every frame carries an explicit entry ID and parent ID, sequence, branch, payload byte count and BLAKE3 hash, previous-frame hash, and domain-separated frame identity. A commit binds the ordered frame hashes and exact active leaf; the active leaf must be the final committed entry.

Safe sidecars and receipts contain entry/order metadata, kinds, counts, redaction classes, policy and head identities, operation and durability classes, lock/recovery status, and BLAKE3 identities. Receipt projection revalidates the descriptor against generated policy and rejects raw messages, tool/provider bodies, credentials, capability or store paths, frame payloads, lock dumps, and unbounded errors.

## Locking and recovery

Opening uses exclusive `create_new` lease admission. A different lease is a conflict and reuse of the same lease is stale; neither path appends. Failed lock creation setup removes the unowned lease before returning. Explicit `close` releases the lease.

Recovery accepts only a contiguous sequence and BLAKE3 chain whose parent IDs continue from the prior committed active leaf. The store preserves authority-profile records as typed private frames but does not interpret, infer, authorize, or enforce them; [`coding_session_host`](coding-session-host.md) and the [authority-profile core](session-runtime-authority-profile.md) own those checks. It ignores only an incomplete final frame or uncommitted final transaction. It fails closed on committed corruption, parent or hash mismatch, sequence gaps, stale heads, duplicate entry IDs, unsupported schema, and mid-file corruption. Recovery never truncates, rewrites, compacts past, or silently repairs committed history.

## Verification

Run:

```sh
scripts/cairn/check-durable-session-store --root .
```

The focused rail checks Nickel drift and negative fixtures, default-feature isolation, pure-core authority isolation, fake/file conformance, real child-process restart, torn tails, corruption, lock conflict and setup cleanup, permission and synchronization failure, required-encryption rejection, bounds, and receipt leak rejection. Its safe receipt is written to `target/cairn/focused-rails/durable-session-store.md`.

## Non-claims

BLAKE3 chaining detects bounded corruption and ordering drift; it is not authentication. This host does not provide encryption or key discovery, power-loss proof, filesystem or kernel correctness, backup, replication, retention, daemon recovery, remote storage, multi-host coordination, multi-writer merge, production readiness, or release eligibility. `sync_all_each_commit` records the selected host call succeeding; it cannot prove hardware persistence beyond that boundary.
