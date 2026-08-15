## Why

Animus already tracks semantic agent status and coordinates subagents, but it has no session-local watch surface. Consumers cannot observe a delegated agent's self-reported status or sanitized provider retry progress without receiving the agent's hidden prompts or raw provider data, and nothing prevents a watch configuration from closing a cycle.

A pure watch-graph reducer should own typed watch edges, reject any enable that would close a cycle, and project watcher-only redacted status. Dispatch, messaging, and provider retry execution remain host-owned.

## What Changes

- Add a session-local typed watch-graph reducer. r[agent_harness_runtime.agent_watch.graph]
- Reject any watch enable that would close a cycle without changing watch state. r[agent_harness_runtime.agent_watch.acyclic]
- Project only self-reported semantic status and sanitized provider retry progress to watchers. r[agent_harness_runtime.agent_watch.status]
- Reject unknown agents, duplicate edges, stale watchers, and non-monotonic generation. r[agent_harness_runtime.agent_watch.fail_closed]
- Emit metadata-only receipts. r[agent_harness_runtime.agent_watch.receipts]
- Add positive and negative fixtures. r[agent_harness_runtime.agent_watch.verification]

## Impact

- **Rust**: a pure no-std watch-graph reducer plus a non-default adapter surface; the subagent facade and host-control event contracts stay authoritative.
- **Consumers**: delegated work can be observed without leaking conversation internals.
- **Claims**: session-local only; self-reported status is not ground truth.
