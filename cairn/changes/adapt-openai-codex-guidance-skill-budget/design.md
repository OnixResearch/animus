# Design: Adapt OpenAI Codex guidance and skill context budgeting for Animus

## Context

Codex discovers global and project guidance in a precedence chain and uses progressive disclosure for skills: names and descriptions are visible for selection, while full skill instructions and references are loaded only after the skill is chosen. Animus can adapt these ideas into a typed context assembly pipeline that remains independent of Codex's exact filesystem conventions.

## Decisions

### 1. Guidance sources are explicit and attributable

**Choice:** Represent each guidance document as a source-attributed input with scope, precedence, override semantics, origin, and content hash.

**Rationale:** Prompt context should be explainable and auditable, especially when nested guidance changes behavior.

### 2. Prompt assembly has a budget ledger

**Choice:** Context assembly tracks budget consumption, included sources, skipped sources, truncation decisions, and reasons.

**Rationale:** Silent prompt truncation makes agent behavior hard to debug.

### 3. Skills use progressive disclosure

**Choice:** The initial prompt receives bounded skill metadata. Full skill instructions, references, and assets are loaded only when the skill is explicitly invoked or selected by policy.

**Rationale:** Large skill libraries should not crowd out task-specific context.

### 4. Invocation policy is part of skill metadata

**Choice:** Skills declare whether implicit invocation is allowed, which dependencies are required, and which tool capabilities are needed.

**Rationale:** A sensitive or expensive skill should not run because of an accidental fuzzy match.

### 5. Scripts remain shell-owned and trust-gated

**Choice:** Skill package descriptors may reference scripts, but runtime core treats them as resources requiring shell policy and trust checks before execution.

**Rationale:** Animus should support reusable workflows without turning skills into privileged code.

## Risks / Trade-offs

- Budget accounting can feel verbose. Keep detailed ledgers in reports while exposing concise summaries to users.
- Implicit skill invocation can surprise users. Make it opt-out per skill and record why a skill was selected.
- Codex compatibility imports can drift. Treat compatibility as adapters with tests, not as the canonical Animus format.
