# agent-harness-runtime Change Spec: Clankers plugin manifest boundary evaluation

## Requirements

### Requirement: Plugin manifests are bounded descriptor references

r[agent_harness_runtime.clankers_plugin_manifest.scope] Animus MUST treat Clankers plugin manifests as bounded descriptor references and MUST NOT adopt plugin execution, stdio supervision, Wasm loading, Zellij, MCP, or sandbox enforcement in this change.

#### Scenario: Descriptor scope rejects runtime adoption

r[agent_harness_runtime.clankers_plugin_manifest.scope.rejects_runtime]
- GIVEN maintainers evaluate Clankers plugin manifests
- WHEN the adoption scope is reviewed
- THEN it documents descriptor fields separately from runtime execution
- AND it does not add plugin process launch, Wasm loading, MCP/Zellij integration, or sandbox enforcement as default Animus behavior.

### Requirement: Optional manifest schema

r[agent_harness_runtime.clankers_plugin_manifest.schema] Animus MAY define plugin descriptor fields for identity, runtime kind, declared tools, capabilities, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction.

#### Scenario: Manifest descriptors declare authority needs

r[agent_harness_runtime.clankers_plugin_manifest.schema.authority]
- GIVEN a plugin descriptor is parsed
- WHEN validation inspects it
- THEN declared tools, capabilities, runtime kind, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction are explicit
- AND missing required descriptor fields fail validation.

### Requirement: Manifest validation rejects unsafe declarations

r[agent_harness_runtime.clankers_plugin_manifest.validation] Manifest validation MUST reject unknown runtime kinds, missing required fields, duplicate or built-in tool collisions, path escapes, absolute writable roots, environment value leakage, and unsafe network/sandbox combinations.

#### Scenario: Invalid manifest fixtures fail

r[agent_harness_runtime.clankers_plugin_manifest.fixtures]
- GIVEN manifest fixtures include path escapes, absolute writable roots, env values, missing commands, unknown runtime kinds, tool collisions, or unsafe network/sandbox combinations
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors normalize deterministically.

### Requirement: Manifest parsing is runtime-neutral

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral] Manifest parsing MUST NOT launch stdio processes, load Wasm, inherit environment values, read secrets, connect to network, or execute tools.

#### Scenario: Runtime-neutrality rail catches plugin side effects

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral.rail]
- GIVEN plugin descriptor parsing is validated
- WHEN the runtime-neutrality rail runs
- THEN it proves parsing is data-only
- AND no plugin execution, Wasm load, secret read, environment inheritance, or network access is required.

#### Scenario: Plugin manifest closeout is evidenced

r[agent_harness_runtime.clankers_plugin_manifest.closeout]
- GIVEN the plugin manifest boundary evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, manifest fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
