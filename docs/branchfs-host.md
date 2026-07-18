# BranchFS speculative host

`animus-std/branchfs-host` is a non-default Linux host adapter for disposable, root-level sibling candidate workspaces. It is bound to BranchFS revision `a4b6592d31aacb4d2f94d0c80f43d47d38063fa9` and cohort `branchfs-a4b6592-jj-abort-only-v1`.

## Prerequisites

The operator must provide the reviewed `branchfs` executable, a usable FUSE device and helper, and Jujutsu. Preflight is behavioral: a bounded temporary mount must create, isolate, read, write, abort, and unmount successfully, and `jj` must identify the source and create a fresh destination workspace. Binary presence alone does not pass.

## Ownership and lifecycle

The pure planner admits one mount and root-level sibling candidates only. The command model has no BranchFS commit operation; nested candidates and commit requests fail before mounting. Candidate execution and selection are injected. Losers are aborted.

A winner is durable only after the adapter rechecks the source identity, creates a fresh Jujutsu workspace, copies the selected view through capability-scoped operations, and proves bounded manifest equality. `.branchfs_ctl`, virtual `@branch` entries, `.jj`, `.git`, escaping links, and special files are excluded. The operator's current workspace is never the promotion destination. The selected BranchFS candidate is aborted only after durable handoff evidence exists.

Failed materialization retains the winner as `cleanup-required` rather than silently deleting recovery state. Retention is bounded; failed abort or unmount requires explicit operator cleanup.

## Evidence boundary

Receipts contain safe labels and domain-separated BLAKE3 identities for source, plan, cohort, candidate, manifest, cleanup, selection, and Jujutsu handoff. They exclude host paths, raw manifests, file bodies, prompts, credentials, command output, daemon logs, and unbounded events.

This evidence does not establish authorization, hostile-code containment, BranchFS or kernel correctness, merge correctness, repository completion, durable backup, or release eligibility.
