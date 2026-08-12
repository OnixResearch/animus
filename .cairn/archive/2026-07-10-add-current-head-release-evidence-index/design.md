# Design: Current-head release evidence index

## Context

`check-release-evidence-index` validates line-oriented evidence index fixtures, but it does not render an index for the current repository head. The release evidence docs require VCS collection to stay shell-owned, and the index must remain a summary over already-run checks rather than a hidden test runner.

## Decisions

### 1. Render from explicit check statuses

**Choice:** Add a Rust-backed generator that accepts repeated `--check MODE=STATUS`, `--stale-check MODE=STATUS`, and `--missing-mode MODE=REASON` arguments.

**Rationale:** The generator records evidence supplied by the caller. It does not claim a check ran just because the generator executed.

### 2. Collect actual Git head only in the shell

**Choice:** By default the generator uses `git rev-parse`, `git symbolic-ref`, and `git status --porcelain` to fill VCS kind, commit, branch, and dirty status. Tests may provide `--commit`, `--branch`, and `--dirty` for synthetic fixtures.

**Rationale:** The release evidence core remains deterministic over supplied lines, while the imperative shell owns repository state discovery. The repo is git-only, so `git` is the correct local VCS command.

### 3. Make current-head proof explicit

**Choice:** The generator emits `claim|current-head-pure-evidence` only when the caller passes `--claim-current-head`.

**Rationale:** Draft indexes can record partial evidence and missing modes without overclaiming. The checker rejects current-head claims for dirty heads, stale receipts, or missing required pure modes.

### 4. Include packaged dogfood smoke as a selected pure mode

**Choice:** Add `packaged-headless-dogfood-smoke` as a recognized pure check mode in the schema/checker, but do not make it a required pure mode for every current-head claim.

**Rationale:** The packaged smoke rung is useful current-head evidence after the prior change, while the existing required pure modes remain stable release-readiness gates.

### 5. Keep generated artifacts local

**Choice:** The current-head rail writes actual-head draft output under `target/cairn/release-evidence/current-head-draft.lines` when a Git worktree is available.

**Rationale:** Draft receipts can be inspected locally without committing host-specific head metadata or dirty-state evidence.

## Risks / Trade-offs

- A passing generated index depends on honest supplied check statuses; it is not a substitute for running those checks.
- Nix checks use synthetic head metadata because flake source copies do not include `.git`.
- Actual-head draft generation may record dirty state during development; dirty drafts are allowed, but they are not current-head proof unless all required conditions pass.
