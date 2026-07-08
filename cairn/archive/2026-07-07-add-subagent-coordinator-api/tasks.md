# Tasks

- [x] [serial] Add a host-injected `SubagentService` trait with enqueue, status, cancel, and collect operations for validated child-run admissions. r[agent_harness_runtime.subagent_coordinator.service]
- [x] [depends:service] Add a pure coordinator state machine for admitted, queued, running, cancelling, completed, failed, timed-out, and collected states. r[agent_harness_runtime.subagent_coordinator.state_machine]
- [x] [depends:state_machine] Map service outcomes into safe lifecycle events and typed failure diagnostics without raw child logs. r[agent_harness_runtime.subagent_coordinator.lifecycle]
- [x] [depends:service] Add negative tests proving missing services, stale handles, denied service calls, timeouts, malformed reports, and ambient fallback attempts fail closed. r[agent_harness_runtime.subagent_coordinator.fail_closed]
- [x] [depends:state_machine,lifecycle,fail_closed] Update docs, facade integration notes, and focused coordinator verification evidence. r[agent_harness_runtime.subagent_coordinator.verification]
