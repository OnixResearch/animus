# agent-harness-runtime Change Spec: Roadmap sequencing

## Requirements

### Requirement: Roadmap taxonomy

r[agent_harness_runtime.roadmap_sequence.taxonomy] Animus MUST classify active Cairn changes by roadmap role, including foundational lifecycle, core-runtime contract, host-adapter, optional-reference, product-surface, and evaluation-only roles.

#### Scenario: Active changes have roadmap roles

r[agent_harness_runtime.roadmap_sequence.taxonomy.classified]
- GIVEN the Cairn tree contains active changes
- WHEN the roadmap classification is checked
- THEN each active change has exactly one primary role
- AND optional secondary relationships are represented as explicit prerequisite or related-change links.

### Requirement: Dependency order

r[agent_harness_runtime.roadmap_sequence.dependency_order] The roadmap MUST define prerequisite tiers that land accepted baseline specs and local validation wiring before runtime behavior, reusable runtime contracts before concrete host adapters, and core safety contracts before product surfaces.

#### Scenario: Dependent change waits for prerequisite contracts

r[agent_harness_runtime.roadmap_sequence.dependency_order.blocks_dependents]
- GIVEN a change depends on runtime loop, injected services, receipts, ledger, event, policy, or host-adapter contracts
- WHEN the prerequisite requirement IDs are not accepted, not in the same archive batch, and not explicitly waived
- THEN roadmap validation reports the dependency as blocked
- AND the dependent change is not marked archive-ready.

### Requirement: Overlap resolution

r[agent_harness_runtime.roadmap_sequence.overlap_resolution] The roadmap MUST assign ownership for overlapping ledger, receipt, provider, hook, manifest, catalog, and tool-contract concerns so duplicate changes converge on shared requirement IDs instead of creating incompatible contracts.

#### Scenario: Overlapping changes share owners

r[agent_harness_runtime.roadmap_sequence.overlap_resolution.shared_owner]
- GIVEN two active changes describe similar ledger, receipt, provider, hook, manifest, catalog, or tool behavior
- WHEN maintainers classify the changes
- THEN one owning concept and prerequisite path is recorded
- AND the non-owning change links to that owner or narrows itself to evaluation-only scope.

### Requirement: Roadmap validation

r[agent_harness_runtime.roadmap_sequence.validation] Animus SHOULD provide a deterministic roadmap fixture or static check that detects unknown change IDs, unknown prerequisite requirement IDs, dependency cycles, unclassified active changes, and product-surface changes that bypass foundational safety contracts.

#### Scenario: Invalid roadmap fails closed

r[agent_harness_runtime.roadmap_sequence.validation.negative]
- GIVEN roadmap fixtures include an unknown change ID, unknown prerequisite ID, dependency cycle, unclassified change, or unsafe product-surface shortcut
- WHEN roadmap validation runs
- THEN validation fails with deterministic issue codes
- AND the diagnostics do not depend on filesystem ordering or ambient state.

### Requirement: Roadmap closeout evidence

r[agent_harness_runtime.roadmap_sequence.closeout] Roadmap sequencing work MUST produce reviewable evidence that lifecycle validation, roadmap validation, and Cairn gates pass before the change is archived.

#### Scenario: Sequencing evidence is reviewable

r[agent_harness_runtime.roadmap_sequence.closeout.evidence]
- GIVEN the roadmap sequencing change is ready to archive
- WHEN reviewers inspect the evidence
- THEN the evidence names successful lifecycle validation, focused roadmap checks, and Cairn proposal/design/tasks gates
- AND any unavailable check has a blocker and a next best validation path.
