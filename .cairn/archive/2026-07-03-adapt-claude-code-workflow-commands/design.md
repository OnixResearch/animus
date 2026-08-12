# Design: Adapt Claude Code workflow commands for Animus

## Context

Claude Code exposes reusable commands as a lightweight user interface for common coding workflows. Animus should adapt the concept as typed command manifests that can be rendered into prompts and dispatch plans without letting arbitrary prompt text bypass policy.

Existing skill/resource work covers progressive disclosure. This change focuses on invocation, parameter binding, declared capabilities, side-effect posture, and workflow output contracts.

## Decisions

### 1. Commands are manifests, not raw prompts

**Choice:** Represent each workflow command as a typed manifest with name, summary, parameters, capability requirements, side-effect posture, retrieval budget, validation gates, output contract, and source metadata.

**Rationale:** The model and runtime need to know what a command may do before executing a workflow. Raw prompt snippets cannot express this safely.

### 2. Lookup and argument binding are pure

**Choice:** Build a deterministic command registry from in-memory descriptors, then bind invocation arguments through pure validation before prompt construction or tool dispatch.

**Rationale:** Duplicate-name handling and malformed input should be testable without file reads, shell execution, or provider calls.

### 3. Workflow side effects must be explicit

**Choice:** Commands declare whether they are read-only, mutation-capable, VCS-mutating, network-capable, or host-specific, and runtime permission/policy checks gate the resulting actions.

**Rationale:** Claude Code workflows can automate powerful actions. Animus should preserve convenience while making side effects reviewable.

### 4. Outputs are contract-bound

**Choice:** A workflow command declares a model-visible output shape such as explanation, patch summary, validation report, review report, commit message proposal, or structured DTO.

**Rationale:** Commands are more useful and safer when downstream tools can verify expected output instead of parsing arbitrary prose.

## Risks / Trade-offs

- Too much manifest structure can make commands heavy. Keep required fields focused on safety, inputs, side effects, and outputs.
- Project-local commands can become prompt-injection vectors. Require trust decisions before advertisement or expansion.
- Commit/PR commands are attractive but risky. VCS actions must remain explicit adapters, and PR creation must not be part of default automation.
