# Design: Adapt OpenAI Codex exec policy rules for Animus

## Context

Codex documents an exec policy layer that applies command-prefix rules when a command requests capability outside the active sandbox. It also treats common shell wrappers conservatively: simple command chains can be split into individual commands for policy evaluation, while advanced shell constructs remain opaque and require the stricter decision.

Animus already keeps execution behind adapters. This change defines the policy contract that shells, process adapters, and future sandbox adapters must call before crossing capability boundaries.

## Decisions

### 1. Nickel owns rule authoring

**Choice:** Author Animus exec policy in typed Nickel contracts and export a checked, runtime-readable policy artifact.

**Rationale:** Nickel contracts provide type checking and merge semantics while avoiding another embedded rule language in the runtime.

### 2. Policy core evaluates parsed command intent

**Choice:** The pure core receives an argv-like command intent plus sandbox context and returns an allow, prompt, or forbid decision with matched rule evidence.

**Rationale:** Runtime behavior is deterministic and testable without spawning processes or reading files.

### 3. Shell splitting is deliberately narrow

**Choice:** Split only simple wrapper invocations made of plain words joined by safe chain operators. Unsupported shell syntax remains a single opaque command intent and receives the most restrictive matching decision.

**Rationale:** This preserves the useful Codex safety property without pretending to fully interpret shell.

### 4. Rule examples are executable validation cases

**Choice:** Each policy rule may include `match` and `not_match` examples that policy validation evaluates during config checks.

**Rationale:** Inline examples catch overbroad or underbroad command prefixes before the agent relies on them.

### 5. Decisions are observable but redacted

**Choice:** Runtime events record rule identity, decision, rationale, sandbox context, and command fingerprints, while leaving raw arguments subject to redaction and output bounds.

**Rationale:** Operators need audit evidence without turning receipts into secret stores.

## Risks / Trade-offs

- Narrow shell parsing may require approval more often, but fail-closed behavior is preferable for command execution.
- Prefix rules are simple by design and may not express every enterprise policy. More complex policies can be layered later through signed policy bundles.
- Nickel export drift must be checked in CI so runtime policy matches authored policy.
