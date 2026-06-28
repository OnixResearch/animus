# Design: Adapt Hermes Kanban orchestration for Animus

## Context

Hermes uses Kanban tasks to route durable work between profiles. The transferable ideas are the data model and lifecycle rules, not the specific dashboard or dispatcher implementation.

Reference material inspected:

- `/home/brittonr/.hermes/skills/devops/kanban-worker/SKILL.md`
- `/home/brittonr/.hermes/skills/devops/kanban-orchestrator/SKILL.md`
- `/home/brittonr/.hermes/SOUL.md`

## Decisions

### 1. Kanban is optional orchestration

**Choice:** Define task orchestration DTOs and adapter traits outside the mandatory core turn loop. Hosts may attach a board service, but the runtime can operate without one.

**Rationale:** Animus should embed into small applications as well as durable multi-agent systems.

### 2. Task graph logic is pure

**Choice:** Parent/child promotion, cycle detection, valid state transitions, and stale completion checks operate on in-memory task graph snapshots.

**Rationale:** Deterministic core logic is testable without a board database.

### 3. Workspaces are explicit capabilities

**Choice:** A task declares a workspace kind such as scratch, directory, or worktree with explicit allowed roots and mutation boundaries supplied by the host.

**Rationale:** Workers should not infer write authority from ambient current directories.

### 4. Handoffs use structured metadata

**Choice:** Completion, block, heartbeat, and comment events include bounded typed metadata so downstream workers can consume results without parsing prose when possible.

**Rationale:** Durable orchestration needs machine-readable handoffs.

## Risks / Trade-offs

- A generic task model may be too broad. Keep the first contract focused on lifecycle state, dependencies, workspace scope, and handoff metadata.
- Tenant labels are not authorization by themselves. Hosts still enforce storage and secret isolation.
- Worktree semantics differ across VCS backends. Animus should model workspace capabilities, not hard-code Git operations.
