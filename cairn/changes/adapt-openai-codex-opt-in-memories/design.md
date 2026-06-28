# Design: Adapt OpenAI Codex opt-in memories for Animus

## Context

Codex memories are disabled unless enabled by settings or configuration in some regions, and Codex recommends keeping required team guidance in durable project files rather than memory. Animus can adapt this as a strict separation between policy/guidance and generated recall state.

## Decisions

### 1. Memory use and generation are separately controlled

**Choice:** Define independent controls for using existing memories and generating new memories, with user, project, and thread scope.

**Rationale:** A thread may be safe to use prior context but inappropriate as a source for future memory generation.

### 2. Memories are evidence-backed generated records

**Choice:** Memory records include kind, summary, scope, freshness, source thread identity, supporting evidence handles, redaction status, and confidence.

**Rationale:** Future prompt assembly can explain why a memory was included and whether it may be stale.

### 3. Required behavior does not live only in memory

**Choice:** Runtime treats memories as optional context. Required rules must come from guidance, policy, or checked-in contracts.

**Rationale:** Memories may be disabled, skipped, stale, or local to one user.

### 4. Generation eligibility is conservative

**Choice:** Memory generation is denied for threads without consent, threads containing disallowed external context, secret-bearing content, or content that fails redaction policy.

**Rationale:** Generated memory should not silently persist sensitive or third-party context.

### 5. Injection is budgeted and auditable

**Choice:** Prompt assembly records included, skipped, expired, redacted, and over-budget memories in the context ledger.

**Rationale:** Agents and users need to understand which remembered context shaped a run.

## Risks / Trade-offs

- Conservative generation may miss useful context. Users can promote stable rules into guidance files when they must always apply.
- Memory extraction can be provider-specific. Keep extractors behind adapters and store typed results.
- Evidence handles can expose sensitive locations. Store bounded display references and hashes rather than raw bodies when needed.
