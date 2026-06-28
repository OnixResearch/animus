# agent-harness-runtime Change Spec: Clankers Steel runtime policy

## Requirements

### Requirement: Nickel-authored Steel runtime policy

r[agent_harness_runtime.clankers_steel_policy.nickel] Animus Steel runtime profiles MUST be authored in Nickel with typed contracts for profile identity, script bindings, BLAKE3 script hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction.

#### Scenario: Policy contract accepts valid profiles

r[agent_harness_runtime.clankers_steel_policy.nickel.valid]
- GIVEN a maintained Steel runtime profile under `policy/steel-runtime/`
- WHEN Nickel validation runs
- THEN valid profile identity, script binding, hash, budget, host-action, fallback, and receipt-redaction fields pass
- AND the policy remains reviewable before runtime use.

### Requirement: Checked export and invalid fixtures

r[agent_harness_runtime.clankers_steel_policy.export] Rust runtime tests MUST consume checked policy data or typed fixtures and MUST NOT evaluate Nickel during turn execution.

#### Scenario: Policy export drift fails

r[agent_harness_runtime.clankers_steel_policy.export.drift]
- GIVEN generated policy data exists for Rust consumption
- WHEN the drift check runs
- THEN it fails if generated data differs from Nickel source
- AND it reports a deterministic update path.

#### Scenario: Invalid policies fail before activation

r[agent_harness_runtime.clankers_steel_policy.invalid_fixtures]
- GIVEN a policy fixture has a missing script, bad name, non-positive budget, unknown host action, hash mismatch, path escape, or unsupported fallback mode
- WHEN policy validation runs
- THEN validation returns a typed policy error
- AND Steel is not loaded or evaluated.

### Requirement: Policy enforcement before Steel evaluation

r[agent_harness_runtime.clankers_steel_policy.enforced] Animus MUST validate selected profile, script hash, runtime budget, action allowlist, fallback mode, and receipt redaction before Steel loop evaluation can influence model or tool dispatch.

#### Scenario: Policy denial blocks adapter calls

r[agent_harness_runtime.clankers_steel_policy.enforced.blocks]
- GIVEN a runtime turn selects a Steel script that fails policy activation
- WHEN the runtime prepares the turn
- THEN it returns a typed policy-denied failure
- AND no model or tool adapter call is made.

### Requirement: Redacted policy receipts and rails

r[agent_harness_runtime.clankers_steel_policy.receipts] Steel policy activation receipts MUST expose safe status, profile labels, script hash, policy hash, denied reason class, fallback decision, and receipt hash only.

#### Scenario: Receipts omit raw sensitive material

r[agent_harness_runtime.clankers_steel_policy.receipts.redacted]
- GIVEN activation observes prompt text, script source, provider payloads, credentials, or tool bodies
- WHEN a receipt is rendered
- THEN those values are omitted or represented by redaction markers and BLAKE3 hashes
- AND the receipt remains usable as review evidence.

#### Scenario: Focused rails gate closeout

r[agent_harness_runtime.clankers_steel_policy.rails]
- GIVEN maintainers claim Steel runtime policy is adopted
- WHEN focused policy drift and activation rails run
- THEN they cover valid policy, invalid fixtures, enforcement-before-evaluation, and receipt redaction
- AND closeout evidence names the successful rails or explicit blockers.

#### Scenario: Steel policy closeout is evidenced

r[agent_harness_runtime.clankers_steel_policy.closeout]
- GIVEN the Steel policy change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused policy rails, Steel tests, Nickel/config drift checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
