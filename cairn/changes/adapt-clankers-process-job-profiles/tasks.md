# Tasks

- [ ] [serial] Add Nickel process-job profile contracts or checked fixtures for schema, backend choices, command/program shape, args, cwd, writable paths, environment allowlist, timeout/resource bounds, notification policy, and safe metadata. r[agent_harness_runtime.clankers_process_job_profiles.nickel]
- [ ] [depends:nickel] Add generated-policy drift and invalid-schema tests for process-job profile data. r[agent_harness_runtime.clankers_process_job_profiles.nickel.drift]
- [ ] [serial] Define pure profile resolution DTOs that produce backend-neutral queue start requests and safe identity evidence from loaded sources and policy. r[agent_harness_runtime.clankers_process_job_profiles.resolution]
- [ ] [depends:resolution] Add valid profile tests proving no shell, pueue, systemd, filesystem, queue, or storage operation occurs during resolution. r[agent_harness_runtime.clankers_process_job_profiles.resolution.valid]
- [ ] [serial] Implement deterministic source precedence and duplicate-profile rejection for same-precedence sources. r[agent_harness_runtime.clankers_process_job_profiles.precedence]
- [ ] [depends:precedence] Add duplicate workspace/global/explicit source tests proving no arbitrary source ordering is used. r[agent_harness_runtime.clankers_process_job_profiles.precedence.duplicate]
- [ ] [serial] Add profile policy validation for env prefixes, secret-like env keys, cwd/writable path scopes, backend allowlists, command/program exclusivity, args shape, and resource limits. r[agent_harness_runtime.clankers_process_job_profiles.policy]
- [ ] [depends:policy] Add secret environment key, path escape, unsupported backend, invalid args, and limit violation tests. r[agent_harness_runtime.clankers_process_job_profiles.policy.secret_env]
- [ ] [serial] Define safe profile identity envelopes with schema/source/policy/profile/backend/hash metadata. r[agent_harness_runtime.clankers_process_job_profiles.identity]
- [ ] [depends:identity] Add identity leak fixtures for raw environment values, credentials, full path dumps, and backend secret locators. r[agent_harness_runtime.clankers_process_job_profiles.identity.rejects_env]
- [ ] [depends:nickel.drift,resolution.valid,precedence.duplicate,policy.secret_env,identity.rejects_env] Update docs and run focused profile tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_process_job_profiles.verification]
