# agent-harness-runtime Change Spec: Concrete std runtime adapters

## Requirements

### Requirement: Adapter families

r[agent_harness_runtime.std_runtime_adapters.adapter_families] Animus MUST define concrete std-shell adapter families for model providers, filesystem tools, process execution, HTTP/network helpers, and deterministic fixture backends before those effects are exposed through runtime workflows.

#### Scenario: Concrete effects stay in shell crates

r[agent_harness_runtime.std_runtime_adapters.adapter_families.shell_owned]
- GIVEN a concrete adapter opens a provider connection, reads or writes host files, starts a process, accesses HTTP/network resources, reads credentials, or depends on platform APIs
- WHEN maintainers inspect crate dependencies
- THEN that adapter is owned by `animus-std` or an optional host crate
- AND reusable runtime/core crates consume only injected traits and DTOs.

### Requirement: Effect gates

r[agent_harness_runtime.std_runtime_adapters.effect_gates] Concrete adapters MUST check accepted provider registry, tool contract, policy, sandbox, receipt, and event lifecycle contracts before performing host effects, and MUST fail closed when a required contract or approval is absent.

#### Scenario: Adapter refuses ungated effect

r[agent_harness_runtime.std_runtime_adapters.effect_gates.rejects_ungated]
- GIVEN an adapter request lacks required policy authorization, sandbox compatibility, tool schema validation, provider capability support, or host approval
- WHEN the adapter receives the request
- THEN it returns a typed denial before performing the effect
- AND it emits only safe status evidence.

### Requirement: Deterministic fixture adapters

r[agent_harness_runtime.std_runtime_adapters.fixtures] Animus MUST provide deterministic fixture adapters for default tests of model, filesystem, process, network, clock, and credential behavior without live providers, real host mutation, or ambient secrets.

#### Scenario: Default tests avoid live effects

r[agent_harness_runtime.std_runtime_adapters.fixtures.no_live_effects]
- GIVEN default workspace checks run on a developer or CI machine
- WHEN concrete adapter tests execute
- THEN they use in-memory or fixture-backed adapters
- AND they do not require network access, credentials, process side effects, or host filesystem mutation beyond test-owned temporary state.

### Requirement: Positive adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_positive] Concrete adapter work MUST include positive tests for successful injected model, tool, filesystem, process, and HTTP helper flows using deterministic fixtures.

#### Scenario: Fixture-backed adapters produce runtime events

r[agent_harness_runtime.std_runtime_adapters.tests_positive.events]
- GIVEN fixture-backed adapters are injected into runtime or shell orchestration
- WHEN a supported request is dispatched
- THEN the adapter returns a typed success
- AND runtime events and receipts record safe identities, status classes, counts, and hashes.

### Requirement: Negative adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_negative] Concrete adapter work MUST include negative tests for missing credentials, denied capabilities, sandbox mismatch, malformed outputs, timeout or cancellation, unsupported provider/tool features, and receipt leak attempts.

#### Scenario: Adapter failures do not leak sensitive payloads

r[agent_harness_runtime.std_runtime_adapters.tests_negative.safe_failures]
- GIVEN an adapter fails because credentials are missing, capability is denied, sandbox cannot be enforced, output is malformed, or operation times out
- WHEN failure evidence is emitted
- THEN the evidence contains typed issue codes and safe metadata
- AND raw credentials, prompts, provider payloads, full paths, private file bodies, and unbounded command output are omitted.

### Requirement: Dependency isolation

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation] Animus MUST provide dependency-isolation checks proving reusable runtime/core crates do not depend on concrete provider, filesystem, process, network, credential, or platform-enforcement crates.

#### Scenario: Concrete dependency enters reusable crate

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation.rejects_leak]
- GIVEN a concrete adapter dependency is added to a reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to move the dependency behind a shell adapter.

### Requirement: Adapter closeout evidence

r[agent_harness_runtime.std_runtime_adapters.closeout] Concrete adapter work MUST document opt-in live smoke-test configuration and run deterministic adapter tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Adapter evidence is reviewable

r[agent_harness_runtime.std_runtime_adapters.closeout.evidence]
- GIVEN concrete adapter work is ready to archive
- WHEN reviewers inspect the evidence
- THEN deterministic tests and dependency-isolation checks are named with status
- AND live smoke tests are either explicitly run with safe configuration or marked unavailable with a blocker.
