# Proposal: Add Turnstone-inspired headless eval scorer

## Why

Turnstone's eval harness provides a useful measurement substrate: isolated test cases, expected tool-call sequences, match modes, multiple runs, holdouts, and structured failure summaries. Animus has batch trajectory and eval-feedback contracts, but it needs a focused headless scorer that can measure model/tool-loop behavior without online prompt self-modification or product runtime dependencies.

The scorer should make evaluation evidence reproducible and safe: raw prompts, provider payloads, full paths, and raw tool output stay outside durable receipts unless a host explicitly stores them in a separate, reviewed artifact.

## What Changes

- Add typed headless eval suite, case, setup summary, expected action, run observation, score, and aggregate DTOs.
- Implement pure scoring for exact, ordered-subset, subset, and contains-any match modes over observed tool-call ledgers.
- Add multi-run aggregate reducers for pass rate, average score, flaky cases, unmatched actions, extra tools, holdout separation, and JSON-dump-like malformed tool-call signals.
- Define a host-shell boundary for fixture setup, provider/tool execution, temporary workspaces, timeouts, and persistence.
- Produce safe eval receipts with suite identity, case labels, counts, score summaries, policy hashes, and BLAKE3 evidence hashes.

## Impact

- **Files**: runtime/adapters eval DTOs, pure scorer core, safe receipt helpers, fixtures, docs, and focused tests.
- **Testing**: positive exact/ordered/subset/contains-any scoring and aggregate cases plus negative malformed action specs, duplicate matches, empty expected-action policies, unsafe receipt fields, holdout leakage, and invalid score inputs.
- **Non-claims**: this does not add Turnstone's optimizer, online prompt self-modification, live provider runner, filesystem fixture writer, product CLI, or storage of raw model/tool transcripts.