# Tasks

- [ ] [depends:add-tui-conversation-render-model] Add an agent status dashboard rendering status-authority states (busy/idle/streaming/waiting/error) with no TUI-local status. r[agent_harness_runtime.tui_control.dashboard]
- [ ] [depends:dashboard] Add start/stop/interrupt controls that issue bounded typed commands to the runtime/host and hold no ambient process, worker, or network authority. r[agent_harness_runtime.tui_control.start_stop]
- [ ] [depends:dashboard] Add a background-work queue view projecting background-work-queues DTOs read-only, with cancel and inspect issuing typed commands and a bounded visible window. r[agent_harness_runtime.tui_control.queue_view]
- [ ] [depends:dashboard] Add subagent/peers and process panels projecting worker-task/subagent-thread and process-job-profile DTOs, with mutations issuing typed commands. r[agent_harness_runtime.tui_control.subagent_panels]
- [ ] [depends:dashboard,start_stop,queue_view,subagent_panels] Consume host-control-protocol `HostEvent`/`HostTopologySnapshot` to target controls and re-resolve selectors after topology changes. r[agent_harness_runtime.tui_control.topology]
- [ ] [depends:dashboard,start_stop,queue_view,subagent_panels,topology] Add negative tests for missing worker service, denied controls, topology mismatches, unbounded queue, and ambient-spawn leakage, plus positive dashboard/control/queue/subagent tests. r[agent_harness_runtime.tui_control.verification]
