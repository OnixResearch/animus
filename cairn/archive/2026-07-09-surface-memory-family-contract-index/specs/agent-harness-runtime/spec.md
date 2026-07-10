# agent-harness-runtime Change Spec: Memory contract index surface

## Requirements

### Requirement: Memory family is indexed

r[agent_harness_runtime.memory_contract_index.family_surface] The maintained contracts index MUST list `family:memory` with owner module, stability posture, authority boundary, safe receipt behavior, serialization posture, focused surface, and focused checks whenever memory DTOs are maintained as public adapter contracts.

#### Scenario: Contracts index exposes memory surface

r[agent_harness_runtime.memory_contract_index.family_surface.listed]
- GIVEN memory DTOs and planners are exported from public adapter modules
- WHEN maintainers render the maintained contracts index
- THEN the index includes a `family:memory` row with safe owner and focused-check metadata
- AND the row links to memory adapter contracts without claiming concrete host storage authority.

### Requirement: Memory conformance manifest backs the index

r[agent_harness_runtime.memory_contract_index.conformance_manifest] Memory contract-index claims MUST be backed by checked conformance metadata that records required matrix cases, positive and negative evidence labels, unsupported or not-applicable reasons, and safe evidence hashes.

#### Scenario: Missing memory conformance row fails the rail

r[agent_harness_runtime.memory_contract_index.conformance_manifest.required]
- GIVEN `docs/contracts-index.md` lists `family:memory`
- AND the checked conformance manifest omits the memory family or required case dispositions
- WHEN the adapter conformance index check runs
- THEN validation fails with a typed missing-family or missing-case diagnostic
- AND no release evidence claims the memory family is fully indexed.

### Requirement: External memory reference is bounded

r[agent_harness_runtime.memory_contract_index.bounded_reference] Agent Memory Techniques MAY be recorded as a bounded reference for memory taxonomy, retrieval, evaluation, and production patterns, but Animus MUST NOT adopt its Python notebooks, framework dependencies, managed services, provider calls, benchmark datasets, prompt text, or storage implementations as canonical contracts.

#### Scenario: Reference entry preserves non-adoption claims

r[agent_harness_runtime.memory_contract_index.bounded_reference.non_adoption]
- GIVEN README or docs mention Agent Memory Techniques as a reference
- WHEN reference validation or review checks the entry
- THEN the entry describes bounded pattern usage
- AND it states that Animus-owned DTOs and adapter contracts remain canonical.

### Requirement: Memory contract index verification

r[agent_harness_runtime.memory_contract_index.verification] The contract-index surfacing change MUST include focused positive and negative checks for indexed memory family metadata, conformance manifest coverage, bounded reference wording, stale check labels, unsafe references, and leak rejection.

#### Scenario: Closeout evidence covers contract index fixtures

r[agent_harness_runtime.memory_contract_index.verification.closeout]
- GIVEN maintainers intend to archive the memory contract-index surfacing change
- WHEN closeout validation runs
- THEN evidence names contract-index, conformance-manifest, bounded-reference, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
