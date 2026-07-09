# Design: Adapt Hermes done-review lifecycle contracts

## Context

The accepted review-report contract parses structured markdown reports and read-only promotion candidates. Hermes also has a larger post-completion workflow: collect evidence, run required rails, choose reviewers, execute attempts, record metrics, and possibly queue a clean-session follow-up. Animus should expose the reusable parts as framework contracts only.

## Decisions

### 1. Review lifecycle state is pure DTO data

**Choice:** Add request, evidence, rail, attempt, outcome, and receipt DTOs that are constructed from host-supplied values and validated by reusable logic.

**Rationale:** Hosts differ in their VCS, model, UI, storage, and review policy. The SDK should validate and coordinate data, not own the product workflow.

### 2. Completion rails are declarative and host-executed

**Choice:** Rails describe IDs, labels, policy reasons, required status, command/profile labels, timeout classes, and safe result summaries. Runtime logic consumes supplied rail results and never runs commands.

**Rationale:** Completion rails can be project-specific and side-effecting. Keeping execution in the host prevents baked-in repo behavior.

### 3. Reviewer attempt planning does not call providers

**Choice:** Reviewer policy can produce ordered attempt descriptors with model role labels, reasoning/verbosity hints, budgets, timeout classes, and retry/escalation reasons. A host reviewer service owns actual model calls.

**Rationale:** Animus should support review-service interop without shipping default reviewer accounts, providers, or credentials.

### 4. Outcomes request continuation, not mutation

**Choice:** FAIL/WARN outcomes can produce safe continuation intents such as `blocked`, `manual-follow-up`, or `retry-requested`. They do not automatically edit code, start fresh sessions, mutate Cairn tasks, write metrics, or approve completion.

**Rationale:** Post-review action is product policy and must remain explicit.

## Risks / Trade-offs

- Too much lifecycle structure could look like a built-in product gate. Documentation and tests must state opt-in host ownership.
- Review evidence may be incomplete. The outcome model should distinguish insufficient evidence from verified failure.
- Completion rail descriptors can drift toward shell command transport. Prefer profile/action labels and bounded result summaries.
