# Pi compaction and branch summaries

Animus adapts Pi's long-session compaction mechanics as pure runtime contracts. The runtime prepares deterministic plans and validates safe metadata; hosts own token estimation, persistence, provider/model calls, concrete summarizer execution, clocks, files, UI, and branch navigation side effects.

The reusable surface lives in `animus-runtime::pi_compaction` and is re-exported by `animus-runtime`.

## Policy and trigger

`PiCompactionPolicy` contains the typed inputs required for compaction decisions:

- context budget units
- response reserve units
- kept-recent budget units
- summary format
- summarizer selection
- redaction behavior
- enabled/disabled state

`evaluate_pi_compaction_trigger` returns disabled, missing-estimate, under-budget, required, or invalid-policy outcomes. Missing estimates fail closed because runtime core must not guess provider tokenization.

## Safe cut points and split turns

`prepare_pi_compaction` works over already-loaded in-memory `PiCompactionReplayEntry` values. It selects a kept suffix and summarized prefix while preserving tool-call/result bindings. If a candidate cut would keep a tool result while dropping its tool call, the cut moves earlier so the resulting model-visible suffix remains structurally valid.

When a single turn crosses the kept-recent boundary, the plan records `split_turn_prefix_entry_ids` separately from the kept suffix. This makes lossy split-turn compaction explicit instead of pretending a clean turn boundary exists.

Compaction does not repair an assistant call that lacks a result. Before compaction or provider continuation, the selected active branch must be clean or durably repaired through `animus_runtime::interrupted_tool_repair`. Synthetic failures then participate in cut-point validation as committed terminal results; an in-memory-only or partially applied plan cannot satisfy the invariant.

## Branch handoff summaries

`prepare_pi_branch_handoff_summary` plans host-requested branch handoff summaries from an old leaf, target leaf, parent links, and a request mode. It identifies the common ancestor, abandoned old-branch span, and target branch span without reading or writing storage.

## Injected summarizer services

`evaluate_required_pi_summarizer` prepares a `PiCompactionSummarizerRequest` only when compaction is required and an injected summarizer is ready. Missing, denied, cancelled, or malformed summarizer states halt the required compaction and do not fall back to constructing provider/model/tool clients inside runtime core.

## Safe summary metadata

`validate_pi_summary_entry` accepts bounded summary text, positive source counts, redacted resource IDs, cumulative read/modified resource metadata, and BLAKE3 content hashes. It rejects raw tool output markers, provider payload markers, credential-looking markers, unredacted host paths, invalid hashes, empty text, and overlong text.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime pi_compaction
```

The tests cover enabled/disabled/under-budget/over-budget/missing-estimate trigger decisions, safe tool-call/result cut points, split-turn prefix planning, branch handoff spans, missing summarizer fail-closed behavior, ready summarizer dispatch, valid redacted metadata, and unsafe raw metadata rejection.
