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
8. Before provider continuation, the host projects the selected replay into `animus_runtime::interrupted_tool_repair`. Orphaned results remain structural errors; unmatched assistant calls may produce only an append-only synthetic-failure plan. The original ledger does not infer cancellation or rerun a tool.

## Interrupted-call recovery integration

The neutral ledger owns active-leaf selection and replay ordering. Interrupted-tool repair owns exact call/result diagnosis and an append-only application request over the host-supplied active branch. The host must durably append the complete plan, move the active leaf, and rerun ledger plus repair validation before provider dispatch. Sibling branches remain unchanged.

## Redacted human export

The ledger remains the owner of validated topology, storage order, and active-leaf selection. [Redacted session-tree export](redacted-session-tree-export.md) consumes a separate safe metadata projection after ledger/replay validation; it never opens the ledger store or promotes sibling entries into model-visible replay. Raw journal administration and dataset export remain separate products.

## Non-claims

This is not Pi's JSONL file format, a database API, a TUI branch navigator, a human export renderer, or a persistence implementation. Hosts choose storage and ID generation; reusable Animus code validates and replays caller-supplied snapshots only.
