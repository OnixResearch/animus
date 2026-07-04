# Hugging Face dataset export

Animus exports training data from validated Animus session snapshots or selected replay contexts. External transcripts, Hugging Face libraries, tokenizers, trainer frameworks, filesystem discovery, upload, and sharding stay outside the pure exporter.

## Safe defaults

The default export policy excludes system text, tool calls, tool results, hidden reasoning, metadata projection, raw environment details, and private provenance. Opt-in policies can admit safe labels for selected categories, but raw tool output and hidden reasoning are redacted before row rendering.

## Supervised chat rows

The SFT target emits Hugging Face-compatible `messages` rows for downstream tools that call `tokenizer.apply_chat_template()`:

```json
{"messages":[{"role":"user","content":"What should I change?"},{"role":"assistant","content":"Use the safe runtime DTO."}]}
```

`animus-runtime` constructs rows through `export_chat_messages_row` and exposes host-facing JSONL projection with `render_chat_row_jsonl` and `render_chat_rows_jsonl`. These projection helpers render strings only; hosts own file writing, dataset naming, upload, and tokenization.

## Preference and RL rows

Preference export uses explicit Animus outcome annotations. It preserves prompt messages plus chosen and rejected assistant alternatives, and rejects examples with missing prompts, duplicate candidates, missing labels, invalid prompt roles, or unsafe metadata.

RL trajectory export uses explicit trajectory annotations. It emits ordered prompt/action/observation steps with a reward label, terminal outcome, safe metadata, and structured action-observation links. Missing rewards, non-terminal trajectories, orphaned observations, invalid action links, and private metadata fail closed.

## Inspection reports

`inspect_dataset_exports` returns structured counts, skipped-row reason classes, leakage warnings, quality warnings for empty or short turns, safe policy decisions, and an optional usage summary field for host-supplied token/usage data. Reports intentionally avoid raw hidden reasoning, raw tool payloads, private provenance, full paths, credentials, or provider bodies.

## Non-goals

This export surface does not train models, call Hugging Face APIs, make Claude Code JSONL an Animus source format, infer preference labels from branch order, fabricate RL rewards, or expose private session payloads by default.
