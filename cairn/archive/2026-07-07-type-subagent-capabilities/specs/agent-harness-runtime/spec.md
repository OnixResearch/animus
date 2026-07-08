# agent-harness-runtime Change Spec: Typed subagent capabilities

## Requirements

### Requirement: Runtime uses typed subagent capabilities

r[agent_harness_runtime.subagent_typed_capabilities.types] Subagent runtime decisions MUST represent capabilities as typed values for context reads, search, named tool use, named mutation intents, workspace access, model use, and host-admitted external services rather than relying only on unstructured strings.

#### Scenario: Valid catalog label becomes typed capability

- GIVEN a capability catalog contains a valid label for a read-only search tool
- WHEN adapter capability labels are converted for runtime resolution
- THEN the label becomes a typed search or named-tool capability
- AND the original stable label remains available for receipts and diagnostics.

### Requirement: Capability intersections report denial source

r[agent_harness_runtime.subagent_typed_capabilities.intersection] Effective subagent capability resolution MUST intersect typed parent policy, recipe or profile ceilings, request ceilings, workspace scope, and host admission, and MUST report which source denied each rejected capability.

#### Scenario: Workspace scope denies mutation intent

- GIVEN parent policy and host admission allow a mutation intent but the workspace scope is read-only
- WHEN a subagent requests the mutation intent
- THEN capability resolution denies the typed capability with a workspace-scope denial
- AND the child run cannot dispatch the mutation.

### Requirement: String labels remain boundary compatible

r[agent_harness_runtime.subagent_typed_capabilities.compatibility] Animus MUST preserve stable capability string labels at adapter, fixture, policy, and receipt boundaries while requiring catalog validation before those labels influence typed runtime decisions.

#### Scenario: Receipt keeps stable capability label

- GIVEN a typed capability is accepted for a subagent run
- WHEN a safe receipt is produced
- THEN the receipt records the stable catalog label and safe capability class
- AND it does not expose internal enum debug output as the public contract.

### Requirement: Capability conversion fails closed

r[agent_harness_runtime.subagent_typed_capabilities.fail_closed] Capability conversion and resolution MUST reject unknown labels, duplicate normalized labels, wildcard authority, malformed catalog entries, unauthorized tools, unsupported capability classes, and label collisions before child-run dispatch.

#### Scenario: Wildcard label is rejected

- GIVEN a subagent request includes a capability label that normalizes to wildcard authority
- WHEN capability conversion runs
- THEN conversion returns an overbroad-authority diagnostic
- AND no typed capability is admitted for dispatch.

### Requirement: Typed capability closeout is evidenced

r[agent_harness_runtime.subagent_typed_capabilities.verification] Typed subagent capability adoption MUST include positive conversion/intersection tests, negative fail-closed tests, compatibility fixture tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Typed capability evidence is complete

- GIVEN the typed capability change is ready to archive
- WHEN closeout validation runs
- THEN evidence names catalog conversion tests, intersection tests, compatibility tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
