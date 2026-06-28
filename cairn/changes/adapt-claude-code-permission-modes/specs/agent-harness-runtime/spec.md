# agent-harness-runtime Change Spec: Claude Code permission mode adaptation

## Requirements

### Requirement: Permission modes are bounded UX caveats

r[agent_harness_runtime.claude_code_permissions.scope] Animus MUST treat Claude Code permission modes as a bounded UX reference for allow, ask, and deny outcomes, and MUST NOT treat local permission allows as authority beyond Nickel contract checks and verified UCAN grants.

#### Scenario: Local allow lacks authority

r[agent_harness_runtime.claude_code_permissions.scope.local_allow_denied]
- GIVEN local permission policy says to allow a guarded filesystem, exec, network, model, Steel, or tool action
- AND the requested action lacks matching contract and UCAN authority
- WHEN permission resolution runs
- THEN the action is denied before dispatch
- AND the receipt separates missing authority from local permission status.

### Requirement: Typed permission policy descriptors

r[agent_harness_runtime.claude_code_permissions.policy] Permission policy descriptors MUST map guarded action classes and resource labels to typed allow, ask, or deny outcomes with source identity, trust scope, precedence, and prompt-summary policy.

#### Scenario: Malformed permission policy fails closed

r[agent_harness_runtime.claude_code_permissions.policy.invalid]
- GIVEN a permission descriptor has an unknown action class, missing resource label, unsupported outcome, or missing source identity
- WHEN permission policy validation runs
- THEN validation returns typed diagnostics
- AND the malformed descriptor cannot authorize or prompt for an action.

### Requirement: Pure permission resolution

r[agent_harness_runtime.claude_code_permissions.resolution] Permission resolution MUST be a pure deterministic decision over in-memory action, authority, policy, and prompt-decision inputs that returns execute, prompt, or deny with typed reasons.

#### Scenario: Ask mode pauses before execution

r[agent_harness_runtime.claude_code_permissions.resolution.ask]
- GIVEN authority checks pass and local policy maps the action to ask
- WHEN permission resolution runs without a matching prompt-decision token
- THEN the resolver returns a prompt-required decision
- AND no model, tool, shell, filesystem, or network dispatch occurs.

### Requirement: Host-mediated prompt decisions

r[agent_harness_runtime.claude_code_permissions.prompt] Ask decisions MUST be completed only through an injected host prompt service that receives redacted action summaries and returns scoped prompt-decision tokens before dispatch can continue.

#### Scenario: Stale prompt token is rejected

r[agent_harness_runtime.claude_code_permissions.prompt.stale]
- GIVEN a prompt-decision token was issued for a different action identity, policy hash, session, or resource label
- WHEN permission resolution evaluates the token
- THEN the token is rejected as stale or mismatched
- AND the guarded action remains unexecuted.

### Requirement: Safe permission receipts

r[agent_harness_runtime.claude_code_permissions.receipts] Permission receipts MUST record safe metadata for action class, redacted resource identity or BLAKE3 resource hash, authority status, local outcome, prompt status, policy hash, and decision reasons, and MUST NOT include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs.

#### Scenario: Denied action emits redacted receipt

r[agent_harness_runtime.claude_code_permissions.receipts.denied]
- GIVEN permission resolution denies a guarded action
- WHEN the runtime emits a permission receipt
- THEN the receipt records denial reason, action class, policy hash, and redacted resource metadata
- AND sensitive request material is omitted or represented only by approved hashes and redaction markers.
