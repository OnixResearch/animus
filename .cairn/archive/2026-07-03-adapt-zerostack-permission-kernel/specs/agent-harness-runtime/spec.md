# agent-harness-runtime Change Spec: Zerostack permission kernel adaptation

## Requirements

### Requirement: Permission modes are advisory defaults

r[agent_harness_runtime.zerostack_permission_kernel.modes] Animus MUST define typed permission modes for restrictive, read-only, guarded, standard, and yolo-style UX defaults, and those modes MUST remain advisory beneath authority, capability, sandbox, and redaction checks.

#### Scenario: Yolo mode cannot bypass authority

r[agent_harness_runtime.zerostack_permission_kernel.modes.authority]
- GIVEN a guarded action lacks required authority or sandbox allowance
- AND the active permission mode would locally allow the action
- WHEN permission resolution runs
- THEN the action is denied before adapter dispatch
- AND the receipt separates local mode outcome from missing authority.

### Requirement: Pattern rule descriptors

r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules] Permission rule descriptors MUST support glob-like and regex-like match kinds over action class and resource label with explicit source identity, trust scope, precedence, and allow, ask, or deny outcome.

#### Scenario: Malformed pattern cannot authorize

r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules.invalid]
- GIVEN a rule descriptor has an unknown match kind, unsupported outcome, missing action class, invalid regex, or absent source identity
- WHEN rule validation runs
- THEN validation returns typed diagnostics
- AND the rule cannot authorize, prompt for, or deny an action.

### Requirement: Scoped session allowlists

r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist] Session allowlist entries MUST be scoped to action identity, redacted resource identity or hash, policy identity, session identity, and invalidation metadata before they can satisfy an ask decision.

#### Scenario: Stale allowlist entry is rejected

r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist.stale]
- GIVEN an allowlist entry was created for a different action, resource, policy hash, or session
- WHEN permission resolution evaluates the entry
- THEN the entry is rejected as stale or mismatched
- AND the action remains unexecuted until another valid decision is provided.

### Requirement: Repeated action detection

r[agent_harness_runtime.zerostack_permission_kernel.repeated_action] Animus SHOULD detect repeated identical guarded action identities and return policy-selected outcomes for coach, ask, deny, or ignore without directly dispatching the repeated action first.

#### Scenario: Repeated destructive action is denied

r[agent_harness_runtime.zerostack_permission_kernel.repeated_action.deny]
- GIVEN policy marks an action class as destructive when repeated with the same resource identity
- AND the repeated action threshold has been reached
- WHEN permission resolution runs for the repeated action
- THEN the resolver returns a denied repeated-action outcome
- AND no filesystem, process, network, model, or tool adapter is called.

### Requirement: Safe permission receipts

r[agent_harness_runtime.zerostack_permission_kernel.receipts] Permission receipts MUST record bounded metadata for action class, redacted resource identity or BLAKE3 resource hash, matched rule identity, mode outcome, authority outcome, repeated-action status, prompt-decision status, and final decision.

#### Scenario: Receipt omits sensitive request material

r[agent_harness_runtime.zerostack_permission_kernel.receipts.redacted]
- GIVEN a permission request contains a prompt, credential-looking value, provider payload, full host path, or raw tool body
- WHEN a permission receipt is produced
- THEN the sensitive value is omitted or represented only by an approved redaction marker or BLAKE3 hash
- AND the receipt still includes enough typed reason codes to audit the decision.
