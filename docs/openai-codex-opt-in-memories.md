# OpenAI Codex opt-in memories reference

Animus treats Codex-style memories as optional, generated context. Memories are not authority: required team rules belong in guidance, policy, or checked-in contracts, and disabling memory never disables those required rules.

## Opt-in controls

`OptInMemoryPolicy` keeps memory use and memory generation separate across user, project, and thread scopes. A behavior is enabled only when all scopes explicitly opt in. Missing consent, disabled scopes, unavailable adapters, or exhausted budgets fail closed before storage, retrieval, extraction, or prompt injection.

## Memory records

`OptInMemoryRecord` is an evidence-backed generated record with kind, safe summary label, scope, freshness, source thread identity, supporting evidence handle, redaction status, confidence, BLAKE3 body hash, and budget units. Validation rejects missing source-thread or evidence metadata, invalid scope, expired freshness, invalid confidence, invalid hashes, and zero budget units.

## Generation eligibility

`evaluate_memory_generation` is a pure conservative gate. Generation is denied for missing consent, disallowed external context, secret-bearing content, redaction failures, and exhausted generation budget. Denials record typed reasons without storing raw external context or secret-bearing bodies.

## Adapter boundary

Storage, retrieval/search, update/delete, extraction, and availability are host-owned adapter boundaries. Reusable Animus logic consumes typed policies, records, generation decisions, and injection ledgers; it does not depend on a concrete memory database, vector index, provider extractor, or host path layout.

## Prompt injection ledger

`plan_opt_in_memory_injection` produces a bounded safe ledger with included, skipped, expired, redacted, and over-budget states. Entries include record identity, source thread, evidence handle, consumed units, and BLAKE3 body hash, never raw memory bodies.

## Non-claims

This contract does not add a concrete memory database, vector store, provider extractor, model training flow, automatic persistence, or mandatory memory dependency for required policy/guidance behavior.
