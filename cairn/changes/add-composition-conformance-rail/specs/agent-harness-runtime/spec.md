# agent-harness-runtime Change Spec: Composition conformance rail

## Requirements

### Requirement: Composition conformance matrix

r[agent_harness_runtime.composability.conformance_matrix] Animus MUST maintain a composition conformance matrix that names representative cross-module scenarios, owners, feature requirements, positive cases, negative cases, and safe evidence labels.

#### Scenario: Matrix entries include positive and negative coverage

r[agent_harness_runtime.composability.conformance_matrix.coverage]
- GIVEN a conformance matrix entry covers provider/model dispatch, tool/provenance receipts, headless/runtime events, middleware/failure reports, or capability/no-fallback behavior
- WHEN matrix validation runs
- THEN each entry has an owner, feature requirements, at least one positive case, at least one negative case, and a safe evidence label
- AND incomplete entries fail validation.

### Requirement: Positive cross-module conformance

r[agent_harness_runtime.composability.positive_conformance] Composition tests MUST include deterministic positive scenarios that combine independent modules using pure fixtures and fake adapters rather than live host authority.

#### Scenario: Cross-module happy paths compose without live services

r[agent_harness_runtime.composability.positive_conformance.fake_backed]
- GIVEN fake adapters and in-memory DTO fixtures for selected conformance scenarios
- WHEN the composition rail runs
- THEN provider hardening, model dispatch, tool contracts, provenance receipts, headless event projection, middleware, and runtime reports compose according to their typed contracts
- AND no live provider, tool process, network, terminal, filesystem mutation, or persistence backend is required.

### Requirement: Negative cross-module conformance

r[agent_harness_runtime.composability.negative_conformance] Composition tests MUST include deterministic negative scenarios for malformed responses, unauthorized tools, missing services, oversized output, unsafe receipts, disabled feature surfaces, and ambient fallback attempts.

#### Scenario: Cross-module failure paths fail closed

r[agent_harness_runtime.composability.negative_conformance.fail_closed]
- GIVEN a conformance scenario receives a malformed response, unauthorized capability, missing service, oversized output, unsafe receipt, disabled surface, or attempted ambient fallback
- WHEN the composition rail runs
- THEN runtime returns the matching typed failure or denial
- AND no downstream module claims completed effects or accepts unsafe evidence.

### Requirement: Focused composition rail

r[agent_harness_runtime.composability.conformance_rail] Animus MUST provide a focused composition conformance command or script that runs deterministic in-memory cross-module scenarios and emits bounded safe evidence.

#### Scenario: Conformance evidence remains safe

r[agent_harness_runtime.composability.conformance_rail.safe_evidence]
- GIVEN the conformance rail completes or fails
- WHEN it emits evidence output
- THEN evidence includes scenario names, classes, counts, and hashes
- AND raw prompts, provider payloads, credentials, full paths, and unbounded tool outputs are omitted or rejected by leak checks.

### Requirement: Composition conformance verification

r[agent_harness_runtime.composability.conformance.verification] The composition conformance change MUST include focused positive and negative tests for matrix coverage, fake-backed composition, fail-closed composition, safe evidence, workspace checks, optional Nix check coverage, and Cairn gates before archive.

#### Scenario: Closeout evidence covers composition rail behavior

r[agent_harness_runtime.composability.conformance.verification.closeout]
- GIVEN maintainers intend to archive the composition conformance change
- WHEN closeout validation runs
- THEN evidence names matrix, positive conformance, negative conformance, safe evidence, workspace, Nix if added, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
