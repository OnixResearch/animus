# agent-harness-runtime Change Spec: Hermes skill manifest adaptation

## Requirements

### Requirement: Hermes-style skill manifest descriptors

r[agent_harness_runtime.hermes_skill_manifest.descriptor] Animus MUST define typed skill manifest descriptors for host-provided workflow guidance with validated name, description, version, source category, tags, related skill names, provenance, trust status, and safe content identity.

#### Scenario: Valid skill manifest is accepted

r[agent_harness_runtime.hermes_skill_manifest.descriptor.valid]
- GIVEN a host supplies a Hermes-style skill manifest with a valid name, description, version, category, tags, related skill names, body identity, provenance, and trust status
- WHEN Animus validates descriptors for resource advertisement
- THEN the manifest is accepted as a skill resource descriptor
- AND the default model-visible summary contains only the validated name, description, category, tags, and safe provenance label.

### Requirement: Manifest validation fails closed

r[agent_harness_runtime.hermes_skill_manifest.validation] Animus MUST reject malformed skill manifests before prompt construction, command lookup, or full-content loading.

#### Scenario: Invalid manifest fields are rejected

r[agent_harness_runtime.hermes_skill_manifest.validation.invalid]
- GIVEN a skill manifest has an empty name, missing description, overlong description, unsupported version shape, duplicate tags, missing body identity, malformed related skill name, or unsafe content handle
- WHEN manifest validation runs
- THEN validation returns typed diagnostics naming every failed invariant
- AND the invalid manifest is not advertised or loaded.

### Requirement: Skill manifests map into progressive disclosure

r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure] Hermes-style skill manifests MUST integrate with Animus progressive disclosure so summary metadata can be advertised independently from the full skill body.

#### Scenario: Explicit load includes only selected body

r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure.selected]
- GIVEN multiple trusted skill manifests are advertised by summary
- WHEN a host-approved full-content load selects one skill by name
- THEN only that skill body is made available to prompt construction
- AND unrelated skill bodies, unresolved related skills, and untrusted project-local bodies remain out of model context.

### Requirement: Hermes runtime layout remains host policy

r[agent_harness_runtime.hermes_skill_manifest.layout_boundary] Animus MUST NOT require Hermes local directories, symlink conventions, runtime caches, user-local categories, or plugin installation paths in reusable runtime crates.

#### Scenario: Host supplies manifests from a non-Hermes store

r[agent_harness_runtime.hermes_skill_manifest.layout_boundary.host_store]
- GIVEN a host extracts equivalent skill manifest data from an application database, embedded bundle, or non-Hermes directory
- WHEN it passes validated manifest descriptors to Animus
- THEN runtime behavior matches descriptors parsed from Hermes-style `SKILL.md` files
- AND no reusable Animus crate attempts to inspect `~/.hermes`, `~/.pi`, or repository-local skill directories.

### Requirement: Manifest adaptation verification

r[agent_harness_runtime.hermes_skill_manifest.verification] The Hermes skill manifest adaptation MUST include positive and negative tests for valid manifests, malformed manifest rejection, progressive disclosure, untrusted-body omission, related-skill diagnostics, duplicate-name handling, and host-layout isolation.

#### Scenario: Closeout evidence covers manifest fixtures

r[agent_harness_runtime.hermes_skill_manifest.verification.closeout]
- GIVEN maintainers intend to archive the Hermes skill manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest fixture tests, prompt disclosure tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
