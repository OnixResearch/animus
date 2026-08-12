# Design: Adapt Hermes background work queue semantics for Animus

## Context

Hermes and Pi workflows use pueue to manage work that exceeds one agent turn. The transferable contract is not pueue itself; it is explicit background-task lifecycle state, bounded logs, and safe control operations.

Reference material inspected:

- `/home/brittonr/git/hermes/_global/skills/pueue/SKILL.md`
- `/home/brittonr/git/hermes/openspec/specs/pueue-integration/spec.md`
- `/home/brittonr/git/hermes/_global/extensions/lib/pueue-panel-core.ts`
- `/home/brittonr/.hermes/SOUL.md`

## Decisions

### 1. Queue service is host-injected

**Choice:** Animus defines a queue service trait/contract and DTOs. The host chooses pueue, another scheduler, or no queue service.

**Rationale:** Runtime code should not spawn processes or assume pueue availability.

### 2. Destructive operations require explicit targets

**Choice:** Pause, kill, restart, clean, and group-wide controls require a task ID, group selector, or explicit all-groups selector. Default-group fallback is rejected.

**Rationale:** Background queue controls can destroy user work.

### 3. Logs are bounded viewports

**Choice:** Log reads return visible tail/window text, total line/byte counts, truncation state, safe references, and optional BLAKE3 content hashes.

**Rationale:** Model-visible logs should be useful but never unbounded.

### 4. Wait semantics are explicit

**Choice:** Tool callers choose quick wait, background enqueue, poll, or stream behavior with explicit timeout and progress policy.

**Rationale:** Agents need predictable behavior for long-running commands.

## Risks / Trade-offs

- Queue backends differ in status vocabulary. Normalize stable status classes and preserve backend-specific labels as metadata.
- Stdin/send semantics vary. Model them as optional capabilities and fail closed when unsupported.
- Log truncation can hide root causes. Provide safe references and hashes so hosts can inspect full logs outside model context.
