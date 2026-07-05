# Hermes Kanban orchestration adaptation

Animus adapts Hermes-style Kanban orchestration as bounded, optional runtime contracts. The reusable runtime owns only in-memory DTO validation and deterministic decisions; hosts own board storage, scheduling, process supervision, clocks, VCS commands, filesystem access, and persistence.

## Runtime surface

- `r[agent_harness_runtime.hermes_kanban.task_graph]`: `KanbanTaskGraphDraft`, `KanbanTaskDraft`, `KanbanTaskGraph`, and `KanbanTask` define task identity, title/body, assignee label, lifecycle state, dependencies, child links, priority metadata, explicit workspace scope, tenant label, comments, heartbeats, block records, completion summary, and bounded metadata.
- `r[agent_harness_runtime.hermes_kanban.task_graph.valid]`: `validate_kanban_graph` accepts unique IDs, reciprocal links, bounded workspace scopes, tenant labels, bounded metadata, and stale-free completion records without a board database, dispatcher, process supervisor, VCS command, or filesystem authority.
- `r[agent_harness_runtime.hermes_kanban.dependencies]`: `evaluate_kanban_promotion` deterministically evaluates parent completion status and fails closed on missing parents, duplicate links, cycles, blocked/cancelled ambiguous dependencies, and stale completion evidence.
- `r[agent_harness_runtime.hermes_kanban.dependencies.promote]`: a child with multiple completed parents becomes ready only when every parent has an accepted completion record for the current revision.
- `r[agent_harness_runtime.hermes_kanban.dependencies.cycle]`: graph validation rejects cycles before promotion or scheduling.
- `r[agent_harness_runtime.hermes_kanban.workspace_scope]`: `KanbanWorkspaceScope` describes scratch, directory, and worktree-style host capabilities with safe target labels instead of ambient paths.
- `r[agent_harness_runtime.hermes_kanban.workspace_scope.constrained]`: `validate_kanban_workspace_mutation` admits declared writable labels, reports host-approval requirements for guarded labels, and rejects out-of-scope mutation attempts.
- `r[agent_harness_runtime.hermes_kanban.tenant_scope]`: task records, handoffs, and `KanbanContextAssembly` preserve tenant labels when supplied.
- `r[agent_harness_runtime.hermes_kanban.tenant_scope.reject_cross_tenant]`: graph validation rejects cross-tenant dependencies; context assembly can reject or quarantine cross-tenant handoffs without admitting their bodies.
- `r[agent_harness_runtime.hermes_kanban.handoffs]`: comments, heartbeats, block records, completion records, and context handoffs carry actor labels, shell-supplied sequence metadata, bounded text, tenant labels, and safe metadata.
- `r[agent_harness_runtime.hermes_kanban.handoffs.block]`: block records carry a short reason, optional bounded context, optional next action, actor attribution, and sequence metadata without marking the task completed.

## Adapter isolation

`KanbanBoardService` is an optional host-injection surface. `load_kanban_graph` and `append_kanban_handoff` fail closed when no service is supplied and never discover storage, credentials, files, processes, or network resources by ambient authority.

## Non-adopted Hermes surfaces

This adaptation does not adopt Hermes' SQLite board schema, dispatcher, profile roster, CLI, automatic process spawning, dashboard, or storage lifecycle. Host applications may map these DTOs onto their own services, but reusable Animus crates remain pure over supplied data.

## Verification

Focused verification lives in `crates/animus-runtime/src/kanban.rs` unit tests and covers positive graph validation, fan-out/fan-in promotion, workspace admission, tenant-preserving context assembly, missing service failure, missing workspace rejection, duplicate IDs, cycles, ambiguous dependencies, stale completions, cross-tenant leaks, unsafe handoff bodies, and out-of-scope mutation attempts.
