# Daemon client, supervisor, and worker contracts

Animus provides an optional `daemon-workers` contract family. It adapts the reviewed Prime Agent daemon cohort at revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`.

## Role ownership

- The **client** owns rendering, input, and local UI preferences. It does not execute prompts, tools, queues, schedules, or kernels.
- The **supervisor** owns discovery, routing, client attachments, worker health classification, and catalog scans. It does not execute session work.
- The **worker** owns exactly one root session tree. This includes its queue, schedules, kernel handles, and descendants.

All cross-role commands use versioned typed envelopes. The reusable contract does not spawn processes, open sockets, read files, or persist transcripts.

## Detach and reattach

Attach creates a bounded client subscription. Detach ends that subscription only. The worker execution decision does not contain attachment state, so schedules, goals, and autonomous work continue after a client disconnect.

Reattach selects one of two sources:

- a live stream with a valid sequence;
- a current host-owned recovery snapshot.

A stale or unknown snapshot is rejected before use. Snapshot repair remains owned by the session-restore contract and its host.

## Containment boundary

Worker and kernel process separation supplies lifecycle facts: independent crash domains, explicit teardown, and restart classification. It is **not a security sandbox**. The daemon contract rejects sandbox, confinement, security-isolation, crash-freedom, and arbitrary durability claims.

Use a separate sandbox contract when a host needs security isolation. Do not infer security properties from process separation.

## Receipts

Daemon receipts contain policy, role, envelope, attachment, recovery, health, bounded-count, and BLAKE3 identity fields. They also contain the `not-security-sandbox` non-claim. Prompt bodies, client input, transcript content, credentials, paths, and overclaims fail closed.

The focused rail is:

```text
scripts/cairn/check-daemon-workers --root .
```

The rail checks Nickel parity, cohort provenance, positive and negative Rust tests, no-std source shape, and the feature-gated compatibility facade. It does not prove process correctness, transcript durability, security sandboxing, production readiness, or release eligibility.
