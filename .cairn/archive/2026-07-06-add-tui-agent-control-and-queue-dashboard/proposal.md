# Proposal: TUI agent control and queue dashboard

## Why

A pi-style harness lets the user see and control the running agent: status (busy/idle/streaming/waiting-for-approval), start/stop/interrupt, the background-work queue, and subagent/peers/process panels. This change adds that dashboard to `animus-tui`, consuming status-authority, background-work-queues, controller-continuation-policy, worker-task/subagent-thread, process-job-profile, and host-control-protocol DTOs downstream. The TUI issues bounded commands only; it never spawns workers or processes itself.

## What Changes

- Add an agent status dashboard rendering status-authority states (busy/idle/streaming/waiting/error).
- Add start/stop/interrupt controls that issue bounded typed commands to the runtime/host, not ambient authority.
- Add a background-work queue view from background-work-queues DTOs with status, cancel, and inspect commands.
- Add subagent/peers and process panels from worker-task/subagent-thread and process-job-profile DTOs.
- Consume host-control-protocol `HostEvent`/`HostTopologySnapshot` for topology-aware controls.
- Compose on the conversation render model; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` dashboard, control, queue, subagent, peers, and process panel modules and tests.
- **Testing**: positive tests for status rendering, control command issuing, queue listing/inspect/cancel, and subagent/process projection; negative tests for missing worker service, denied controls, topology mismatches, unbounded queue, and ambient-spawn leakage.
- **Non-claims**: this change does not spawn workers/processes from the TUI, does not open pull requests, controls are bounded commands not ambient authority, and no reusable-crate contracts are added.
