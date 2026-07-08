# agent-harness-runtime Change Spec: External subagent conformance fixtures

## Requirements

### Requirement: External subagent fixtures cover admitted descriptors

r[agent_harness_runtime.subagent_external_conformance.positive_fixtures] Animus MUST include positive external subagent fixtures for admitted ACP and MCP descriptors with explicit protocol labels, safe external identities, injected service labels, declared tools, optional credential reference labels, and bounded capability ceilings.

#### Scenario: Valid MCP descriptor fixture is accepted

- GIVEN a fixture declares an MCP external subagent with an injected service label and declared read-only tools
- WHEN descriptor conformance validation runs
- THEN the fixture is accepted
- AND no live MCP server, CLI, network connection, or credential read is required.

### Requirement: External subagent fixtures cover denied descriptors

r[agent_harness_runtime.subagent_external_conformance.negative_fixtures] Animus MUST include negative external subagent fixtures for missing service labels, undeclared tools, ambient CLI discovery, inherited credentials, malformed protocol labels, unsafe output imports, and overbroad capability claims.

#### Scenario: Ambient CLI fixture is denied

- GIVEN a fixture requests external subagent discovery from PATH or another ambient CLI source
- WHEN descriptor conformance validation runs
- THEN validation returns an ambient-discovery-denied failure class
- AND no CLI lookup is attempted.

### Requirement: Conformance index is machine-checkable

r[agent_harness_runtime.subagent_external_conformance.index] External subagent conformance fixtures SHOULD be indexed with fixture identity, expected status, expected failure class when denied, protocol kind, declared tools, and covered requirement identifiers.

#### Scenario: Missing expected failure class is rejected

- GIVEN a negative fixture is listed without an expected failure class
- WHEN the conformance index check runs
- THEN the check fails with a missing-expected-failure diagnostic
- AND the fixture is not counted as coverage.

### Requirement: External conformance has no live dependency

r[agent_harness_runtime.subagent_external_conformance.no_live_dependency] External subagent conformance validation MUST run over checked-in fixtures and pure descriptor validation without requiring live ACP services, MCP services, network access, ambient CLI lookup, filesystem discovery outside the fixture set, or credentials.

#### Scenario: Offline conformance succeeds

- GIVEN only checked-in external subagent fixtures are available
- WHEN focused conformance validation runs
- THEN accepted and denied fixtures are classified as expected
- AND no external process, network, or credential dependency is required.

### Requirement: External conformance closeout is evidenced

r[agent_harness_runtime.subagent_external_conformance.verification] External subagent conformance adoption MUST include positive and negative fixture tests, index validation, offline validation evidence, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: External conformance evidence is complete

- GIVEN the external conformance change is ready to archive
- WHEN closeout validation runs
- THEN evidence names fixture tests, index checks, offline validation, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
