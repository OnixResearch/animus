# agent-harness-runtime Change Spec: OpenAI Codex exec policy rules adaptation

## Requirements

### Requirement: Typed exec policy contract

r[agent_harness_runtime.openai_codex_execpolicy_rules.policy_contract] Animus exec policy MUST represent command intents, sandbox context, rule identities, rationales, decisions, and validation examples as typed data before runtime evaluation.

#### Scenario: Policy rule carries validation examples

r[agent_harness_runtime.openai_codex_execpolicy_rules.nickel_source]
- GIVEN a policy rule with a command prefix, rationale, positive examples, and negative examples
- WHEN Animus validates the authored policy
- THEN every positive example matches the rule
- AND every negative example does not match the rule.

### Requirement: Deterministic prefix-rule evaluation

r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation] Exec policy evaluation MUST be a deterministic pure function over parsed command intent and sandbox context, returning the most restrictive matching decision.

#### Scenario: Most restrictive matching decision wins

r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation.examples]
- GIVEN one matching allow rule and one matching forbid rule for the same command intent
- WHEN the policy evaluator computes the decision
- THEN the result is forbid
- AND the matched-rule evidence includes both contributing rules.

### Requirement: Conservative shell-wrapper handling

r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting] Shell-wrapper analysis MUST split only simple command chains made from plain command words and safe chain operators, and MUST treat unsupported shell constructs as opaque commands requiring conservative policy handling.

#### Scenario: Destructive command cannot hide behind allowed prefix

r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting.smuggling]
- GIVEN a shell wrapper contains an allowed command followed by a forbidden destructive command in a simple chain
- WHEN Animus evaluates the wrapper
- THEN each command in the simple chain is evaluated separately
- AND the wrapper receives the forbidden decision.

### Requirement: Sandbox escalation binding

r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding] Runtime sandbox escalation handling MUST use exec policy decisions to allow, prompt, or deny command execution outside the active sandbox boundary.

#### Scenario: Policy decision is recorded without leaking command bodies

r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding.audit]
- GIVEN a command requests capability outside the active sandbox
- WHEN policy evaluation returns a prompt decision
- THEN runtime emits an escalation event with rule identity, decision, sandbox context, and redacted command evidence
- AND unbounded raw command output is not embedded in the event.
