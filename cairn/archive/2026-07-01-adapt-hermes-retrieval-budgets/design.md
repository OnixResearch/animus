# Design: Adapt Hermes retrieval budgets for Animus

## Context

Hermes retrieval-oriented skills include explicit budgets and stopping rules. That pattern is useful beyond Hermes because retrieval-heavy agents otherwise spend context and tool time on unsupported polish.

Reference material inspected:

- `/home/brittonr/git/hermes/_global/skills/multi-lens/SKILL.md`
- `/home/brittonr/git/hermes/_global/skills/property-discovery/SKILL.md`
- `/home/brittonr/git/hermes/openspec/specs/agent-orchestration/spec.md`
- `/home/brittonr/.hermes/SOUL.md`

## Decisions

### 1. Retrieval budgets are typed descriptors

**Choice:** Define descriptors with retrieval purpose, first action, allowed repeat triggers, stop condition, missing-evidence behavior, citation requirement, progress note guidance, and maximum budget hints.

**Rationale:** Budgets should be validated and reused across prompts, tools, and receipts.

### 2. Retrieval capability declares budget requirement

**Choice:** Retrieval-capable resources can declare that a budget is required before prompt assembly advertises them.

**Rationale:** Tool and skill metadata should encode when unbounded retrieval is unsafe or wasteful.

### 3. Runtime records decisions, not raw bodies

**Choice:** Receipts and reports record budget decisions, evidence labels, counts, hashes, and safe references. Raw retrieved bodies remain tool output or host storage policy.

**Rationale:** Evidence should be auditable without turning receipts into transcript or web-cache dumps.

### 4. Checkers are deterministic

**Choice:** Prompt-hygiene checks operate on descriptors and prompt snippets, not live web behavior.

**Rationale:** Validation must be reproducible in CI.

## Risks / Trade-offs

- Too-strict budgets could block legitimate research. Allow host policy to raise budgets with explicit rationale.
- Citation requirements vary by task. Model them as expectation metadata, not mandatory URL fields for local code reads.
- Budget decisions can become noisy. Keep report metadata bounded and aggregateable.
