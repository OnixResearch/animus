# Design: Adapt OpenAI Codex worktree automations for Animus

## Context

Codex app worktrees give background tasks a separate checkout so the user's foreground files stay stable. Automations can run on schedules, on dedicated worktrees, or as thread-bound wakeups. Animus can adapt these ideas as task-workspace and automation contracts while leaving concrete Git, Jujutsu, filesystem, or hosted implementations to adapters.

## Decisions

### 1. Isolated workspace is VCS-neutral

**Choice:** Define an isolated task workspace around source identity, workspace identity, writable roots, snapshot identity, and restore metadata instead of Git worktree primitives.

**Rationale:** Animus should support Jujutsu, Git, archive snapshots, and hosted filesystems without changing runtime orchestration.

### 2. Automations are typed run definitions

**Choice:** Model standalone scheduled runs and thread-bound wakeups as automation definitions with prompt, cadence, target workspace policy, sandbox profile, and stop criteria.

**Rationale:** The runtime can reason about unattended work without embedding a scheduler.

### 3. Handoff is snapshot-based

**Choice:** Moving a thread between foreground and isolated contexts requires a snapshot record that names source state, target state, included files, and conflict status.

**Rationale:** Handoff must be auditable and reversible instead of relying on implicit checkout state.

### 4. Ignored local files require explicit inclusion

**Choice:** Local-only files may be copied into an isolated workspace only when an allowlist entry permits it and redaction policy accepts the file classification.

**Rationale:** Codex's `.worktreeinclude` pattern is useful, but Animus should generalize it and avoid accidental secret propagation.

### 5. Cleanup preserves restore evidence

**Choice:** Cleanup policies may delete isolated workspace materialization only after preserving enough snapshot evidence to restore or report the lost workspace state.

**Rationale:** Background tasks should not silently discard reviewable work.

## Risks / Trade-offs

- VCS-neutral abstractions can hide important backend differences. Adapter capabilities should report what operations are actually supported.
- Automations can be risky when unattended. Sandbox defaults and policy rules must remain explicit per automation definition.
- Snapshot evidence can grow large. Receipts should store hashes and bounded manifests rather than raw file contents.
