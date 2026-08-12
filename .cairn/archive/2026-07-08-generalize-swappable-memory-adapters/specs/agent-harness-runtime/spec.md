# agent-harness-runtime Change Spec: Swappable memory adapters

## Requirements

### Requirement: Memory adapter family

r[agent_harness_runtime.memory_adapters.family] Reusable Animus memory behavior MUST be split across swappable host-injected adapters for source/import, durable storage, retrieval/search, extraction, embedding or search support, admission, update/delete, and instruction-source behavior, and reusable core/runtime logic MUST NOT construct concrete memory databases, file formats, MCP clients, provider extractors, vector indexes, or host path layouts.

#### Scenario: Missing memory adapter fails closed

r[agent_harness_runtime.memory_adapters.family.missing]
- GIVEN a run enables memory use or memory generation
- AND the required memory adapter for storage, retrieval, extraction, admission, or instruction-source import is absent
- WHEN reusable memory planning evaluates the operation
- THEN the result records a typed unavailable or unsupported outcome
- AND no ambient filesystem, database, MCP, network, provider extraction, or vector-index fallback is attempted.

### Requirement: Normalized memory record envelope

r[agent_harness_runtime.memory_adapters.record_envelope] Memory records crossing adapter boundaries MUST normalize into typed envelopes with stable record ID, kind, scope, safe source label, evidence handle, trust or admission status, freshness state, confidence, sensitivity or redaction class, BLAKE3 body hash, prompt-budget units, and optional expiration, supersession, and conflict metadata before they can influence retrieval planning or prompt assembly.

#### Scenario: Malformed memory record is rejected

r[agent_harness_runtime.memory_adapters.record_envelope.invalid]
- GIVEN a memory adapter returns a record without evidence metadata, with an invalid scope, invalid confidence, zero budget units, expired freshness, malformed BLAKE3 hash, or unsafe raw-body marker
- WHEN record envelope validation runs
- THEN validation rejects the record with a typed diagnostic
- AND the record is ineligible for retrieval, prompt injection, storage updates, and influence receipts.

### Requirement: Store-agnostic retrieval planning

r[agent_harness_runtime.memory_adapters.retrieval_plan] Memory retrieval planning MUST be pure deterministic logic over query context, scope policy, normalized candidate records, adapter-declared match reasons, freshness, confidence, conflicts, and prompt budgets, producing selected, skipped, stale, redacted, conflicting, and over-budget outcomes without calling storage, embedding, provider, filesystem, clock, or network services.

#### Scenario: Equivalent backend records produce equivalent plans

r[agent_harness_runtime.memory_adapters.retrieval_plan.backend_equivalence]
- GIVEN two different memory adapters return the same normalized candidate records and match reasons for the same query context
- WHEN retrieval planning runs with the same policy and budget
- THEN selected memories, skipped memories, conflict diagnostics, budget use, and ledger entries are equivalent
- AND backend-specific identifiers do not affect model-visible ordering.

### Requirement: Extraction and admission are separate

r[agent_harness_runtime.memory_adapters.extraction_admission] Memory extraction, host admission, storage, update/delete, and prompt influence MUST be separate typed steps; extractor output MAY propose candidate memories, but only admitted candidates MAY persist, update existing records, or become eligible for prompt injection.

#### Scenario: Denied candidate is not persisted or injected

r[agent_harness_runtime.memory_adapters.extraction_admission.denied]
- GIVEN an extraction adapter proposes a candidate memory from a prior interaction
- WHEN host admission denies the candidate or policy marks the source ineligible
- THEN no store, update, or prompt-injection operation is planned for that candidate
- AND the denial receipt records safe candidate identity and reason without embedding the raw candidate body.

### Requirement: Safe memory operation receipts

r[agent_harness_runtime.memory_adapters.receipts] Memory operation and influence receipts MUST record adapter family label, operation kind, safe record or candidate IDs, source labels, counts, status, redaction class, policy hash, and BLAKE3 body hashes, and MUST NOT include raw memory bodies, credentials, full path dumps, provider payloads, hidden memory bodies, or unbounded prompt text.

#### Scenario: Memory receipt leak is rejected

r[agent_harness_runtime.memory_adapters.receipts.rejects_leaks]
- GIVEN a memory operation receipt fixture includes raw memory body text, a credential-looking value, an absolute path dump, provider JSON, hidden memory body, or unbounded prompt text
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Memory adapter verification

r[agent_harness_runtime.memory_adapters.verification] The generic memory adapter adaptation MUST include focused positive and negative tests for adapter availability, normalized records, malformed records, equivalent backend planning, extraction/admission separation, denied candidates, safe receipts, leak rejection, budget handling, stale records, and conflicts.

#### Scenario: Closeout evidence covers memory adapter fixtures

r[agent_harness_runtime.memory_adapters.verification.closeout]
- GIVEN maintainers intend to archive the swappable memory adapter adaptation
- WHEN closeout validation runs
- THEN evidence names focused adapter-family, record-envelope, retrieval-plan, extraction/admission, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
