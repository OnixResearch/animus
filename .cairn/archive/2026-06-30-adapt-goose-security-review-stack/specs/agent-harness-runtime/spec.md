# agent-harness-runtime Change Spec: Goose security review stack adaptation

## Requirements

### Requirement: Security review inputs are bounded and redacted

r[agent_harness_runtime.goose_security_review_stack.review_input] Animus MUST define bounded security review inputs for original task labels, recent context summaries, tool call descriptor, action class, active policy labels, and redacted evidence identities.

#### Scenario: Review input omits sensitive raw material

r[agent_harness_runtime.goose_security_review_stack.review_input.redacted]
- GIVEN a tool call includes raw arguments, paths, credentials, provider text, or prompt content
- WHEN security review input is constructed
- THEN the input exposes only safe labels, bounded summaries, and BLAKE3 evidence hashes
- AND raw prompts, credentials, full paths, and unbounded tool arguments are omitted.

### Requirement: Deterministic pattern findings are available

r[agent_harness_runtime.goose_security_review_stack.pattern_findings] Animus MUST provide deterministic scanner findings for prompt injection, destructive actions, exfiltration, privilege escalation, obfuscated remote execution, and out-of-scope mutation attempts.

#### Scenario: Dangerous payloads produce findings

r[agent_harness_runtime.goose_security_review_stack.pattern_findings.detects]
- GIVEN a requested tool call attempts hidden instruction following, credential access, unknown-domain upload, recursive deletion, privilege escalation, or download-and-execute behavior
- WHEN deterministic security scanning runs
- THEN scanning returns one or more findings with category, confidence, finding ID, and safe explanation
- AND the finding evidence does not include the raw secret or full sensitive payload.

### Requirement: Classifier and adversary reviews are advisory

r[agent_harness_runtime.goose_security_review_stack.advisory_review] Optional classifier and adversary reviewer outputs MUST be modeled as advisory allow, warn, block, unavailable, timeout, or malformed outcomes that cannot grant missing authority or weaken active policy.

#### Scenario: Reviewer allow cannot bypass authority

r[agent_harness_runtime.goose_security_review_stack.advisory_review.no_authority]
- GIVEN active capability or sandbox policy denies a tool call
- WHEN an adversary reviewer recommends allowing the call
- THEN permission resolution preserves the denial
- AND no provider, tool, host-effect, or confirmation bypass is executed.

### Requirement: Security review failure behavior is policy controlled

r[agent_harness_runtime.goose_security_review_stack.fail_behavior] Security review outages, timeouts, and malformed results MUST resolve through explicit policy-controlled fail behavior by action risk class.

#### Scenario: High-risk review outage fails closed

r[agent_harness_runtime.goose_security_review_stack.fail_behavior.closed]
- GIVEN policy marks network exfiltration or destructive filesystem mutation as high risk
- WHEN the configured adversary reviewer times out before evaluating that action
- THEN runtime records a review-unavailable denial or required confirmation according to policy
- AND the risky action is not silently allowed by default.

### Requirement: Findings bind through permission and receipt policy

r[agent_harness_runtime.goose_security_review_stack.policy_binding] Security findings MUST flow through permission resolution, confirmation prompts, sandbox descriptors, and safe receipts before any reviewed effect is dispatched.

#### Scenario: Blocked finding stops dispatch

r[agent_harness_runtime.goose_security_review_stack.policy_binding.blocks]
- GIVEN a deterministic scanner or admitted advisory reviewer produces a blocking finding for a tool call
- WHEN runtime evaluates the tool call
- THEN dispatch stops before provider, tool, or host-effect execution
- AND the denial receipt contains only safe finding metadata and redacted evidence identities.

### Requirement: Security review closeout is evidenced

r[agent_harness_runtime.goose_security_review_stack.verification] Security review stack adoption MUST include positive benign-review tests, negative prompt-injection and dangerous-action fixtures, advisory-authority bypass tests, fail-behavior tests, receipt redaction tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Security evidence is complete

- GIVEN the Goose security review stack change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused scanner tests, advisory-review tests, fail-behavior tests, policy-binding tests, receipt redaction tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
