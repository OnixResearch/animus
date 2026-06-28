# Proposal: Adapt OpenAI Codex exec policy rules for Animus

## Why

OpenAI Codex separates technical sandbox boundaries from policy decisions about when commands may run outside those boundaries. Its command-prefix rules, approval policies, inline rule examples, and conservative shell splitting are useful safety patterns for Animus, which needs deterministic tool execution policy before a runtime can safely dispatch shell-like or process-backed adapters.

Animus should adapt the idea as an Animus-owned, VCS-neutral policy contract. Policy should be authored through typed Nickel configuration, validated before runtime use, and evaluated by a pure policy core over parsed command intent rather than by ad hoc shell strings.

## What Changes

- Define typed exec policy rules with allow, prompt, and forbid decisions, explicit rationales, and positive/negative command examples.
- Add a pure command-intent parser that can split only simple shell wrapper chains and fails closed for unsupported shell constructs.
- Bind exec policy evaluation to sandbox escalation requests so the runtime can decide whether to allow, prompt, or deny a command outside the current sandbox.
- Store rule and decision evidence in runtime events without embedding secrets or unbounded command output.
- Keep Codex's Starlark/TOML formats out of Animus; Nickel remains the source of truth for Animus policy.

## Impact

- **Files**: policy DTOs, Nickel contracts, parser/evaluator core, runtime event integration, shell-adapter tests, docs, and generated policy artifacts if needed.
- **Testing**: positive tests for matching allowed/prompted commands and simple shell splitting; negative tests for forbidden commands, unsafe shell wrappers, malformed rules, empty patterns, and prompt bypass attempts.
- **Non-claims**: this change does not add a concrete process executor, Linux/macOS sandbox implementation, or Codex-compatible `.rules` parser.
