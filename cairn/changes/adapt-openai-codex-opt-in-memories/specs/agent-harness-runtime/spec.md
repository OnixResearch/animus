# agent-harness-runtime Change Spec: OpenAI Codex opt-in memories adaptation

## Requirements

### Requirement: Separate opt-in controls for memory use and generation

r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls] Animus memories MUST have independent opt-in controls for using existing memories and generating new memories at user, project, and thread scopes.

#### Scenario: Missing consent blocks memory generation

r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls.consent]
- GIVEN a thread does not allow memory generation
- WHEN the thread ends with content that would otherwise be memorable
- THEN Animus does not create a memory-generation request
- AND the report records memory generation as disabled for that thread scope.

### Requirement: Evidence-backed memory records

r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract] Memory records MUST include kind, summary, scope, freshness, source thread identity, supporting evidence handles, redaction status, and confidence.

#### Scenario: Malformed memory record is rejected

r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract.validation]
- GIVEN a generated memory record has no source thread identity or evidence handle
- WHEN Animus validates the memory record
- THEN validation rejects the record
- AND the memory is not eligible for future prompt injection.

### Requirement: Swappable memory storage and extraction boundary

r[agent_harness_runtime.openai_codex_opt_in_memories.adapter_boundary] Memory storage, retrieval, search, update/delete, and provider-assisted extraction MUST be represented as swappable injected adapter contracts that exchange typed memory records, generation requests, extraction outcomes, and availability statuses; reusable core/runtime logic MUST NOT depend on a concrete memory database, vector index, provider extractor, or host path layout.

#### Scenario: Alternate backend preserves memory semantics

r[agent_harness_runtime.openai_codex_opt_in_memories.adapter_boundary.swappable]
- GIVEN two hosts back memories with different storage or extraction implementations
- WHEN the same validated memory records, extraction outcomes, and policy decisions reach reusable logic
- THEN generation eligibility, prompt injection, and ledgers use only typed adapter DTOs and produce equivalent reusable outcomes
- AND missing adapters produce explicit unavailable outcomes rather than ambient fallback storage, search, or extraction.

### Requirement: Conservative memory generation eligibility

r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility] Memory generation MUST be denied for threads without consent, disallowed external context, secret-bearing content, failed redaction, or exhausted memory-generation budget.

#### Scenario: External context prevents memory generation

r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility.exclusions]
- GIVEN a thread consumed external context that policy marks ineligible for persistence
- WHEN memory generation is considered
- THEN Animus skips memory generation for that thread
- AND the skip reason is recorded without storing the external context body.

### Requirement: Bounded memory injection ledger

r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger] Prompt assembly MUST record included, skipped, expired, redacted, and over-budget memories with source attribution and bounded content.

#### Scenario: Memory injection is bounded

r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger.bounded]
- GIVEN available memories exceed the configured memory context budget
- WHEN Animus assembles prompt context
- THEN only policy-selected bounded memory summaries are injected
- AND skipped memories are recorded by identity and reason.

### Requirement: Memories are non-authoritative context

r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative] Animus MUST treat memories as optional context and MUST keep required rules in guidance, policy, or checked-in contracts.

#### Scenario: Disabled memories do not disable required rules

r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative.guidance]
- GIVEN memories are disabled for a run
- WHEN prompt assembly loads required project guidance and policy
- THEN those required rules remain active
- AND no memory state is required for the rules to apply.
