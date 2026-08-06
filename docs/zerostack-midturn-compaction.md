# Zerostack mid-turn compaction

Animus adapts Zerostack's mid-turn compaction patterns as pure runtime contracts. Runtime core classifies prompt pressure, prepares explicit stop/compact/resume plans, validates summaries, and bounds tool-output artifact descriptors. Hosts own provider calls, token estimation, summarizer execution, ledger persistence, files, clocks, tools, UI, and continuation side effects.

The reusable surface lives in `animus-runtime::zerostack_midturn` and is re-exported by `animus-runtime`.

## Prompt pressure calibration

`calculate_zerostack_prompt_pressure` keeps provider-reported prompt pressure separate from estimated replay tokens. Route metadata controls whether cached-token fields are included in provider input tokens, reported separately, absent, or unknown. Unknown or contradictory route/usage metadata fails closed instead of guessing and risking undercounting or double-counting.

## Mid-turn continuation planning

`plan_zerostack_midturn_compaction` evaluates calibrated pressure against policy-owned context window, response reserve, and threshold basis points. If mid-turn compaction is disabled, the policy-selected continue/warn/fail outcome is returned without hidden compaction. If pressure crosses the threshold and services are ready, runtime returns an explicit stop/compact/resume plan. Missing ledger or summarizer services fail closed and do not fall back to model or tool dispatch.

## Persistent RLM kernel state

Mid-turn stop, compact, and resume planning does not clear an active model-facing Steel kernel. The optional RLM plan records kernel-held context as counts and identities. The summary can reference that retained state instead of repeating its values. An implicit clear fails before summary append or resume.

## Explicit ledger summaries

Mid-turn compaction is represented as an explicit summary append requirement rather than an unrecorded transcript rewrite. `validate_zerostack_midturn_summary` accepts only supported summary kinds, bounded non-empty summary text, safe metadata labels, and BLAKE3 source/summary hashes. Empty, overlong, unsupported, unsafe-marker, and non-BLAKE3 summaries are rejected before ledger append.

## Bounded tool-output artifacts

`artifactize_zerostack_tool_output` represents long tool output with model-visible head/tail windows, total size metadata, truncation state, an artifact label, and a BLAKE3 full-output hash. The descriptor never embeds the full raw output in durable evidence by default.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime zerostack_midturn
```

The tests cover cached-token included/separate/absent/malformed cases, enabled and disabled planning, threshold behavior, missing-pressure fail policy, missing-ledger fail-closed behavior, summary validation failures, and bounded long tool-output artifacts.
