# Tasks

- [x] [serial] Add UI-neutral subagent status projection DTOs for active, terminal, failed, blocked, and cancellation states. r[agent_harness_runtime.subagent_ux_hooks.status_projection]
- [x] [depends:status_projection] Include safe budget usage, denial reasons, handoff summary references, evidence references, and pending mutation-intent summaries. r[agent_harness_runtime.subagent_ux_hooks.safe_metadata]
- [x] [depends:status_projection] Add command descriptors for cancel, bounded detail import request, acknowledgement, and mutation-intent arbitration request. r[agent_harness_runtime.subagent_ux_hooks.command_descriptors]
- [x] [depends:command_descriptors] Route UX commands through coordinator, capability, confirmation, and mutation policy without direct effects. r[agent_harness_runtime.subagent_ux_hooks.policy_routing]
- [x] [depends:safe_metadata,policy_routing] Add negative tests for raw-log exposure, unbounded detail import, unauthorized cancel, mutation bypass, stale handle, and credential-like metadata. r[agent_harness_runtime.subagent_ux_hooks.fail_closed]
- [x] [depends:fail_closed] Update docs and focused UX hook verification evidence. r[agent_harness_runtime.subagent_ux_hooks.verification]
