# agent-harness-runtime Change Spec: Ornith trust-boundary monitor adaptation

## Requirements

### Requirement: Immutable outer trust boundary

r[agent_harness_runtime.ornith_trust_boundary.outer_boundary] Animus MUST represent the outer trust boundary as immutable typed data covering sanctioned tools, withheld path labels, verifier artifact identity, sandbox profile identity, approval requirements, evidence policy, and BLAKE3 policy hashes before scaffold, Steel, model, or tool execution can claim success.

#### Scenario: Invalid boundary descriptor fails closed

r[agent_harness_runtime.ornith_trust_boundary.outer_boundary.invalid]
- GIVEN a trust-boundary descriptor has a missing verifier hash, duplicate path label, empty sanctioned-tool set, raw sensitive path serialization, or a request to mutate boundary authority from inside the runtime loop
- WHEN boundary validation runs
- THEN validation returns a typed trust-boundary error
- AND no scaffold, model, tool, verifier, or evidence acceptance path is invoked.

### Requirement: Deterministic boundary monitor

r[agent_harness_runtime.ornith_trust_boundary.monitor] Animus MUST provide a deterministic monitor that evaluates planned and observed action descriptors against the trust boundary and returns typed issue codes for exact violations before evidence is accepted.

#### Scenario: Boundary violations deny trajectory evidence

r[agent_harness_runtime.ornith_trust_boundary.monitor.violations]
- GIVEN an action descriptor attempts a withheld-path read, verifier modification, unsanctioned tool call, sandbox weakening, approval suppression, or evidence-policy bypass
- WHEN the deterministic monitor evaluates the action
- THEN the monitor returns the matching violation issue code
- AND the trajectory is not accepted as successful runtime, batch, or evaluation evidence.

### Requirement: Judge veto cannot grant authority

r[agent_harness_runtime.ornith_trust_boundary.judge_veto] Animus MAY accept a host-provided judge veto for suspicious intent, but judge output MUST be reject-only and MUST NOT override deterministic monitor denials, grant permissions, mutate boundary descriptors, suppress approvals, or mark denied evidence successful.

#### Scenario: Judge approval cannot override monitor denial

r[agent_harness_runtime.ornith_trust_boundary.judge_veto.no_override]
- GIVEN the deterministic monitor denies a trajectory for a trust-boundary violation
- AND a judge output labels the trajectory acceptable
- WHEN policy binding combines monitor and judge outcomes
- THEN the denial remains final
- AND the combined outcome records that judge output cannot grant authority.

### Requirement: Anti-gaming fixture coverage

r[agent_harness_runtime.ornith_trust_boundary.anti_gaming] Animus MUST maintain explicit anti-gaming fixture classes for oracle copying, hardcoded expected outputs or artifacts, visible-test overfitting, verifier edits, and allowed-tool intent gaming.

#### Scenario: Anti-gaming fixtures are reproducible

r[agent_harness_runtime.ornith_trust_boundary.anti_gaming.fixtures]
- GIVEN anti-gaming fixtures contain both allowed work patterns and gaming attempts
- WHEN focused monitor validation runs
- THEN allowed patterns pass with safe evidence
- AND gaming attempts fail with deterministic issue codes without depending on hidden provider calls.

### Requirement: Redacted boundary violation receipts

r[agent_harness_runtime.ornith_trust_boundary.receipts] Trust-boundary violation receipts MUST expose safe metadata such as policy hash, issue code, sanitized labels, counts, and redaction status, and MUST NOT include hidden paths, raw verifier bodies, prompts, tool inputs, tool outputs, provider payloads, credentials, or full path dumps.

#### Scenario: Boundary receipt rejects leaks

r[agent_harness_runtime.ornith_trust_boundary.receipts.redacted]
- GIVEN a boundary violation receipt fixture contains hidden paths, raw verifier bodies, prompt text, tool input/output bodies, provider payloads, credential-looking values, or full path dumps
- WHEN receipt validation runs
- THEN validation fails with a typed redaction diagnostic
- AND the receipt is not accepted as review or release evidence.

### Requirement: Trust-boundary monitor verification

r[agent_harness_runtime.ornith_trust_boundary.verification] The trust-boundary monitor adaptation MUST include focused positive and negative tests for boundary validation, monitor denials, judge-veto binding, anti-gaming fixtures, receipt redaction, workspace checks, roadmap validation, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers boundary monitor behavior

r[agent_harness_runtime.ornith_trust_boundary.verification.closeout]
- GIVEN maintainers intend to archive the trust-boundary monitor adaptation
- WHEN closeout validation runs
- THEN evidence names focused monitor, judge-veto, anti-gaming, receipt, workspace, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
