# agent-harness-runtime Change Spec: Clankers tool catalog evaluation

## Requirements

### Requirement: Bounded tool catalog evaluation

r[agent_harness_runtime.clankers_tool_catalog.scope] Animus MUST evaluate Clankers tool-catalog and capability-pack patterns as bounded references and MUST define any adopted catalog schema in Animus-owned terms.

#### Scenario: Catalog adoption rejects product tools

r[agent_harness_runtime.clankers_tool_catalog.scope.rejects_product]
- GIVEN maintainers evaluate Clankers tool catalogs
- WHEN the adoption decision is documented
- THEN Animus distinguishes descriptor/capability patterns from Clankers product tools and plugin runtimes
- AND it does not import Clankers built-in tools as Animus runtime dependencies.

### Requirement: Optional catalog descriptor schema

r[agent_harness_runtime.clankers_tool_catalog.schema] An Animus tool catalog MAY define descriptors for tool name, description, runtime kind, capability classes, side-effect level, approval policy, input/output schema references, and redaction policy.

#### Scenario: Descriptor schema is explicit

r[agent_harness_runtime.clankers_tool_catalog.schema.explicit]
- GIVEN a tool descriptor is maintained by a host or repo
- WHEN catalog validation reads it
- THEN the descriptor names capabilities, side-effect level, approval behavior, schema references, and redaction behavior
- AND missing required fields fail validation.

### Requirement: Catalog validation fails unsafe descriptors

r[agent_harness_runtime.clankers_tool_catalog.validation] Catalog validation MUST reject duplicate names, built-in collisions, unknown runtime kinds, invalid schemas, dangerous capabilities without approval, and secret-adjacent capabilities without redaction.

#### Scenario: Unsafe catalog fixtures fail

r[agent_harness_runtime.clankers_tool_catalog.fixtures]
- GIVEN catalog fixtures include duplicate names, invalid schemas, unknown runtime kinds, unsafe capabilities, missing approval, or missing redaction
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors still normalize deterministically.

### Requirement: Catalog parsing is runtime-neutral

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral] Catalog parsing and validation MUST NOT execute product tools, start plugins, load providers, read secrets, inherit environment, or open network connections.

#### Scenario: Runtime-neutral rail catches side effects

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral.rail]
- GIVEN a catalog manifest is parsed for validation
- WHEN the runtime-neutrality rail runs
- THEN it proves validation is data-only
- AND no tool execution, plugin launch, provider startup, secret read, environment inheritance, or network access is required.

#### Scenario: Tool catalog closeout is evidenced

r[agent_harness_runtime.clankers_tool_catalog.closeout]
- GIVEN the tool catalog evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes adoption-scope docs, manifest validation fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
