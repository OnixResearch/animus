# Proposal: Adapt Pi compaction and branch summaries for Animus

## Why

Pi handles long sessions by compacting older context and by summarizing abandoned branches when navigating a session tree. The important ideas are not Pi's exact prompts, but the durable mechanics: configurable context budgets, safe cut points that do not orphan tool results, split-turn handling, structured summaries, cumulative file-operation metadata, and extension/custom summarizer hooks.

Animus needs the same bounded-context behavior once it has a runtime loop and neutral ledger. Adapting this pattern keeps context management explicit and testable instead of hiding transcript rewrites inside a shell.

## What Changes

- Define compaction policy inputs for context-window budget, response reserve budget, kept-recent budget, summary format, and summarizer selection.
- Add deterministic compaction preparation over ledger branches, including valid cut-point selection and split-turn handling.
- Define summary entries for compaction and branch handoff with bounded text, safe metadata, content hashes, and cumulative read/modified resource metadata.
- Add host-injected summarizer services so runtime can request summaries without constructing provider clients directly.
- Add positive and negative tests for budget triggers, cut points, split turns, branch summaries, failed summarizers, and missing services.

## Impact

- **Files**: runtime compaction preparation, summary DTOs, policy/config types, summarizer adapter contracts, tests, and docs.
- **Testing**: positive compaction and branch-summary fixtures; negative tests for invalid cut points, orphaned tool results, unavailable summarizer service, malformed summaries, and unsafe metadata.
- **Non-claims**: this change does not copy Pi's summarization prompt verbatim, add automatic provider calls inside core, implement a TUI `/tree`, or make compaction lossless.
