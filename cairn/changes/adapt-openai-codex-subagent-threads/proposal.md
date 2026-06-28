# Proposal: Adapt OpenAI Codex subagent thread patterns for Animus

## Why

OpenAI Codex frames subagents as explicitly requested parallel workers that reduce context pollution by performing noisy exploration, tests, triage, or review work off the main thread and returning distilled summaries. That pattern fits Animus's goal of typed, observable agent orchestration, especially for large read-heavy tasks.

Animus should adapt explicit delegation, role-specific profiles, isolated subagent transcripts, and bounded summary handoff while being careful with write-heavy parallelism and adapter capability boundaries.

## What Changes

- Define explicit subagent delegation requests with role, task, allowed tools, workspace scope, model profile, and expected summary shape.
- Add isolated subagent thread records that keep noisy intermediate output out of the parent thread by default.
- Define summary handoff contracts for findings, evidence references, confidence, unresolved questions, and recommended next actions.
- Add write-conflict policy for subagents so parallel write-heavy work requires isolated workspaces or explicit serialization.
- Record subagent start, stop, failure, continuation, and aggregation events in runtime reports.

## Impact

- **Files**: subagent DTOs, thread ledger records, summary contracts, aggregation logic, workspace-policy integration, docs, and positive/negative tests.
- **Testing**: positive tests for parallel read-only reviews and summary aggregation; negative tests for implicit subagent spawning, raw-log flooding, unauthorized tools, write conflicts, failed subagents, and malformed summaries.
- **Non-claims**: this change does not add automatic subagent spawning, uncontrolled multi-agent writes, Codex CLI thread switching, or provider-specific model routing.
