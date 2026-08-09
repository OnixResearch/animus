# Drain evidence guard

A Cairn drain uses disposable linked Git worktrees. Ignored `.pi/` data can remain invisible to Git status and can be deleted with a completed worktree.

Run the guard before each worktree removal:

```console
CARGO_TARGET_DIR=target/cargo-script \
  scripts/check-drain-evidence-guard.rs --worktree /absolute/path/to/worktree
```

Omit `--worktree` to scan all registered worktrees. Add `--show-empty` to include safe and persistent targets in the report. Run the credential-free positive and negative tests with:

```console
CARGO_TARGET_DIR=target/cargo-script \
  scripts/check-drain-evidence-guard.rs --self-test
```

## Decision rules

The guard classifies a `.git` directory as the persistent primary worktree. It does not treat that location as a teardown target. A `.git` file identifies a linked worktree.

A linked worktree passes when `.pi/` is absent, empty, or a symlink that resolves outside the worktree. It blocks when `.pi/` contains entries, resolves inside the worktree, has an unsupported shape, or cannot be inspected. Missing paths and unsupported Git markers also block.

If the guard blocks, move owner-only evidence to the primary repository `.pi/` directory or another secure persistent location. Run the guard again. Remove the worktree only after it passes.

## Evidence boundary

The guard reads filesystem metadata only. It does not read evidence contents. Output contains a sanitized worktree label, bounded entry count, truncation state, classification, and verdict. It omits evidence filenames, WIDs, receipt bodies, credentials, private evidence paths, and content-derived identities.

An external `.pi` symlink proves only that removing the linked worktree does not remove the resolved target. It does not prove target durability, confidentiality, ownership, freshness, correctness, or release eligibility.
