# agent-harness-runtime Change Spec: TUI agent control and queue dashboard

## Requirements

### Requirement: Status dashboard follows semantic authority

r[agent_harness_runtime.tui_control.dashboard] The agent status dashboard MUST render busy/idle/streaming/waiting/error states from status-authority DTOs and MUST NOT invent TUI-local status precedence.

#### Scenario: Waiting-for-approval renders distinctly

r[agent_harness_runtime.tui_control.dashboard.waiting]
- GIVEN status-authority reports a waiting-for-approval state
- WHEN the dashboard renders it
- THEN it shows the waiting state distinctly from busy and idle
- AND no TUI-local status overrides the authority value.

### Requirement: Controls are bounded typed commands

r[agent_harness_runtime.tui_control.start_stop] Start, stop, and interrupt controls MUST issue bounded typed commands to the runtime or host, and the TUI MUST hold no ambient process, worker, or network authority.

#### Scenario: Missing worker service blocks control

r[agent_harness_runtime.tui_control.start_stop.missing_service]
- GIVEN a control requests a worker delegation and no worker service is available
- WHEN the control is issued
- THEN runtime returns a worker-service-unavailable failure
- AND no fallback thread, process, shell command, or network request is attempted from the TUI.

### Requirement: Background-work queue is a bounded projection

r[agent_harness_runtime.tui_control.queue_view] The background-work queue view MUST project background-work-queues DTOs read-only with a bounded visible window, and cancel or inspect actions MUST issue typed commands rather than acting on queue state directly.

#### Scenario: Oversize queue is windowed

r[agent_harness_runtime.tui_control.queue_view.bounded]
- GIVEN the queue exceeds the configured visible window
- WHEN the queue view renders
- THEN it shows a bounded window with safe metadata for evicted entries
- AND no unbounded queue growth occurs in render state.

### Requirement: Subagent and process panels are read-only projections

r[agent_harness_runtime.tui_control.subagent_panels] Subagent/peers and process panels MUST project worker-task/subagent-thread and process-job-profile DTOs read-only, and any mutation MUST issue a typed command to the host rather than spawning workers or processes from the TUI.

#### Scenario: No ambient spawn from the TUI

r[agent_harness_runtime.tui_control.subagent_panels.no_spawn]
- GIVEN a user requests a subagent start from the panel
- WHEN the panel handles the request
- THEN it issues a typed host command
- AND no OS process, thread, or worker is spawned from `animus-tui`.

### Requirement: Topology-aware controls

r[agent_harness_runtime.tui_control.topology] Controls MUST consume host-control-protocol `HostEvent`/`HostTopologySnapshot` to target the correct workspace, tab, pane, or agent, and MUST re-resolve compact selectors after topology changes rather than acting on stale selectors.

#### Scenario: Stale selector is rejected

r[agent_harness_runtime.tui_control.topology.stale]
- GIVEN a compact selector is bound to an older topology revision
- WHEN the topology has changed and a control targets that selector
- THEN the control re-resolves or reports a topology-mismatch diagnostic
- AND no command is dispatched against the stale selector.

### Requirement: Control dashboard verification

r[agent_harness_runtime.tui_control.verification] The change MUST include focused positive and negative tests for status rendering, control command issuing, missing worker service, queue bounding and inspect/cancel, subagent no-spawn, topology staleness, denied controls, and ambient-spawn leakage before closeout.

#### Scenario: Closeout evidence covers control fixtures

r[agent_harness_runtime.tui_control.verification.closeout]
- GIVEN maintainers intend to archive the agent control and queue dashboard change
- WHEN closeout validation runs
- THEN evidence names focused dashboard, control, queue, subagent, topology, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
