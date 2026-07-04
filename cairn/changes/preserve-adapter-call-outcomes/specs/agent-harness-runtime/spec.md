# agent-harness-runtime Change Spec: Typed adapter call outcomes

## Requirements

### Requirement: Typed adapter call envelope

r[agent_harness_runtime.adapter_outcomes.typed_envelope] Animus adapter calls MUST be able to return a typed envelope that distinguishes completed responses from absent, denied, unavailable, malformed, cancelled, timed-out, and execution-failure outcomes without relying on prose parsing.

#### Scenario: Adapter outcome classes validate before runtime mapping

r[agent_harness_runtime.adapter_outcomes.typed_envelope.validation]
- GIVEN a model or tool adapter returns a typed outcome envelope
- WHEN envelope validation runs
- THEN completed responses and supported fail-closed outcome classes are accepted with bounded safe metadata
- AND unsafe message text, unsafe receipt metadata, raw credentials, raw paths, raw provider payloads, and unbounded bodies are rejected before runtime mapping.

### Requirement: Legacy adapter compatibility

r[agent_harness_runtime.adapter_outcomes.compatibility] Existing `ComponentAdapter::call` implementations MUST remain compatible or have explicit migration helpers, and typed adapter outcomes MUST be additive rather than required for current embedders.

#### Scenario: Legacy and typed adapters preserve equivalent success behavior

r[agent_harness_runtime.adapter_outcomes.compatibility.equivalent]
- GIVEN one runtime loop uses a legacy adapter and another loop uses a typed-outcome adapter for equivalent successful requests
- WHEN both adapters return equivalent validated responses
- THEN runtime reports equivalent completed behavior
- AND neither path gains ambient provider, filesystem, process, network, credential, or plugin authority.

### Requirement: Runtime preserves adapter outcome classes

r[agent_harness_runtime.adapter_outcomes.runtime_mapping] Runtime dispatch MUST preserve typed adapter outcome classes in reports and receipts when supplied, and MUST map legacy display errors only to conservative execution-failure diagnostics.

#### Scenario: Fail-closed outcomes do not become string-only success claims

r[agent_harness_runtime.adapter_outcomes.runtime_mapping.fail_closed]
- GIVEN a typed adapter reports denied, unavailable, malformed, cancelled, timed-out, or execution-failure outcome
- WHEN runtime normalizes the dispatch result
- THEN the report carries the matching fail-closed class or safe outcome kind
- AND no success receipt, provenance fact, fallback provider call, shell command, environment read, filesystem scan, or network request is produced.

### Requirement: Adapter outcome verification

r[agent_harness_runtime.adapter_outcomes.verification] The typed adapter outcome change MUST include focused positive and negative tests for envelope validation, compatibility adapters, runtime mapping, leak rejection, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers adapter outcome hardening

r[agent_harness_runtime.adapter_outcomes.verification.closeout]
- GIVEN maintainers intend to archive typed adapter outcome hardening
- WHEN closeout validation runs
- THEN evidence names envelope validation, compatibility, runtime mapping, fail-closed, leak-rejection, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
