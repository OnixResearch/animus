# Proposal: Add Animus Hugging Face dataset export

## Why

Animus session files should become a reusable training corpus without forcing downstream users to hand-roll conversion scripts. The useful idea from `claude_converter` is the small, inspectable pipeline from agent session records to Hugging Face `messages` rows, but Animus needs that pipeline over Animus-owned session ledgers rather than Claude Code JSONL.

Adding an explicit export contract lets Animus turn validated session replay into supervised fine-tuning rows, preference-learning rows, and RL trajectory records while keeping hidden reasoning, tool payloads, provenance, and unsafe metadata under policy control.

## What Changes

- Define an Animus session-to-dataset export contract that consumes validated Animus session snapshots or replay contexts, not Claude Code records.
- Add Hugging Face Transformers chat-message export shaped for `tokenizer.apply_chat_template()` and SFT datasets.
- Add explicit export policies for system context, tool calls, tool results, hidden reasoning, metadata, redaction, and minimum-quality thresholds.
- Add preference/RL export shapes for prompt/chosen/rejected rows, reward-bearing trajectories, action observations, terminal outcomes, and safe metadata.
- Add dataset inspection reports for turn counts, skipped rows, empty messages, tool/reasoning leakage warnings, token/usage summaries when available, and export policy decisions.
- Keep JSONL/file discovery/writing in std or host adapters; pure export logic operates over in-memory Animus DTOs.

## Impact

- **Files**: likely dataset/export modules or a small crate, std-shell JSONL writer adapters, policy DTOs, fixtures, docs, and Cairn artifacts.
- **Testing**: positive tests for SFT, chat-template, preference, and RL trajectory rows; negative tests for hidden-reasoning leakage, orphaned tool results, malformed choices, missing rewards, invalid roles, and empty exports.
- **Non-claims**: this does not make Claude Code JSONL the source format, train models inside Animus, require Hugging Face dependencies at runtime, or expose private session payloads by default.
