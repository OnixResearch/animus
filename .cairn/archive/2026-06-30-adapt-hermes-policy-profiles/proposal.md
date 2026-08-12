# Proposal: Adapt Hermes policy profiles for Animus

## Why

Hermes uses a compact durable operating posture in `SOUL.md`: execute obvious non-destructive steps, continue through multi-step work, add durable prevention for repeated errors, run smallest relevant verification, write positive and negative tests, and keep functional-core / imperative-shell boundaries. Animus should adapt that posture as typed policy/profile data that hosts can compose into prompts and runtime guardrails without hard-coding one monolithic system prompt.

## What Changes

- Define policy/profile descriptors for goal-first instructions, success criteria, side-effect limits, validation requirements, persistence rules, and durable-prevention guidance.
- Add prompt assembly rules that preserve policy source attribution and deterministic precedence.
- Add validation for required safety constraints, overbroad absolute wording, prompt bloat, and missing validation/stop rules.
- Keep policy text host-supplied and versioned; reusable runtime consumes typed descriptors.
- Add fixtures for Hermes-style global posture and project-local policy overlays.

## Impact

- **Files**: future policy DTOs, prompt assembly core, policy validation tests, profile composition docs, and optional std adapters for config input.
- **Testing**: valid policy composition plus missing safety rules, conflicting side-effect limits, untrusted project policy, stale model guidance, and prompt-bloat rejection.
- **Non-claims**: this change does not embed Hermes `SOUL.md` verbatim, does not force one global prompt, and does not make policy descriptors a substitute for runtime enforcement.
