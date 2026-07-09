# agent-harness-runtime Change Spec: Cognitive memory taxonomy

## Requirements

### Requirement: Category-aware memory records

r[agent_harness_runtime.memory_taxonomy.record_categories] Memory records crossing reusable Animus boundaries MUST support explicit safe category metadata for episodic, semantic, procedural, reflection or pitfall, instruction, fact or preference, summary, candidate, and unsupported records without requiring a concrete memory store.

#### Scenario: Episodic and procedural records remain distinct

r[agent_harness_runtime.memory_taxonomy.record_categories.distinct]
- GIVEN a host supplies one admitted timestamped episode record and one admitted reusable procedure record
- WHEN reusable memory validation normalizes the records
- THEN each record preserves its safe category label
- AND the labels are available to planning and evidence without exposing raw memory bodies.

### Requirement: Compatibility mapping for existing generic kinds

r[agent_harness_runtime.memory_taxonomy.compatibility] Existing generic memory record kinds MUST map deterministically into the category-aware taxonomy, and unsupported or unknown category labels MUST fail closed when a policy requires explicit category admission.

#### Scenario: Unknown category is not silently coerced

r[agent_harness_runtime.memory_taxonomy.compatibility.unknown]
- GIVEN a memory adapter returns a record with an unknown category label
- WHEN category-aware validation runs under a policy that requires explicit categories
- THEN validation rejects the record with a typed unsupported-category diagnostic
- AND the record is ineligible for retrieval, prompt influence, or influence receipts.

### Requirement: Taxonomy is non-authoritative metadata

r[agent_harness_runtime.memory_taxonomy.non_authority] Memory category labels MAY influence routing, retrieval ordering, compaction, and evaluation, but MUST NOT grant prompt visibility, persistence, tool authority, policy authority, or capability access without existing admission, scope, freshness, redaction, and budget checks.

#### Scenario: Procedural label does not bypass policy

r[agent_harness_runtime.memory_taxonomy.non_authority.policy]
- GIVEN a record is categorized as procedural
- AND its scope is denied or its redaction class is secret
- WHEN memory retrieval or prompt assembly evaluates the record
- THEN the record is skipped before prompt influence
- AND receipts record only safe skip metadata and BLAKE3 body identity.

### Requirement: Memory taxonomy verification

r[agent_harness_runtime.memory_taxonomy.verification] The taxonomy adaptation MUST include focused positive and negative tests for category validation, compatibility mapping, non-authority enforcement, stale or redacted records, unsupported categories, and receipt leak rejection.

#### Scenario: Closeout evidence covers taxonomy fixtures

r[agent_harness_runtime.memory_taxonomy.verification.closeout]
- GIVEN maintainers intend to archive the memory taxonomy adaptation
- WHEN closeout validation runs
- THEN evidence names focused positive and negative taxonomy fixtures, docs updates, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
