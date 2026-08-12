# Design: Adapt Claude Code agent-team orchestration for Animus

## Context

Claude Code supports subagents and background agents that can work in parallel and report back. The transferable value is scoped delegation and report aggregation, not Anthropic-managed infrastructure. Animus should model workers as host-backed services controlled by explicit task contracts.

Existing changes cover background work queues and Kanban orchestration. This change focuses on agent-worker authority, context minimization, report contracts, and coordinator merge semantics.

## Decisions

### 1. Worker tasks are scoped contracts

**Choice:** A worker task includes objective, scope, input context packet, allowed tools/capabilities, budgets, expected output schema, parent trace, and authority metadata.

**Rationale:** Delegation should be reviewable before worker execution and testable as plain data.

### 2. Context packets are minimized by default

**Choice:** Workers receive explicit context packets assembled from selected parent messages, summaries, files, memory descriptors, and tool results. Full parent transcripts and memory are excluded unless policy explicitly admits them.

**Rationale:** Subagents can otherwise leak sensitive context or amplify prompt injection across scopes.

### 3. Worker execution is host-owned

**Choice:** Reusable runtime defines worker service traits/DTOs for enqueue, status, cancel, and collect. The shell or embedder owns threads, processes, remote sessions, schedulers, and clocks.

**Rationale:** Animus remains embeddable and `no_std`-friendly where intended.

### 4. Coordinator merge is pure and evidence-bearing

**Choice:** Coordinator logic consumes in-memory worker reports, validation results, conflicts, and mutation intents, then returns a merged report or structured conflict requiring operator or policy resolution.

**Rationale:** A lead agent needs deterministic evidence, not an unstructured pile of assistant messages.

## Risks / Trade-offs

- Worker orchestration can duplicate Kanban or background queues. Keep this contract at the agent-task/report layer and depend on host services for scheduling.
- Context minimization can starve workers. Let manifests request specific context types and make omissions visible in report metadata.
- Parallel mutation is dangerous. Require existing mutation coordination and permission checks before any worker mutation intent executes.
