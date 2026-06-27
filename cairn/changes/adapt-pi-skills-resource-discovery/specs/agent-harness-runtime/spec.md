# agent-harness-runtime Change Spec: Pi skills and resource-discovery adaptation

## Requirements

### Requirement: Typed resource descriptors

r[agent_harness_runtime.pi_resources.descriptors] Animus MUST define typed descriptors for skills, prompt templates, context guidance, and host-provided resources with name, description, kind, source scope, provenance metadata, trust status, and safe content identity.

#### Scenario: Descriptor validation rejects missing descriptions

r[agent_harness_runtime.pi_resources.descriptors.validate]
- GIVEN a skill descriptor lacks a description
- WHEN resource descriptors are validated for prompt availability
- THEN validation fails with a typed descriptor error
- AND the resource is not advertised to the model.

### Requirement: Host-owned discovery boundary

r[agent_harness_runtime.pi_resources.host_discovery] Animus runtime MUST consume host-provided resource descriptors and MUST NOT scan project directories, install packages, execute package managers, or load extension code as part of core resource discovery.

#### Scenario: Runtime receives descriptors only

r[agent_harness_runtime.pi_resources.host_discovery.no_scan]
- GIVEN a host supplies an explicit list of resource descriptors
- WHEN runtime builds prompt resource context
- THEN it uses only those descriptors
- AND it performs no filesystem traversal, package installation, or dynamic code loading.

### Requirement: Project trust gating

r[agent_harness_runtime.pi_resources.project_trust] Project-local dynamic resources MUST be unavailable for prompt influence, command expansion, or tool activation until a host-supplied project trust decision permits them.

#### Scenario: Untrusted project resource is ignored

r[agent_harness_runtime.pi_resources.project_trust.denied]
- GIVEN a project-local skill descriptor is marked untrusted
- WHEN prompt construction enumerates available skills
- THEN the skill is omitted or marked unavailable according to policy
- AND its content is not loaded into model context.

### Requirement: Progressive skill disclosure

r[agent_harness_runtime.pi_resources.skill_disclosure] Skills SHOULD use progressive disclosure where only validated name and description are advertised by default, and full skill content is loaded only by explicit command, host policy, or approved matching decision.

#### Scenario: Full skill loads on explicit command

r[agent_harness_runtime.pi_resources.skill_disclosure.command]
- GIVEN a trusted skill descriptor is advertised by name and description
- WHEN an explicit skill-load command selects that skill
- THEN the host-provided content for that skill is added to the next prompt context
- AND unrelated skill bodies remain out of context.

### Requirement: Deterministic resource command and collision handling

r[agent_harness_runtime.pi_resources.commands] Resource-backed commands and names MUST resolve deterministically and report duplicate names, unsupported resource kinds, invalid names, and unsafe content handles through typed diagnostics.

#### Scenario: Duplicate skill names produce diagnostic

r[agent_harness_runtime.pi_resources.commands.duplicate]
- GIVEN two trusted skill descriptors have the same command name
- WHEN command/resource lookup is built
- THEN policy selects a deterministic winner or disables the duplicate name
- AND a diagnostic identifies the collision without exposing unsafe host paths.
