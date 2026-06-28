# Tasks

- [ ] [serial] Define batch request, job, plan, run identity, output format, concurrency policy, and safe output-label DTOs. r[agent_harness_runtime.clankers_batch_runner.planning]
- [ ] [depends:planning] Add positive local planning tests and negative HTTP/HTTPS/S3/remote input-output rejection tests. r[agent_harness_runtime.clankers_batch_runner.planning.remote_rejected]
- [ ] [serial] Define injected headless/session execution service contracts and unavailable-execution failures. r[agent_harness_runtime.clankers_batch_runner.execution]
- [ ] [depends:execution] Add missing execution-service tests proving no provider, daemon, filesystem, or shell fallback occurs. r[agent_harness_runtime.clankers_batch_runner.execution.unavailable]
- [ ] [serial] Implement pure resume-manifest matching over run ID, job ID, plan hash, output format, redaction policy, and output evidence. r[agent_harness_runtime.clankers_batch_runner.resume]
- [ ] [depends:resume] Add stale/malformed manifest tests proving mismatched evidence does not skip work silently. r[agent_harness_runtime.clankers_batch_runner.resume.stale]
- [ ] [serial] Define normalized trajectory outcome and renderer adapter contracts with deterministic plan-order rendering. r[agent_harness_runtime.clankers_batch_runner.output]
- [ ] [depends:output] Add concurrent completion fixtures proving rendered records remain in stable plan order. r[agent_harness_runtime.clankers_batch_runner.output.stable_order]
- [ ] [serial] Define safe run/job receipt metadata with prompt sizes, statuses, handles, objective summaries, output labels, redaction markers, and BLAKE3 hashes. r[agent_harness_runtime.clankers_batch_runner.metadata]
- [ ] [depends:metadata] Add leak fixtures for raw prompts, provider payloads, credentials, and remote destinations. r[agent_harness_runtime.clankers_batch_runner.metadata.rejects_prompt]
- [ ] [depends:planning.remote_rejected,execution.unavailable,resume.stale,output.stable_order,metadata.rejects_prompt] Update docs and run focused batch tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_batch_runner.verification]
