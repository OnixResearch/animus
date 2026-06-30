# Design: Adapt Hermes policy profiles for Animus

## Context

Hermes keeps high-value operating guidance in a short global posture. Animus should represent similar guidance as typed data so embedders can compose platform, user, project, and tool policies while preserving trust boundaries.

Reference material inspected:

- `/home/brittonr/.hermes/SOUL.md`
- `/home/brittonr/git/hermes/openspec/specs/agent-orchestration/spec.md`
- `/home/brittonr/git/hermes/_global/skills/*/SKILL.md`

## Decisions

### 1. Policies are data, not prompt blobs

**Choice:** Define descriptors with policy kind, source scope, trust status, priority, goal text, success criteria, constraints, validation rules, stop rules, and optional prompt snippets.

**Rationale:** Typed policy makes composition testable and avoids invisible prompt concatenation.

### 2. Prompt assembly is deterministic and attributable

**Choice:** Prompt construction emits policy snippets in deterministic precedence order and labels their source or capability domain.

**Rationale:** Agents and reviewers should know which policy produced a rule.

### 3. Safety constraints remain enforceable separately

**Choice:** Policy descriptors can inform prompts, but runtime-side guardrails such as capability denial, read-only mode, and mutation checks remain separate typed enforcement paths.

**Rationale:** Prompt text is not a security boundary.

### 4. Project-local policy is trust-gated

**Choice:** Project-local policy descriptors are ignored or marked unavailable until host trust allows them.

**Rationale:** Project guidance can attempt to override global safety rules.

## Risks / Trade-offs

- Policy composition can become noisy. Add prompt-budget checks and force concise outcome-first fields.
- Conflicting policy is inevitable. Deterministic precedence and diagnostics must explain which rule wins.
- Host policy formats will vary. Nickel is preferred for repo-owned config, but core consumes typed structs after host parsing.
