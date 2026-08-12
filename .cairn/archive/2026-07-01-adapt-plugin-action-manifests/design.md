# Design: Adapt plugin/action manifests for Animus

## Context

Herdr's plugin API is useful because the manifest declares workflow actions, event hooks, terminal panes, and link handlers ahead of time. The host links the manifest, validates platform and version compatibility, and later invokes actions or opens panes through the host. Animus can adapt this role vocabulary while preserving its stricter descriptor boundary and policy gates.

## Decisions

### 1. Manifest roles are data-only declarations

**Choice:** Extend plugin descriptors with action, event-hook, pane-entrypoint, and link-handler declarations. Validation records command labels or artifact labels as data only.

**Rationale:** Reusable runtime code should understand available capabilities without executing untrusted plugin code.

### 2. Effective compatibility is explicit

**Choice:** Each manifest and entrypoint can declare minimum host capability/version and platform support. Validation computes effective compatibility before link or invocation planning.

**Rationale:** A plugin can be valid data while still being unavailable on the current host.

### 3. Invocation context is typed and redacted

**Choice:** Action and event planning produce a typed context with safe workspace, tab, pane, agent, worktree/session provenance, request, plugin root/config/state labels, and source metadata.

**Rationale:** Plugins need context, but context injection must not leak private paths, transcripts, credentials, or unbounded state by default.

### 4. Registry operations are host-owned

**Choice:** Hosts own linking, enable/disable state, registry persistence, install sources, plugin logs, pane opening, and command execution. Reusable logic validates manifests and plans invocations.

**Rationale:** Registry state and command execution are side effects that belong in the imperative shell.

### 5. Warnings are distinct from hard failures

**Choice:** Unknown future event names, missing optional metadata, or unavailable optional platforms can produce warnings, while missing identity/version, unsafe command labels, disabled plugins, or policy violations fail closed.

**Rationale:** Hosts need forward-compatible manifests without silently accepting unsafe invocations.

## Risks / Trade-offs

- Supporting warnings can hide real mistakes. Tests should classify which diagnostics are warnings and which block invocation.
- Context DTOs can sprawl. Start with safe identifiers and provenance summaries rather than full host snapshots.
- Plugin pane entrypoints can couple to a TUI product surface. Keep them as optional host entrypoints, not reusable core behavior.
