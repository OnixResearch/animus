# agent-harness-runtime Change Spec: Clankers provider contract parity evaluation

## Requirements

### Requirement: Provider parity remains a future bounded pattern

r[agent_harness_runtime.clankers_provider_parity.scope] Animus MUST treat Clankers provider/router contract parity as a future bounded pattern and MUST NOT add provider clients, router daemons, OAuth, credential lookup, network transport, or live provider checks in this evaluation change.

#### Scenario: Provider parity scope is explicit

r[agent_harness_runtime.clankers_provider_parity.scope.future]
- GIVEN maintainers evaluate Clankers provider parity patterns
- WHEN the adoption note is reviewed
- THEN it documents parity criteria for future provider abstractions
- AND it keeps concrete provider clients, router daemons, OAuth, credentials, network transport, and live checks out of scope.

### Requirement: Shared provider request fields are defined when needed

r[agent_harness_runtime.clankers_provider_parity.request_fields] If Animus introduces multiple provider or router request DTOs, it MUST define shared request-field expectations and parity checks for fields such as model, messages, system prompt, output limits, temperature, tools, metadata, cache policy, and extra parameters as applicable.

#### Scenario: Shared fields stay aligned

r[agent_harness_runtime.clankers_provider_parity.request_fields.aligned]
- GIVEN multiple provider request DTOs coexist
- WHEN parity tests run
- THEN shared fields serialize and project consistently
- AND missing cross-cutting metadata is detected before provider dispatch.

### Requirement: Constructor inventory catches request drift

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory] Concrete provider request structs with multiple construction sites SHOULD have a constructor-inventory rail that requires each real constructor to set all required shared fields.

#### Scenario: Constructor inventory fails on omitted fields

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory.omission]
- GIVEN a new provider request constructor omits a required shared field
- WHEN the constructor-inventory rail runs
- THEN it fails with an actionable diagnostic
- AND maintainers update either the constructor or the accepted inventory count.

### Requirement: Provider-native message conversion

r[agent_harness_runtime.clankers_provider_parity.message_conversion] Provider routing adapters MUST use provider-native message conversion fixtures when provider-specific message shapes are required and MUST NOT rely on lossy generic JSON conversion.

#### Scenario: Native conversion preserves content

r[agent_harness_runtime.clankers_provider_parity.message_conversion.fixture]
- GIVEN a provider backend requires a native message shape
- WHEN conversion tests compare against an explicit fixture
- THEN user, assistant, tool, summary, and metadata content required by the backend are preserved
- AND lossy generic conversion that drops content fails.

### Requirement: Streaming and provider-prefix failures are tested

r[agent_harness_runtime.clankers_provider_parity.streaming] Future streaming provider claims MUST include parser-entrypoint tests that feed raw stream bytes through the real parser.

r[agent_harness_runtime.clankers_provider_parity.fail_closed_prefix] Future provider-qualified model IDs MUST fail closed for known-but-unavailable providers rather than falling back silently.

#### Scenario: Provider parity closeout is evidenced

r[agent_harness_runtime.clankers_provider_parity.closeout]
- GIVEN provider parity evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, request-field criteria, constructor-inventory pattern, message-conversion fixtures, streaming parser-entrypoint criteria, fail-closed prefix criteria, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
