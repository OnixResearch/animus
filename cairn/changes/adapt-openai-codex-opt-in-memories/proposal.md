# Proposal: Adapt OpenAI Codex opt-in memories for Animus

## Why

OpenAI Codex treats memories as opt-in generated state that can carry stable preferences, recurring workflows, project conventions, and known pitfalls across threads. It also distinguishes required team guidance from local memory, redacts secrets, and provides thread-level controls for whether a session can use or generate memories.

Animus should adapt the safety semantics: memories are useful context, not authoritative policy; generation and use must be explicit; and memory records must be bounded, redacted, attributable, and auditable.

## What Changes

- Define opt-in memory use and memory generation policies at user, project, and thread scopes.
- Add memory record DTOs for summaries, durable facts, preferences, pitfalls, supporting evidence, source scope, and freshness metadata.
- Add redaction, external-context exclusion, and budget-gating requirements before thread content can generate memories.
- Add memory injection records that show which memories were used, skipped, expired, or redacted during prompt assembly.
- Keep memory storage and model extraction as shell/provider concerns rather than core runtime dependencies.

## Impact

- **Files**: memory policy DTOs, memory record contracts, prompt assembly integration, redaction/freshness metadata, docs, and positive/negative tests.
- **Testing**: positive tests for opted-in memory use and generation; negative tests for disabled memories, external-context exclusions, stale memories, secret-bearing content, missing consent, and over-budget injection.
- **Non-claims**: this change does not make memories authoritative policy, add a concrete memory database, or train models on user data.
