# Pi session tree ledger adaptation

Animus adapts Pi's branchable session history as a neutral, in-memory runtime contract. The contract is append-only and selected by an active leaf; storage remains host-owned.

## DTO shape

- `LedgerEntryId`: normalized non-empty entry identifier.
- `LedgerEntry`: model-visible user, assistant, tool-result, summary entries plus safe metadata observations and unsupported/adapter-owned fail-closed markers.
- `LedgerNode`: one entry plus an optional `parent_id` link.
- `LedgerSnapshot`: ordered nodes, `active_leaf`, and replay-neutral `LedgerLabel` metadata.
- `LedgerSummaryKind`: `compaction` or `branch_handoff`.
- `LedgerReplayReport`: final model-visible context, safe replay counts, and preserved labels.

## Replay rules

1. Validate duplicate IDs, missing parents, cycles, invalid active leaves, unsupported entries, orphaned tool results, and missing label targets before replay.
2. Walk parent links from `active_leaf` to root, then replay root-to-leaf.
3. Include only the active branch in model-visible context; sibling branch entries remain ledger history.
4. Tool results must directly follow an assistant/model parent.
5. Summary entries replace earlier model-visible context on the active branch and then emit bounded summary text.
6. Labels are metadata only and never enter model-visible context unless a host explicitly injects them.
7. Missing ledger service, missing session, and unavailable service states are typed fail-closed errors.

## Non-claims

This is not Pi's JSONL file format, a database API, a TUI branch navigator, or a persistence implementation. Hosts choose storage and ID generation; reusable Animus code validates and replays caller-supplied snapshots only.
