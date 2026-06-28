# Proposal: Adapt Claude Code workflow commands for Animus

## Why

Claude Code's slash-command model makes repeatable workflows easy to invoke: review, test, explain, fix, commit, and similar tasks become named commands instead of one-off prompts. Animus already has skills and resource discovery work, but it needs a stricter command/workflow contract that can bind inputs, declare allowed tools, specify validation gates, and produce predictable outputs.

This change adapts the user-facing workflow ergonomics while keeping command expansion typed, policy-gated, and independent of any Claude-specific command file format.

## What Changes

- Define workflow command manifests with validated names, invocation schema, inputs, allowed capabilities, retrieval budget, validation gates, output contract, and provenance.
- Add deterministic command lookup and collision diagnostics across user, project, and host-provided command sources.
- Add pure argument binding and validation before any workflow prompt or tool dispatch occurs.
- Require workflows to declare side-effect posture and verification expectations.
- Add tests for valid commands, invalid names, duplicate commands, malformed input, unsafe side effects, missing validation gates, and untrusted project commands.

## Impact

- **Files**: future command manifest DTOs, command registry core, argument binding helpers, policy contracts, docs, tests, and optional std importers.
- **Testing**: positive command resolution/binding plus negative tests for duplicate names, unsafe manifests, untrusted project commands, missing output contracts, malformed arguments, and undeclared side effects.
- **Non-claims**: this change does not copy Claude Code's slash-command implementation, does not execute commands as shell scripts by default, does not open pull requests, and does not make command text a substitute for runtime policy enforcement.
