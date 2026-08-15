# Agent watch graph

The `animus-adapter-agent-watch` crate owns a session-local, acyclic agent watch graph. It is a pure no-std decision core. It contacts no processes, sockets, persistence, or providers.

## Boundary

- `apply_watch_action` applies one enable or disable transition over an owned graph state.
- A cycle-closing enable is rejected atomically. Watch state never changes on rejection.
- `project_watcher_status` gives each watcher only the self-reported semantic status and sanitized retry progress of the agents it watches.
- `validate_watch_graph` rejects unknown agents, duplicate edges, and non-monotonic generations before effects.
- `build_watch_receipt` emits a metadata-only receipt with no prompts, transcripts, or provider payloads.

## Use

```rust
use animus_adapter_agent_watch::{
    AgentLabel, WatchAction, WatchGraph, apply_watch_action, validate_watch_graph,
};
```

Enable one edge, then validate the graph. Watch the positive and negative tests in `src/tests.rs`.

## Non-claims

- The graph is session-local. Cross-session discovery stays outside this crate.
- Self-reported status is not ground truth. The redacted projection makes that explicit.
- Acyclic enforcement rejects bidirectional collaboration unless the reverse edge is modeled separately.
