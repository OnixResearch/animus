# agent-harness-runtime Change Spec: Hermes Kanban orchestration adaptation

## Requirements

### Requirement: Mode-neutral task graph DTOs

r[agent_harness_runtime.hermes_kanban.task_graph] Animus MUST define mode-neutral task graph DTOs for durable orchestration with task ID, title, body, assignee label, lifecycle state, parent dependencies, child links, priority metadata, workspace scope, tenant label, comments, heartbeats, block records, completion summary, and bounded completion metadata.

#### Scenario: Valid task graph snapshot is accepted

r[agent_harness_runtime.hermes_kanban.task_graph.valid]
- GIVEN a host supplies a task graph snapshot with unique IDs, valid parent links, workspace scopes, tenant labels, and bounded metadata
- WHEN task graph validation runs
- THEN the graph is accepted for orchestration decisions
- AND no board database, process supervisor, VCS command, or dispatcher service is required by the pure validation logic.

### Requirement: Dependency promotion is deterministic

r[agent_harness_runtime.hermes_kanban.dependencies] Animus MUST provide deterministic parent/child dependency evaluation for fan-out/fan-in orchestration and MUST reject cycles, missing parents, duplicate IDs, and ambiguous dependency state.

#### Scenario: Child becomes ready after all parents complete

r[agent_harness_runtime.hermes_kanban.dependencies.promote]
- GIVEN a child task depends on two parent tasks
- WHEN both parents transition to completed with accepted handoff metadata
- THEN dependency evaluation marks the child eligible for ready state
- AND repeated evaluation over the same graph snapshot returns the same result.

#### Scenario: Cyclic dependencies fail closed

r[agent_harness_runtime.hermes_kanban.dependencies.cycle]
- GIVEN task A depends on task B and task B depends on task A
- WHEN graph validation runs
- THEN validation returns a typed cycle diagnostic
- AND neither task is promoted or scheduled.

### Requirement: Workspace scopes are explicit

r[agent_harness_runtime.hermes_kanban.workspace_scope] Task orchestration MUST represent workspace scopes explicitly and MUST NOT infer filesystem mutation authority from ambient process state.

#### Scenario: Worker is constrained to declared workspace

r[agent_harness_runtime.hermes_kanban.workspace_scope.constrained]
- GIVEN a task declares a scratch workspace with a host-supplied root capability
- WHEN a worker attempts to record a mutation outside that workspace capability
- THEN orchestration validation reports an out-of-scope mutation or requires a separate host approval
- AND the task completion cannot claim unrestricted workspace authority.

### Requirement: Tenant labels prevent context leakage

r[agent_harness_runtime.hermes_kanban.tenant_scope] Task handoffs, comments, memory references, and downstream context summaries MUST preserve tenant labels when supplied and MUST support host enforcement that prevents cross-tenant leakage.

#### Scenario: Cross-tenant handoff is rejected

r[agent_harness_runtime.hermes_kanban.tenant_scope.reject_cross_tenant]
- GIVEN a child task belongs to tenant `alpha` and a parent handoff belongs to tenant `beta`
- WHEN dependency promotion or context assembly evaluates the handoff
- THEN validation rejects or quarantines the cross-tenant dependency according to host policy
- AND the beta handoff body is not inserted into alpha model context.

### Requirement: Lifecycle handoffs are structured and bounded

r[agent_harness_runtime.hermes_kanban.handoffs] Task lifecycle events MUST support structured heartbeat, block, comment, and completion records with bounded text, safe metadata, and clear actor/source attribution.

#### Scenario: Block reason carries actionable context

r[agent_harness_runtime.hermes_kanban.handoffs.block]
- GIVEN a worker cannot continue without a human or upstream decision
- WHEN it emits a block record
- THEN the record includes a short reason, optional bounded context comment, actor attribution, timestamp or sequence metadata supplied by the shell, and next-action hint
- AND the task is not marked completed.

### Requirement: Kanban adaptation verification

r[agent_harness_runtime.hermes_kanban.verification] The Kanban adaptation MUST include focused positive and negative tests for graph validation, dependency promotion, cycle rejection, workspace scope checks, tenant isolation, handoff bounds, stale completion rejection, and optional adapter isolation.

#### Scenario: Closeout evidence covers orchestration fixtures

r[agent_harness_runtime.hermes_kanban.verification.closeout]
- GIVEN maintainers intend to archive the Hermes Kanban adaptation
- WHEN closeout validation runs
- THEN evidence names focused graph tests, dependency tests, workspace/tenant tests, handoff tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
