# Tasks

- [ ] [serial] Define host-injected background queue service contracts and DTOs for enqueue, status, wait, logs, send-input, pause/resume/restart/cancel/clean, groups, and parallelism. r[agent_harness_runtime.hermes_background_queue.service]
- [ ] [depends:service] Add positive enqueue/status fixtures proving reusable runtime does not spawn processes directly. r[agent_harness_runtime.hermes_background_queue.service.enqueue]
- [ ] [serial] Add explicit target selector validation for destructive queue controls. r[agent_harness_runtime.hermes_background_queue.explicit_targets]
- [ ] [depends:explicit_targets] Add negative tests for ambiguous kill/pause/restart/clean/default-group operations. r[agent_harness_runtime.hermes_background_queue.explicit_targets.reject_ambiguous]
- [ ] [serial] Define typed group and parallelism DTOs with stable status classes and safe backend labels. r[agent_harness_runtime.hermes_background_queue.groups]
- [ ] [depends:groups] Add group status tests with multiple groups and differing parallelism limits. r[agent_harness_runtime.hermes_background_queue.groups.status]
- [ ] [serial] Add bounded log viewport/result DTOs with line/byte counts, truncation state, safe references, and BLAKE3 hashes. r[agent_harness_runtime.hermes_background_queue.logs]
- [ ] [depends:logs] Add tests proving large stdout/stderr logs are truncated and raw full logs are not embedded by default. r[agent_harness_runtime.hermes_background_queue.logs.truncated]
- [ ] [serial] Model wait, stream, poll, and send-input as explicit optional capabilities with timeout behavior. r[agent_harness_runtime.hermes_background_queue.wait_send]
- [ ] [depends:wait_send] Add negative tests proving unsupported send-input does not fall back to shelling out. r[agent_harness_runtime.hermes_background_queue.wait_send.unsupported_send]
- [ ] [depends:service.enqueue,explicit_targets.reject_ambiguous,groups.status,logs.truncated,wait_send.unsupported_send] Update docs and run focused queue tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_background_queue.verification]
