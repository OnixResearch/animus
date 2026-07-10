# agent-harness-runtime Change Spec: ResearchSpec promotion DAGs

## Requirements

### Requirement: ResearchSpec promotion groups are explicit DAG nodes

r[agent_harness_runtime.hermes_researchspec.promotion_dag] Animus MUST represent ResearchSpec promotion ordering as explicit DAG data containing promotion group IDs, kept run IDs, dependency edges, touched-file summaries, metric evidence labels, guardrail status labels, host-held patch or diff hashes, blocked-group explanations, topological batches, and safe promotion receipt identities.

#### Scenario: Dependent promotion groups are represented safely

r[agent_harness_runtime.hermes_researchspec.promotion_dag.dependent]
- GIVEN two kept promotion groups touch disjoint files
- AND one group declares that it depends on the other group's accepted behavior
- WHEN promotion DAG DTO validation runs
- THEN Animus records the dependency as safe graph structure
- AND no patch body, raw diff, benchmark log, provider payload, or full host path is copied into reusable runtime state.

### Requirement: Promotion DAG validation rejects unsafe or ambiguous groups

r[agent_harness_runtime.hermes_researchspec.promotion_dag.validation] ResearchSpec promotion DAG validation MUST reject duplicate groups, duplicate edges, unknown dependencies, self-dependencies, cycles, overlapping touched files without an explicit block decision, stale run evidence, failed or unkept runs, unresolved hypothesis decisions, guarded-file conflicts, unsafe labels, and malformed receipt metadata.

#### Scenario: Cycle blocks promotion planning

r[agent_harness_runtime.hermes_researchspec.promotion_dag.validation.cycle]
- GIVEN promotion group A depends on group B and group B depends on group A through one or more edges
- WHEN promotion DAG validation runs
- THEN validation returns a cycle diagnostic with safe group identity evidence
- AND no promotion eligibility, archive-ready status, VCS operation, or workspace mutation is produced.

### Requirement: Promotion order is deterministic and non-mutating

r[agent_harness_runtime.hermes_researchspec.promotion_dag.order] Animus MUST produce deterministic topological promotion batches and blocked-group explanations over valid ResearchSpec promotion DAGs, and MUST NOT apply patches, invoke VCS commands, edit files, run benchmarks, persist study archives, or merge work from reusable runtime logic.

#### Scenario: Independent groups become a stable batch

r[agent_harness_runtime.hermes_researchspec.promotion_dag.order.batch]
- GIVEN several promotion groups have no unmet dependencies and pass keep, guardrail, and overlap checks
- WHEN promotion DAG ordering runs
- THEN those groups are returned in a deterministic sorted batch
- AND actual promotion, branch movement, file edits, archive mutation, and conflict resolution remain host-owned.

### Requirement: Promotion DAG receipts are safe evidence

r[agent_harness_runtime.hermes_researchspec.promotion_dag.receipts] ResearchSpec promotion DAG receipts MUST include only schema labels, safe group IDs, sorted dependency edges, bounded counts, issue codes, metric evidence hashes, touched-file summary hashes, guardrail status labels, patch or diff hashes, BLAKE3 graph receipt identities, and redaction markers.

#### Scenario: Raw diff leak is rejected

r[agent_harness_runtime.hermes_researchspec.promotion_dag.receipts.rejects_raw_diff]
- GIVEN a promotion receipt fixture contains raw diff content, raw benchmark output, a raw prompt, provider payload, credential-looking value, absolute path dump, raw model output, raw tool output, or unbounded log text
- WHEN promotion DAG receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND the promotion group cannot be counted as safe durable evidence.

### Requirement: Promotion DAG documentation states host-owned mutation boundaries

r[agent_harness_runtime.hermes_researchspec.promotion_dag.docs] ResearchSpec promotion DAG documentation MUST explain that the DAG only validates ordering, dependency evidence, and safe receipts, while benchmark execution, patch application, VCS operations, conflict resolution, archive mutation, scheduling, and persistence remain host-owned.

#### Scenario: Maintainer reviews promotion non-claims

r[agent_harness_runtime.hermes_researchspec.promotion_dag.docs.non_claims]
- GIVEN maintainers inspect ResearchSpec promotion DAG documentation
- WHEN they compare reusable runtime behavior with host adapter behavior
- THEN the documentation identifies pure ordering and receipt projection as reusable claims
- AND it records benchmark execution, branch mutation, file writes, patch application, and study archive persistence as non-claims.

### Requirement: Promotion DAG verification covers positive and negative cases

r[agent_harness_runtime.hermes_researchspec.promotion_dag.verification] ResearchSpec promotion DAG adoption MUST include focused positive and negative tests for independent promotion batches, dependent ordering, blocked-group explanations, duplicate groups, duplicate edges, unknown dependencies, cycles, overlapping files, stale run evidence, failed or unkept runs, unresolved hypotheses, guarded-file conflicts, unsafe labels, receipt redaction, raw diff leak rejection, documentation, Cairn validation, and Cairn gates.

#### Scenario: Closeout evidence names promotion DAG coverage

r[agent_harness_runtime.hermes_researchspec.promotion_dag.verification.closeout]
- GIVEN maintainers intend to archive ResearchSpec promotion DAG support
- WHEN closeout validation runs
- THEN evidence names promotion DAG DTOs, validation, ordering, receipts, docs, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records VCS mutation, patch application, benchmark execution, conflict resolution, archive writes, and persistence as non-claims.
