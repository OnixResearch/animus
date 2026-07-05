# Proposal: Adapt Clankers controller continuation policy for Animus

## Why

Clankers has a useful controller continuation brick: after a prompt completes, pure policy decides whether to replay a queued prompt, continue a loop, run an auto-test follow-up, or finish. Animus will need similar behavior as runtime loops, workflows, validation, and headless/session surfaces mature. Without a single continuation policy, queued user input, tool-driven loops, validation follow-ups, and session state can race or clear each other incorrectly.

## What Changes

- Define continuation state DTOs for queued prompts, active loop work, validation/auto-test follow-ups, cancellation, busy state, and completion acknowledgements.
- Implement pure decision priority over continuation candidates with deterministic outcomes and typed rejected/stale/duplicate failures.
- Require shell code to acknowledge dispatch and completion using exact pending work IDs before state is cleared.
- Keep provider execution, timers, queueing, UI events, and session persistence in host/runtime shells.
- Add positive and negative fixtures for queued-prompt priority, loop continuation, validation follow-up, no-work completion, stale IDs, duplicate completions, cancellation, and receipt redaction.

## Impact

- **Files**: future continuation DTOs, reducer/core policy, shell adapter tests, docs, receipts, and Cairn artifacts.
- **Testing**: positive continuation-priority fixtures and negative stale/duplicate/mismatched/cancelled/leak fixtures.
- **Non-claims**: this does not implement a product loop tool, auto-test runner, daemon controller, or TUI behavior by itself.
