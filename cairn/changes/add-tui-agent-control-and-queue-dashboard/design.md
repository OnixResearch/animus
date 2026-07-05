# Design: TUI agent control and queue dashboard

## Context

Animus already defines status authority (`adapt-agent-status-authority`), background work queues (`adapt-hermes-background-work-queues`), controller continuation policy (`adapt-clankers-controller-continuation-policy`), worker-task/subagent-thread contracts (`adapt-openai-codex-subagent-threads`, `adapt-claude-code-agent-teams`), process-job profiles (`adapt-clankers-process-job-profiles`), and the host-control protocol (`adapt-host-control-protocol`). A control dashboard should project and command these without re-owning execution.

## Decisions

### 1. Status dashboard follows semantic authority

**Choice:** The dashboard renders busy/idle/streaming/waiting/error states from status-authority DTOs, not TUI-local status.

**Rationale:** Keeps the TUI consistent with headless and receipt surfaces.

### 2. Controls are bounded typed commands

**Choice:** Start/stop/interrupt and queue cancel/inspect issue typed commands to the runtime/host. The TUI holds no ambient process, worker, or network authority.

**Rationale:** Matches the missing-service-fails-closed and no-ambient-spawn baselines.

### 3. Queue and subagent views are projections

**Choice:** The background-work queue and subagent/peers/process panels project existing DTOs read-only; mutations issue typed commands.

**Rationale:** Keeps ownership of execution in the std shell and runtime.

### 4. Topology-aware via host-control protocol

**Choice:** Controls consume `HostEvent`/`HostTopologySnapshot` to target the correct workspace, tab, pane, or agent, re-resolving compact selectors after topology changes.

**Rationale:** Matches the archived host-control protocol lifetime semantics.

### 5. Downstream-only composition

**Choice:** The dashboard consumes accepted DTOs as-is. No new field is added to a reusable crate.

**Rationale:** Keeps the TUI strictly downstream.

## Risks / Trade-offs

- Controls could be mistaken for authority; receipts must separate command issuance from authorization.
- Queue views can grow large; bound the visible queue window with safe metadata.
- Topology changes can invalidate selectors; re-resolve and report mismatches rather than acting on stale selectors.
