# Tasks

- [x] [serial] Define mode-neutral task graph DTOs with task identity, title/body, assignee label, lifecycle state, dependencies, workspace scope, tenant label, comments, heartbeats, block records, completion summary, and bounded metadata. r[agent_harness_runtime.hermes_kanban.task_graph]
- [x] [depends:task_graph] Add positive graph validation fixtures for unique IDs, valid links, workspace scopes, tenant labels, and bounded metadata. r[agent_harness_runtime.hermes_kanban.task_graph.valid]
- [x] [serial] Add pure dependency promotion logic with missing-parent, duplicate-ID, ambiguous-state, and cycle diagnostics. r[agent_harness_runtime.hermes_kanban.dependencies]
- [x] [depends:dependencies] Add tests for fan-out/fan-in promotion after all parents complete. r[agent_harness_runtime.hermes_kanban.dependencies.promote]
- [x] [depends:dependencies] Add cycle rejection tests proving cyclic tasks are not promoted. r[agent_harness_runtime.hermes_kanban.dependencies.cycle]
- [x] [serial] Define explicit workspace scope capability DTOs for scratch, directory, and worktree-style contexts. r[agent_harness_runtime.hermes_kanban.workspace_scope]
- [x] [depends:workspace_scope] Add tests proving out-of-scope mutations are rejected or require explicit host approval. r[agent_harness_runtime.hermes_kanban.workspace_scope.constrained]
- [x] [serial] Preserve tenant labels in handoffs and context assembly inputs. r[agent_harness_runtime.hermes_kanban.tenant_scope]
- [x] [depends:tenant_scope] Add negative tests for cross-tenant dependency and context leakage. r[agent_harness_runtime.hermes_kanban.tenant_scope.reject_cross_tenant]
- [x] [serial] Add structured heartbeat, block, comment, and completion handoff records with bounds and actor attribution. r[agent_harness_runtime.hermes_kanban.handoffs]
- [x] [depends:handoffs] Add block/completion metadata tests including stale completion rejection. r[agent_harness_runtime.hermes_kanban.handoffs.block]
- [x] [depends:task_graph.valid,dependencies.promote,dependencies.cycle,workspace_scope.constrained,tenant_scope.reject_cross_tenant,handoffs.block] Update docs and run focused Kanban orchestration tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_kanban.verification]
