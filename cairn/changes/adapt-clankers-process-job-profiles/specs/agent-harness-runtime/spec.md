# agent-harness-runtime Change Spec: Clankers process-job profile adaptation

## Requirements

### Requirement: Nickel-authored process-job profiles

r[agent_harness_runtime.clankers_process_job_profiles.nickel] Animus process-job profile schemas and policy MUST be authored as Nickel contracts or checked fixtures covering profile schema, backend choices, command/program shape, args, cwd, writable paths, environment allowlist, timeout/resource bounds, notification policy, and safe metadata.

#### Scenario: Generated profile policy stays in sync

r[agent_harness_runtime.clankers_process_job_profiles.nickel.drift]
- GIVEN a maintained process-job profile Nickel source and generated runtime fixture
- WHEN drift validation runs
- THEN generated data matches the Nickel source
- AND schema-invalid profiles fail before runtime dispatch.

### Requirement: Pure profile resolution

r[agent_harness_runtime.clankers_process_job_profiles.resolution] Profile resolution MUST be pure over loaded profile sources and policy, producing backend-neutral queue start requests plus safe identity evidence without spawning processes, reading the filesystem, contacting queue backends, or writing storage.

#### Scenario: Valid profile resolves without side effects

r[agent_harness_runtime.clankers_process_job_profiles.resolution.valid]
- GIVEN loaded profile sources and policy contain a valid profile
- WHEN the resolver selects that profile
- THEN it returns a backend-neutral start request and safe identity envelope
- AND no shell, pueue, systemd, filesystem, or storage operation is invoked.

### Requirement: Deterministic source precedence

r[agent_harness_runtime.clankers_process_job_profiles.precedence] Profile source precedence MUST be deterministic and MUST reject duplicate profile names at the same precedence level.

#### Scenario: Duplicate workspace profiles fail closed

r[agent_harness_runtime.clankers_process_job_profiles.precedence.duplicate]
- GIVEN two workspace-level sources define the same profile name
- WHEN profile resolution runs
- THEN resolution fails with a duplicate-profile diagnostic
- AND no arbitrary source order is used to choose a winner.

### Requirement: Profile policy validation

r[agent_harness_runtime.clankers_process_job_profiles.policy] Profile validation MUST reject secret-like environment keys, env keys outside allowed prefixes, cwd or writable-path escapes, unsupported backends, conflicting command/program fields, invalid args, and timeout/resource-limit violations before dispatch.

#### Scenario: Secret environment key rejected

r[agent_harness_runtime.clankers_process_job_profiles.policy.secret_env]
- GIVEN a profile declares an environment key that looks like a token, password, key, or secret
- WHEN profile validation runs
- THEN validation rejects the profile with a secret-environment diagnostic
- AND no backend-neutral start request is produced.

### Requirement: Safe profile identity receipts

r[agent_harness_runtime.clankers_process_job_profiles.identity] Resolved profile identity metadata MUST include schema version, source label, policy hash, profile name, selected backend, and BLAKE3 profile/content identities, and MUST NOT include raw environment values, credentials, full path dumps, or backend-specific secret locators.

#### Scenario: Identity metadata rejects raw env value

r[agent_harness_runtime.clankers_process_job_profiles.identity.rejects_env]
- GIVEN a profile identity fixture includes a raw environment value or credential-looking backend locator
- WHEN identity validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Process-job profile verification

r[agent_harness_runtime.clankers_process_job_profiles.verification] The adaptation MUST include focused positive and negative tests for Nickel policy drift, valid profile resolution, duplicate names, invalid command shapes, secret env keys, path escapes, unsupported backends, resource-limit violations, identity envelopes, and leak rejection.

#### Scenario: Closeout evidence covers profile fixtures

r[agent_harness_runtime.clankers_process_job_profiles.verification.closeout]
- GIVEN maintainers intend to archive the process-job profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused Nickel, resolver, precedence, policy, identity, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
