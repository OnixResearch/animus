# agent-harness-runtime Change Spec: Zerostack prompt mode profile adaptation

## Requirements

### Requirement: Typed prompt profile descriptors

r[agent_harness_runtime.zerostack_prompt_profiles.descriptors] Animus MUST represent prompt profiles as typed descriptors with profile ID, purpose, action posture, required capabilities, source identity, trust scope, precedence, body text, and prompt-inclusion policy.

#### Scenario: Untrusted profile is not silently injected

r[agent_harness_runtime.zerostack_prompt_profiles.descriptors.untrusted]
- GIVEN a prompt profile comes from an untrusted source or lacks source identity
- WHEN prompt assembly evaluates the profile
- THEN the profile is rejected or withheld pending host admission according to policy
- AND its body text is not silently injected into model-visible context.

### Requirement: Prompt directives are parsed metadata

r[agent_harness_runtime.zerostack_prompt_profiles.directives] Prompt directives such as mode or chain hints MUST be parsed into typed metadata before prompt assembly and MUST NOT remain hidden control syntax in model-visible profile body text.

#### Scenario: Unknown mode directive fails closed

r[agent_harness_runtime.zerostack_prompt_profiles.directives.invalid]
- GIVEN a profile body contains a directive with an unknown mode, malformed syntax, unsupported chain target, or duplicate exclusive directive
- WHEN directive parsing runs
- THEN parsing returns typed diagnostics
- AND the directive cannot change runtime mode or continuation state.

### Requirement: Profile-requested modes are policy inputs

r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding] Profile-requested permission modes MUST flow through permission policy and MUST NOT grant authority, suppress required approvals, or weaken sandbox constraints.

#### Scenario: Code profile cannot grant write authority

r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding.no_grant]
- GIVEN a code profile requests a write-capable posture
- AND authority or policy denies the target write action
- WHEN the runtime resolves the write request
- THEN the write remains denied or ask-gated according to permission policy
- AND the profile request is recorded only as posture metadata.

### Requirement: Chain-of-prompts continuation plans

r[agent_harness_runtime.zerostack_prompt_profiles.chain] Animus SHOULD support chain-of-prompts continuation plans that declare source profile, target profile, transition condition, user or host decision, and bounded additional instruction text.

#### Scenario: Chain transition is declined

r[agent_harness_runtime.zerostack_prompt_profiles.chain.declined]
- GIVEN a prompt profile has a configured transition to another profile
- WHEN the user or host declines the transition
- THEN the active profile does not change
- AND the declined transition is recorded as safe metadata rather than model-visible instruction.

### Requirement: Prompt profile receipts

r[agent_harness_runtime.zerostack_prompt_profiles.receipts] Prompt profile assembly SHOULD emit bounded receipts naming profile IDs, source identities, directive outcomes, chain decisions, prompt section hashes, and omitted-profile reasons without exposing raw secrets or unbounded profile bodies.

#### Scenario: Receipt uses hashes for sensitive profile body

r[agent_harness_runtime.zerostack_prompt_profiles.receipts.hashes]
- GIVEN a profile body is not safe to include in durable evidence
- WHEN a prompt profile receipt is emitted
- THEN the body is represented by a BLAKE3 hash and redaction marker
- AND source identity, profile ID, and decision reason remain reviewable.
