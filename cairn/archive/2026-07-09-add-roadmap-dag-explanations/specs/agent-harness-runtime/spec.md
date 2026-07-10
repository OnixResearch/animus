# agent-harness-runtime Change Spec: Roadmap DAG explanations

## Requirements

### Requirement: Roadmap validation explains DAG ordering and blockers

r[agent_harness_runtime.roadmap_dag.explanations] Animus roadmap validation MUST be able to explain active change sequencing as deterministic DAG evidence including topological batches, blocked-change explanations, cycle path summaries, prerequisite issue summaries, and transitive safety-path summaries.

#### Scenario: Blocked dependent is explained

r[agent_harness_runtime.roadmap_dag.explanations.blocked]
- GIVEN an active change lists another active change as a prerequisite
- WHEN roadmap DAG explanation runs
- THEN the output names the dependent change, the prerequisite change, and a blocked status class
- AND it does not copy proposal, design, task, spec body, command log, or raw file content into the explanation.

### Requirement: Roadmap DAG core stays pure over loaded records

r[agent_harness_runtime.roadmap_dag.core_shell] Roadmap DAG explanation logic MUST operate as pure deterministic logic over exported roadmap records, active change IDs, and accepted requirement IDs, while Nickel export, filesystem discovery, stdout rendering, process exits, and optional artifact writes remain script-shell concerns.

#### Scenario: Fixture records test graph logic without filesystem access

r[agent_harness_runtime.roadmap_dag.core_shell.fixture]
- GIVEN a test supplies in-memory roadmap records, active change IDs, and accepted requirement IDs
- WHEN the roadmap DAG core evaluates the fixture
- THEN it returns ordering, blocker, cycle, and safety-path diagnostics without reading directories, invoking Nickel, writing files, or inspecting environment variables.

### Requirement: Roadmap graph projections are safe review artifacts

r[agent_harness_runtime.roadmap_dag.review_projection] Roadmap DAG review projection MAY emit Markdown or DOT-style graph summaries, but MUST include only change IDs, roles, tiers, prerequisite IDs, accepted requirement IDs, issue codes, bounded counts, content hashes when needed, and redaction markers.

#### Scenario: Projection leak is rejected

r[agent_harness_runtime.roadmap_dag.review_projection.rejects_leaks]
- GIVEN a roadmap projection candidate includes a raw prompt, raw change body, provider payload, credential-looking value, absolute private path, environment value, raw command output, raw diff, or unbounded log
- WHEN projection safety validation runs
- THEN validation rejects the projection with a leak diagnostic
- AND the projection is not accepted as durable closeout evidence.

### Requirement: Product-surface safety paths are explicit

r[agent_harness_runtime.roadmap_dag.product_safety] Roadmap DAG explanations for product-surface changes MUST identify whether transitive prerequisites include accepted foundational safety markers and runtime safety markers, and MUST name either the safe marker path or the missing marker class.

#### Scenario: Product surface lacks runtime safety path

r[agent_harness_runtime.roadmap_dag.product_safety.missing_runtime]
- GIVEN a product-surface change has foundational prerequisites but no transitive accepted runtime safety marker
- WHEN product-safety DAG explanation runs
- THEN the result names the missing runtime safety marker class
- AND it does not promote the product surface as ready or stable.

### Requirement: Roadmap DAG verification covers positive and negative fixtures

r[agent_harness_runtime.roadmap_dag.verification] Roadmap DAG explanation work MUST include positive and negative fixtures for valid topological batches, blocked dependents, cycle paths, unknown prerequisites, duplicate IDs, unclassified active changes, unsafe product surfaces, malformed records, review projection safety, and leak rejection.

#### Scenario: Negative cycle fixture remains actionable

r[agent_harness_runtime.roadmap_dag.verification.cycle_fixture]
- GIVEN a fixture creates a change prerequisite cycle
- WHEN roadmap DAG validation runs
- THEN the fixture fails with a cycle issue code and safe cycle evidence
- AND no sync, archive, spec mutation, or change-package mutation occurs.

### Requirement: Roadmap DAG docs define closeout evidence shape

r[agent_harness_runtime.roadmap_dag.docs] Roadmap documentation MUST describe how maintainers should cite DAG batches, blocked dependents, product-safety paths, and skipped graph review artifacts in closeout evidence without copying raw change bodies, command transcripts, private file paths, credentials, provider payloads, or unbounded logs.

#### Scenario: Closeout cites bounded roadmap graph evidence

r[agent_harness_runtime.roadmap_dag.docs.closeout]
- GIVEN maintainers intend to archive a roadmap-affecting change
- WHEN they prepare closeout evidence
- THEN documentation directs them to cite bounded DAG counts, blocker IDs, safe marker paths, command identities, and artifact hashes
- AND it records any unavailable graph projection with a blocker and next-best validation path.
