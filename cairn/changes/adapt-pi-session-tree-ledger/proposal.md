# Proposal: Adapt Pi session tree ledger for Animus

## Why

Pi's session format shows a compact way to preserve conversation history, in-place branching, labels, compaction summaries, and branch summaries without forcing every embedder into a database or UI. Animus needs the same replay and resume shape, but as a neutral runtime contract that stays independent of Pi's JSONL files, terminal UI, provider stack, and package system.

Adapting the ledger pattern now gives future runtime, RPC, receipt, compaction, and provenance work one shared source of truth for model-visible history instead of ad hoc transcript vectors.

## What Changes

- Define a neutral append-only session tree ledger contract with entry IDs, parent links, active leaf selection, labels, model-visible messages, tool results, summaries, and safe replay metadata.
- Add deterministic replay/context-building behavior from the active leaf to model input, including compaction and branch-summary entries.
- Add typed resume and ledger errors for missing sessions, broken parent links, duplicate IDs, invalid active leaves, unsupported entry kinds, and unavailable ledger services.
- Keep persistence as an injected shell concern; core/runtime code consumes ledger services and in-memory fixtures only.
- Add positive and negative tests for branch replay, labels, summaries, malformed ledgers, and missing services.

## Impact

- **Files**: likely runtime/session modules or a small crate, Preserves boundary DTOs, replay tests, and documentation under `docs/`.
- **Testing**: positive tests for ordered replay and branch navigation; negative tests for duplicate IDs, missing parents, summary boundary mistakes, and absent ledger services.
- **Non-claims**: this change does not copy Pi's session JSONL format as Animus' canonical storage, add a TUI tree navigator, implement Pi `/fork` or `/clone` commands, or introduce a database.
