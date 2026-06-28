# agent-harness-runtime Change Spec: OpenAI Codex hook trust lifecycle adaptation

## Requirements

### Requirement: Typed lifecycle hook registry

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry] Animus MUST expose typed lifecycle hook events for session, prompt, tool, permission, compaction, subagent, and stop boundaries without requiring concrete hook execution in the runtime core.

#### Scenario: Hook event routes to matching definitions

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry.routing]
- GIVEN a trusted hook definition matches a pre-tool-use event
- WHEN runtime emits that lifecycle event
- THEN the shell receives a typed hook request for the matching definition
- AND definitions for unrelated event kinds are not selected.

### Requirement: Content-addressed hook trust

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity] Hook definitions MUST carry stable identities and BLAKE3 content hashes so trust decisions apply to the reviewed definition content.

#### Scenario: Changed hook definition loses prior trust

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity.changed]
- GIVEN a user-trusted hook definition is edited after review
- WHEN Animus computes the hook definition hash for a later run
- THEN the old trust record does not authorize the changed definition
- AND the hook is skipped or blocked according to policy until reviewed again.

### Requirement: Event-scoped hook input and output contracts

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract] Hook inputs and outputs MUST be validated against the event-specific contract before hook effects are applied.

#### Scenario: Unsupported hook effect is rejected

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract.unsupported]
- GIVEN a prompt-submit hook returns an output field reserved for tool-input rewriting
- WHEN Animus validates the hook output
- THEN the unsupported effect is rejected for that event
- AND the runtime records a malformed hook-output failure without applying the effect.

### Requirement: Hooks cannot bypass policy boundaries

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding] Hook decisions MUST flow through runtime policy and MUST NOT directly invoke adapters, weaken sandbox policy, or suppress required approvals.

#### Scenario: Hook cannot override a denied action

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding.bypass]
- GIVEN exec policy forbids a command outside the active sandbox
- WHEN a hook attempts to allow or rewrite that denied command
- THEN the command remains denied
- AND the runtime records the hook attempt as policy-bounded context rather than executing the command.

### Requirement: Hook outcome reporting

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting] Runtime reports MUST include hook outcome events for skipped, untrusted, failed, denied, malformed, unsupported, and applied hook results with redacted inputs and auditable hook identities.

#### Scenario: Hook report redacts sensitive input

r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting.redacted]
- GIVEN a hook input contains prompt or tool data marked sensitive by redaction policy
- WHEN runtime records the hook outcome
- THEN the report includes hook identity, event kind, trust status, and outcome class
- AND the sensitive input body is omitted or replaced by approved redaction metadata.
