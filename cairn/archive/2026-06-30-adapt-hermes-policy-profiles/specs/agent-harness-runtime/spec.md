# agent-harness-runtime Change Spec: Hermes policy profile adaptation

## Requirements

### Requirement: Typed policy/profile descriptors

r[agent_harness_runtime.hermes_policy_profiles.descriptor] Animus MUST define typed policy/profile descriptors for host-supplied operating guidance with policy kind, source scope, trust status, priority, goal text, success criteria, constraints, validation rules, stop rules, provenance, and optional prompt snippets.

#### Scenario: Valid policy profile composes into prompt input

r[agent_harness_runtime.hermes_policy_profiles.descriptor.valid]
- GIVEN a trusted host supplies a policy profile with goal, success criteria, constraints, validation rules, stop rules, provenance, and priority
- WHEN prompt assembly prepares policy context
- THEN the profile contributes an attributed bounded prompt segment
- AND the descriptor remains available as structured metadata for runtime/report evidence.

### Requirement: Deterministic policy precedence and conflict diagnostics

r[agent_harness_runtime.hermes_policy_profiles.precedence] Animus MUST compose multiple policy descriptors in deterministic precedence order and report conflicts between side-effect limits, validation requirements, or stop rules.

#### Scenario: Global safety rule defeats project override

r[agent_harness_runtime.hermes_policy_profiles.precedence.safety_wins]
- GIVEN a trusted global policy forbids a destructive action without explicit approval and a project-local policy attempts to allow that action implicitly
- WHEN policy composition runs
- THEN the global safety constraint remains active
- AND diagnostics identify the overridden project-local rule without embedding unsafe project prompt text.

### Requirement: Policy prompts are outcome-first and bounded

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene] Policy-derived prompt text MUST prioritize goal, success criteria, constraints, required output shape, validation, and stop rules, and MUST be bounded to prevent context bloat.

#### Scenario: Process-heavy policy is rejected or warned

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene.process_heavy]
- GIVEN a policy descriptor contains a long process script with no observable success criteria or stop rule
- WHEN policy validation runs
- THEN validation rejects it or emits a prompt-hygiene diagnostic according to policy severity
- AND prompt assembly does not silently include unbounded process-heavy text.

### Requirement: Project-local policy is trust-gated

r[agent_harness_runtime.hermes_policy_profiles.project_trust] Project-local policy descriptors MUST NOT influence prompts, tool activation, command expansion, or runtime guardrails until host trust permits them.

#### Scenario: Untrusted project policy is omitted

r[agent_harness_runtime.hermes_policy_profiles.project_trust.omit]
- GIVEN a project-local policy descriptor is marked untrusted
- WHEN policy context is assembled
- THEN the descriptor is omitted or marked unavailable according to host policy
- AND its constraints and prompt snippets do not override trusted global policy.

### Requirement: Policy descriptors do not replace enforcement

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary] Animus MUST treat policy descriptors as guidance and configuration input, not as the sole enforcement mechanism for capability denial, read-only mode, mutation serialization, or credential redaction.

#### Scenario: Read-only policy also has runtime guard

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary.readonly]
- GIVEN policy context says the session is read-only
- WHEN a tool call requests filesystem mutation
- THEN runtime enforcement denies or blocks the mutation through a typed capability boundary
- AND the denial does not rely only on model obedience to prompt text.

### Requirement: Policy profile adaptation verification

r[agent_harness_runtime.hermes_policy_profiles.verification] The policy profile adaptation MUST include focused tests for valid composition, conflict precedence, prompt hygiene, trust gating, enforcement-boundary integration, prompt budget bounds, stale model guidance rejection, and safe diagnostics.

#### Scenario: Closeout evidence covers policy composition

r[agent_harness_runtime.hermes_policy_profiles.verification.closeout]
- GIVEN maintainers intend to archive the Hermes policy profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused policy composition tests, prompt-hygiene tests, trust-boundary tests, enforcement-boundary tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
