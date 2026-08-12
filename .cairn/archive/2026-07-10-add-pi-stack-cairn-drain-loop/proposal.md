## Why

Animus has a bounded Steel Ralph profile and a single-repository Cairn drain helper, but the OnixResearch workspace contains many independent repositories with native Cairn lifecycles. Running the current helper manually in each repository loses stack-wide progress visibility, retries blocked repositories unnecessarily, and does not independently check that a Pi run archived only one change.

A temporary Pi-hosted supervisor lets the stack dogfood the existing agent harness now, before a dedicated Animus Rust host and Cairn cross-repository dependency receipts are ready. The supervisor must remain opt-in, bounded, and explicit about its weaker cross-repository ordering claim.

## What Changes

- Add a Steel Scheme stack drain helper that discovers direct child repositories with `cairn/changes`, prints a plan by default, and launches fresh `pi -p` sessions only with `--run`.
- Scope each Pi round to at most one dependency-ready Cairn change in one repository.
- Rescan lifecycle state after every Pi round, distinguish archived-change progress, task-checkbox progress, no progress, and scope breaches, and continue independent repositories after local blockers.
- Require a newly created archive entry and a passing canonical Cairn validation after an observed archive.
- Document that Pi owns temporary host effects, Steel does not establish lifecycle completion, and cross-repository ordering remains advisory until receipt-backed Cairn dependencies land.

## Impact

- **Files**: `scripts/ralph-drain-stack.scm`, `docs/ralph-loop.md`, `README.md`, `AGENTS.md`, and this native Cairn change package.
- **Testing**: Steel positive and negative self-tests, plan-mode discovery against the local stack, focused Cairn gates, and repository validation.
- **Non-claims**: The helper does not provide production orchestration, trusted cross-repository dependency validation, concurrent mutation, automatic push behavior, or proof that a successful Pi process completed lifecycle work.
